use chrono::{TimeZone, Utc};
use mailboar_backend::http::run_http_server;
use mailboar_backend::repository::{Message, MessageRepository};
use mailboar_backend::sse_clients::SseClients;
use reqwest::{Method, Response};
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;

fn create_repository_with_messages(messages: Vec<Message>) -> Arc<Mutex<MessageRepository>> {
    let mut repository = MessageRepository::new();

    for message in messages {
        repository.persist(message);
    }

    Arc::new(Mutex::new(repository))
}

async fn launch_test_http_server(
    repository: Arc<Mutex<MessageRepository>>,
    sse_clients: Arc<SseClients>,
    port: u16,
) -> JoinHandle<()> {
    let addr = SocketAddr::from_str(format!("127.0.0.1:{}", port).as_str()).unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tokio::spawn(async move {
        run_http_server(listener, repository.clone(), sse_clients.clone())
            .await
            .unwrap()
    })
}

async fn do_request(
    repository: Arc<Mutex<MessageRepository>>,
    sse_clients: Arc<SseClients>,
    method: Method,
    uri: &str,
) -> Response {
    let server = launch_test_http_server(repository, sse_clients, 62043).await;
    let client = reqwest::Client::new();
    let req = client
        .request(method, "http://127.0.0.1:62043".to_string() + uri)
        .build()
        .unwrap();
    let resp = client.execute(req).await.unwrap();

    server.abort();

    resp
}

#[tokio::test]
async fn test_http_server_is_reachable() {
    let repository = create_repository_with_messages(vec![Message {
        id: Some(1),
        size: 42,
        subject: Some("This is the subject".to_string()),
        sender: Some("sender@example.com".to_string()),
        recipients: vec!["recipient@example.com".to_string()],
        created_at: Utc.timestamp_opt(1431648000, 0).unwrap(),
        typ: "text/plain".to_string(),
        parts: vec![],
        charset: "UTF-8".to_string(),
        source: b"Subject: This is the subject\r\n\r\nHello world!\r\n".to_vec(),
    }]);

    let sse_clients = Arc::new(SseClients::new());

    let res = do_request(repository, sse_clients, Method::GET, "/messages").await;
    let json = res.json::<serde_json::Value>().await.unwrap();

    let expected = serde_json::json!([
        {
            "created_at": "2015-05-15T00:00:00+00:00",
            "id": 1,
            "recipients": ["recipient@example.com"],
            "sender": "sender@example.com",
            "size": "42",
            "subject": "This is the subject",
        }
    ]);
    assert_eq!(expected, json);
}

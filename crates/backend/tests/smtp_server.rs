use lettre::{AsyncSmtpTransport, AsyncTransport, Message as EmailMessage, Tokio1Executor};
use mailboar_backend::repository::MessageRepository;
use mailboar_backend::smtp::run_smtp_server;
use mailboar_backend::sse_clients::SseClients;
use std::net::{SocketAddr, TcpListener};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;

fn launch_test_smtp_server(
    repository: Arc<Mutex<MessageRepository>>,
    sse_clients: Arc<SseClients>,
    port: u16,
) -> JoinHandle<()> {
    let addr = SocketAddr::from_str(format!("127.0.0.1:{}", port).as_str()).unwrap();
    let tcp_listener = TcpListener::bind(addr).unwrap();

    tokio::spawn(async move {
        run_smtp_server(tcp_listener, repository.clone(), sse_clients.clone())
            .await
            .unwrap()
    })
}

async fn send_mail(
    repository: Arc<Mutex<MessageRepository>>,
    sse_clients: Arc<SseClients>,
    sender: &str,
    recipient: &str,
) {
    let port = 62044;
    let _server = launch_test_smtp_server(repository, sse_clients, port).await;

    let email = EmailMessage::builder()
        .from(sender.parse().unwrap())
        .to(recipient.parse().unwrap())
        .subject("Hello there")
        .body(String::from("Abc123"))
        .unwrap();

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous("127.0.0.1")
        .port(port)
        .build();

    mailer.send(email).await.unwrap();

    // TODO: abort the server?
    // server.abort();
}

#[tokio::test]
async fn test_smtp_server_is_reachable() {
    let repository = Arc::new(Mutex::new(MessageRepository::new()));
    let sse_clients = Arc::new(SseClients::new());

    send_mail(
        repository.clone(),
        sse_clients.clone(),
        "test@example.com",
        "recipient@example.com",
    )
    .await;

    let repository = repository.lock().unwrap();
    assert_eq!(1, repository.find_all().len());
    assert_eq!(
        Some("<test@example.com>".to_string()),
        repository.find(1).unwrap().sender
    );
}

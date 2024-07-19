use crate::repository::MessageRepository;
use axum::{
    extract::{Path, State},
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response, Sse},
    routing::{delete, get},
    Json, Router,
};
use futures::stream::Stream;
use log::info;
use serde::Serialize;
use std::io;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use crate::sse_clients::SseClients;

#[derive(Clone)]
struct AppState {
    repository: Arc<Mutex<MessageRepository>>,
    sse_clients: Arc<SseClients>,
}

fn router(repository: Arc<Mutex<MessageRepository>>, sse_clients: Arc<SseClients>) -> Router {
    let state = AppState {
        repository,
        sse_clients,
    };

    Router::new()
        .route("/events", get(sse_handler))
        .route("/messages/:id", delete(delete_message))
        .route("/messages/:id/json", get(get_message_json))
        .route("/messages/:id/source", get(get_message_source))
        .route("/messages/:id/html", get(get_message_html))
        .route("/messages/:id/eml", get(get_message_eml))
        .route("/messages/:id/plain", get(get_message_plain))
        .route("/messages/:id/parts/:cid", get(get_message_part))
        .route("/messages", get(get_messages).delete(delete_messages))
        .with_state(state)
}

pub async fn run_http_server(
    listener: TcpListener,
    repository: Arc<Mutex<MessageRepository>>,
    sse_clients: Arc<SseClients>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Starting HTTP server on {}", listener.local_addr().unwrap());

    let app = router(repository, sse_clients).layer(CorsLayer::permissive());

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[derive(Serialize)]
struct GetMessagesListItem {
    id: usize,
    sender: Option<String>,
    recipients: Vec<String>,
    subject: Option<String>,
    size: String,
    created_at: String,
}

async fn get_messages(
    State(state): State<AppState>,
) -> Result<Json<Vec<GetMessagesListItem>>, StatusCode> {
    let repository = &state.repository;

    let messages: Vec<GetMessagesListItem> = repository
        .lock()
        .unwrap()
        .find_all()
        .into_iter()
        .map(|message| GetMessagesListItem {
            id: message.id.unwrap(),
            sender: message.sender.clone(),
            recipients: message.recipients.clone(),
            subject: message.subject.clone(),
            size: message.size.to_string(),
            created_at: message.created_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(messages))
}

async fn delete_messages(State(state): State<AppState>) -> StatusCode {
    let repository = &state.repository;

    repository.lock().unwrap().delete_all();

    StatusCode::NO_CONTENT
}

#[derive(Serialize)]
struct GetMessage {
    id: usize,
    sender: Option<String>,
    recipients: Vec<String>,
    subject: Option<String>,
    size: String,

    #[serde(rename = "type")]
    ty: String,
    created_at: String,
    formats: Vec<String>,
    attachments: Vec<GetMessageAttachment>,
}

#[derive(Serialize)]
struct GetMessageAttachment {
    pub cid: String,
    #[serde(rename = "type")]
    pub typ: String,
    pub filename: String,
    pub size: usize,
    pub href: String,
}

async fn sse_handler(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<axum::response::sse::Event, io::Error>>> {
    let mut rx = state.sse_clients.tx.subscribe();

    let stream = async_stream::stream! {
        while let Ok(msg) = rx.recv().await {
            yield Ok(axum::response::sse::Event::default()
                .event("message")
                .data(msg));
        }
    };

    Sse::new(stream)
}

async fn get_message_json(
    Path(id): Path<usize>,
    State(state): State<AppState>,
) -> Result<Json<GetMessage>, StatusCode> {
    let repository = &state.repository;

    let message = repository
        .lock()
        .unwrap()
        .find(id)
        .map(|message| {
            let mut formats = vec!["source".to_string()];
            if message.html().is_some() {
                formats.push("html".to_string());
            }

            if message.plain().is_some() {
                formats.push("plain".to_string());
            }

            GetMessage {
                id,
                sender: message.sender.clone(),
                recipients: message.recipients.clone(),
                subject: message.subject.clone(),
                size: message.size.to_string(),
                ty: message.typ.clone(),
                created_at: message.created_at.to_rfc3339(),
                formats,
                attachments: message
                    .parts
                    .iter()
                    .filter(|p| p.is_attachment)
                    .map(|attachment| GetMessageAttachment {
                        cid: attachment.cid.clone(),
                        typ: attachment.typ.clone(),
                        filename: attachment.filename.clone(),
                        size: attachment.size,
                        href: format!("/messages/{}/parts/{}", id, attachment.cid),
                    })
                    .collect(),
            }
        })
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(message))
}

async fn get_message_html(
    Path(id): Path<usize>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let repository = &state.repository;

    let html_part = repository
        .lock()
        .unwrap()
        .find(id)
        .and_then(|message| message.html())
        .ok_or(StatusCode::NOT_FOUND)?
        .clone();

    let body = html_part.body.clone();
    let body_as_string = String::from_utf8(body).unwrap();

    let mut response = Response::new(body_as_string);
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(&format!("text/html; charset={}", html_part.charset)).unwrap(),
    );

    Ok(response)
}

async fn get_message_plain(
    Path(id): Path<usize>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let repository = &state.repository;

    let plain_part = repository
        .lock()
        .unwrap()
        .find(id)
        .and_then(|message| message.plain())
        .ok_or(StatusCode::NOT_FOUND)?
        .clone();

    let body = plain_part.body.clone();
    let body_as_string = String::from_utf8(body).unwrap();

    let mut response = Response::new(body_as_string);
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(&format!("text/plain; charset={}", plain_part.charset)).unwrap(),
    );

    Ok(response)
}

async fn get_message_source(
    Path(id): Path<usize>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let repository = &state.repository;

    let message = repository
        .lock()
        .unwrap()
        .find(id)
        .ok_or(StatusCode::NOT_FOUND)?
        .clone();

    let source = message.source.clone();
    let source_as_string = String::from_utf8(source).unwrap();

    let mut response = Response::new(source_as_string);
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(&format!("text/plain; charset={}", message.charset)).unwrap(),
    );

    Ok(response)
}

async fn get_message_eml(
    Path(id): Path<usize>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let repository = &state.repository;

    let message = repository
        .lock()
        .unwrap()
        .find(id)
        .ok_or(StatusCode::NOT_FOUND)?
        .clone();

    let source = message.source.clone();
    let source_as_string = String::from_utf8(source).unwrap();

    let mut response = Response::new(source_as_string);
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(&format!("message/rfc822; charset={}", message.charset)).unwrap(),
    );

    Ok(response)
}

async fn get_message_part(
    Path((id, cid)): Path<(usize, String)>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let repository = &state.repository;

    let part = repository
        .lock()
        .unwrap()
        .find(id)
        .and_then(|message| message.parts.iter().find(|part| part.cid.as_str() == cid))
        .ok_or(StatusCode::NOT_FOUND)?
        .clone();

    let body = part.body.clone();
    let body_as_string = String::from_utf8(body).unwrap();

    let mut response = Response::new(body_as_string);
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(&format!("{}; charset={}", part.typ, part.charset)).unwrap(),
    );

    if part.is_attachment {
        response.headers_mut().insert(
            header::CONTENT_DISPOSITION,
            HeaderValue::from_str(&format!("attachment; filename=\"{}\"", part.filename)).unwrap(),
        );
    }

    Ok(response)
}

async fn delete_message(Path(id): Path<usize>, State(state): State<AppState>) -> StatusCode {
    let repository = &state.repository;

    if repository.lock().unwrap().delete(id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

#[cfg(test)]
mod tests {
    use crate::http::router;
    use crate::repository::{Message, MessagePart, MessageRepository};
    use crate::sse_clients::SseClients;
    use axum::body;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use chrono::{TimeZone, Utc};
    use std::sync::{Arc, Mutex};
    use tower::ServiceExt;

    async fn body_to_string(body: Body) -> String {
        return String::from_utf8(body::to_bytes(body, usize::MAX).await.unwrap().to_vec())
            .unwrap();
    }

    async fn body_to_json(body: Body) -> serde_json::Value {
        return serde_json::from_str(body_to_string(body).await.as_str()).unwrap();
    }

    fn create_test_message() -> Message {
        Message {
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
        }
    }

    #[tokio::test]
    async fn test_get_messages_returns_messages_in_repository() {
        let repository = Arc::new(Mutex::new(MessageRepository::new()));
        repository.lock().unwrap().persist(create_test_message());

        let sse_clients = Arc::new(SseClients::new());

        let app = router(repository, sse_clients);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/messages")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body();

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
        assert_eq!(expected, body_to_json(body).await);
    }

    #[tokio::test]
    async fn test_delete_messages_clears_repository() {
        let repository = Arc::new(Mutex::new(MessageRepository::new()));
        repository.lock().unwrap().persist(create_test_message());

        let sse_clients = Arc::new(SseClients::new());
        let app = router(repository.clone(), sse_clients);

        let response = app
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri("/messages")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
        assert_eq!(repository.lock().unwrap().find_all().len(), 0);
    }

    #[tokio::test]
    async fn test_get_message_json() {
        let repository = Arc::new(Mutex::new(MessageRepository::new()));
        repository.lock().unwrap().persist(create_test_message());

        let sse_clients = Arc::new(SseClients::new());
        let app = router(repository, sse_clients);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/messages/1/json")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body();
        let json = body_to_json(body).await;

        assert_eq!(json["id"], 1);
        assert_eq!(json["subject"], "This is the subject");
        assert_eq!(json["sender"], "sender@example.com");
    }

    #[tokio::test]
    async fn test_get_message_html() {
        let mut message = create_test_message();
        message.parts.push(MessagePart {
            cid: "html".to_string(),
            typ: "text/html".to_string(),
            is_attachment: false,
            filename: "".to_string(),
            charset: "UTF-8".to_string(),
            body: "<html><body>Hello world!</body></html>".as_bytes().to_vec(),
            size: 38,
        });

        let repository = Arc::new(Mutex::new(MessageRepository::new()));
        repository.lock().unwrap().persist(message);

        let sse_clients = Arc::new(SseClients::new());
        let app = router(repository, sse_clients);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/messages/1/html")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body();
        assert_eq!(
            body_to_string(body).await,
            "<html><body>Hello world!</body></html>"
        );
    }

    #[tokio::test]
    async fn test_get_message_plain() {
        let mut message = create_test_message();
        message.parts.push(MessagePart {
            cid: "plain".to_string(),
            typ: "text/plain".to_string(),
            is_attachment: false,
            filename: "".to_string(),
            charset: "UTF-8".to_string(),
            body: "Hello world!".as_bytes().to_vec(),
            size: 12,
        });

        let repository = Arc::new(Mutex::new(MessageRepository::new()));
        repository.lock().unwrap().persist(message);

        let sse_clients = Arc::new(SseClients::new());
        let app = router(repository, sse_clients);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/messages/1/plain")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body();
        assert_eq!(body_to_string(body).await, "Hello world!");
    }

    #[tokio::test]
    async fn test_get_message_source() {
        let repository = Arc::new(Mutex::new(MessageRepository::new()));
        repository.lock().unwrap().persist(create_test_message());

        let sse_clients = Arc::new(SseClients::new());
        let app = router(repository, sse_clients);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/messages/1/source")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body();
        assert_eq!(
            body_to_string(body).await,
            "Subject: This is the subject\r\n\r\nHello world!\r\n"
        );
    }

    #[tokio::test]
    async fn test_get_message_eml() {
        let repository = Arc::new(Mutex::new(MessageRepository::new()));
        repository.lock().unwrap().persist(create_test_message());

        let sse_clients = Arc::new(SseClients::new());
        let app = router(repository, sse_clients);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/messages/1/eml")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let status = response.status().clone();
        let headers = response.headers().clone();
        let body = response.into_body();
        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            body_to_string(body).await,
            "Subject: This is the subject\r\n\r\nHello world!\r\n"
        );
        assert_eq!(
            headers.get("Content-Type").unwrap(),
            "message/rfc822; charset=UTF-8"
        );
    }

    #[tokio::test]
    async fn test_get_message_part() {
        let mut message = create_test_message();
        message.parts.push(MessagePart {
            cid: "attachment".to_string(),
            typ: "application/pdf".to_string(),
            is_attachment: true,
            filename: "test.pdf".to_string(),
            charset: "UTF-8".to_string(),
            body: vec![1, 2, 3, 4],
            size: 4,
        });

        let repository = Arc::new(Mutex::new(MessageRepository::new()));
        repository.lock().unwrap().persist(message);

        let sse_clients = Arc::new(SseClients::new());
        let app = router(repository, sse_clients);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/messages/1/parts/attachment")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let headers = response.headers().clone();
        let body = response.into_body();
        assert_eq!(
            body::to_bytes(body, usize::MAX).await.unwrap().to_vec(),
            vec![1, 2, 3, 4]
        );
        assert_eq!(
            headers.get("Content-Type").unwrap(),
            "application/pdf; charset=UTF-8"
        );
        assert_eq!(
            headers.get("Content-Disposition").unwrap(),
            "attachment; filename=\"test.pdf\""
        );
    }

    #[tokio::test]
    async fn test_delete_message() {
        let repository = Arc::new(Mutex::new(MessageRepository::new()));
        repository.lock().unwrap().persist(create_test_message());

        let sse_clients = Arc::new(SseClients::new());
        let app = router(repository.clone(), sse_clients);

        let response = app
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri("/messages/1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
        assert_eq!(repository.lock().unwrap().find(1), None);
    }
}

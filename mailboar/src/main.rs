use crate::html_template::HtmlTemplate;
use crate::options::Options;
use crate::templates::index::IndexTemplate;
use axum::extract::State;
use axum::handler::HandlerWithoutStateExt;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use std::error::Error;
use std::net::{Ipv4Addr, SocketAddr, TcpListener};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use structopt::StructOpt;
use tiny_mailcatcher::repository::MessageRepository;
use tiny_mailcatcher::{http, smtp};
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod asset;
mod html_template;
mod options;
mod templates;

struct AppState {
    api_url: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "mailboar=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args: Options = Options::from_args();

    // Shared state
    let api_url = args.api_url;
    let state = Arc::new(AppState { api_url });

    let repository = Arc::new(Mutex::new(MessageRepository::new()));

    tracing::info!("Mailboar is starting");

    // Start API
    let api_address = format!("{}:{}", &args.ip, args.api_port);
    let api_listener = TcpListener::bind(&api_address).unwrap();
    let api_handle = tokio::spawn(http::run_http_server(api_listener, repository.clone()));
    tracing::debug!("API listening on {}", api_address);

    // Start SMTP
    let smtp_address = format!("{}:{}", &args.ip, args.smtp_port);
    let smtp_listener = TcpListener::bind(&smtp_address).unwrap();
    let smtp_handle = tokio::spawn(smtp::run_smtp_server(smtp_listener, repository.clone()));
    tracing::debug!("SMTP listening on {}", smtp_address);

    // Start frontend
    let service = handle_404.into_service();
    let serve_dir = ServeDir::new("../../static").not_found_service(service);

    let app = Router::new()
        .route("/", get(index))
        .nest_service("/static", serve_dir.clone())
        .fallback_service(serve_dir)
        .fallback(get(index))
        .with_state(state);

    let ip = Ipv4Addr::from_str(&args.ip)?;
    let addr = SocketAddr::from((ip, args.http_port));
    tracing::debug!("Frontend listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    #[allow(clippy::let_unit_value)]
    let res = axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
    let http_handle = tokio::spawn(async move { res });

    // Wait for all tasks to finish
    let (api_res, smtp_res, http_res) = tokio::try_join!(api_handle, smtp_handle, http_handle)?;

    api_res.and(smtp_res).and(Ok(http_res))
}

// Start Frontend
async fn handle_404() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not found")
}

async fn index(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let api_url = state.api_url.clone();
    let template = IndexTemplate { api_url };

    HtmlTemplate(template)
}
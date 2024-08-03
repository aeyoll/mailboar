use crate::options::Options;
use axum::extract::State;
use axum::handler::HandlerWithoutStateExt;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use mailboar_backend::repository::MessageRepository;
use mailboar_backend::sse_clients::SseClients;
use mailboar_backend::{http, smtp};
use mailboar_frontend::asset::set_assets_path;
use mailboar_frontend::html_template::HtmlTemplate;
use mailboar_frontend::templates::index::IndexTemplate;
use std::error::Error;
use std::process;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use structopt::StructOpt;
use tokio::signal;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod options;

struct AppState {
    api_url: String,
}

async fn run_frontend_server(
    listener: tokio::net::TcpListener,
    api_url: String,
    assets_path: String,
    token: CancellationToken,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    tracing::info!(
        "Starting Frontend HTTP server on http://{}",
        listener.local_addr().unwrap()
    );

    let state = Arc::new(AppState { api_url });

    let service = handle_404.into_service();
    set_assets_path(&assets_path.to_string());
    let serve_dir = ServeDir::new(assets_path).not_found_service(service);

    let app: Router = Router::new()
        .route("/", get(index))
        .nest_service("/static", serve_dir.clone())
        .fallback_service(serve_dir)
        .fallback(get(index))
        .with_state(state);

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(Box::leak(Box::new(token)).cancelled())
        .await
        .unwrap();

    Ok(())
}

async fn run() -> i32 {
    let args: Options = Options::from_args();

    let repository = Arc::new(Mutex::new(MessageRepository::new()));
    let sse_clients = Arc::new(SseClients::new());

    tracing::info!("Mailboar is starting");

    let token = CancellationToken::new();
    let abort_token = CancellationToken::new();
    let mut set = JoinSet::new();

    // Start API
    let api_address = format!("{}:{}", &args.ip, &args.api_port);
    let api_listener = tokio::net::TcpListener::bind(&api_address).await.unwrap();
    let api_handle = http::run_http_server(
        api_listener,
        repository.clone(),
        sse_clients.clone(),
        token.clone(),
    );

    // Start SMTP
    let smtp_address = format!("{}:{}", &args.ip, &args.smtp_port);
    let smtp_listener = tokio::net::TcpListener::bind(&smtp_address).await.unwrap();
    let smtp_handle = smtp::run_smtp_server(
        smtp_listener,
        repository.clone(),
        sse_clients.clone(),
        token.clone(),
    );

    // Start frontend
    let frontend_address = format!("{}:{}", &args.ip, &args.http_port);
    let frontend_listener = tokio::net::TcpListener::bind(&frontend_address)
        .await
        .unwrap();
    let frontend_handle = run_frontend_server(
        frontend_listener,
        args.api_url,
        args.assets_path,
        token.clone(),
    );

    set.spawn(api_handle);
    set.spawn(smtp_handle);
    set.spawn(frontend_handle);

    tokio::spawn({
        let abort_token = abort_token.clone();
        async move {
            shutdown_signal().await;
            tracing::info!("Received shutdown signal");
            token.cancel();
            tokio::time::sleep(Duration::from_secs(5)).await;
            abort_token.cancel();
        }
    });

    loop {
        tokio::select! {
            r = set.join_next() => match r {
                Some(Ok(_)) => {},
                Some(Err(e)) => tracing::error!("{e}"),
                None => {
                    tracing::info!("Graceful shutdown successful");

                    return 0;
                },
            },
            _ = abort_token.cancelled() => {
                set.abort_all();
                tracing::error!("Service aborted");

                return 1;
            }
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "mailboar=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let exit_code = run().await;

    process::exit(exit_code);
}

// Start Frontend
async fn handle_404() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not found")
}

async fn index(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let api_url = state.api_url.clone();
    let version = env!("CARGO_PKG_VERSION").to_string();
    let template = IndexTemplate { api_url, version };

    HtmlTemplate(template)
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

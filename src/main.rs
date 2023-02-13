use crate::options::Options;
use crate::templates::index::IndexTemplate;
use askama::Template;
use axum::extract::State;
use axum::response::{Html, Response};
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};
use log::info;
use std::error::Error;
use std::io;
use std::net::{Ipv4Addr, SocketAddr, TcpListener};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use structopt::StructOpt;
use tiny_mailcatcher::repository::MessageRepository;
use tiny_mailcatcher::{http, smtp};
use tower_http::services::ServeDir;

mod asset;
mod options;
mod templates;

struct AppState {
    api_url: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    env_logger::init();

    let args: Options = Options::from_args();

    // Shared state
    let api_url = args.api_url;
    let state = Arc::new(AppState { api_url });

    let repository = Arc::new(Mutex::new(MessageRepository::new()));

    info!("Mailboar is starting");

    // Start API
    let api_address = format!("{}:{}", &args.ip, args.api_port);
    let api_listener = TcpListener::bind(&api_address).unwrap();
    let api_handle = tokio::spawn(http::run_http_server(api_listener, repository.clone()));

    // Start SMTP
    let smtp_address = format!("{}:{}", &args.ip, args.smtp_port);
    let smtp_listener = TcpListener::bind(smtp_address).unwrap();
    let smtp_handle = tokio::spawn(smtp::run_smtp_server(smtp_listener, repository.clone()));

    // Start Frontend
    let serve_dir = get_service(ServeDir::new("static")).handle_error(handle_error);

    let app = Router::new()
        .route("/", get(index))
        .nest_service("/static", serve_dir.clone())
        .fallback_service(serve_dir)
        .fallback(get(index))
        .with_state(state);

    let ip = Ipv4Addr::from_str(&args.ip)?;
    let addr = SocketAddr::from((ip, args.http_port));
    #[allow(clippy::let_unit_value)]
    let res = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    let http_handle = tokio::spawn(async move { res });

    let (api_res, smtp_res, http_res) = tokio::try_join!(api_handle, smtp_handle, http_handle)?;

    api_res.and(smtp_res).and(Ok(http_res))
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

async fn index(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let api_url = state.api_url.clone();
    let template = IndexTemplate { api_url };

    HtmlTemplate(template)
}

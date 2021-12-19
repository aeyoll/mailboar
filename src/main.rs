use askama::Template;
use log::info;
use std::error::Error;
use std::net::{IpAddr, TcpListener};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use structopt::StructOpt;
use tiny_mailcatcher::repository::MessageRepository;
use tiny_mailcatcher::{http, smtp};
use warp::Filter;

use crate::templates::index::IndexTemplate;

mod asset;
mod templates;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    env_logger::init();

    let args: Options = Options::from_args();

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
    let index = warp::any().map(move || {
        let template = IndexTemplate {
            api_address: &api_address,
        };
        warp::http::Response::builder()
            .body(template.render().unwrap())
            .unwrap()
    });
    let static_dir = warp::path("static")
        .and(warp::fs::dir("static"))
        .with(warp::compression::gzip());

    let routes = static_dir.or(index);

    let addr = IpAddr::from_str(&args.ip)?;
    let res = warp::serve(routes).run((addr, args.http_port)).await;
    let http_handle = tokio::spawn(async move { res });

    let (api_res, smtp_res, http_res) = tokio::try_join!(api_handle, smtp_handle, http_handle)?;

    api_res.and(smtp_res).and(Ok(http_res))
}

#[derive(Debug, StructOpt)]
#[structopt(name = "mailboar", about)]
struct Options {
    #[structopt(long, default_value = "127.0.0.1")]
    ip: String,

    #[structopt(long, name = "smtp-port", default_value = "1025")]
    smtp_port: u16,

    #[structopt(long, name = "api-port", default_value = "1080")]
    api_port: u16,

    #[structopt(long, name = "http-port", default_value = "8025")]
    http_port: u16,
}

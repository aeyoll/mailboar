use log::info;
use std::error::Error;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use structopt::StructOpt;
use tiny_mailcatcher::{http, smtp};
use tiny_mailcatcher::repository::MessageRepository;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    env_logger::init();

    let args: Options = Options::from_args();

    let repository = Arc::new(Mutex::new(MessageRepository::new()));

    info!("Mailboar is starting");
    let api_address = format!("{}:{}", &args.ip, args.api_port);
    let api_listener = TcpListener::bind(api_address).unwrap();
    let api_handle = tokio::spawn(http::run_http_server(api_listener, repository.clone()));

    let smtp_address = format!("{}:{}", &args.ip, args.smtp_port);
    let smtp_listener = TcpListener::bind(smtp_address).unwrap();
    let smtp_handle = tokio::spawn(smtp::run_smtp_server(smtp_listener, repository.clone()));

    let (api_res, smtp_res) = tokio::try_join!(api_handle, smtp_handle)?;

    api_res.and(smtp_res)
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
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "mailboar", about)]
pub struct Options {
    #[structopt(long, default_value = "127.0.0.1")]
    pub ip: String,

    #[structopt(long, default_value = "http://127.0.0.1:1080")]
    pub api_url: String,

    #[structopt(long, name = "smtp-port", default_value = "1025")]
    pub smtp_port: u16,

    #[structopt(long, name = "api-port", default_value = "1080")]
    pub api_port: u16,

    #[structopt(long, name = "http-port", default_value = "8025")]
    pub http_port: u16,

    #[structopt(long, name = "assets-path", default_value = "crates/frontend/static")]
    pub assets_path: String,
}

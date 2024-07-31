use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub api_url: String,
    pub version: String,
}

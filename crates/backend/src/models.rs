use serde::Serialize;

#[derive(Serialize)]
pub struct GetMessagesListItem {
    pub id: usize,
    pub sender: Option<String>,
    pub recipients: Vec<String>,
    pub subject: Option<String>,
    pub size: String,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct GetMessage {
    pub id: usize,
    pub sender: Option<String>,
    pub recipients: Vec<String>,
    pub subject: Option<String>,
    pub size: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub created_at: String,
    pub formats: Vec<String>,
    pub attachments: Vec<GetMessageAttachment>,
}

#[derive(Serialize)]
pub struct GetMessageAttachment {
    pub cid: String,
    #[serde(rename = "type")]
    pub typ: String,
    pub filename: String,
    pub size: usize,
    pub href: String,
}

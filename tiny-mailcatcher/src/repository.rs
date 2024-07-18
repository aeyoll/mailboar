use std::collections::HashMap;

use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq)]
pub struct Message {
    pub id: Option<usize>,
    pub size: usize,
    pub subject: Option<String>,
    pub sender: Option<String>,
    pub recipients: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub typ: String,
    pub parts: Vec<MessagePart>,
    pub charset: String,
    pub source: Vec<u8>,
}

impl Message {
    pub fn plain(&self) -> Option<&MessagePart> {
        return self.parts.iter().find(|&p| p.typ == "text/plain");
    }

    pub fn html(&self) -> Option<&MessagePart> {
        return self
            .parts
            .iter()
            .find(|&p| p.typ == "text/html" || p.typ == "application/xhtml+xml");
    }
}

#[derive(Debug, PartialEq)]
pub struct MessagePart {
    pub cid: String,
    pub typ: String,
    pub filename: String,
    pub size: usize,
    pub charset: String,
    pub body: Vec<u8>,
    pub is_attachment: bool,
}

pub struct MessageRepository {
    last_insert_id: usize,
    messages: HashMap<usize, Message>,
}

impl MessageRepository {
    pub fn new() -> Self {
        MessageRepository {
            last_insert_id: 0,
            messages: HashMap::new(),
        }
    }

    pub fn persist(&mut self, mut message: Message) {
        let id = self.last_insert_id + 1;
        self.last_insert_id += 1;
        message.id = Some(id);
        self.messages.insert(id, message);
    }

    pub fn find_all(&self) -> Vec<&Message> {
        self.messages.values().collect()
    }

    pub fn find(&self, id: usize) -> Option<&Message> {
        self.messages.get(&id)
    }

    pub fn delete_all(&mut self) {
        self.messages.clear();
        self.last_insert_id = 0;
    }

    pub fn delete(&mut self, id: usize) -> Option<Message> {
        self.messages.remove(&id)
    }
}

use std::fmt::Display;

pub struct Event {
    event_type: Option<String>,
    data: String,
    id: Option<String>,
    retry: Option<u64>,
}

impl Event {
    pub(crate) fn new() -> Self {
        Event {
            event_type: None,
            data: String::new(),
            id: None,
            retry: None,
        }
    }

    pub(crate) fn event_type(mut self, event_type: &str) -> Self {
        self.event_type = Some(event_type.to_string());
        self
    }

    pub(crate) fn data(mut self, data: &str) -> Self {
        self.data = data.to_string();
        self
    }

    fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    fn retry(mut self, retry: u64) -> Self {
        self.retry = Some(retry);
        self
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = Vec::new();

        if let Some(event_type) = &self.event_type {
            lines.push(format!("event: {}", event_type));
        }

        for data_line in self.data.lines() {
            lines.push(format!("data: {}", data_line));
        }

        if let Some(id) = &self.id {
            lines.push(format!("id: {}", id));
        }

        if let Some(retry) = &self.retry {
            lines.push(format!("retry: {}", retry));
        }

        lines.push(String::new());
        write!(f, "{}", lines.join("\n"))
    }
}

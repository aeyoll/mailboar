use tokio::sync::broadcast;

pub struct SseClients {
    pub tx: broadcast::Sender<String>,
}

impl SseClients {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(100);
        SseClients { tx }
    }

    pub fn send(&self, message: String) {
        let _ = self.tx.send(message);
    }
}

use tokio::sync::mpsc;

pub struct EventQueue {
    sender: mpsc::Sender<Event>,
}

pub struct Event {
    pub function_name: String,
    pub payload: String,
}

impl EventQueue {
    pub fn new(buffer_size: usize) -> (Self, mpsc::Receiver<Event>) {
        let (sender, receiver) = mpsc::channel(buffer_size);
        (Self { sender }, receiver)
    }

    pub async fn enqueue(&self, event: Event) {
        self.sender.send(event).await.unwrap();
    }
}

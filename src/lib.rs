use tokio::sync::{mpsc, oneshot};

struct Actor {
    receiver: mpsc::Receiver<ActorMessage>,
    next_id: u32,
}

enum ActorMessage {
    GetUniqueId { respond_to: oneshot::Sender<u32> },
}

impl Actor {
    fn new(receiver: mpsc::Receiver<ActorMessage>) -> Self {
        Self {
            receiver,
            next_id: 0,
        }
    }

    fn handle_message(&mut self, msg: ActorMessage) {
        match msg {
            ActorMessage::GetUniqueId { respond_to } => {
                self.next_id += 1;
                let _ = respond_to.send(self.next_id);
            }
        }
    }
}

async fn run_actor(mut actor: Actor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg);
    }
}

#[derive(Clone)]
pub struct ActorHandle {
    sender: mpsc::Sender<ActorMessage>,
}

impl ActorHandle {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = Actor::new(receiver);
        tokio::spawn(run_actor(actor));

        Self { sender }
    }

    pub async fn get_unique_id(&self) -> u32 {
        let (sender, receiver) = oneshot::channel();
        let msg = ActorMessage::GetUniqueId { respond_to: sender };
        let _ = self.sender.send(msg).await;
        receiver.await.expect("Actor task has been killed")
    }
}

impl Default for ActorHandle {
    fn default() -> Self {
        Self::new()
    }
}

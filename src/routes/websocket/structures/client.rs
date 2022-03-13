use serde_json::Value;
use tokio::sync::mpsc::UnboundedSender;
use warp::ws::{Message};

pub struct Interaction {
    pub data: Value
}


pub struct ClientWs {
    pub _id: String,
    pub(crate) tx: UnboundedSender<Message>,
}

impl ClientBot {
    #[allow(dead_code)]
    fn update_id(&self) -> &String { &self.ws._id }
    #[allow(dead_code)]
    fn update_tx(&self) -> &UnboundedSender<Message> { &self.ws.tx }
}


pub struct ClientBot {
    pub ws: ClientWs
}

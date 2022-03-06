
use std::collections::HashMap;
use std::sync::Arc;
use crossbeam::sync::WaitGroup;
use futures::stream::{SplitSink, SplitStream};
use serde_json::Value;
use tokio::sync::{mpsc, RwLock};
use tokio::sync::mpsc::UnboundedSender;
use warp::ws::{Message, WebSocket};
use derive_more::Display;
use tokio_stream::wrappers::UnboundedReceiverStream;

pub struct Interaction {
    pub data: Value
}


pub struct ClientWs {
    pub _id: String,
    pub(crate) tx: UnboundedSender<Message>,
}

impl ClientBot {
    fn update_id(&self) -> &String { &self.ws._id }
    fn update_tx(&self) -> &UnboundedSender<Message> { &self.ws.tx }
}


pub struct ClientBot {
    pub ws: ClientWs
}

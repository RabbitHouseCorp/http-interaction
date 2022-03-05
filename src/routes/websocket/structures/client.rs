
use std::collections::HashMap;
use std::sync::Arc;
use futures::stream::{SplitSink, SplitStream};
use serde_json::Value;
use tokio::sync::{mpsc, RwLock};
use tokio::sync::mpsc::UnboundedSender;
use warp::ws::{Message, WebSocket};
use derive_more::Display;
use tokio_stream::wrappers::UnboundedReceiverStream;


pub struct ClientBot {
    pub _id: String,
    pub tx: UnboundedSender<Message>,
}

impl ClientBot {
    fn _id(&self) -> &String { &self._id }
    fn tx(&self) -> &UnboundedSender<Message> { &self.tx }
}
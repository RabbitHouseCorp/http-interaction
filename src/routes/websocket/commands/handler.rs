use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::io::Read;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use futures::sink::Close;
use futures::SinkExt;
use futures::stream::{SplitSink, SplitStream};
use rand::distributions::uniform::SampleBorrow;
use serde::Deserializer;
use serde_json::{json, Value};
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::RwLock;
use warp::body::json;
use warp::ws::{Message, WebSocket};
use crate::{ClientBot, Clients, Interaction};
use crate::routes::websocket::websocket_server::{convert_to_binary, send_metadata};

pub async fn load_commands(data: Value, mut tx: &UnboundedSender<Message>, clients: &mut Clients, id: String, interactions: Arc<RwLock<HashMap<String, Interaction>>>) {
    let type_command_is_none= data["type"].as_u64().is_none();
    if type_command_is_none {
        return;
    }
    let type_command= data["type"].as_u64().unwrap().to_owned();

    // Confirm metadata.
    if type_command == 0 {
        tx.send(Message::binary(convert_to_binary(&json!({
                            "type": 2,
                            "data": {
                                 "bits": 1 >> 22,
                                 "_id": id,
                                 "compress_data": true
                            },
                        }))));
        return;
    }

    if type_command == 89 {
        tx.send(Message::binary(convert_to_binary(&json!({
                            "type": 200,
                            "data": {}
                        }))));
    }

    if type_command == 10002 {
        if data["id"].as_str().is_none() {
            return;
        }
        tokio::task::spawn(async move {
            interactions.write().await.insert(data["id"].to_string(), Interaction {
                data: data["data"].clone(),
            });
            tokio::time::sleep(Duration::from_secs(4)).await; // Remove cache
            interactions.write().await.remove(&*data["id"].to_string());
        });
    }

}
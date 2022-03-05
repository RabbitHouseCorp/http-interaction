use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::io::Read;
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
use crate::{ClientBot, Clients};
use crate::routes::websocket::websocket_server::{convert_to_binary, send_metadata};

pub fn load_commands(data: Value, mut tx: &UnboundedSender<Message>, clients: &mut Clients, id: String) {
    let type_command_is_none= data["type"].as_u64().is_none();
    if type_command_is_none {
        return;
    }
    let type_command= data["type"].as_u64().unwrap();
    match type_command {
        // Confirm metadata.
        0 => {
            tx.send(Message::binary(convert_to_binary(&json!({
                            "type": 2,
                            "data": {
                                 "bits": 1 >> 22,
                                 "_id": id,
                                 "compress_data": true
                            },
                        }))));

        }
        _ => {

        }
    }
    async {




    };

}
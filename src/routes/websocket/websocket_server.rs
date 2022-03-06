use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::fmt::Debug;
use warp;
use warp::ws::{Message, WebSocket};
use futures::{SinkExt, StreamExt};
use futures::FutureExt;
use futures::stream::{SplitSink, SplitStream};
use serde_json::{json, Value};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use crate::{ClientBot, Clients, TryFutureExt};
use std::io::prelude::*;
use ed25519_dalek::{SecretKey, Sha512, SignatureError};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use rustc_serialize::json;
use tokio::sync::mpsc::UnboundedSender;
use tracing::instrument::WithSubscriber;
use warp::body::json;
use warp::Error;
use crate::cryptography::encode::encode_data;
use crate::routes::websocket::commands::handler::load_commands;
use crate::routes::websocket::structures::client::ClientWs;

pub async fn websocket_message(ws: WebSocket, mut clients: Clients, id: String, secret: String, shard_in: usize, shard_total: usize) {
    let (mut tx_client, mut rx_client) = ws.split();
    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);
    let msg = "WARNING: THIS API IS FULLY LIMITED FOR SYNC REASONS SOMETIMES THE INTERACTION MAY TAKE TIME BECAUSE OF THE FOREGOING OR BAND BAND WHY MAY DELAY THE ENCRYPTED DATA LATTERY.";
    let rate = "API THERE ARE LIMITATIONS ON SENDING OVERCOMMANDS. TAKE CARE BEFORE SENDING MULTIPLE COMMANDS.";
    let (id_client) = id.clone();
    let found_client = clients.read().await.get(&id_client).is_none();
    if shard_in > shard_total {
        let inf = &json!({"type": 0, "possible_error": true, "message": "Excuse me! I'm terminating the connection due to too many shards.", "data": {}, "rate_limit": true});
        tx_client.send(Message::binary(convert_to_binary(inf)).clone()).await;
        tx_client.send(Message::close().clone()).await;
        return;}



    tx_client.send(Message::binary(convert_to_binary(&json!({
        "type": 1,
        "possible_error": false,
        "message": "",
        "data": {},
        "rate_limit": false
    })))).await;
    match found_client {
        true => {
            let mut client = ClientBot {
               ws: ClientWs {
                   _id: id.clone(),
                   tx: tx.clone()
               }
            };
            clients.write().await.insert(id_client, client);

        }
        false => {
            let mut client = ClientBot {
                ws: ClientWs {
                    _id: id.clone(),
                    tx: tx.clone()
                }
            };
            clients.write().await.remove(id.clone().as_str());
            clients.write().await.insert(id_client, client);

        }
    }
    let a = tx;


    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            tx_client
                .send(message)
                .unwrap_or_else(|e| {
                    eprintln!("websocket send error: {}", e);
                })
                .await;
        }
    });



    while let Some(result) = rx_client.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error {}", e);
                break;
            }
        };
        message_interface(msg, &a, id.clone(), clients.clone());
    }
}


fn move_value(tx_client: SplitSink<WebSocket, Message>) -> SplitSink<WebSocket, Message> {
    let mut muttx = tx_client;
    muttx
}

fn search_shard(guild_id: usize) -> i32 {
    return ((guild_id >> 22) % 2) as i32
}

pub fn encrypt_data_str(inf: String) -> (String, Sha512, Result<SecretKey, SignatureError>) {
    let (data, sha, key) = encode_data(String::from("testing"), inf);
    return (data, sha, key)
}

pub async fn send_metadata(mut tx_client: SplitSink<WebSocket, Message>, x: &Value) -> Result<(), Error> {
    tx_client.send(Message::binary(convert_to_binary(x)).clone()).await
}

pub fn txclient(client: &SplitSink<WebSocket, Message>) -> &SplitSink<WebSocket, Message> {
    client
}

pub fn convert_to_binary(inf: &Value) -> Vec<u8> {
    let mut data = ZlibEncoder::new(Vec::new(), Compression::default());
    data.write_all(inf.to_string().as_ref());
    return data.finish().unwrap()
}

fn message_interface(message: Message, mut x: &UnboundedSender<Message>, id: String, mut client: Clients) {
    let tx = x;
    let message = if let Ok(a) = message.to_str()
    { a } else { return; };
    if message == "" {
        return;
    }

    let data: Value = serde_json::from_str(&message.to_string()).unwrap();

    load_commands(data, tx, client.borrow_mut(), id.clone());
}
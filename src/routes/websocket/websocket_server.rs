use crate::routes::websocket::commands::handler::load_commands;
use crate::{ClientBot, Clients, Interaction, Interactions};
use flate2::read::GzDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use futures::future::err;
use futures::{SinkExt, StreamExt, TryFutureExt};
use serde_json::{json, Value};
use std::borrow::{Borrow, BorrowMut};
use std::collections::{HashMap, HashSet};
use std::io;
use std::io::prelude::*;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::{mpsc, RwLock};
use tokio::sync::mpsc::error::SendError;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tracing::log::error;
use tracing::log::Level::Error;
use warp;
use warp::ws::{Message, WebSocket};

pub async fn websocket_message(
    ws: WebSocket,
    clients: Clients,
    id: String,
    secret: String,
    shard_in: usize,
    shard_total: usize,
    interactions: Interactions,
    x: (String, String, String, String),
) {
    let (pub_key, secret_key, pub_key_discord, bot_discord) = x;
    let mut check = 0;
    if secret_key.as_str() == secret {
        check += 1
    }
    let keys = pub_key.split(" ");
    for key in keys {
        if pub_key_discord.to_string() == key.to_string() {
            check += 1
        }
    }
    let keys_bot = pub_key.split(" ");
    for _key in keys_bot {
        if bot_discord.to_string() == id.to_string() {
            check += 2
        }
    }

    if (check > 3) == false {
        if let Err(err) = ws.close().await {
            eprintln!("Error closing connection: {}", err)
        };
        return;
    }

    let (mut tx_client, mut rx_client) = ws.split();
    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);
    let id_client = id.clone();

    if shard_in > shard_total {
        send_message(
            &tx,
            &json!({"type": 0, "possible_error": true, "message": "Excuse me! I'm terminating the connection due to too many shards.", "data": {}, "rate_limit": true}),
        ).await;
        if let Err(_) = tx_client.send(Message::close().clone()).await {};
        return;
    }

    let mut gateway_not_found = true;
    if shard_in > 0 {
        for (id_client, _client) in clients.read().await.iter() {
            if id.clone().as_str() == id_client.clone() {
                gateway_not_found = false
            }
        }
    } else {
        gateway_not_found = false
    }

    let mut event_state = "GATEWAY_HELLO";
    let mut message = "";
    if gateway_not_found == true {
        event_state = "GATEWAY_NO_IDENTIFIED";
        message = "I could not recognize this information in Gateway. Remember this is limited here don't try to reconnect.";
        return;
    }

    send_message(
        &tx,
        &json!({
                "type": 1,
                "event": event_state,
                "service": "gateway",
                "possible_error": false,
                "message": message,
                "data": {
                    "starting_shards": if shard_in == 0
                    {
                        true
                    } else {
                        false
                    },
                },
                "rate_limit": true
        }),
    )
    .await;

    if gateway_not_found == true {
        if let Err(_) = tx_client.send(Message::close()).await {};
        return;
    }

    let found_client = clients.read().await.get(&id_client).clone().is_none();

    match found_client {
        true => {}
        false => {
            let mut found = false;
            for (id_client, client) in clients.read().await.iter() {
                if id.clone().as_str() == id_client.clone() {
                    found = true;

                    let mut client = &mut client.ws.shards.clone();
                    let _ = &client.remove(&shard_in); // Update state
                }
            }
            if !found {
                send_message(&tx, &json!({"type": 2, "event": "GATEWAY_ERROR", "possible_error": true, "service": "gateway", "message": "Excuse me! I'm terminating the connection due to too many **error**.", "data": {}, "rate_limit": false})).await;
                if let Err(_) = tx_client.send(Message::close().clone()).await {};
            }
        }
    }

    let a = tx;

    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            tx_client
                .send(message)
                .unwrap_or_else(|e| {
                    rx.close();
                    error!("websocket send error: {}", e);
                })
                .await;
        }
    });

    while let Some(result) = rx_client.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                error!("websocket error {}", e);
                bot_disconnected(
                    clients.clone(),
                    id.clone(),
                    (pub_key.clone(), shard_in, shard_total, secret_key.clone()),
                ).await;
                break;
            }
        };
        message_interface(
            msg,
            &a,
            id.clone(),
            clients.clone(),
            interactions.clone(),
            (pub_key.clone(), shard_in, shard_total, secret_key.clone()),
        )
        .await;
    }
    bot_disconnected(
        clients.clone(),
        id.clone(),
        (pub_key.clone(), shard_in, shard_total, secret_key.clone()),
    )
    .await;
}

pub async fn update_state(clients: Clients, id: String) {
    clients.write().await.remove(&id.clone());
}

async fn bot_disconnected(clients: Clients, id: String, d: (String, usize, usize, String)) {
    let (key, shard_in, shard_total, secret) = d;

    for (id_bot, c) in clients.read().await.iter() {
        if id_bot.to_string() == id.clone() {
            let client = c.clone().ws.others;
        }
    }
}

pub(crate) fn search_shard(guild_id: u64, shard_size: usize) -> usize {
    let shard_id = (guild_id >> 22) % shard_size as u64;
    return shard_id as usize;
}

// pub fn encrypt_data_str(inf: String) -> (String, Sha512, Result<SecretKey, SignatureError>) {
//     let (data, sha, key) = encode_data(String::from("testing"), inf);
//     return (data, sha, key)
// }

// pub async fn send_metadata(mut tx_client: SplitSink<WebSocket, Message>, x: &Value) -> Result<(), Error> {
//     tx_client.send(Message::binary(convert_to_binary(x)).clone()).await
// }

pub(crate) async fn send_message(tx: &UnboundedSender<Message>, x: &Value) {
    if tx.is_closed() {
        return;
    }
    if let Err(err) = tx.send(Message::binary(convert_to_binary(x))) {
        error!("websocket error [send]: {}", err);
    }
}


pub(crate) async fn send_message_interaction(tx: &UnboundedSender<Message>, x: (usize, &Value, usize, usize)) -> Result<(), SendError<Message>> {
    if tx.is_closed() {
        error!("Connection closed!");
    }
    let (type_interaction, metadata, shard_id, shard_total) = x;
    let data: &Value = &json!({
        "type": 8,
        "service": "http_gateway",
        "event": "INTERACTION_DISCORD",
        "data": {
            "shard_id": shard_id,
            "shard_total": shard_total,
            "flag": type_interaction,
            "metadata_http": [metadata]
        }
    });
    tx.send(Message::binary(convert_to_binary(data)))
}

pub(crate) async fn send_message_with_client(
    settings: (ClientBot, usize, &UnboundedSender<Message>),
    x: &Value,
) {
    let (client, shard_id, tx) = settings;
    if tx.is_closed() {
        return;
    }

    if let Err(err) = tx.send(Message::binary(convert_to_binary(x))) {
        error!("websocket error [send]({}): {}", shard_id, err);
    }
}

pub fn convert_to_binary(inf: &Value) -> Vec<u8> {
    let mut data = ZlibEncoder::new(Vec::new(), Compression::new(10));
    if let Err(_) = data.write_all(inf.to_string().as_ref()) {};
    return data.finish().unwrap();
}

pub async fn read_compress(b: &[u8]) -> io::Result<String> {
    let mut a = GzDecoder::new(&*b);
    let mut s = String::new();
    a.read_to_string(&mut s).unwrap();
    Ok(s.to_string())
}

async fn message_interface(
    message: Message,
    x: &UnboundedSender<Message>,
    id: String,
    mut client: Clients,
    arc: Arc<RwLock<HashMap<String, Interaction>>>,
    d: (String, usize, usize, String),
) {
    let tx = x;
    let data_compress = read_compress(message.as_bytes()).await;
    if data_compress.is_err() == true {
        return;
    }
    let data = serde_json::from_str(&data_compress.unwrap().as_str());
    let json_data: Value = data.unwrap();

    load_commands(json_data, tx, client, id.clone(), arc.clone(), d).await;
}

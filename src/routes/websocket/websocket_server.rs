use std::borrow::Borrow;
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
use flate2::Compression;
use flate2::write::ZlibEncoder;

pub(crate) async fn websocket_message(ws: WebSocket, mut clients: Clients, id: String, secret: String, shard_in: usize, shard_total: usize) {
    let (mut tx_client, mut rx_client) = ws.split();
    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);
    let msg = "WARNING: THIS API IS FULLY LIMITED FOR SYNC REASONS SOMETIMES THE INTERACTION MAY TAKE TIME BECAUSE OF THE FOREGOING OR BAND BAND WHY MAY DELAY THE ENCRYPTED DATA LATTERY.";
    let rate = "API THERE ARE LIMITATIONS ON SENDING OVERCOMMANDS. TAKE CARE BEFORE SENDING MULTIPLE COMMANDS.";
    let (id_client) = id;
    let found_client = clients.read().await.get(&id_client).is_none();
    if shard_in > shard_total {
        let inf = &json!({"type": 0, "possible_error": true, "message": "Excuse me! I'm terminating the connection due to too many shards.", "data": {}, "rate_limit": true});
        tx_client.send(Message::binary(convert_to_binary(inf)).clone()).await;
        tx_client.send(Message::close().clone()).await;
        return;}
    let mut client;
    tx_client.send(Message::binary(convert_to_binary(&json!({
        "type": 1,
        "possible_error": false,
        "message": "",
        "data": {},
        "rate_limit": false
    })))).await;
    match found_client {
        true => {
            client = ClientBot {
                api_note: msg.parse().unwrap(),
                rate_limit_note: rate.parse().unwrap(),
                bandwidth_rx: 128 * 1000000000000 / 25 + 9249420456 / 2,
                bandwidth_tx: 3200 * 1000000000000 / 25 + 9249420456 / 2,
                shards: vec![],
                ws: vec![],
                latency: vec![],
                connected: false,
                stop_sending: false,
                is_sharding: false,
                is_confirmed: false,
                is_connection_secured: false,
                is_connection_tls: false,
                sentry: false,
                encrypted_data: false,
                encryption_part: "".to_string(),
                interaction_not_sync: vec![],
                clusters_api: vec![],
                errors: vec![],
                rate_limit: vec![]
            };
            clients.write().await.insert(id_client, client);
        }
        false => {}
    }


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
        message_interface(msg)
    }

}

fn search_shard(guild_id: usize) -> i32 {
    return ((guild_id >> 22) % 2) as i32
}

fn encrypt_data_str(inf: String) {

}

fn convert_to_binary(inf: &Value) -> Vec<u8> {
    let mut data = ZlibEncoder::new(Vec::new(), Compression::default());
    data.write_all(inf.to_string().as_ref());
    return data.finish().unwrap()
}

fn message_interface(message: Message) {
    let message = if let Ok(a) = message.to_str()
    { a } else { return; };
}
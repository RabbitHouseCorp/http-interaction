use crate::routes::websocket::structures::client::{Shard, Shardings};
use crate::routes::websocket::websocket_server::{
    convert_to_binary, send_message, send_message_with_client,
};
use crate::{Clients, Interaction};
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::ops::Index;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::RwLock;
use warp::ws::Message;

pub async fn register_shard(
    data: Value,
    tx: &UnboundedSender<Message>,
    mut _clients: Clients,
    id: String,
    interactions: Arc<RwLock<HashMap<String, Interaction>>>,
) {
    if data.get("d").is_none() {
        return;
    }
    if data.get("d").unwrap().get("public_key").is_none() {
        return;
    }
    if data.get("d").unwrap().get("shard_id").is_none() {
        return;
    }

    for (id_client, client) in _clients.read().await.iter() {
        if id.to_string() == id_client.to_string() {
            // Check Public_key
            let pub_key = data
                .get("d")
                .unwrap()
                .get("public_key")
                .unwrap()
                .as_str()
                .unwrap();
            if client.application_bot.public_key.to_string() != pub_key.to_string() {
                return;
            }
            // Create new hash
            let hash: String = rand::thread_rng()
                .sample_iter(Alphanumeric)
                .take(15)
                .map(char::from)
                .collect();

            // Registering the shard.
            let shard_id = data
                .get("d")
                .unwrap()
                .get("shard_id")
                .unwrap()
                .as_u64()
                .unwrap() as usize;
            let mut shard_save: Shard;
            let mut position = 0;

            let mut registered = false;
            // Check Register!
            for shard_stat in client.ws.others.shards_stats.read().await.iter() {
                position += 1;
                if shard_stat.shard_id == shard_id {
                    shard_save = shard_stat.clone();
                    registered = true
                }
            }

            if registered {
                return;
            }

            client
                .ws
                .others
                .shards_hash
                .write()
                .await
                .insert(0, hash.clone().to_string());
            client.ws.shards.clone().insert(shard_id, tx.clone());
            client.ws.others.shards_stats.write().await.insert(
                0,
                Shard {
                    shard_id: shard_id.clone(),
                    development: false,
                    shard_hash: hash.clone().to_string(),
                    send_ping: 0,
                    receibe_ping: 0,
                    sending: 0,
                    received: 0,
                    disconnected: false,
                },
            );

            //            client.ws.others.shard_missing.write().await.remove(shard_id.clone());
            let data =
                serde_json::to_value(client.ws.others.shards_stats.read().await.to_vec()).unwrap();

            send_message_with_client(
                (client.clone(), shard_id.clone(), tx),
                &json!({
                    "type": 1,
                    "event": "GATEWAY_SHARD_INFO",
                    "service": "gateway",
                    "data": [[[data]]]
                }),
            )
            .await;
        }
    }
}

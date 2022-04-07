use crate::routes::websocket::structures::client::{Application, ClientWs, Shard, ShardDefault, Shardings, ShardsHashDefault};
use crate::routes::websocket::websocket_server::{
    convert_to_binary, send_message, send_message_with_client,
};
use crate::{ClientBot, Clients, Interaction};
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::ops::Index;
use std::ptr::hash;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::RwLock;
use warp::ws::Message;


pub async fn resume_gateway(
    data: Value,
    tx: &UnboundedSender<Message>,
    mut clients: Clients,
    id: String,
    interactions: Arc<RwLock<HashMap<String, Interaction>>>,
    d: (String, usize, usize, String),
) {
    let (pub_key, shard_in, shard_total, secret_key) = d;
    if shard_in != 0 {
        return;
    }
    if data.get("d").is_none() {
        return;
    }
    if data.get("d").unwrap().get("flags").is_none() {
        return;
    }
    if data.get("d").unwrap().get("session_id").is_none() {
        return;
    }
    
    let session_id =  data.get("d").unwrap().get("session_id").unwrap().as_str().unwrap();
    
    let mut found = false;
    for (id_bot, client) in clients.write().await.iter() {
        if id_bot.to_string() == id.clone().to_string() {
            if client.ws.session_id != session_id.to_string() {
                return;
            }

            send_message(
                tx,
                &json!({
                    "type": 1,
                    "event": "GATEWAY_RESUME",
                    "service": "gateway",
                    "data": {
                        "application_bot": {
                            "shards_config": [[[[shard_in, shard_total]]]],
                            "public_key": "[REDACTED]",
                            "id": client.application_bot.id.clone(),
                            "flags": null
                        },
                        "shards_stats": [[[[[[[]]]]]]]
                    }
                }),
            )
                .await;
            found = true
        }
    }
    
}

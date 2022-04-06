use crate::routes::websocket::structures::client::{
    Application, ClientWs, Shard, ShardDefault, Shardings, ShardsHashDefault, ShardsMissingDefault,
};
use crate::routes::websocket::websocket_server::{convert_to_binary, send_message, update_state};
use crate::{ClientBot, Clients, Interaction};
use crossbeam::channel::internal::SelectHandle;
use futures::TryFutureExt;
use rustc_serialize::json::ToJson;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::ptr;
use std::ptr::null;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::RwLock;
use warp::ws::Message;

pub async fn register_client(
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
    let flags = data
        .get("d")
        .unwrap()
        .get("flags")
        .unwrap()
        .as_u64()
        .unwrap();
    let mut flag_count = 0;
    let mut shards = HashMap::default();
    shards.insert(shard_in.clone(), tx.clone());
    let sharding_info = Shardings {
        id: 0,
        encode: if !data.get("d").unwrap().get("encode").is_none() {
            data.get("d")
                .unwrap()
                .get("encode")
                .unwrap()
                .as_bool()
                .unwrap()
        } else {
            false
        },
        decode: if !data.get("d").unwrap().get("decode").is_none() {
            data.get("d")
                .unwrap()
                .get("encode")
                .unwrap()
                .as_bool()
                .unwrap()
        } else {
            false
        },
        shards_stats: ShardDefault::default(),
        shards_hash: ShardsHashDefault::default(),
    };

    for flag in 0..3 {
        if (flag & (flags)) == flag {
            flag_count += 1 << flag
        }
    }

    let client = ClientBot {
        ws: ClientWs {
            _id: id.clone(),
            shard_in: shard_in.clone(),
            shard_total: shard_total.clone(),
            shards: shards.clone(),
            others: sharding_info,
        },
        application_bot: Application {
            public_key: pub_key.clone(),
            id: id.clone(),
            _flags: flag_count,
        },
    };

    update_state(clients.clone(), id.clone()).await;
    let mut found = false;
    for (id_bot, client) in clients.write().await.iter() {
        if id_bot.to_string() == id.clone().to_string() {
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
                            "flags": flag_count
                        },
                        "shards_stats": [[[[[[[]]]]]]]
                    }
                }),
            )
            .await;
            found = true
        }
    }
    if !found {
        clients.write().await.insert(id.clone(), client.clone());
        send_message(
            tx,
            &json!({
                "type": 1,
                "event": "GATEWAY_READY",
                "service": "gateway",
                "data": {
                    "shards_config": [[[[shard_in, shard_total]]]],
                    "application_bot": {
                        "public_key": "[REDACTED]",
                        "id": client.application_bot.id.clone(),
                        "flags": flag_count
                    },
                    "shards_stats": [[[[[[[]]]]]]]
                }
            }),
        )
        .await;
    }
}

//async fn preparing_list_shards_missing(
//    shards: ShardsMissingDefault,
//    shard_in: usize,
//    shards_total: usize,
//) {
//    for shard in shard_in..shards_total {
//        shards
//            .write()
//            .await
//            .insert(shard, (shard as u128).try_into().unwrap())
//    }
//}

use crate::routes::websocket::commands::client::ping_client::ping_client;
use crate::routes::websocket::commands::client::register_client::register_client;
use crate::routes::websocket::commands::client::register_shard::register_shard;
use crate::routes::websocket::structures::client::Shard;
use crate::routes::websocket::websocket_server::convert_to_binary;
use crate::{Clients, Interaction};
use jwt::ToBase64;
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::Read;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::RwLock;
use tracing::debug;
use warp::ws::Message;

pub async fn load_commands(
    data: Value,
    tx: &UnboundedSender<Message>,
    mut _clients: Clients,
    id: String,
    interactions: Arc<RwLock<HashMap<String, Interaction>>>,
    d: (String, usize, usize, String),
) {
    let type_command_is_none = data["type"].as_u64().is_none();

    if type_command_is_none {
        return;
    }

    let type_command = data["type"].as_u64().unwrap().to_owned();

    debug!(
        "Handler Command: id={} type_command={} metadata_ws={} ",
        id.clone(),
        type_command,
        data.to_string()
    );

    // Register Client
    match type_command {
        1 => {
            register_client(
                data.clone(),
                tx,
                _clients,
                id.clone(),
                interactions.clone(),
                d,
            )
            .await;
        }
        2 => {
            ping_client(
                data.clone(),
                tx,
                _clients.clone(),
                id.clone(),
                interactions.clone(),
            )
            .await;
        }
        3 => {
            register_shard(
                data.clone(),
                tx,
                _clients.clone(),
                id.clone(),
                interactions.clone(),
            )
            .await;
        }
        4 => {}
        _ => {}
    }
}

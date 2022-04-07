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
use crate::routes::websocket::commands::interaction::interaction_return::interaction_return;
use crate::routes::websocket::messages::utils::{CLIENT_RETURN_METADATA_FOR_API, PING_CLIENT, REGISTER_CLIENT, REGISTER_SHARDING, RESUME_GATEWAY_MASTER, RESUME_SHARD};

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
        REGISTER_CLIENT => {
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
        PING_CLIENT => {
            ping_client(
                data.clone(),
                tx,
                _clients.clone(),
                id.clone(),
                interactions.clone(),
            )
            .await;
        }
        REGISTER_SHARDING => {
            register_shard(
                data.clone(),
                tx,
                _clients.clone(),
                id.clone(),
                interactions.clone(),
            )
            .await;
        }
        CLIENT_RETURN_METADATA_FOR_API => {
            interaction_return(
                data.clone(),
                tx,
                _clients.clone(),
                id.clone(),
                interactions.clone(),
            ).await;
        }
        RESUME_GATEWAY_MASTER => {}
        RESUME_SHARD => {}
        _ => {}
    }
}

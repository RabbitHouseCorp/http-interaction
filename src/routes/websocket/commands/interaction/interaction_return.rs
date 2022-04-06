use crate::routes::websocket::websocket_server::{convert_to_binary, send_message};
use crate::{Clients, Interaction};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::RwLock;
use warp::ws::Message;

pub async fn interaction_return(
    data: Value,
    _tx: &UnboundedSender<Message>,
    mut _clients: Clients,
    id: String,
    interactions: Arc<RwLock<HashMap<String, Interaction>>>,
) {
    // Limit API access to avoid creating unnecessary API ping and flooding.
    if _clients.read().await.get(&id.clone()).is_none() {
        return;
    }

    if data["id"].as_str().is_none() {
        return;
    }
    tokio::task::spawn(async move {
        interactions.write().await.insert(
            data["id"].to_string(),
            Interaction {
                data: data["data"].clone(),
            },
        );
        tokio::time::sleep(Duration::from_secs(4)).await; // Remove cache
        interactions.write().await.remove(&*data["id"].to_string());
    });
}

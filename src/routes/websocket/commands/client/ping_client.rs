use crate::routes::websocket::websocket_server::{convert_to_binary, send_message};
use crate::{Clients, Interaction};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::RwLock;
use warp::ws::Message;

pub async fn ping_client(
    data: Value,
    tx: &UnboundedSender<Message>,
    mut _clients: Clients,
    id: String,
    interactions: Arc<RwLock<HashMap<String, Interaction>>>,
) {
    // Limit API access to avoid creating unnecessary API ping and flooding.
    if _clients.read().await.get(&id.clone()).is_none() {
        return;
    }
    send_message(
        tx,
        &json!({
            "type": 3,
            "service": "gateway",
            "event": "GATEWAY_PING",
            "data": {}
        }),
    )
    .await;
}

use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{alloc::System, collections::HashMap, time::{SystemTime, UNIX_EPOCH}};
use warp::Filter;
pub mod structures;

struct ResponseData {
    status_code: u64,
}

#[tokio::main]
async fn main() {
    let mut interactions: HashMap<&str, structures::interaction::InteractionData> = HashMap::new();
    let mut connection: HashMap<&str, structures::connection_state::ConnectionStateKraken> =
        HashMap::new();
    let mut bot_connection_state: HashMap<&str, structures::bot_interaction::BotInteraction> =
        HashMap::new();
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let map_routes = warp::path::end().map(|| {
        warp::reply::json(&json!({ "status_code": 404, "message": "Not found!", "error": true }).as_object_mut())
    });
    let gateway = warp::path!("gateway").map(|| {
        warp::reply::json(&json!({ "status_code": 200, "message": "Not found!", "error": true }).as_object_mut())
    });
    let routes = warp::get().and(map_routes.or(gateway));
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await; 
}

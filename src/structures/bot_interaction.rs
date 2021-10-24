use std::collections::HashMap;

use super::connection_state::{ConnectionStateKraken};

pub struct BotInteraction {
    connection_state: ConnectionStateKraken,
    id: String,
    bot_name: String,
    public_key: String,
    date: u64,
    session: HashMap<String, ConnectionStateKraken>,
    localhost: bool, // If this is enabled and the API tries to connect on an ip that is not localhost it will be refused.
    check_gateway: bool, // /gateway (Post)
    secret_via_gateway: String, // /gateway (Get)
    shards: Vec<u64>, // Shards connected
    shards_died: Vec<u64>, // Array of shards died
    shards_stats: Vec<u64>, // Latency
    status: bool,    // /gateway?status_profile=0x304
}

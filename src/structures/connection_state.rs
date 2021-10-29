use serde::{Serialize, Deserialize};
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct ConnectionStateKraken {
    pub session_id: String,
    pub id: String,
    pub bot_id: String,
    pub master_shard: String,
    pub secret: String,
    pub scope: Vec<u64>,   // Flags
    pub type_interaction: u64, // Flag
}




pub struct ConnectionStateKraken {
    pub session_id: String,
    pub id: String,
    pub bot_id: String,
    pub master_shard: String,
    secret: String,
    ip_listening: Vec<String>,
    pub scope: Vec<u64>,      // Flags
    type_interaction: u64     // Flag
}

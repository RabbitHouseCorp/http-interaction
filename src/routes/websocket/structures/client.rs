use std::collections::HashMap;
use std::sync::Arc;
use serde_json::Value;
use tokio::sync::{mpsc, RwLock};
use tokio::sync::mpsc::UnboundedSender;
use warp::ws::Message;

pub struct ClientBot {
     // pub(crate) _id: String,
     pub(crate) api_note: String,
     pub(crate) rate_limit_note: String,
     pub(crate) bandwidth_rx: u128,
     pub(crate) bandwidth_tx: u128,
     pub(crate) shards: Vec<usize>,
     pub(crate) ws: Vec<Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Message>>>>>,
     pub(crate) latency: Vec<usize>,
     pub(crate) connected: bool,
     pub(crate) stop_sending: bool,
     pub(crate) is_sharding: bool,
     pub(crate) is_confirmed: bool,
     pub(crate) is_connection_secured: bool,
     pub(crate) is_connection_tls: bool,
     pub(crate) sentry: bool,
     pub(crate) encrypted_data: bool,
     pub(crate) encryption_part: String,
     pub(crate) interaction_not_sync: Vec<HashMap<String, Value>>,
     pub(crate) clusters_api: Vec<HashMap<String, Value>>,
     pub(crate) errors: Vec<HashMap<String, Value>>,
     pub(crate) rate_limit: Vec<HashMap<String, Value>>,
}

impl ClientBot {
    // fn _id(&self) -> &String { &self._id }
    fn api_note(&self) -> &String { &self.api_note }
    fn rate_limit_note(&self) -> &String { &self.rate_limit_note }
    fn bandwidth_rx(&self) -> &u128 { &self.bandwidth_rx }
    fn bandwidth_tx(&self) -> &u128 { &self.bandwidth_tx }
    fn shards(&self) -> &Vec<usize> { &self.shards }
    fn ws(&self) -> &Vec<Arc<RwLock<HashMap<usize, UnboundedSender<Message>>>>> { &self.ws }
    fn latency(&self) -> &Vec<usize> { &self.latency }
    fn connected(&self) -> &bool { &self.connected }
    fn stop_sending(&self) -> &bool { &self.stop_sending }
    fn is_sharding(&self) -> &bool { &self.is_sharding }
    fn is_confirmed(&self) -> &bool { &self.is_confirmed }
    fn is_connection_secured(&self) -> &bool { &self.is_connection_secured }
    fn is_connection_tls(&self) -> &bool { &self.is_connection_tls }
    fn sentry(&self) -> &bool { &self.sentry }
    fn encrypted_data(&self) -> &bool { &self.encrypted_data }
    fn encryption_part(&self) -> &String { &self.encryption_part }
    fn interaction_not_sync(&self) -> &Vec<HashMap<String, Value>> { &self.interaction_not_sync }
    fn clusters_api(&self) -> &Vec<HashMap<String, Value>> { &self.clusters_api }
    fn errors(&self) -> &Vec<HashMap<String, Value>> { &self.errors }
    fn rate_limit(&self) -> &Vec<HashMap<String, Value>> { &self.rate_limit }
}
use derive_more::Display;
use rustc_serialize::json::Array;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::RwLock;
use warp::ws::Message;
// Flags
//
// These flags are saved in the app which can be configured in the websocket.

const FLAG_ENCODE_ZLIB: u64 = 1 << 0; // Compress the data to send to the Client
const FLAG_DECODE_ZLIB: u64 = 1 << 1; // Decompress the data you receive from the Client
const FLAG_SEND_BINARY: u64 = 1 << 2; // Sending the data in binary can offer better data transmit/receive speed
const FLAG_SHARD_MODE: u64 = 1 << 3; // Enable shard mode.

pub type ShardDefault = Arc<RwLock<Vec<Shard>>>;
pub type ShardsHashDefault = Arc<RwLock<Vec<String>>>;
pub type ShardsMissingDefault = Arc<RwLock<Vec<usize>>>;

pub struct Interaction {
    pub data: Value,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Application {
    pub public_key: String,
    pub id: String,
    pub _flags: usize,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Shard {
    pub shard_id: usize,
    pub development: bool,
    pub shard_hash: String,
    pub send_ping: usize,
    pub receibe_ping: usize,
    pub sending: usize,
    pub received: usize,
    pub disconnected: bool,
}

impl Shard {}
#[derive(Clone)]
pub struct Shardings {
    pub id: usize,
    pub encode: bool,
    pub decode: bool,
    pub shards_stats: ShardDefault,
    pub shards_hash: ShardsHashDefault,
    //
    // This type of implementation will be removed due to state updates
    //    #[deprecated]
    //    pub(crate) shard_missing,
}
#[derive(Clone)]
pub struct ClientWs {
    pub _id: String,
    // pub(crate) tx: UnboundedSender<Message>,
    pub(crate) shards: HashMap<usize, UnboundedSender<Message>>,
    pub(crate) others: Shardings,
    pub(crate) shard_in: usize,
    pub(crate) shard_total: usize,
}

impl ClientBot {
    #[allow(dead_code)]
    fn update_id(&self) -> &String {
        &self.ws._id
    }
    #[allow(dead_code)]
    // fn update_tx(&self) -> &UnboundedSender<Message> { &self.ws.tx }
    fn shards(&self) -> &HashMap<usize, UnboundedSender<Message>> {
        &self.ws.shards
    }
    fn shard_in(&self) -> &usize {
        &self.ws.shard_in
    }
    fn shard_total(&self) -> &usize {
        &self.ws.shard_total
    }
}

#[derive(Clone)]
pub struct ClientBot {
    pub ws: ClientWs,
    pub application_bot: Application,
}

use crossbeam::sync::WaitGroup;
use serde_derive::{Deserialize, Serialize};
#[derive(Clone)]
pub struct ConnectionStateKraken {
    pub session_id: String,
    pub id: String,
    pub bot_id: String,
    pub master_shard: String,
    pub secret: String,
    pub scope: Vec<u64>,   // Flags
    pub type_interaction: u64, // Flag,
    pub sync: WaitGroup
}
trait ConnectionStateKrakenTrait {
    fn session_id(self) -> String;
    fn id(self) -> String;
    fn bot_id(self) -> String;
    fn master_shard(self) -> String;
    fn secret(self) -> String;
    fn scope(self) -> Vec<u64>;
    fn type_interaction(self) -> u64;
    fn sync(self) -> WaitGroup;
}
impl ConnectionStateKrakenTrait for ConnectionStateKraken {
    fn session_id(self) -> String { self.session_id }
    fn id(self) -> String { self.id }
    fn bot_id(self) -> String { self.bot_id }
    fn master_shard(self) -> String { self.master_shard }
    fn secret(self) -> String { self.secret }
    fn scope(self) -> Vec<u64> { self.scope }
    fn type_interaction(self) -> u64 { self.type_interaction }
    fn sync(self) -> WaitGroup { self.sync }
}




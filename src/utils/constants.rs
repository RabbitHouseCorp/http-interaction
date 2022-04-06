// HTTP
pub const INTERACTION_PING: u64 = 1;

// Interaction UI
pub const INTERACTION_COMMAND: u64 = 2;
pub const INTERACTION_BUTTON: u64 = 3;
pub const INTERACTION_AUTOCOMPLETE: u64 = 4;
pub const INTERACTION_MODAL_SUBMIT: u64 = 5;

// FLAG_HTTP
pub const FLAG_INTERACTION_PING: usize = 1 << 1;

// Interaction UI
pub const FLAG_INTERACTION_COMMAND: usize = 1 << 2;
pub const FLAG_INTERACTION_BUTTON: usize = 1 << 3;
pub const FLAG_INTERACTION_AUTOCOMPLETE: usize = 1 << 4;
pub const FLAG_INTERACTION_MODAL_SUBMIT: usize = 1 << 5;

// Flags
//
// These flags are saved in the app which can be configured in the websocket.

pub const FLAG_ENCODE_ZLIB: u64 = 1 << 0; // Compress the data to send to the Client
pub const FLAG_DECODE_ZLIB: u64 = 1 << 1; // Decompress the data you receive from the Client
pub const FLAG_SEND_BINARY: u64 = 1 << 2; // Sending the data in binary can offer better data transmit/receive speed
pub const FLAG_SHARD_MODE: u64 = 1 << 3; // Enable shard mode.

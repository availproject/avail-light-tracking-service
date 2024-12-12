use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PingMessage {
    pub timestamp: i64,
    pub multiaddr: String,
    pub peer_id: String,
    pub block_number: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignedPingMessage {
    pub message: PingMessage,
    pub signature: Vec<u8>,
    pub public_key: String,
}

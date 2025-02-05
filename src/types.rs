use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PingMessage {
    pub timestamp: i64,
    pub multiaddr: Option<String>,
    pub peer_id: Option<String>,
    pub block_number: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignedPingMessage {
    pub message: PingMessage,
    pub signature: Vec<u8>,
    pub public_key: String,
}

#[derive(Debug, Serialize)]
pub struct ClientInfo {
    pub first_seen: i64,
    pub last_seen: i64,
    pub peer_id: Option<String>,
    pub last_multiaddr: Option<String>,
}

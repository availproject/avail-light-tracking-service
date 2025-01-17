use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize)]
pub struct ClientInfo {
    pub first_seen: i64,
    pub last_seen: i64,
    pub total_uptime: i64,
    pub peer_id: String,
    pub last_multiaddr: String,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PingMessage {
    pub timestamp: i64,
    pub multiaddr: Option<String>,
    pub peer_id: Option<String>,
    pub latest_block: Option<u32>,
    pub operator_address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignedPingMessage {
    pub message: PingMessage,
    pub signature: Vec<u8>,
    pub public_key: String,
}

#[derive(Debug, Serialize)]
pub struct ClientInfo {
    pub operator_address: Option<String>,
    pub first_seen: Option<i64>,
    pub last_seen: Option<i64>,
    pub peer_id: Option<String>,
    pub last_multiaddr: Option<String>,
    pub latest_block: Option<u32>,
}

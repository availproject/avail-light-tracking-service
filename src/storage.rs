use crate::{
    error::ApiError,
    types::{ClientInfo, PingMessage},
};
use rocksdb::{IteratorMode, Options, TransactionDB, TransactionDBOptions};
use std::path::Path;

pub struct RocksStorage {
    db: TransactionDB,
}

impl RocksStorage {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, ApiError> {
        let mut opts = Options::default();
        opts.set_max_background_jobs(4);
        opts.set_bytes_per_sync(1024 * 1024); // 1mb

        let db = TransactionDB::open(&opts, &TransactionDBOptions::default(), path)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;
        Ok(Self { db })
    }

    pub fn store(&self, key: &[u8], value: &[u8]) -> Result<(), ApiError> {
        self.db
            .put(key, value)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))
    }

    pub fn get_client_info(&self, public_key: &str) -> Result<Option<ClientInfo>, ApiError> {
        let mut timestamps = Vec::new();
        let mut last_ping: Option<PingMessage> = None;

        let iter = self.db.iterator(IteratorMode::End);

        for item in iter {
            let (key, value) = item.map_err(|e| ApiError::DatabaseError(e.to_string()))?;

            let key_str = String::from_utf8_lossy(&key);
            let parts: Vec<&str> = key_str.split(':').collect();

            if parts.len() == 2 && parts[1] == public_key {
                let ping: PingMessage = serde_json::from_slice(&value)
                    .map_err(|e| ApiError::SerializationError(e.to_string()))?;

                timestamps.push(ping.timestamp);
                if last_ping.is_none() {
                    last_ping = Some(ping);
                }
            }
        }

        if let Some(last_message) = last_ping {
            Ok(Some(ClientInfo {
                first_seen: timestamps.last().copied(),
                last_seen: timestamps.first().copied(),
                peer_id: last_message.peer_id,
                last_multiaddr: last_message.multiaddr,
                latest_block: last_message.latest_block,
            }))
        } else {
            Ok(None)
        }
    }
}

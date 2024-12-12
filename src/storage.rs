use crate::error::ApiError;
use rocksdb::DB;
use std::path::Path;

pub struct RocksStorage {
    db: DB,
}

impl RocksStorage {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, ApiError> {
        let db = DB::open_default(path).map_err(|e| ApiError::DatabaseError(e.to_string()))?;
        Ok(Self { db })
    }

    pub fn store(&self, key: &[u8], value: &[u8]) -> Result<(), ApiError> {
        self.db
            .put(key, value)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))
    }
}

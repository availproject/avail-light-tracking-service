use crate::{error::ApiError, storage::RocksStorage, types::SignedPingMessage};
use actix_web::{web, HttpResponse};
use signature_verifier::verify_sr25519_signature;

mod signature_verifier;

pub async fn handle_ping(
    data: web::Json<SignedPingMessage>,
    storage: web::Data<RocksStorage>,
) -> Result<HttpResponse, ApiError> {
    let is_valid =
        verify_sr25519_signature(&data.public_key, &data.message, data.signature.clone())?;

    if !is_valid {
        return Err(ApiError::InvalidSignature);
    }

    let key = format!("{}:{}", data.message.timestamp, data.message.peer_id);
    let value = serde_json::to_vec(&data.message)
        .map_err(|e| ApiError::SerializationError(e.to_string()))?;

    storage.store(key.as_bytes(), &value)?;

    Ok(HttpResponse::Ok().json("Ping received and stored"))
}

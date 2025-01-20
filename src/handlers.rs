use crate::{error::ApiError, storage::RocksStorage, types::SignedPingMessage};
use actix_web::{web, HttpResponse};
use signature_verifier::verify_sr25519_signature;
use tracing::trace;

mod signature_verifier;

pub async fn handle_ping(
    data: web::Json<SignedPingMessage>,
    storage: web::Data<RocksStorage>,
) -> Result<HttpResponse, ApiError> {
    let is_valid =
        verify_sr25519_signature(&data.public_key, &data.message, data.signature.clone())?;

    // Signature string is already in the SS58 form
    trace!("Public key received: {}", data.public_key);

    if !is_valid {
        return Err(ApiError::InvalidSignature);
    }

    // Key is in a format timestamp:public_key
    let key = format!("{}:{}", data.message.timestamp, data.public_key);
    let value = serde_json::to_vec(&data.message)
        .map_err(|e| ApiError::SerializationError(e.to_string()))?;

    storage.store(key.as_bytes(), &value)?;

    Ok(HttpResponse::Ok().json("Ping received and stored"))
}

pub async fn handle_client_info(
    public_key: web::Path<String>,
    storage: web::Data<RocksStorage>,
) -> Result<HttpResponse, ApiError> {
    match storage.get_client_info(&public_key)? {
        Some(client_info) => Ok(HttpResponse::Ok().json(client_info)),
        None => Ok(HttpResponse::NotFound().json("No data found for this peer")),
    }
}

use crate::{error::ApiError, storage::RocksStorage, types::SignedPingMessage};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde_json::json;
use signature_verifier::verify_sr25519_signature;
use tracing::{error, info, trace};

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
        trace!("Invalid signature for client: {}", data.public_key);
        return Err(ApiError::InvalidSignature);
    }

    let current_time = Utc::now().timestamp();

    // TODO: additional timestamp checks might be needed
    if current_time < data.message.timestamp {
        error!("Invalid timestamp");
        return Err(ApiError::InvalidTimestamp);
    }

    info!("Stored message: {:?}", data.message);

    // Key is in a format timestamp:public_key
    let key = format!("{}:{}", current_time, data.public_key);
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

pub async fn status() -> Result<HttpResponse, ApiError> {
    return Ok(HttpResponse::Ok().json(json!({"message": "Server is up and running!"})));
}

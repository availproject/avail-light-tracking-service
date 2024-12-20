use anyhow::Result;
use avail_light_tracking_service::types::{PingMessage, SignedPingMessage};
use chrono::Utc;
use reqwest;
use serde_json;
use sp_core::Pair;
use sp_core::{crypto::Ss58Codec, sr25519::Pair as Sr25519Pair};

#[tokio::main]
async fn main() -> Result<()> {
    // Generate a new keypair
    let pair = Sr25519Pair::generate().0;
    let public_key = pair.public();
    let ss58_public_key = public_key.to_ss58check();

    // Create a ping message
    let ping_message = PingMessage {
        timestamp: Utc::now().timestamp(),
        multiaddr: "/ip4/127.0.0.1/tcp/30333".to_string(),
        peer_id: "12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2".to_string(),
        block_number: "123456".to_string(),
    };

    // Serialize the message to bytes for signing
    let message_bytes = serde_json::to_vec(&ping_message)?;

    // Sign the message
    let signature = pair.sign(&message_bytes);

    // Create the signed message
    let signed_message = SignedPingMessage {
        message: ping_message,
        signature: signature.to_vec(),
        public_key: ss58_public_key,
    };

    // Send the request
    let client = reqwest::Client::new();
    let response = client
        .post("http://127.0.0.1:8080/ping")
        .json(&signed_message)
        .send()
        .await?;

    println!("Response status: {}", response.status());
    println!("Response body: {}", response.text().await?);

    Ok(())
}

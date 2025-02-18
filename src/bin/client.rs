use anyhow::Result;
use avail_light_tracking_service::cli::ClientCliOpts;
use avail_light_tracking_service::types::{PingMessage, SignedPingMessage};
use chrono::Utc;
use clap::Parser;
use reqwest;
use serde_json;
use sp_core::Pair;
use sp_core::{crypto::Ss58Codec, sr25519::Pair as Sr25519Pair};
use tracing::{info, Level, Subscriber};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub fn default_subscriber(log_level: Level) -> impl Subscriber {
    FmtSubscriber::builder()
        .with_env_filter(EnvFilter::new(format!("test_client={log_level}")))
        .finish()
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = ClientCliOpts::parse();
    tracing::subscriber::set_global_default(default_subscriber(opts.verbosity))?;
    info!("Starting test client");
    // Generate a new keypair
    let pair = Sr25519Pair::generate().0;
    let public_key = pair.public();
    info!("Public key: {}", public_key.to_string());
    let ss58_public_key = public_key.to_ss58check();

    // Create a ping message
    let ping_message = PingMessage {
        timestamp: Utc::now().timestamp(),
        multiaddr: Some("/ip4/127.0.0.1/tcp/30333".to_string()),
        peer_id: Some("12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2".to_string()),
        latest_block: Some(123456),
    };

    // Serialize the message to bytes for signing
    let message_bytes = serde_json::to_vec(&ping_message)?;

    // Sign the message
    let signature = pair.sign(&message_bytes);

    let signed_message = SignedPingMessage {
        message: ping_message,
        signature: signature.to_vec(),
        public_key: ss58_public_key,
    };

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}:{}/ping", opts.server_addr, opts.server_port))
        .json(&signed_message)
        .send()
        .await?;

    info!("Response status: {}", response.status());
    info!("Response body: {}", response.text().await?);

    Ok(())
}

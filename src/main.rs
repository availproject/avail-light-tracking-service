use actix_web::{web, App, HttpServer};
use anyhow::Result;
use clap::Parser;
use cli::CliOpts;
use std::sync::Arc;
use storage::RocksStorage;
use tracing::{info, Level, Subscriber};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

mod cli;
mod error;
mod handlers;
mod storage;
mod types;

pub fn default_subscriber(log_level: Level) -> impl Subscriber {
    FmtSubscriber::builder()
        .with_env_filter(EnvFilter::new(format!(
            "avail_light_tracking_service={log_level}"
        )))
        .finish()
}

#[actix_web::main]
async fn main() -> Result<()> {
    let opts = CliOpts::parse();
    tracing::subscriber::set_global_default(default_subscriber(opts.verbosity))?;

    info!("Starting Avail Light Tracking Service");
    let storage = Arc::new(RocksStorage::new(opts.db_path).expect("Failed to initialize storage"));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(Arc::clone(&storage)))
            .route("/ping", web::get().to(handlers::handle_ping))
    })
    .bind(format!("{}:{}", opts.server_addr, opts.server_port))?
    .run()
    .await
    .map_err(anyhow::Error::from)
}

use actix_web::{web, App, HttpServer};
use anyhow::Result;
use avail_light_tracking_service::{cli::ServerCliOpts, handlers, storage::RocksStorage};
use clap::Parser;
use std::sync::Arc;
use tracing::{info, Level, Subscriber};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub fn default_subscriber(log_level: Level) -> impl Subscriber {
    FmtSubscriber::builder()
        .with_env_filter(EnvFilter::new(format!(
            "avail_light_tracking_service={log_level}"
        )))
        .finish()
}

#[actix_web::main]
async fn main() -> Result<()> {
    let opts = ServerCliOpts::parse();
    tracing::subscriber::set_global_default(default_subscriber(opts.verbosity))?;

    info!("Starting Avail Light Tracking Service");
    info!(
        "Listening on: {}",
        format!("{}:{}", opts.server_addr, opts.server_port)
    );
    let storage = Arc::new(RocksStorage::new(opts.db_path).expect("Failed to initialize storage"));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(Arc::clone(&storage)))
            .route("/ping", web::post().to(handlers::handle_ping))
            .route(
                "/client-info/{public_key}",
                web::get().to(handlers::handle_client_info),
            )
            .route("/status", web::get().to(handlers::status))
    })
    .bind(format!("{}:{}", opts.server_addr, opts.server_port))?
    .run()
    .await
    .map_err(anyhow::Error::from)
}

mod config;
mod error;
mod handlers;
mod storage;
mod types;

use actix_web::{web, App, HttpServer};
use config::AppConfig;
use std::sync::Arc;
use storage::RocksStorage;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = AppConfig::default();
    let storage =
        Arc::new(RocksStorage::new(&config.db_path).expect("Failed to initialize storage"));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(Arc::clone(&storage)))
            .route("/ping", web::get().to(handlers::handle_ping))
    })
    .bind(format!("{}:{}", config.server_addr, config.server_port))?
    .run()
    .await
}

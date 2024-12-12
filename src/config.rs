use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server_addr: String,
    pub server_port: u16,
    pub db_path: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server_addr: "127.0.0.1".to_string(),
            server_port: 8080,
            db_path: "./ping_db".to_string(),
        }
    }
}

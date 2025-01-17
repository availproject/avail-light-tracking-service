use clap::Parser;
use tracing::Level;

#[derive(Parser, Clone, Debug)]
pub struct ServerCliOpts {
    #[arg(
        long,
        default_value = "127.0.0.1",
        help = "Avail Light Client Tracker listen address. Default: 127.0.0.1"
    )]
    pub server_addr: String,
    #[arg(
        long,
        default_value = "8080",
        help = "Avail Light Client Tracker listen port. Default: 8080"
    )]
    pub server_port: u16,
    #[arg(
        long,
        default_value = "./ping_db",
        help = "Avail Light Client Tracker local data location. Default: \"./ping_db\""
    )]
    pub db_path: String,
    #[arg(long, default_value = "info", help = "Log verbosity. Default: info")]
    pub verbosity: Level,
}

#[derive(Parser, Clone, Debug)]
pub struct ClientCliOpts {
    #[arg(
        long,
        default_value = "http://127.0.0.1",
        help = "Avail Light Client Tracker listen address. Default: 127.0.0.1"
    )]
    pub server_addr: String,
    #[arg(
        long,
        default_value = "8080",
        help = "Avail Light Client Tracker listen port. Default: 8080"
    )]
    pub server_port: u16,
    #[arg(long, default_value = "info", help = "Log verbosity. Default: info")]
    pub verbosity: Level,
}

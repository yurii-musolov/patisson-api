use crate::cli::Serve;

#[derive(Debug)]
pub struct Config {
    pub db_url: String,
    pub db_max_connections: u32,
    pub address: String,
}

const DEFAULT_ADDRESS: &str = "127.0.0.1:3000";

pub fn get_config(params: Serve) -> Config {
    let db_url = std::env::var("PATISSON__AUTH__DATABASE_URL")
        .expect("PATISSON__AUTH__DATABASE_URL must be set");

    let address = params.address.unwrap_or_else(|| {
        std::env::var("PATISSON__AUTH__ADDR").unwrap_or(DEFAULT_ADDRESS.to_string())
    });

    Config {
        db_url,
        db_max_connections: 5,
        address,
    }
}

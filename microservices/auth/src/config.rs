#[derive(Debug)]
pub struct Config {
    pub db_url: String,
    pub db_max_connections: u32,
    pub addr: String,
}

pub fn get_config() -> Config {
    let db_url = std::env::var("PATISSON__AUTH__DATABASE_URL").expect("PATISSON__AUTH__DATABASE_URL must be set");
    let addr =
        std::env::var("PATISSON__AUTH__ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());

    Config {
        db_url,
        db_max_connections: 5,
        addr,
    }
}

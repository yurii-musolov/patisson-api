#[derive(Debug)]
pub struct Config {
    pub http_address: String,
    pub rpc_address: String,
    pub ws_address: String,
}

const DEFAULT_HTTP_ADDRESS: &str = "127.0.0.1:3000";
const DEFAULT_RPC_ADDRESS: &str = "127.0.0.1:3001";
const DEFAULT_WS_ADDRESS: &str = "127.0.0.1:3002";

pub fn get_config() -> Config {
    let http_address =
        std::env::var("PATISSON__PROVIDER__HTTP_ADDR").unwrap_or(DEFAULT_HTTP_ADDRESS.to_string());
    let rpc_address =
        std::env::var("PATISSON__PROVIDER__RPC_ADDR").unwrap_or(DEFAULT_RPC_ADDRESS.to_string());
    let ws_address =
        std::env::var("PATISSON__PROVIDER__WS_ADDR").unwrap_or(DEFAULT_WS_ADDRESS.to_string());

    Config {
        http_address,
        rpc_address,
        ws_address,
    }
}

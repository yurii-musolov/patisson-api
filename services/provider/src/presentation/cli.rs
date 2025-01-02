use clap::{Args, Parser};

#[derive(Parser)]
#[clap(about = "Microservice `provider`")]
#[command(version, about)]
pub enum Command {
    Serve(Serve),
}

#[derive(Debug, Args)]
pub struct Serve {
    #[clap(long, env = "PATISSON__PROVIDER__HTTP_ADDR", default_value_t = String::from("127.0.0.1:3000"))]
    pub http_bind: String,
    #[clap(long, env = "PATISSON__PROVIDER__RPC_ADDR", default_value_t = String::from("127.0.0.1:3001"))]
    pub rpc_bind: String,
    #[clap(long, env = "PATISSON__PROVIDER__WS_ADDR", default_value_t = String::from("127.0.0.1:3002"))]
    pub ws_bind: String,
}

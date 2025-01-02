use clap::{Args, Parser};

#[derive(Parser)]
#[clap(about = "Microservice `auth`")]
#[command(version, about)]
pub enum Command {
    Serve(Serve),
}

#[derive(Debug, Args)]
pub struct Serve {
    #[clap(long, env = "PATISSON__AUTH__DATABASE_URL")]
    pub db_url: String,

    #[clap(
        long,
        env = "PATISSON__AUTH__DATABASE_MAX_CONNECTION",
        default_value_t = 10
    )]
    pub db_max_connections: u32,

    #[clap(long, env = "PATISSON__AUTH__HTTP_ADDR", default_value_t = String::from("127.0.0.1:3000"))]
    pub http_bind: String,
}

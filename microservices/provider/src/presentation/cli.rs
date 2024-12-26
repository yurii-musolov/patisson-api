use clap::Parser;

#[derive(Debug, Parser)]
#[clap(about = "Microservice `provider`")]
#[command(version, about)]
pub enum Command {
    Serve(Serve),
}

#[derive(Debug, clap::Args)]
pub struct Serve {
    #[clap(long)]
    pub http: bool,
    #[clap(long)]
    pub rpc: bool,
    #[clap(long)]
    pub ws: bool,
}

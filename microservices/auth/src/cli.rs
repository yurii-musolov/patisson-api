use clap::Parser;

#[derive(Debug, Parser)]
#[clap(about = "Microservice `auth`")]
#[command(version, about)]
pub enum Command {
    Serve(Serve),
}

#[derive(Debug, clap::Args)]
pub struct Serve {
    #[clap(short, long)]
    pub address: Option<String>,
}

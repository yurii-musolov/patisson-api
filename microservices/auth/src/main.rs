mod config;

use config::get_config;

#[tokio::main]
async fn main() {
    let cfg = get_config();
    println!("{cfg:?}");
    println!("Microservice `auth` running...");
}

mod config;
mod router;
mod cli;
mod api;
mod handler;

use anyhow;
use clap::Parser;
use cli::Command;
use config::get_config;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let command = Command::parse();
    println!("DEBUG (command): {command:?}");

    let cfg = get_config();

    let pool = PgPoolOptions::new()
        .max_connections(cfg.db_max_connections)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&cfg.db_url)
        .await
        .expect("can't connect to database");

    sqlx::migrate!().run(&pool).await?;

    let router = router::v1(pool);

    let listener = tokio::net::TcpListener::bind(cfg.addr).await.unwrap();

    println!("Microservice `auth` running...");
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, router).await.unwrap();

    Ok(())
}

mod api;
mod cli;
mod config;
mod handler;
mod router;

use clap::Parser;
use cli::Command;
use config::get_config;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    match Command::parse() {
        Command::Serve(params) => {
            tracing::debug!("CLI command: {:?}", params);

            let cfg = get_config(params);

            let pool = PgPoolOptions::new()
                .max_connections(cfg.db_max_connections)
                .acquire_timeout(Duration::from_secs(3))
                .connect(&cfg.db_url)
                .await
                .expect("can't connect to database");

            sqlx::migrate!().run(&pool).await?;

            let router = router::v1(pool).layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::default().include_headers(true)),
            );

            let listener = tokio::net::TcpListener::bind(cfg.address).await.unwrap();

            tracing::debug!("listening on {}", listener.local_addr().unwrap());

            axum::serve(listener, router).await.unwrap();
        }
    }

    Ok(())
}

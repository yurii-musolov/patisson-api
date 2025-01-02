mod api;
mod cli;
mod handler;
mod router;

use axum::http::{header::CONTENT_TYPE, HeaderValue};
use clap::Parser;
use cli::Command;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, TraceLayer},
};
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
        Command::Serve(args) => {
            tracing::debug!("CLI command: Serve, args: {:?}", args);

            let pool = PgPoolOptions::new()
                .max_connections(args.db_max_connections)
                .acquire_timeout(Duration::from_secs(3))
                .connect(&args.db_url)
                .await
                .expect("can't connect to database");

            sqlx::migrate!().run(&pool).await?;

            let router = router::v1(pool)
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::default().include_headers(true)),
                )
                .layer(
                    CorsLayer::new()
                        .allow_origin("*".parse::<HeaderValue>().unwrap())
                        .allow_headers([CONTENT_TYPE]),
                );

            let listener = tokio::net::TcpListener::bind(args.http_bind).await.unwrap();

            tracing::debug!("listening on {}", listener.local_addr().unwrap());

            axum::serve(listener, router).await.unwrap();
        }
    }

    Ok(())
}

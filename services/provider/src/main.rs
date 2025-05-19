mod application;
mod common;
mod infrastructure;
mod presentation;

use axum::{
    http::{header::CONTENT_TYPE, HeaderValue},
    routing::{any, get},
    Router,
};
use bybit::v5::BASE_URL_API_MAINNET_1;
use clap::Parser;
use std::sync::Arc;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use application::Application;
use infrastructure::{BinanceExchange, BybitExchange};
use presentation::{get_candles, get_symbols, get_trades, websocket_handler, Command, Serve};

type App = Application<BinanceExchange, BybitExchange>;

#[tokio::main]
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
        Command::Serve(args) => command_serve(args).await?,
    }

    Ok(())
}

async fn command_serve(args: Serve) -> anyhow::Result<()> {
    tracing::info!("CLI command: Serve, args: {:?}", args);

    let client_bybit = bybit::v5::Client::new(BASE_URL_API_MAINNET_1);

    let binance = BinanceExchange::new();
    let bybit = BybitExchange::new(client_bybit);

    let application = Application::new(binance, bybit);
    let application = Arc::new(application);

    let v1 = Router::new()
        .route("/symbols/{exchange}/{schema}", get(get_symbols::<App>))
        .route(
            "/candles/{exchange}/{schema}/{symbol}/{interval}",
            get(get_candles::<App>),
        )
        .route(
            "/trades/{exchange}/{schema}/{symbol}",
            get(get_trades::<App>),
        )
        .route("/websocket", any(websocket_handler::<App>));

    let router = Router::new()
        .nest("/v1", v1)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_headers([CONTENT_TYPE]),
        )
        .with_state(application);

    let listener = tokio::net::TcpListener::bind(args.http_bind).await?;

    tracing::info!("listening on {}", listener.local_addr()?);

    axum::serve(listener, router).await?;

    Ok(())
}

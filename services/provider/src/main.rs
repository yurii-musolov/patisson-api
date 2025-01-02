mod application;
mod infrastructure;
mod presentation;

use axum::{
    http::{header::CONTENT_TYPE, HeaderValue},
    routing::get,
    Router,
};
use clap::Parser;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use application::Application;
use bybit_sdk::{Client as BybitClient, URL_BASE_API_MAINNET_1};
use infrastructure::{BinanceExchange, BingXExchange, BybitExchange, KrakenExchange, MEXCExchange};
use presentation::{get_candles, get_symbols, get_trades, Command};

type App<'a> =
    Application<BinanceExchange, BingXExchange, BybitExchange<'a>, KrakenExchange, MEXCExchange>;

#[tokio::main]
async fn main() {
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

            let client_bybit = BybitClient::new(URL_BASE_API_MAINNET_1);

            let binance = BinanceExchange::new();
            let bingx = BingXExchange::new();
            let bybit = BybitExchange::new(client_bybit);
            let kraken = KrakenExchange::new();
            let mexc = MEXCExchange::new();

            let application = Application::new(binance, bingx, bybit, kraken, mexc);

            let router = Router::new()
                .route("/symbols/{exchange}/{schema}", get(get_symbols::<App>))
                .route(
                    "/candles/{exchange}/{schema}/{symbol}/{interval}",
                    get(get_candles::<App>),
                )
                .route(
                    "/trades/{exchange}/{schema}/{symbol}",
                    get(get_trades::<App>),
                )
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

            let listener = tokio::net::TcpListener::bind(args.http_bind).await.unwrap();

            tracing::debug!("listening on {}", listener.local_addr().unwrap());

            axum::serve(listener, router).await.unwrap();
        }
    }
}

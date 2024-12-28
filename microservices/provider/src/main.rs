mod application;
mod infrastructure;
mod presentation;

use application::Application;
use axum::{routing::get, Router};
use clap::Parser;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use infrastructure::{BinanceExchange, BingXExchange, BybitExchange, KrakenExchange, MEXCExchange};
use presentation::{get_candles, get_config, get_symbols, get_trades, Command};

type App = Application<BinanceExchange, BingXExchange, BybitExchange, KrakenExchange, MEXCExchange>;

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
        Command::Serve(params) => {
            tracing::debug!("CLI command: {:?}", params);

            let cfg = get_config();
            tracing::debug!("Config: {:?}", cfg);

            let binance = BinanceExchange::new();
            let bingx = BingXExchange::new();
            let bybit = BybitExchange::new();
            let kraken = KrakenExchange::new();
            let mexc = MEXCExchange::new();

            let application = Application::new(binance, bingx, bybit, kraken, mexc);

            let router = Router::new()
                .route("/symbols/:exchange/:schema", get(get_symbols::<App>))
                .route(
                    "/candles/:exchange/:schema/:symbol/:interval",
                    get(get_candles::<App>),
                )
                .route("/trades/:exchange/:schema/:symbol", get(get_trades::<App>))
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::default().include_headers(true)),
                )
                .with_state(application);

            let listener = tokio::net::TcpListener::bind(cfg.http_address)
                .await
                .unwrap();

            tracing::debug!("listening on {}", listener.local_addr().unwrap());

            axum::serve(listener, router).await.unwrap();
        }
    }
}

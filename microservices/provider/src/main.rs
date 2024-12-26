mod application;
mod infrastructure;
mod presentation;

use application::Application;
use axum::{routing::get, Router};
use clap::Parser;
use infrastructure::{BinanceExchange, BingXExchange, BybitExchange, KrakenExchange, MEXCExchange};
use presentation::{get_candles, get_config, get_symbols, Command};

type App = Application<BinanceExchange, BingXExchange, BybitExchange, KrakenExchange, MEXCExchange>;

#[tokio::main]
async fn main() {
    match Command::parse() {
        Command::Serve(params) => {
            println!("CLI command: {:?}", params);

            let cfg = get_config();
            println!("Config: {:?}", cfg);

            let binance = BinanceExchange::new();
            let bingx = BingXExchange::new();
            let bybit = BybitExchange::new();
            let kraken = KrakenExchange::new();
            let mexc = MEXCExchange::new();

            let application = Application::new(binance, bingx, bybit, kraken, mexc);

            let router = Router::new()
                .route("/symbols/:exchange/:schema", get(get_symbols::<App>))
                .route(
                    "/candles/:exchange/:schema/:symbol",
                    get(get_candles::<App>),
                )
                .with_state(application);

            let listener = tokio::net::TcpListener::bind(cfg.http_address)
                .await
                .unwrap();

            println!("listening on {}", listener.local_addr().unwrap());

            axum::serve(listener, router).await.unwrap();
        }
    }
}

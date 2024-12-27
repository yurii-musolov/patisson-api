mod map;

use crate::application::{Candle, Exchanger, GetCandlesParams, Schema, Symbol};
use bybit_sdk::{
    self, Client, GetKLinesParams, GetTickersParams, KLine, Ticker, URL_BASE_API_MAINNET_1,
};
use map::{
    from_kline_row, from_linear_inverse_ticker, from_option_ticker, from_spot_ticker, to_category,
    to_interval,
};

#[derive(Debug, Clone)]
pub struct BybitExchange {}

impl BybitExchange {
    pub fn new() -> Self {
        Self {}
    }
}

impl Exchanger for BybitExchange {
    async fn get_symbols(&self, schema: Schema, symbol: Option<String>) -> Vec<Symbol> {
        // TODO: Make Client outside.
        let client = Client::new(URL_BASE_API_MAINNET_1);
        let result = client
            .get_tickers(GetTickersParams {
                symbol,
                category: to_category(&schema),
                base_coin: None,
                exp_date: None,
            })
            .await;

        match result {
            Ok(response) => match response.result {
                Ticker::Inverse { list } => list.iter().map(from_linear_inverse_ticker).collect(),
                Ticker::Linear { list } => list.iter().map(from_linear_inverse_ticker).collect(),
                Ticker::Option { list } => list.iter().map(from_option_ticker).collect(),
                Ticker::Spot { list } => list.iter().map(from_spot_ticker).collect(),
            },
            Err(_) => {
                vec![]
            }
        }
    }

    async fn get_candles(&self, schema: Schema, params: GetCandlesParams) -> Vec<Candle> {
        // TODO: make Client outside.
        let client = Client::new(URL_BASE_API_MAINNET_1);
        let result = client
            .get_kline(GetKLinesParams {
                category: to_category(&schema),
                symbol: params.symbol,
                interval: to_interval(&params.interval),
                start: params.start,
                end: params.end,
                limit: params.limit,
            })
            .await;

        match result {
            Ok(response) => match response.result {
                KLine::Inverse { list, symbol: _ } => list.iter().map(from_kline_row).collect(),
                KLine::Linear { list, symbol: _ } => list.iter().map(from_kline_row).collect(),
                KLine::Option { list, symbol: _ } => list.iter().map(from_kline_row).collect(),
                KLine::Spot { list, symbol: _ } => list.iter().map(from_kline_row).collect(),
            },
            Err(_) => {
                vec![]
            }
        }
    }
}

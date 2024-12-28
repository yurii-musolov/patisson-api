mod map;

use crate::application::{
    Candle, Exchanger, GetCandlesParams, GetTradesParams, Schema, Symbol, Trade,
};
use bybit_sdk::{
    self, Client, GetKLinesParams, GetTickersParams, GetTradesParams as BybitGetTradesParams,
    KLine, Ticker, Trade as BybitTrade,
};
use map::{
    from_inverse_linear_spot_trade, from_kline_row, from_linear_inverse_ticker, from_option_ticker,
    from_option_trade, from_spot_ticker, to_category, to_interval,
};

#[derive(Clone)]
pub struct BybitExchange<'a> {
    client: Client<'a>,
}

impl<'a> BybitExchange<'a> {
    pub fn new(client: Client<'a>) -> Self {
        Self { client }
    }
}

impl<'a> Exchanger for BybitExchange<'a> {
    async fn get_symbols(&self, schema: Schema, symbol: Option<String>) -> Vec<Symbol> {
        let result = self
            .client
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
            Err(err) => {
                tracing::error!("{err:?}");
                vec![]
            }
        }
    }

    async fn get_candles(&self, schema: Schema, params: GetCandlesParams) -> Vec<Candle> {
        let result = self
            .client
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
            Err(err) => {
                tracing::error!("{err:?}");
                vec![]
            }
        }
    }

    async fn get_trades(&self, schema: Schema, params: GetTradesParams) -> Vec<Trade> {
        let result = self
            .client
            .get_public_recent_trading_history(BybitGetTradesParams {
                category: to_category(&schema),
                symbol: Some(params.symbol),
                base_coin: None,
                option_type: None,
                limit: params.limit,
            })
            .await;

        match result {
            Ok(response) => match response.result {
                BybitTrade::Inverse { list } => {
                    list.iter().map(from_inverse_linear_spot_trade).collect()
                }
                BybitTrade::Linear { list } => {
                    list.iter().map(from_inverse_linear_spot_trade).collect()
                }
                BybitTrade::Option { list } => list.iter().map(from_option_trade).collect(),
                BybitTrade::Spot { list } => {
                    list.iter().map(from_inverse_linear_spot_trade).collect()
                }
            },
            Err(err) => {
                tracing::error!("{err:?}");
                vec![]
            }
        }
    }
}

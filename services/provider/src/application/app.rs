use std::sync::mpsc;
use tokio_util::sync::CancellationToken;

// TODO: Remove import.
use crate::presentation::{IngoingMessage, OutgoingMessage};

use super::{Candle, Exchange, Interval, Schema, Symbol, Trade};

pub trait Exchanger {
    async fn get_symbols(&self, schema: Schema, symbol: Option<String>) -> Vec<Symbol>;
    async fn get_candles(&self, schema: Schema, params: GetCandlesParams) -> Vec<Candle>;
    async fn get_trades(&self, schema: Schema, params: GetTradesParams) -> Vec<Trade>;
}

// This trait is required for 'axum'.
// Placed here to avoid dependency on 'presentation'.
pub trait IApp {
    fn connect(
        &self,
        token: CancellationToken,
        receiver: mpsc::Receiver<IngoingMessage>,
        sender: mpsc::Sender<OutgoingMessage>,
    );

    async fn get_symbols(
        &self,
        exchange: Exchange,
        schema: Schema,
        symbol: Option<String>,
    ) -> Vec<Symbol>;
    async fn get_candles(
        &self,
        exchange: Exchange,
        schema: Schema,
        params: GetCandlesParams,
    ) -> Vec<Candle>;
    async fn get_trades(
        &self,
        exchange: Exchange,
        schema: Schema,
        params: GetTradesParams,
    ) -> Vec<Trade>;
}

pub struct Application<E1, E2, E3, E4, E5>
where
    E1: Exchanger,
    E2: Exchanger,
    E3: Exchanger,
    E4: Exchanger,
    E5: Exchanger,
{
    binance: E1,
    bingx: E2,
    bybit: E3,
    kraken: E4,
    mexc: E5,
}

impl<E1, E2, E3, E4, E5> Application<E1, E2, E3, E4, E5>
where
    E1: Exchanger,
    E2: Exchanger,
    E3: Exchanger,
    E4: Exchanger,
    E5: Exchanger,
{
    pub fn new(binance: E1, bingx: E2, bybit: E3, kraken: E4, mexc: E5) -> Self {
        Self {
            binance,
            bingx,
            bybit,
            kraken,
            mexc,
        }
    }
}

impl<E1, E2, E3, E4, E5> IApp for Application<E1, E2, E3, E4, E5>
where
    E1: Exchanger,
    E2: Exchanger,
    E3: Exchanger,
    E4: Exchanger,
    E5: Exchanger,
{
    fn connect(
        &self,
        _token: CancellationToken,
        _receiver: mpsc::Receiver<IngoingMessage>,
        _sender: mpsc::Sender<OutgoingMessage>,
    ) {
        // TODO:
        // if !cancel_token.is_cancelled() {
        //     cancel_token.cancel();
        // }
        // token.cancel();
    }

    async fn get_symbols(
        &self,
        exchange: Exchange,
        schema: Schema,
        symbol: Option<String>,
    ) -> Vec<Symbol> {
        match exchange {
            Exchange::Binance => self.binance.get_symbols(schema, symbol).await,
            Exchange::BingX => self.bingx.get_symbols(schema, symbol).await,
            Exchange::Bybit => self.bybit.get_symbols(schema, symbol).await,
            Exchange::Kraken => self.kraken.get_symbols(schema, symbol).await,
            Exchange::Mexc => self.mexc.get_symbols(schema, symbol).await,
        }
    }

    async fn get_candles(
        &self,
        exchange: Exchange,
        schema: Schema,
        params: GetCandlesParams,
    ) -> Vec<Candle> {
        match exchange {
            Exchange::Binance => self.binance.get_candles(schema, params).await,
            Exchange::BingX => self.bingx.get_candles(schema, params).await,
            Exchange::Bybit => self.bybit.get_candles(schema, params).await,
            Exchange::Kraken => self.kraken.get_candles(schema, params).await,
            Exchange::Mexc => self.mexc.get_candles(schema, params).await,
        }
    }

    async fn get_trades(
        &self,
        exchange: Exchange,
        schema: Schema,
        params: GetTradesParams,
    ) -> Vec<Trade> {
        match exchange {
            Exchange::Binance => self.binance.get_trades(schema, params).await,
            Exchange::BingX => self.bingx.get_trades(schema, params).await,
            Exchange::Bybit => self.bybit.get_trades(schema, params).await,
            Exchange::Kraken => self.kraken.get_trades(schema, params).await,
            Exchange::Mexc => self.mexc.get_trades(schema, params).await,
        }
    }
}

pub struct GetCandlesParams {
    pub symbol: String,
    pub interval: Interval,
    pub start: Option<u64>,
    pub end: Option<u64>,
    pub limit: Option<u64>,
}

pub struct GetTradesParams {
    pub symbol: String,
    pub limit: Option<u64>,
}

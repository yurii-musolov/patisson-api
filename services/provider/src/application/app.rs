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

pub struct Application<E1, E2>
where
    E1: Exchanger,
    E2: Exchanger,
{
    binance: E1,
    bybit: E2,
}

impl<E1, E2> Application<E1, E2>
where
    E1: Exchanger,
    E2: Exchanger,
{
    pub fn new(binance: E1, bybit: E2) -> Self {
        Self { binance, bybit }
    }
}

impl<E1, E2> IApp for Application<E1, E2>
where
    E1: Exchanger,
    E2: Exchanger,
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
            Exchange::Bybit => self.bybit.get_symbols(schema, symbol).await,
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
            Exchange::Bybit => self.bybit.get_candles(schema, params).await,
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
            Exchange::Bybit => self.bybit.get_trades(schema, params).await,
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

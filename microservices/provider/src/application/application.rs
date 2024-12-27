use std::sync::Arc;

use super::{Candle, Exchange, Interval, Schema, Symbol};

pub trait Exchanger: Send + Sync {
    async fn get_symbols(&self, schema: Schema, symbol: Option<String>) -> Vec<Symbol>;
    async fn get_candles(&self, schema: Schema, params: GetCandlesParams) -> Vec<Candle>;
}

pub trait IApp: Send + Sync {
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
}

#[derive(Debug, Clone)]
pub struct Application<E1, E2, E3, E4, E5>
where
    E1: Exchanger,
    E2: Exchanger,
    E3: Exchanger,
    E4: Exchanger,
    E5: Exchanger,
{
    binance: Arc<E1>,
    bingx: Arc<E2>,
    bybit: Arc<E3>,
    kraken: Arc<E4>,
    mexc: Arc<E5>,
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
            binance: Arc::new(binance),
            bingx: Arc::new(bingx),
            bybit: Arc::new(bybit),
            kraken: Arc::new(kraken),
            mexc: Arc::new(mexc),
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
            Exchange::MEXC => self.mexc.get_symbols(schema, symbol).await,
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
            Exchange::MEXC => self.mexc.get_candles(schema, params).await,
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

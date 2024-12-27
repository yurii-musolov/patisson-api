use crate::application::{Candle, Exchanger, GetCandlesParams, Schema, Symbol};

#[derive(Debug, Clone)]
pub struct KrakenExchange {}

impl KrakenExchange {
    pub fn new() -> Self {
        Self {}
    }
}

impl Exchanger for KrakenExchange {
    async fn get_symbols(&self, schema: Schema, symbol: Option<String>) -> Vec<Symbol> {
        vec![]
    }
    async fn get_candles(&self, schema: Schema, params: GetCandlesParams) -> Vec<Candle> {
        vec![]
    }
}

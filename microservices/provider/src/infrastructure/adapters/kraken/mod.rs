use crate::application::{Candle, Exchanger, Schema, Symbol};

#[derive(Debug, Clone)]
pub struct KrakenExchange {}

impl KrakenExchange {
    pub fn new() -> Self {
        Self {}
    }
}

impl Exchanger for KrakenExchange {
    async fn get_symbols(&self, schema: Schema) -> Vec<Symbol> {
        vec![]
    }
    async fn get_candles(&self, schema: Schema, symbol: String) -> Vec<Candle> {
        vec![]
    }
}

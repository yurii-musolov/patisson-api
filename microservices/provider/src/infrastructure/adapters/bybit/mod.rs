use crate::application::{Candle, Exchanger, Schema, Symbol};

#[derive(Debug, Clone)]
pub struct BybitExchange {}

impl BybitExchange {
    pub fn new() -> Self {
        Self {}
    }
}

impl Exchanger for BybitExchange {
    async fn get_symbols(&self, schema: Schema) -> Vec<Symbol> {
        vec![]
    }
    async fn get_candles(&self, schema: Schema, symbol: String) -> Vec<Candle> {
        vec![]
    }
}

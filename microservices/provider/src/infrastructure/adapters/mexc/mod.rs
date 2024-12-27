use crate::application::{Candle, Exchanger, GetCandlesParams, Schema, Symbol};

#[derive(Debug, Clone)]
pub struct MEXCExchange {}

impl MEXCExchange {
    pub fn new() -> Self {
        Self {}
    }
}

impl Exchanger for MEXCExchange {
    async fn get_symbols(&self, schema: Schema, symbol: Option<String>) -> Vec<Symbol> {
        vec![]
    }
    async fn get_candles(&self, schema: Schema, params: GetCandlesParams) -> Vec<Candle> {
        vec![]
    }
}

use crate::application::{Candle, Exchanger, Schema, Symbol};

#[derive(Debug, Clone)]
pub struct MEXCExchange {}

impl MEXCExchange {
    pub fn new() -> Self {
        Self {}
    }
}

impl Exchanger for MEXCExchange {
    async fn get_symbols(&self, schema: Schema) -> Vec<Symbol> {
        vec![]
    }
    async fn get_candles(&self, schema: Schema, symbol: String) -> Vec<Candle> {
        vec![]
    }
}

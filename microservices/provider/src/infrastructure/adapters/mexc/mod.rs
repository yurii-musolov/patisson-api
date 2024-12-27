use crate::application::{Candle, Exchanger, GetCandlesParams, Schema, Symbol};

#[derive(Debug, Clone)]
pub struct MEXCExchange {}

impl MEXCExchange {
    pub fn new() -> Self {
        Self {}
    }
}

impl Exchanger for MEXCExchange {
    async fn get_symbols(&self, _schema: Schema, _symbol: Option<String>) -> Vec<Symbol> {
        vec![]
    }
    async fn get_candles(&self, _schema: Schema, _params: GetCandlesParams) -> Vec<Candle> {
        vec![]
    }
}

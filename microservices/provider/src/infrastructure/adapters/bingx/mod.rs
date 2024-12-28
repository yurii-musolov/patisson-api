use crate::application::{
    Candle, Exchanger, GetCandlesParams, GetTradesParams, Schema, Symbol, Trade,
};

#[derive(Clone)]
pub struct BingXExchange {}

impl BingXExchange {
    pub fn new() -> Self {
        Self {}
    }
}

impl Exchanger for BingXExchange {
    async fn get_symbols(&self, _schema: Schema, _symbol: Option<String>) -> Vec<Symbol> {
        vec![]
    }
    async fn get_candles(&self, _schema: Schema, _params: GetCandlesParams) -> Vec<Candle> {
        vec![]
    }

    async fn get_trades(&self, _schema: Schema, _params: GetTradesParams) -> Vec<Trade> {
        vec![]
    }
}

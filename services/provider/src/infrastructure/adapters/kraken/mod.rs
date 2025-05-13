use crate::application::{
    Candle, Exchanger, GetCandlesParams, GetTradesParams, Schema, Symbol, Trade,
};

pub struct KrakenExchange {}

impl KrakenExchange {
    pub fn new() -> Self {
        Self {}
    }
}

impl Exchanger for KrakenExchange {
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

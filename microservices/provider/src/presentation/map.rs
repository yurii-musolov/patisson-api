use crate::application::{Candle, Exchange, Schema, Symbol};

use super::{APICandle, APIExchange, APISchema, APISymbol};

pub fn from_api_exchange(v: APIExchange) -> Exchange {
    match v {
        APIExchange::Binance => Exchange::Binance,
        APIExchange::BingX => Exchange::BingX,
        APIExchange::Bybit => Exchange::Bybit,
        APIExchange::Kraken => Exchange::Kraken,
        APIExchange::MEXC => Exchange::MEXC,
    }
}

pub fn from_api_schema(v: APISchema) -> Schema {
    match v {
        APISchema::Futures => Schema::Futures,
        APISchema::FuturesCoin => Schema::FuturesCoin,
        APISchema::FuturesStandard => Schema::FuturesStandard,
        APISchema::FuturesUSDT => Schema::FuturesUSDT,
        APISchema::Inverse => Schema::Inverse,
        APISchema::Linear => Schema::Linear,
        APISchema::Margin => Schema::Margin,
        APISchema::Spot => Schema::Spot,
    }
}

pub fn to_api_symbol(m: &Symbol) -> APISymbol {
    APISymbol {
        symbol: m.symbol.clone(),
        price: m.price,
    }
}

pub fn to_api_candle(m: &Candle) -> APICandle {
    APICandle {
        timestamp: m.timestamp,
        hight: m.hight,
        close: m.close,
        open: m.open,
        low: m.low,
        size: m.size,
    }
}

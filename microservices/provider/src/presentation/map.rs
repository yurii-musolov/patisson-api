use super::{APICandle, APIExchange, APIInterval, APISchema, APISide, APISymbol, APITrade};
use crate::application::{Candle, Exchange, Interval, Schema, Side, Symbol, Trade};

pub fn from_api_exchange(v: &APIExchange) -> Exchange {
    match v {
        APIExchange::Binance => Exchange::Binance,
        APIExchange::BingX => Exchange::BingX,
        APIExchange::Bybit => Exchange::Bybit,
        APIExchange::Kraken => Exchange::Kraken,
        APIExchange::Mexc => Exchange::Mexc,
    }
}

pub fn from_api_schema(v: &APISchema) -> Schema {
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
        last_price: m.last_price,
        mark_price: m.mark_price,
        index_price: m.index_price,
        bid_price: m.bid_price,
        ask_price: m.ask_price,
        volume24h: m.volume24h,
    }
}

pub fn to_api_candle(m: &Candle) -> APICandle {
    APICandle {
        time: m.time,
        hight: m.hight,
        close: m.close,
        open: m.open,
        low: m.low,
        size: m.size,
    }
}

pub fn to_api_side(m: &Side) -> APISide {
    match m {
        Side::Buy => APISide::Buy,
        Side::Sell => APISide::Sell,
    }
}

pub fn to_api_trade(m: &Trade) -> APITrade {
    APITrade {
        symbol: m.symbol.clone(),
        price: m.price,
        size: m.size,
        side: to_api_side(&m.side),
        time: m.time,
    }
}

pub fn from_api_interval(v: &APIInterval) -> Interval {
    match v {
        APIInterval::Minute1 => Interval::Minute1,
        APIInterval::Minute3 => Interval::Minute3,
        APIInterval::Minute5 => Interval::Minute5,
        APIInterval::Minute15 => Interval::Minute15,
        APIInterval::Minute30 => Interval::Minute30,
        APIInterval::Hour1 => Interval::Hour1,
        APIInterval::Hour2 => Interval::Hour2,
        APIInterval::Hour4 => Interval::Hour4,
        APIInterval::Hour6 => Interval::Hour6,
        APIInterval::Hour12 => Interval::Hour12,
        APIInterval::Day1 => Interval::Day1,
        APIInterval::Week1 => Interval::Week1,
        APIInterval::Month1 => Interval::Month1,
    }
}

use bybit::v5::{
    self, Category, InverseLinearSpotTrade, KLineRow, LinearInverseTicker, OptionTicker,
    OptionTrade, SpotTicker,
};

use crate::application::{Candle, Interval, Schema, Side, Symbol, Trade};

pub fn from_linear_inverse_ticker(v: &LinearInverseTicker) -> Symbol {
    Symbol {
        symbol: v.symbol.clone(),
        last_price: v.last_price,
        mark_price: Some(v.mark_price),
        index_price: Some(v.index_price),
        bid_price: v.bid1_price,
        ask_price: v.ask1_price,
        volume24h: v.volume24h,
    }
}

pub fn from_option_ticker(v: &OptionTicker) -> Symbol {
    Symbol {
        symbol: v.symbol.clone(),
        last_price: v.last_price,
        mark_price: Some(v.mark_price),
        index_price: Some(v.index_price),
        bid_price: v.bid1_price,
        ask_price: v.ask1_price,
        volume24h: v.volume24h,
    }
}

pub fn from_spot_ticker(v: &SpotTicker) -> Symbol {
    Symbol {
        symbol: v.symbol.clone(),
        last_price: v.last_price,
        mark_price: None,
        index_price: None,
        bid_price: v.bid1_price,
        ask_price: v.ask1_price,
        volume24h: v.volume24h,
    }
}

pub fn from_kline_row(v: &KLineRow) -> Candle {
    Candle {
        time: v.start_time,
        hight: v.high_price,
        close: v.close_price,
        open: v.open_price,
        low: v.low_price,
        size: v.volume,
    }
}

pub fn from_inverse_linear_spot_trade(v: &InverseLinearSpotTrade) -> Trade {
    Trade {
        symbol: v.symbol.clone(),
        price: v.price,
        size: v.size,
        side: from_side(&v.side),
        time: v.time,
    }
}

pub fn from_option_trade(v: &OptionTrade) -> Trade {
    Trade {
        symbol: v.symbol.clone(),
        price: v.price,
        size: v.size,
        side: from_side(&v.side),
        time: v.time,
    }
}

pub fn from_side(v: &v5::Side) -> Side {
    match v {
        v5::Side::Buy => Side::Buy,
        v5::Side::Sell => Side::Sell,
    }
}

pub fn to_category(m: &Schema) -> Category {
    match m {
        Schema::Inverse => Category::Inverse,
        Schema::Linear => Category::Linear,
        Schema::Spot => Category::Spot,
        _ => unreachable!(),
    }
}

pub fn to_interval(m: &Interval) -> v5::Interval {
    match m {
        Interval::Minute1 => v5::Interval::Minute1,
        Interval::Minute3 => v5::Interval::Minute3,
        Interval::Minute5 => v5::Interval::Minute5,
        Interval::Minute15 => v5::Interval::Minute15,
        Interval::Minute30 => v5::Interval::Minute30,
        Interval::Hour1 => v5::Interval::Hour1,
        Interval::Hour2 => v5::Interval::Hour2,
        Interval::Hour4 => v5::Interval::Hour4,
        Interval::Hour6 => v5::Interval::Hour6,
        Interval::Hour12 => v5::Interval::Hour12,
        Interval::Day1 => v5::Interval::Day1,
        Interval::Week1 => v5::Interval::Week1,
        Interval::Month1 => v5::Interval::Month1,
    }
}

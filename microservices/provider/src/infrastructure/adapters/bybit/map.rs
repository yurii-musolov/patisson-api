use bybit_sdk::{
    self, Category, InverseLinearSpotTrade, KLineRow, LinearInverseTicker, OptionTicker,
    OptionTrade, Side as BybitSide, SpotTicker,
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

pub fn from_side(v: &BybitSide) -> Side {
    match v {
        BybitSide::Buy => Side::Buy,
        BybitSide::Sell => Side::Sell,
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

pub fn to_interval(m: &Interval) -> bybit_sdk::Interval {
    match m {
        Interval::Minute1 => bybit_sdk::Interval::Minute1,
        Interval::Minute3 => bybit_sdk::Interval::Minute3,
        Interval::Minute5 => bybit_sdk::Interval::Minute5,
        Interval::Minute15 => bybit_sdk::Interval::Minute15,
        Interval::Minute30 => bybit_sdk::Interval::Minute30,
        Interval::Hour1 => bybit_sdk::Interval::Minute60,
        Interval::Hour2 => bybit_sdk::Interval::Minute120,
        Interval::Hour4 => bybit_sdk::Interval::Minute240,
        Interval::Hour6 => bybit_sdk::Interval::Minute360,
        Interval::Hour12 => bybit_sdk::Interval::Minute720,
        Interval::Day1 => bybit_sdk::Interval::Day,
        Interval::Week1 => bybit_sdk::Interval::Week,
        Interval::Month1 => bybit_sdk::Interval::Month,
    }
}

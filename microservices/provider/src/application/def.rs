pub type Price = f64;
pub type Size = f64;
pub type Timestamp = u64; // milliseconds.
pub type Volume = f64;

#[derive(Debug, Clone)]
pub enum Exchange {
    Binance,
    BingX,
    Bybit,
    Kraken,
    Mexc,
}
#[derive(Debug, Clone)]
pub enum Schema {
    Futures,
    FuturesCoin,
    FuturesStandard,
    FuturesUSDT,
    Inverse,
    Linear,
    Margin,
    Spot,
}

#[derive(Debug, Clone)]
pub enum Side {
    Sell,
    Buy,
}

#[derive(Debug)]
pub enum Interval {
    Minute1,
    Minute3,
    Minute5,
    Minute15,
    Minute30,
    Hour1,
    Hour2,
    Hour4,
    Hour6,
    Hour12,
    Day1,
    Week1,
    Month1,
}

#[derive(Debug, Clone)]
pub struct Candle {
    pub timestamp: Timestamp,
    pub hight: Price,
    pub close: Price,
    pub open: Price,
    pub low: Price,
    pub size: Size,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub symbol: String,
    pub last_price: Price,
    pub mark_price: Option<Price>,
    pub index_price: Option<Price>,
    pub bid_price: Price,
    pub ask_price: Price,
    pub volume24h: Volume,
}

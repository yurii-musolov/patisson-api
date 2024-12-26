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
    MEXC,
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
    pub price: Price,
}

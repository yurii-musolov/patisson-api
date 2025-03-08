use crate::Interval;

pub fn topic_ticker(symbol: &str) -> String {
    format!("tickers.{symbol}")
}

pub fn topic_trade(symbol: &str) -> String {
    format!("publicTrade.{symbol}")
}

pub fn topic_kline(symbol: &str, interval: Interval) -> String {
    format!("kline.{interval}.{symbol}")
}

pub fn topic_all_liquidation(symbol: &str) -> String {
    format!("allLiquidation.{symbol}")
}

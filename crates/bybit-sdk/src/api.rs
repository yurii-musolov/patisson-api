use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_number_from_string as number;

use crate::enums::{Category, Interval};

#[derive(Debug, Deserialize)]
pub struct Response<T> {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: T,
    pub time: i64,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: RetExtInfo,
}

#[derive(Debug, Deserialize)]
pub struct RetExtInfo {}

#[derive(Debug, Serialize)]
pub struct GetKLinesParams {
    pub category: Category,
    pub symbol: String,
    pub interval: Interval,
    pub start: Option<u64>,
    pub end: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "category")]
pub enum KLine {
    #[serde(rename = "inverse")]
    Inverse { symbol: String, list: Vec<KLineRow> },
    #[serde(rename = "linear")]
    Linear { symbol: String, list: Vec<KLineRow> },
    #[serde(rename = "option")]
    Option { symbol: String, list: Vec<KLineRow> },
    #[serde(rename = "spot")]
    Spot { symbol: String, list: Vec<KLineRow> },
}

#[derive(Debug, Deserialize)]
pub struct KLineRow {
    #[serde(rename = "startTime", deserialize_with = "number")]
    pub start_time: u64, // Start time of the candle (ms)
    #[serde(rename = "openPrice", deserialize_with = "number")]
    pub open_price: f64, // Open price
    #[serde(rename = "highPrice", deserialize_with = "number")]
    pub high_price: f64, // Highest price
    #[serde(rename = "lowPrice", deserialize_with = "number")]
    pub low_price: f64, // Lowest price
    #[serde(rename = "closePrice", deserialize_with = "number")]
    pub close_price: f64, // Close price. Is the last traded price when the candle is not closed
    #[serde(rename = "volume", deserialize_with = "number")]
    pub volume: f64, // Trade volume. Unit of contract: pieces of contract. Unit of spot: quantity of coins
    #[serde(rename = "turnover", deserialize_with = "number")]
    pub turnover: f64, // Turnover. Unit of figure: quantity of quota coin
}

#[derive(Debug, Serialize)]
pub struct GetTickersParams {
    pub category: Category,
    pub symbol: Option<String>,
    pub base_coin: Option<String>,
    pub exp_date: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "category")]
pub enum Ticker {
    #[serde(rename = "inverse")]
    Inverse { list: Vec<LinearInverseTicker> },
    #[serde(rename = "linear")]
    Linear { list: Vec<LinearInverseTicker> },
    #[serde(rename = "option")]
    Option { list: Vec<OptionTicker> },
    #[serde(rename = "spot")]
    Spot { list: Vec<SpotTicker> },
}

#[derive(Debug, Deserialize)]
pub struct LinearInverseTicker {
    pub symbol: String, // Symbol name
    #[serde(rename = "lastPrice", deserialize_with = "number")]
    pub last_price: f64, // Last price
    #[serde(rename = "markPrice", deserialize_with = "number")]
    pub mark_price: f64, // Mark price
    #[serde(rename = "indexPrice", deserialize_with = "number")]
    pub index_price: f64, // Index price
    #[serde(rename = "prevPrice24h", deserialize_with = "number")]
    pub prev_price24h: f64, // Market price 24 hours ago
    #[serde(rename = "price24hPcnt", deserialize_with = "number")]
    pub price24h_pcnt: f64, // Percentage change of market price in the last 24 hours
    #[serde(rename = "highPrice24h", deserialize_with = "number")]
    pub high_price24h: f64, // The highest price in the last 24 hours
    #[serde(rename = "lowPrice24h", deserialize_with = "number")]
    pub low_price24h: f64, // The lowest price in the last 24 hours
    #[serde(rename = "prevPrice1h", deserialize_with = "number")]
    pub prev_price1h: f64, // Market price an hour ago
    #[serde(rename = "openInterest", deserialize_with = "number")]
    pub open_interest: f64, // Open interest size
    #[serde(rename = "openInterestValue", deserialize_with = "number")]
    pub open_interest_value: f64, // Open interest value
    #[serde(rename = "turnover24h", deserialize_with = "number")]
    pub turnover24h: f64, // Turnover for 24h
    #[serde(rename = "volume24h", deserialize_with = "number")]
    pub volume24h: f64, // Volume for 24h
    #[serde(rename = "fundingRate", deserialize_with = "number")]
    pub funding_rate: f64, // Funding rate
    #[serde(rename = "nextFundingTime", deserialize_with = "number")]
    pub next_funding_time: u64, // Next funding timestamp (ms)
    #[serde(rename = "predictedDeliveryPrice")]
    pub predicted_delivery_price: String, // Predicated delivery price. It has value when 30 min before delivery
    #[serde(rename = "basisRate")]
    pub basis_rate: String, // Basis rate. Unique field for inverse futures & USDC futures
    #[serde(rename = "basis")]
    pub basis: String, // Basis. Unique field for inverse futures & USDC futures
    #[serde(rename = "deliveryFeeRate")]
    pub delivery_fee_rate: String, // Delivery fee rate. Unique field for inverse futures & USDC futures
    #[serde(rename = "deliveryTime")]
    pub delivery_time: String, // Delivery date time (UTC+0). Unique field for inverse futures & USDC futures
    #[serde(rename = "bid1Price", deserialize_with = "number")]
    pub bid1_price: f64, // Best bid price
    #[serde(rename = "bid1Size", deserialize_with = "number")]
    pub bid1_size: f64, // Best bid size
    #[serde(rename = "ask1Price", deserialize_with = "number")]
    pub ask1_price: f64, // Best ask price
    #[serde(rename = "ask1Size", deserialize_with = "number")]
    pub ask1_size: f64, // Best ask size
    #[serde(rename = "preOpenPrice")]
    pub pre_open_price: String, // Estimated pre-market contract open price. The value is meaningless when entering continuous trading phase.
    #[serde(rename = "preQty")]
    pub pre_qty: String, // Estimated pre-market contract open qty. The value is meaningless when entering continuous trading phase.
    #[serde(rename = "curPreListingPhase")]
    pub cur_pre_listing_phase: String, // Enum: NotStarted, Finished, CallAuction, CallAuctionNoCancel, CrossMatching, ContinuousTrading.
}

#[derive(Debug, Deserialize)]
pub struct OptionTicker {
    pub symbol: String, // Symbol name
    #[serde(rename = "bid1Price", deserialize_with = "number")]
    pub bid1_price: f64, // Best bid price
    #[serde(rename = "bid1Size", deserialize_with = "number")]
    pub bid1_size: f64, // Best bid size
    #[serde(rename = "bid1Iv", deserialize_with = "number")]
    pub bid1_iv: f64, // Best bid iv
    #[serde(rename = "ask1Price", deserialize_with = "number")]
    pub ask1_price: f64, // Best ask price
    #[serde(rename = "ask1Size", deserialize_with = "number")]
    pub ask1_size: f64, // Best ask size
    #[serde(rename = "ask1Iv", deserialize_with = "number")]
    pub ask1_iv: f64, // Best ask iv
    #[serde(rename = "lastPrice", deserialize_with = "number")]
    pub last_price: f64, // Last price
    #[serde(rename = "highPrice24h", deserialize_with = "number")]
    pub high_price24h: f64, // The highest price in the last 24 hours
    #[serde(rename = "lowPrice24h", deserialize_with = "number")]
    pub low_price24h: f64, // The lowest price in the last 24 hours
    #[serde(rename = "markPrice", deserialize_with = "number")]
    pub mark_price: f64, // Mark price
    #[serde(rename = "indexPrice", deserialize_with = "number")]
    pub index_price: f64, // Index price
    #[serde(rename = "markIv", deserialize_with = "number")]
    pub mark_iv: f64, // Mark price iv
    #[serde(rename = "underlyingPrice", deserialize_with = "number")]
    pub underlying_price: f64, // Underlying price
    #[serde(rename = "openInterest", deserialize_with = "number")]
    pub open_interest: f64, // Open interest size
    #[serde(rename = "turnover24h", deserialize_with = "number")]
    pub turnover24h: f64, // Turnover for 24h
    #[serde(rename = "volume24h", deserialize_with = "number")]
    pub volume24h: f64, // Volume for 24h
    #[serde(rename = "totalVolume", deserialize_with = "number")]
    pub total_volume: f64, // Total volume
    #[serde(rename = "totalTurnover", deserialize_with = "number")]
    pub total_turnover: f64, // Total turnover
    #[serde(rename = "delta", deserialize_with = "number")]
    pub delta: f64, // Delta
    #[serde(rename = "gamma", deserialize_with = "number")]
    pub gamma: f64, // Gamma
    #[serde(rename = "vega", deserialize_with = "number")]
    pub vega: f64, // Vega
    #[serde(rename = "theta", deserialize_with = "number")]
    pub theta: f64, // Theta
    #[serde(rename = "predictedDeliveryPrice", deserialize_with = "number")]
    pub predicted_delivery_price: f64, // Predicated delivery price. It has value when 30 min before delivery
    #[serde(rename = "change24h", deserialize_with = "number")]
    pub change24h: f64, // The change in the last 24 hous
}

#[derive(Debug, Deserialize)]
pub struct SpotTicker {
    pub symbol: String, // Symbol name
    #[serde(rename = "bid1Price", deserialize_with = "number")]
    pub bid1_price: f64, // Best bid price
    #[serde(rename = "bid1Size", deserialize_with = "number")]
    pub bid1_size: f64, // Best bid size
    #[serde(rename = "ask1Price", deserialize_with = "number")]
    pub ask1_price: f64, // Best ask price
    #[serde(rename = "ask1Size", deserialize_with = "number")]
    pub ask1_size: f64, // Best ask size
    #[serde(rename = "lastPrice", deserialize_with = "number")]
    pub last_price: f64, // Last price
    #[serde(rename = "prevPrice24h", deserialize_with = "number")]
    pub prev_price24h: f64, // Market price 24 hours ago
    #[serde(rename = "price24hPcnt", deserialize_with = "number")]
    pub price24h_pcnt: f64, // Percentage change of market price in the last 24 hours
    #[serde(rename = "highPrice24h", deserialize_with = "number")]
    pub high_price24h: f64, // The highest price in the last 24 hours
    #[serde(rename = "lowPrice24h", deserialize_with = "number")]
    pub low_price24h: f64, // The lowest price in the last 24 hours
    #[serde(rename = "turnover24h", deserialize_with = "number")]
    pub turnover24h: f64, // Turnover for 24h
    #[serde(rename = "volume24h", deserialize_with = "number")]
    pub volume24h: f64, // Volume for 24h
    // USD index price
    // - used to calculate USD value of the assets in Unified account
    // - non-collateral margin coin returns ""
    // - Only those trading pairs like "XXX/USDT" or "XXX/USDC" have the value
    #[serde(rename = "usdIndexPrice", deserialize_with = "number")]
    pub usd_index_price: f64,
}

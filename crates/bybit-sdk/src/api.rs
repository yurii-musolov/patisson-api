use serde::{Deserialize, Serialize};
use serde_aux::prelude::{
    deserialize_number_from_string as number,
    deserialize_option_number_from_string as option_number,
};

use crate::{Category, ContractType, CopyTrading, CurAuctionPhase, Interval, Side, Status};

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

#[derive(Serialize)]
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

#[derive(Serialize)]
pub struct GetInstrumentsInfoParams {
    pub category: Category,
    pub symbol: Option<String>,
    pub status: Option<Status>,
    pub base_coin: Option<String>,
    pub limit: Option<i64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "category")]
pub enum InstrumentsInfo {
    #[serde(rename = "inverse", rename_all = "camelCase")]
    Inverse {
        next_page_cursor: String,
        list: Vec<AllCategoriesInstrumentsInfo>,
    },
    #[serde(rename = "linear", rename_all = "camelCase")]
    Linear {
        next_page_cursor: String,
        list: Vec<AllCategoriesInstrumentsInfo>,
    },
    #[serde(rename = "option", rename_all = "camelCase")]
    Option {
        next_page_cursor: String,
        list: Vec<AllCategoriesInstrumentsInfo>,
    },
    #[serde(rename = "spot", rename_all = "camelCase")]
    Spot {
        next_page_cursor: String,
        list: Vec<AllCategoriesInstrumentsInfo>,
    },
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AllCategoriesInstrumentsInfo {
    pub symbol: String,
    pub contract_type: ContractType,
    pub status: Status,
    pub base_coin: String,
    pub quote_coin: String,
    #[serde(deserialize_with = "number")]
    pub launch_time: i64,
    #[serde(deserialize_with = "number")]
    pub delivery_time: i64,
    #[serde(deserialize_with = "option_number")]
    pub delivery_fee_rate: Option<f64>,
    #[serde(deserialize_with = "number")]
    pub price_scale: i64,
    pub leverage_filter: LeverageFilter,
    pub price_filter: PriceFilter,
    pub lot_size_filter: LotSizeFilter,
    pub unified_margin_trade: bool,
    pub funding_interval: i64,
    pub settle_coin: String,
    pub copy_trading: CopyTrading,
    #[serde(deserialize_with = "number")]
    pub upper_funding_rate: f64,
    #[serde(deserialize_with = "number")]
    pub lower_funding_rate: f64,
    pub risk_parameters: RiskParameters,
    pub is_pre_listing: bool,
    pub pre_listing_info: Option<PreListingInfo>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LeverageFilter {
    #[serde(deserialize_with = "number")]
    pub min_leverage: f64,
    #[serde(deserialize_with = "number")]
    pub max_leverage: f64,
    #[serde(deserialize_with = "number")]
    pub leverage_step: f64,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PriceFilter {
    #[serde(deserialize_with = "number")]
    pub min_price: f64,
    #[serde(deserialize_with = "number")]
    pub max_price: f64,
    #[serde(deserialize_with = "number")]
    pub tick_size: f64,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LotSizeFilter {
    #[serde(deserialize_with = "number")]
    pub min_notional_value: f64,
    #[serde(deserialize_with = "number")]
    pub max_order_qty: f64,
    #[serde(deserialize_with = "number")]
    pub max_mkt_order_qty: f64,
    #[serde(deserialize_with = "number")]
    pub min_order_qty: f64,
    #[serde(deserialize_with = "number")]
    pub qty_step: f64,
    #[serde(deserialize_with = "number")]
    pub post_only_max_order_qty: f64,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RiskParameters {
    #[serde(deserialize_with = "number")]
    pub price_limit_ratio_x: f64,
    #[serde(deserialize_with = "number")]
    pub price_limit_ratio_y: f64,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PreListingInfo {
    pub cur_auction_phase: CurAuctionPhase,
    pub phases: Vec<Phase>,
    pub auction_fee_info: AuctionFeeInfo,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Phase {
    pub phase: CurAuctionPhase,
    #[serde(deserialize_with = "option_number")]
    pub start_time: Option<i64>,
    #[serde(deserialize_with = "option_number")]
    pub end_time: Option<i64>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AuctionFeeInfo {
    #[serde(deserialize_with = "number")]
    pub auction_fee_rate: f64,
    #[serde(deserialize_with = "number")]
    pub taker_fee_rate: f64,
    #[serde(deserialize_with = "number")]
    pub maker_fee_rate: f64,
}

#[derive(Serialize)]
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
#[serde(rename_all = "camelCase")]
pub struct LinearInverseTicker {
    pub symbol: String, // Symbol name
    #[serde(deserialize_with = "number")]
    pub last_price: f64, // Last price
    #[serde(deserialize_with = "number")]
    pub mark_price: f64, // Mark price
    #[serde(deserialize_with = "number")]
    pub index_price: f64, // Index price
    #[serde(deserialize_with = "number")]
    pub prev_price24h: f64, // Market price 24 hours ago
    #[serde(deserialize_with = "number")]
    pub price24h_pcnt: f64, // Percentage change of market price in the last 24 hours
    #[serde(deserialize_with = "number")]
    pub high_price24h: f64, // The highest price in the last 24 hours
    #[serde(deserialize_with = "number")]
    pub low_price24h: f64, // The lowest price in the last 24 hours
    #[serde(deserialize_with = "number")]
    pub prev_price1h: f64, // Market price an hour ago
    #[serde(deserialize_with = "number")]
    pub open_interest: f64, // Open interest size
    #[serde(deserialize_with = "number")]
    pub open_interest_value: f64, // Open interest value
    #[serde(deserialize_with = "number")]
    pub turnover24h: f64, // Turnover for 24h
    #[serde(deserialize_with = "number")]
    pub volume24h: f64, // Volume for 24h
    #[serde(deserialize_with = "number")]
    pub funding_rate: f64, // Funding rate
    #[serde(deserialize_with = "number")]
    pub next_funding_time: u64, // Next funding timestamp (ms)
    pub predicted_delivery_price: String, // Predicated delivery price. It has value when 30 min before delivery
    pub basis_rate: String, // Basis rate. Unique field for inverse futures & USDC futures
    pub basis: String,      // Basis. Unique field for inverse futures & USDC futures
    pub delivery_fee_rate: String, // Delivery fee rate. Unique field for inverse futures & USDC futures
    pub delivery_time: String, // Delivery date time (UTC+0). Unique field for inverse futures & USDC futures
    #[serde(deserialize_with = "number")]
    pub bid1_price: f64, // Best bid price
    #[serde(deserialize_with = "number")]
    pub bid1_size: f64, // Best bid size
    #[serde(deserialize_with = "number")]
    pub ask1_price: f64, // Best ask price
    #[serde(deserialize_with = "number")]
    pub ask1_size: f64, // Best ask size
    pub pre_open_price: String, // Estimated pre-market contract open price. The value is meaningless when entering continuous trading phase.
    pub pre_qty: String, // Estimated pre-market contract open qty. The value is meaningless when entering continuous trading phase.
    pub cur_pre_listing_phase: String, // Enum: NotStarted, Finished, CallAuction, CallAuctionNoCancel, CrossMatching, ContinuousTrading.
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionTicker {
    pub symbol: String, // Symbol name
    #[serde(deserialize_with = "number")]
    pub bid1_price: f64, // Best bid price
    #[serde(deserialize_with = "number")]
    pub bid1_size: f64, // Best bid size
    #[serde(deserialize_with = "number")]
    pub bid1_iv: f64, // Best bid iv
    #[serde(deserialize_with = "number")]
    pub ask1_price: f64, // Best ask price
    #[serde(deserialize_with = "number")]
    pub ask1_size: f64, // Best ask size
    #[serde(deserialize_with = "number")]
    pub ask1_iv: f64, // Best ask iv
    #[serde(deserialize_with = "number")]
    pub last_price: f64, // Last price
    #[serde(deserialize_with = "number")]
    pub high_price24h: f64, // The highest price in the last 24 hours
    #[serde(deserialize_with = "number")]
    pub low_price24h: f64, // The lowest price in the last 24 hours
    #[serde(deserialize_with = "number")]
    pub mark_price: f64, // Mark price
    #[serde(deserialize_with = "number")]
    pub index_price: f64, // Index price
    #[serde(deserialize_with = "number")]
    pub mark_iv: f64, // Mark price iv
    #[serde(deserialize_with = "number")]
    pub underlying_price: f64, // Underlying price
    #[serde(deserialize_with = "number")]
    pub open_interest: f64, // Open interest size
    #[serde(deserialize_with = "number")]
    pub turnover24h: f64, // Turnover for 24h
    #[serde(deserialize_with = "number")]
    pub volume24h: f64, // Volume for 24h
    #[serde(deserialize_with = "number")]
    pub total_volume: f64, // Total volume
    #[serde(deserialize_with = "number")]
    pub total_turnover: f64, // Total turnover
    #[serde(deserialize_with = "number")]
    pub delta: f64, // Delta
    #[serde(deserialize_with = "number")]
    pub gamma: f64, // Gamma
    #[serde(deserialize_with = "number")]
    pub vega: f64, // Vega
    #[serde(deserialize_with = "number")]
    pub theta: f64, // Theta
    #[serde(deserialize_with = "number")]
    pub predicted_delivery_price: f64, // Predicated delivery price. It has value when 30 min before delivery
    #[serde(deserialize_with = "number")]
    pub change24h: f64, // The change in the last 24 hous
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotTicker {
    pub symbol: String, // Symbol name
    #[serde(deserialize_with = "number")]
    pub bid1_price: f64, // Best bid price
    #[serde(deserialize_with = "number")]
    pub bid1_size: f64, // Best bid size
    #[serde(deserialize_with = "number")]
    pub ask1_price: f64, // Best ask price
    #[serde(deserialize_with = "number")]
    pub ask1_size: f64, // Best ask size
    #[serde(deserialize_with = "number")]
    pub last_price: f64, // Last price
    #[serde(deserialize_with = "number")]
    pub prev_price24h: f64, // Market price 24 hours ago
    #[serde(deserialize_with = "number")]
    pub price24h_pcnt: f64, // Percentage change of market price in the last 24 hours
    #[serde(deserialize_with = "number")]
    pub high_price24h: f64, // The highest price in the last 24 hours
    #[serde(deserialize_with = "number")]
    pub low_price24h: f64, // The lowest price in the last 24 hours
    #[serde(deserialize_with = "number")]
    pub turnover24h: f64, // Turnover for 24h
    #[serde(deserialize_with = "number")]
    pub volume24h: f64, // Volume for 24h
    // USD index price
    // - used to calculate USD value of the assets in Unified account
    // - non-collateral margin coin returns ""
    // - Only those trading pairs like "XXX/USDT" or "XXX/USDC" have the value
    #[serde(deserialize_with = "number")]
    pub usd_index_price: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTradesParams {
    pub category: Category,
    // required for spot/linear/inverse
    // optional for option
    pub symbol: Option<String>,
    // Apply to option only
    // If the field is not passed, return BTC data by default
    pub base_coin: Option<String>,
    // optionType	false	string	Option type. Call or Put. Apply to option only
    pub option_type: Option<u64>,
    // spot: [1,60], default: 60
    // others: [1,1000], default: 500
    pub limit: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "category")]
pub enum Trade {
    #[serde(rename = "inverse")]
    Inverse { list: Vec<InverseLinearSpotTrade> },
    #[serde(rename = "linear")]
    Linear { list: Vec<InverseLinearSpotTrade> },
    #[serde(rename = "option")]
    Option { list: Vec<OptionTrade> },
    #[serde(rename = "spot")]
    Spot { list: Vec<InverseLinearSpotTrade> },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InverseLinearSpotTrade {
    pub exec_id: String,
    pub symbol: String,
    #[serde(deserialize_with = "number")]
    pub price: f64,
    #[serde(deserialize_with = "number")]
    pub size: f64,
    pub side: Side,
    #[serde(deserialize_with = "number")]
    pub time: u64,
    pub is_block_trade: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionTrade {
    pub exec_id: String,
    pub symbol: String,
    #[serde(deserialize_with = "number")]
    pub price: f64,
    #[serde(deserialize_with = "number")]
    pub size: f64,
    pub side: Side,
    #[serde(deserialize_with = "number")]
    pub time: u64,
    pub is_block_trade: bool,
    #[serde(rename = "mP", deserialize_with = "number")]
    pub mark_price: f64,
    #[serde(rename = "iP", deserialize_with = "number")]
    pub index_price: f64,
    #[serde(rename = "mIv", deserialize_with = "number")]
    pub mark_iv: f64,
    #[serde(rename = "iv", deserialize_with = "number")]
    pub iv: f64,
}

#[cfg(test)]
mod tests {
    use crate::common::deserialize_slice;

    use super::*;

    #[test]
    fn deserialize_incoming_message_instruments_info_trading() {
        // Official USDT Perpetual instrument structure
        let json = r#"{
            "category": "linear",
            "list": [
                {
                    "symbol": "BTCUSDT",
                    "contractType": "LinearPerpetual",
                    "status": "Trading",
                    "baseCoin": "BTC",
                    "quoteCoin": "USDT",
                    "launchTime": "1585526400000",
                    "deliveryTime": "0",
                    "deliveryFeeRate": "",
                    "priceScale": "2",
                    "leverageFilter": {
                        "minLeverage": "1",
                        "maxLeverage": "100.00",
                        "leverageStep": "0.01"
                    },
                    "priceFilter": {
                        "minPrice": "0.10",
                        "maxPrice": "1999999.80",
                        "tickSize": "0.10"
                    },
                    "lotSizeFilter": {
                        "maxOrderQty": "1190.000",
                        "minOrderQty": "0.001",
                        "qtyStep": "0.001",
                        "postOnlyMaxOrderQty": "1190.000",
                        "maxMktOrderQty": "500.000",
                        "minNotionalValue": "5"
                    },
                    "unifiedMarginTrade": true,
                    "fundingInterval": 480,
                    "settleCoin": "USDT",
                    "copyTrading": "both",
                    "upperFundingRate": "0.00375",
                    "lowerFundingRate": "-0.00375",
                    "isPreListing": false,
                    "preListingInfo": null,
                    "riskParameters": {
                        "priceLimitRatioX": "0.01",
                        "priceLimitRatioY": "0.02"
                    }
                }
            ],
            "nextPageCursor": ""
        }"#;
        let message: InstrumentsInfo = deserialize_slice(json.as_bytes()).unwrap();
        let expected = InstrumentsInfo::Linear {
            next_page_cursor: String::from(""),
            list: vec![AllCategoriesInstrumentsInfo {
                symbol: String::from("BTCUSDT"),
                contract_type: ContractType::LinearPerpetual,
                status: Status::Trading,
                base_coin: String::from("BTC"),
                quote_coin: String::from("USDT"),
                launch_time: 1585526400000,
                delivery_time: 0,
                delivery_fee_rate: None,
                price_scale: 2,
                leverage_filter: LeverageFilter {
                    min_leverage: 1.0,
                    max_leverage: 100.00,
                    leverage_step: 0.01,
                },
                price_filter: PriceFilter {
                    min_price: 0.10,
                    max_price: 1999999.80,
                    tick_size: 0.10,
                },
                lot_size_filter: LotSizeFilter {
                    min_notional_value: 5.0,
                    max_order_qty: 1190.000,
                    max_mkt_order_qty: 500.000,
                    min_order_qty: 0.001,
                    qty_step: 0.001,
                    post_only_max_order_qty: 1190.000,
                },
                unified_margin_trade: true,
                funding_interval: 480,
                settle_coin: String::from("USDT"),
                copy_trading: CopyTrading::Both,
                upper_funding_rate: 0.00375,
                lower_funding_rate: -0.00375,
                risk_parameters: RiskParameters {
                    price_limit_ratio_x: 0.01,
                    price_limit_ratio_y: 0.02,
                },
                is_pre_listing: false,
                pre_listing_info: None,
            }],
        };
        assert_eq!(message, expected);
    }

    #[test]
    fn deserialize_incoming_message_instruments_info_pre_launch() {
        // Pre-market Perpetual instrument structure
        let json = r#"{
        "category": "linear",
        "list": [
            {
                "symbol": "BIOUSDT",
                "contractType": "LinearPerpetual",
                "status": "PreLaunch",
                "baseCoin": "BIO",
                "quoteCoin": "USDT",
                "launchTime": "1735032510000",
                "deliveryTime": "0",
                "deliveryFeeRate": "",
                "priceScale": "4",
                "leverageFilter": {
                    "minLeverage": "1",
                    "maxLeverage": "5.00",
                    "leverageStep": "0.01"
                },
                "priceFilter": {
                    "minPrice": "0.0001",
                    "maxPrice": "1999.9998",
                    "tickSize": "0.0001"
                },
                "lotSizeFilter": {
                    "maxOrderQty": "70000",
                    "minOrderQty": "1",
                    "qtyStep": "1",
                    "postOnlyMaxOrderQty": "70000",
                    "maxMktOrderQty": "14000",
                    "minNotionalValue": "5"
                },
                "unifiedMarginTrade": true,
                "fundingInterval": 480,
                "settleCoin": "USDT",
                "copyTrading": "none",
                "upperFundingRate": "0.05",
                "lowerFundingRate": "-0.05",
                "isPreListing": true,
                "preListingInfo": {
                    "curAuctionPhase": "ContinuousTrading",
                    "phases": [
                        {
                            "phase": "CallAuction",
                            "startTime": "1735113600000",
                            "endTime": "1735116600000"
                        },
                        {
                            "phase": "CallAuctionNoCancel",
                            "startTime": "1735116600000",
                            "endTime": "1735116900000"
                        },
                        {
                            "phase": "CrossMatching",
                            "startTime": "1735116900000",
                            "endTime": "1735117200000"
                        },
                        {
                            "phase": "ContinuousTrading",
                            "startTime": "1735117200000",
                            "endTime": ""
                        }
                    ],
                    "auctionFeeInfo": {
                        "auctionFeeRate": "0",
                        "takerFeeRate": "0.001",
                        "makerFeeRate": "0.0004"
                    }
                },
                "riskParameters": {
                    "priceLimitRatioX": "0.05",
                    "priceLimitRatioY": "0.1"
                }
            }
        ],
        "nextPageCursor": "first%3DBIOUSDT%26last%3DBIOUSDT"
        }"#;
        let message: InstrumentsInfo = deserialize_slice(json.as_bytes()).unwrap();
        let expected = InstrumentsInfo::Linear {
            next_page_cursor: String::from("first%3DBIOUSDT%26last%3DBIOUSDT"),
            list: vec![AllCategoriesInstrumentsInfo {
                symbol: String::from("BIOUSDT"),
                contract_type: ContractType::LinearPerpetual,
                status: Status::PreLaunch,
                base_coin: String::from("BIO"),
                quote_coin: String::from("USDT"),
                launch_time: 1735032510000,
                delivery_time: 0,
                delivery_fee_rate: None,
                price_scale: 4,
                leverage_filter: LeverageFilter {
                    min_leverage: 1.0,
                    max_leverage: 5.00,
                    leverage_step: 0.01,
                },
                price_filter: PriceFilter {
                    min_price: 0.0001,
                    max_price: 1999.9998,
                    tick_size: 0.0001,
                },
                lot_size_filter: LotSizeFilter {
                    min_notional_value: 5.0,
                    max_order_qty: 70000.0,
                    max_mkt_order_qty: 14000.0,
                    min_order_qty: 1.0,
                    qty_step: 1.0,
                    post_only_max_order_qty: 70000.0,
                },
                unified_margin_trade: true,
                funding_interval: 480,
                settle_coin: String::from("USDT"),
                copy_trading: CopyTrading::None,
                upper_funding_rate: 0.05,
                lower_funding_rate: -0.05,
                risk_parameters: RiskParameters {
                    price_limit_ratio_x: 0.05,
                    price_limit_ratio_y: 0.1,
                },
                is_pre_listing: true,
                pre_listing_info: Some(PreListingInfo {
                    cur_auction_phase: CurAuctionPhase::ContinuousTrading,
                    phases: vec![
                        Phase {
                            phase: CurAuctionPhase::CallAuction,
                            start_time: Some(1735113600000),
                            end_time: Some(1735116600000),
                        },
                        Phase {
                            phase: CurAuctionPhase::CallAuctionNoCancel,
                            start_time: Some(1735116600000),
                            end_time: Some(1735116900000),
                        },
                        Phase {
                            phase: CurAuctionPhase::CrossMatching,
                            start_time: Some(1735116900000),
                            end_time: Some(1735117200000),
                        },
                        Phase {
                            phase: CurAuctionPhase::ContinuousTrading,
                            start_time: Some(1735117200000),
                            end_time: None,
                        },
                    ],
                    auction_fee_info: AuctionFeeInfo {
                        auction_fee_rate: 0.0,
                        taker_fee_rate: 0.001,
                        maker_fee_rate: 0.0004,
                    },
                }),
            }],
        };
        assert_eq!(message, expected);
    }
}

use serde::{Deserialize, Serialize};

use crate::application::{Price, Size, Timestamp};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum APIExchange {
    #[serde(rename = "binance")]
    Binance,
    #[serde(rename = "bingx")]
    BingX,
    #[serde(rename = "bybit")]
    Bybit,
    #[serde(rename = "kraken")]
    Kraken,
    #[serde(rename = "mexc")]
    MEXC,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum APISchema {
    #[serde(rename = "futures")]
    Futures,
    #[serde(rename = "futures_coin")]
    FuturesCoin,
    #[serde(rename = "futures_standard")]
    FuturesStandard,
    #[serde(rename = "futures_usdt")]
    FuturesUSDT,
    #[serde(rename = "inverse")]
    Inverse,
    #[serde(rename = "linear")]
    Linear,
    #[serde(rename = "margin")]
    Margin,
    #[serde(rename = "spot")]
    Spot,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum APISide {
    #[serde(rename = "sell")]
    Sell,
    #[serde(rename = "buy")]
    Buy,
}

#[derive(Debug, Clone, Serialize)]
pub struct APICandle {
    #[serde(rename = "type")]
    pub timestamp: Timestamp,
    pub hight: Price,
    pub close: Price,
    pub open: Price,
    pub low: Price,
    pub size: Size,
}

#[derive(Debug, Clone, Serialize)]
pub struct APISymbol {
    pub symbol: String,
    pub price: Price,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_api_exchange() {
        let cases = vec![
            (APIExchange::Binance, "\"binance\""),
            (APIExchange::BingX, "\"bingx\""),
            (APIExchange::Bybit, "\"bybit\""),
            (APIExchange::Kraken, "\"kraken\""),
            (APIExchange::MEXC, "\"mexc\""),
        ];

        cases.iter().for_each(|(value, expected)| {
            let serialized = serde_json::to_string(value).unwrap();
            assert_eq!(serialized, *expected);
        });
    }

    #[test]
    fn deserialize_api_exchange() {
        let cases = vec![
            ("\"binance\"", APIExchange::Binance),
            ("\"bingx\"", APIExchange::BingX),
            ("\"bybit\"", APIExchange::Bybit),
            ("\"kraken\"", APIExchange::Kraken),
            ("\"mexc\"", APIExchange::MEXC),
        ];

        cases.iter().for_each(|(value, expected)| {
            let deserialized: APIExchange = serde_json::from_str(value).unwrap();
            assert_eq!(deserialized, *expected);
        });
    }

    #[test]
    fn serialize_api_schema() {
        let cases = vec![
            (APISchema::Futures, "\"futures\""),
            (APISchema::FuturesCoin, "\"futures_coin\""),
            (APISchema::FuturesStandard, "\"futures_standard\""),
            (APISchema::FuturesUSDT, "\"futures_usdt\""),
            (APISchema::Inverse, "\"inverse\""),
            (APISchema::Linear, "\"linear\""),
            (APISchema::Margin, "\"margin\""),
            (APISchema::Spot, "\"spot\""),
        ];

        cases.iter().for_each(|(value, expected)| {
            let serialized = serde_json::to_string(value).unwrap();
            assert_eq!(serialized, *expected);
        });
    }

    #[test]
    fn deserialize_api_schema() {
        let cases = vec![
            ("\"futures\"", APISchema::Futures),
            ("\"futures_coin\"", APISchema::FuturesCoin),
            ("\"futures_standard\"", APISchema::FuturesStandard),
            ("\"futures_usdt\"", APISchema::FuturesUSDT),
            ("\"inverse\"", APISchema::Inverse),
            ("\"linear\"", APISchema::Linear),
            ("\"margin\"", APISchema::Margin),
            ("\"spot\"", APISchema::Spot),
        ];

        cases.iter().for_each(|(value, expected)| {
            let deserialized: APISchema = serde_json::from_str(value).unwrap();
            assert_eq!(deserialized, *expected);
        });
    }

    #[test]
    fn serialize_api_side() {
        let cases = vec![(APISide::Sell, "\"sell\""), (APISide::Buy, "\"buy\"")];

        cases.iter().for_each(|(value, expected)| {
            let serialized = serde_json::to_string(value).unwrap();
            assert_eq!(serialized, *expected);
        });
    }

    #[test]
    fn deserialize_api_side() {
        let cases = vec![("\"sell\"", APISide::Sell), ("\"buy\"", APISide::Buy)];

        cases.iter().for_each(|(value, expected)| {
            let deserialized: APISide = serde_json::from_str(value).unwrap();
            assert_eq!(deserialized, *expected);
        });
    }
}

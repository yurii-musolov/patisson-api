use serde::{Deserialize, Serialize};

use crate::application::{Price, Size, Timestamp, Volume};

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
    Mexc,
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

impl APISchema {
    pub fn is_valid_with(&self, exchange: &APIExchange) -> bool {
        match exchange {
            APIExchange::Binance => matches!(
                self,
                Self::FuturesCoin | Self::FuturesUSDT | Self::Margin | Self::Spot
            ),
            APIExchange::BingX => false,
            APIExchange::Bybit => matches!(self, Self::Inverse | Self::Linear | Self::Spot),
            APIExchange::Kraken => false,
            APIExchange::Mexc => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum APISide {
    #[serde(rename = "sell")]
    Sell,
    #[serde(rename = "buy")]
    Buy,
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum APIInterval {
    #[serde(rename = "1m")]
    Minute1,
    #[serde(rename = "3m")]
    Minute3,
    #[serde(rename = "5m")]
    Minute5,
    #[serde(rename = "15m")]
    Minute15,
    #[serde(rename = "30m")]
    Minute30,
    #[serde(rename = "1h")]
    Hour1,
    #[serde(rename = "2h")]
    Hour2,
    #[serde(rename = "4h")]
    Hour4,
    #[serde(rename = "6h")]
    Hour6,
    #[serde(rename = "12h")]
    Hour12,
    #[serde(rename = "1D")]
    Day1,
    #[serde(rename = "1W")]
    Week1,
    #[serde(rename = "1M")]
    Month1,
}

impl APIInterval {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "1m" => Some(Self::Minute1),
            "3m" => Some(Self::Minute3),
            "5m" => Some(Self::Minute5),
            "15m" => Some(Self::Minute15),
            "30m" => Some(Self::Minute30),
            "1h" => Some(Self::Hour1),
            "2h" => Some(Self::Hour2),
            "4h" => Some(Self::Hour4),
            "6h" => Some(Self::Hour6),
            "12h" => Some(Self::Hour12),
            "1D" => Some(Self::Day1),
            "1W" => Some(Self::Week1),
            "1M" => Some(Self::Month1),
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct GetCandlesQuery {
    pub start: Option<u64>,
    pub end: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct APICandle {
    pub time: Timestamp,
    pub hight: Price,
    pub close: Price,
    pub open: Price,
    pub low: Price,
    pub size: Size,
}

#[derive(Debug, Deserialize)]
pub struct GetSymbolsQuery {
    pub symbol: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct APISymbol {
    pub symbol: String,
    pub last_price: Price,
    pub mark_price: Option<Price>,
    pub index_price: Option<Price>,
    pub bid_price: Price,
    pub ask_price: Price,
    pub volume24h: Volume,
}

#[derive(Debug, Deserialize)]
pub struct GetTradesQuery {
    pub limit: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct APITrade {
    pub symbol: String,
    pub price: Price,
    pub size: Volume,
    pub side: APISide,
    pub time: Timestamp,
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
            (APIExchange::Mexc, "\"mexc\""),
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
            ("\"mexc\"", APIExchange::Mexc),
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

    #[test]
    fn deserialize_api_interval() {
        let cases = vec![
            ("\"1m\"", APIInterval::Minute1),
            ("\"3m\"", APIInterval::Minute3),
            ("\"5m\"", APIInterval::Minute5),
            ("\"15m\"", APIInterval::Minute15),
            ("\"30m\"", APIInterval::Minute30),
            ("\"1h\"", APIInterval::Hour1),
            ("\"2h\"", APIInterval::Hour2),
            ("\"4h\"", APIInterval::Hour4),
            ("\"6h\"", APIInterval::Hour6),
            ("\"12h\"", APIInterval::Hour12),
            ("\"1D\"", APIInterval::Day1),
            ("\"1W\"", APIInterval::Week1),
            ("\"1M\"", APIInterval::Month1),
        ];

        cases.iter().for_each(|(value, expected)| {
            let deserialized: APIInterval = serde_json::from_str(value).unwrap();
            assert_eq!(deserialized, *expected);
        });
    }

    #[test]
    fn deserialize_api_interval_from_str() {
        let cases = vec![
            ("1m", APIInterval::Minute1),
            ("3m", APIInterval::Minute3),
            ("5m", APIInterval::Minute5),
            ("15m", APIInterval::Minute15),
            ("30m", APIInterval::Minute30),
            ("1h", APIInterval::Hour1),
            ("2h", APIInterval::Hour2),
            ("4h", APIInterval::Hour4),
            ("6h", APIInterval::Hour6),
            ("12h", APIInterval::Hour12),
            ("1D", APIInterval::Day1),
            ("1W", APIInterval::Week1),
            ("1M", APIInterval::Month1),
        ];

        cases.iter().for_each(|(value, expected)| {
            let deserialized: APIInterval = APIInterval::from_str(value).unwrap();
            assert_eq!(deserialized, *expected);
        });
    }
}

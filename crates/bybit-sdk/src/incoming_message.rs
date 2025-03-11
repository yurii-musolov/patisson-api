use serde::Deserialize;
use serde_aux::prelude::{
    deserialize_number_from_string as number,
    deserialize_option_number_from_string as option_number,
};

use crate::{Interval, Side, TickDirection};

#[derive(PartialEq, Deserialize, Debug)]
#[serde(untagged)]
pub enum IncomingMessage {
    Command(CommandMsg),
    Ticker(TickerMsg),
    Trade(TradeMsg),
    KLine(KLineMsg),
    AllLiquidation(AllLiquidationMsg),
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(tag = "op")]
pub enum CommandMsg {
    #[serde(rename = "subscribe")]
    Subscribe {
        req_id: Option<String>,
        ret_msg: Option<String>,
        conn_id: String,
        success: Option<bool>,
    },
    #[serde(rename = "unsubscribe")]
    Unsubscribe {
        req_id: Option<String>,
        ret_msg: Option<String>,
        conn_id: String,
        success: Option<bool>,
    },
    #[serde(rename = "auth")]
    Auth {
        req_id: Option<String>,
        ret_msg: Option<String>,
        conn_id: String,
        success: bool,
    },
    #[serde(rename = "pong")]
    Pong {
        req_id: Option<String>,
        ret_msg: Option<String>,
        conn_id: String,
        args: Option<Vec<String>>,
        success: bool,
    },
    #[serde(rename = "ping")]
    Ping {
        req_id: Option<String>,
        ret_msg: Option<String>,
        conn_id: String,
        args: Option<Vec<String>>,
        success: bool,
    },
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum TickerMsg {
    #[serde(rename = "snapshot")]
    Snapshot {
        topic: String,
        #[serde(default, deserialize_with = "option_number")]
        cs: Option<u64>,
        ts: u64,
        data: TickerSnapshotMsg,
    },
    #[serde(rename = "delta")]
    Delta {
        topic: String,
        #[serde(default, deserialize_with = "option_number")]
        cs: Option<u64>,
        ts: u64,
        data: TickerDeltaMsg,
    },
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TickerSnapshotMsg {
    pub symbol: String,
    pub tick_direction: TickDirection,
    #[serde(deserialize_with = "number")]
    pub last_price: f64,
    #[serde(default, deserialize_with = "option_number")]
    pub pre_open_price: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub pre_qty: Option<f64>,
    pub cur_pre_listing_phase: Option<String>,
    #[serde(deserialize_with = "number")]
    pub prev_price24h: f64,
    #[serde(deserialize_with = "number")]
    pub price24h_pcnt: f64,
    #[serde(deserialize_with = "number")]
    pub high_price24h: f64,
    #[serde(deserialize_with = "number")]
    pub low_price24h: f64,
    #[serde(deserialize_with = "number")]
    pub prev_price1h: f64,
    #[serde(deserialize_with = "number")]
    pub mark_price: f64,
    #[serde(deserialize_with = "number")]
    pub index_price: f64,
    #[serde(deserialize_with = "number")]
    pub open_interest: f64,
    #[serde(deserialize_with = "number")]
    pub open_interest_value: f64,
    #[serde(deserialize_with = "number")]
    pub turnover24h: f64,
    #[serde(deserialize_with = "number")]
    pub volume24h: f64,
    #[serde(deserialize_with = "number")]
    pub funding_rate: f64,
    #[serde(deserialize_with = "number")]
    pub next_funding_time: u64,
    #[serde(deserialize_with = "number")]
    pub bid1_price: f64,
    #[serde(deserialize_with = "number")]
    pub bid1_size: f64,
    #[serde(deserialize_with = "number")]
    pub ask1_price: f64,
    #[serde(deserialize_with = "number")]
    pub ask1_size: f64,
    #[serde(default, deserialize_with = "option_number")]
    pub delivery_time: Option<u64>,
    #[serde(default, deserialize_with = "option_number")]
    pub basis_rate: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub delivery_fee_rate: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub predicted_delivery_price: Option<f64>,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TickerDeltaMsg {
    pub symbol: String,
    pub tick_direction: Option<TickDirection>,
    #[serde(default, deserialize_with = "option_number")]
    pub last_price: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub pre_open_price: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub pre_qty: Option<f64>,
    pub cur_pre_listing_phase: Option<String>,
    #[serde(default, deserialize_with = "option_number")]
    pub prev_price24h: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub price24h_pcnt: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub high_price24h: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub low_price24h: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub prev_price1h: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub mark_price: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub index_price: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub open_interest: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub open_interest_value: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub turnover24h: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub volume24h: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub funding_rate: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub next_funding_time: Option<u64>,
    #[serde(default, deserialize_with = "option_number")]
    pub bid1_price: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub bid1_size: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub ask1_price: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub ask1_size: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub delivery_time: Option<u64>,
    #[serde(default, deserialize_with = "option_number")]
    pub basis_rate: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub delivery_fee_rate: Option<f64>,
    #[serde(default, deserialize_with = "option_number")]
    pub predicted_delivery_price: Option<f64>,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum TradeMsg {
    #[serde(rename = "snapshot")]
    Snapshot {
        id: Option<String>,
        topic: String,
        ts: u64,
        data: Vec<TradeSnapshotMsg>,
    },
}

#[derive(PartialEq, Deserialize, Debug)]
pub struct TradeSnapshotMsg {
    #[serde(rename = "T")]
    pub time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "S")]
    pub side: Side,
    #[serde(rename = "v", deserialize_with = "number")]
    pub size: f64,
    #[serde(rename = "p", deserialize_with = "number")]
    pub price: f64,
    #[serde(rename = "L")]
    pub tick_direction: TickDirection,
    #[serde(rename = "i")]
    pub trade_id: String,
    #[serde(rename = "BT")]
    pub block_trade: bool,
    #[serde(rename = "RPI")]
    pub rpi_trade: Option<bool>,
    #[serde(rename = "mP")]
    pub mark_price: Option<String>,
    #[serde(rename = "iP")]
    pub index_price: Option<String>,
    #[serde(rename = "mlv")]
    pub mark_iv: Option<String>,
    #[serde(rename = "iv")]
    pub iv: Option<String>,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum KLineMsg {
    #[serde(rename = "snapshot")]
    Snapshot {
        topic: String,
        ts: u64,
        data: Vec<KLineSnapshotMsg>,
    },
}

#[derive(PartialEq, Deserialize, Debug)]
pub struct KLineSnapshotMsg {
    pub start: u64,
    pub end: u64,
    pub interval: Interval,
    #[serde(deserialize_with = "number")]
    pub open: f64,
    #[serde(deserialize_with = "number")]
    pub close: f64,
    #[serde(deserialize_with = "number")]
    pub high: f64,
    #[serde(deserialize_with = "number")]
    pub low: f64,
    #[serde(deserialize_with = "number")]
    pub volume: f64,
    #[serde(deserialize_with = "number")]
    pub turnover: f64,
    pub confirm: bool,
    pub timestamp: u64,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum AllLiquidationMsg {
    #[serde(rename = "snapshot")]
    Snapshot {
        topic: String,
        ts: u64,
        data: Vec<AllLiquidationSnapshotMsg>,
    },
}

#[derive(PartialEq, Deserialize, Debug)]
pub struct AllLiquidationSnapshotMsg {
    #[serde(rename = "T")]
    pub time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "S")]
    pub side: Side, // When you receive a Buy update, this means that a long position has been liquidated
    #[serde(rename = "v", deserialize_with = "number")]
    pub size: f64,
    #[serde(rename = "p", deserialize_with = "number")]
    pub price: f64,
}

#[cfg(test)]
mod tests {
    use crate::common::deserialize_slice;

    use super::*;

    #[test]
    fn deserialize_incoming_message_command_subscribe() {
        let json = r#"{"success":true,"ret_msg":"","conn_id":"c0c928a4-daab-460d-b186-45e90a10a3d4","req_id":"","op":"subscribe"}"#;
        let message: IncomingMessage = deserialize_slice(json.as_bytes()).unwrap();
        let expected = IncomingMessage::Command(CommandMsg::Subscribe {
            req_id: Some(String::new()),
            ret_msg: Some(String::new()),
            conn_id: String::from("c0c928a4-daab-460d-b186-45e90a10a3d4"),
            success: Some(true),
        });
        assert_eq!(message, expected);
    }

    #[test]
    fn deserialize_incoming_message_command_unsubscribe() {
        let json = r#"{"success":true,"ret_msg":"","conn_id":"c0c928a4-daab-460d-b186-45e90a10a3d4","req_id":"","op":"unsubscribe"}"#;
        let message: IncomingMessage = deserialize_slice(json.as_bytes()).unwrap();
        let expected = IncomingMessage::Command(CommandMsg::Unsubscribe {
            req_id: Some(String::new()),
            ret_msg: Some(String::new()),
            conn_id: String::from("c0c928a4-daab-460d-b186-45e90a10a3d4"),
            success: Some(true),
        });
        assert_eq!(message, expected);
    }

    #[test]
    fn deserialize_incoming_message_ticker_delta() {
        let json = r#"{
		    "topic": "tickers.BTCUSDT",
		    "type": "delta",
		    "data": {
		        "symbol": "BTCUSDT",
		        "tickDirection": "PlusTick",
		        "price24hPcnt": "-0.015895",
		        "lastPrice": "63948.50",
		        "turnover24h": "6793884423.5518",
		        "volume24h": "105991.3760",
		        "bid1Price": "63948.40",
		        "bid1Size": "3.439",
		        "ask1Price": "63948.50",
		        "ask1Size": "2.566"
		    },
		    "cs": 195377749067,
		    "ts": 1718995014034
		}"#;
        let message: IncomingMessage = deserialize_slice(json.as_bytes()).unwrap();
        let expected = IncomingMessage::Ticker(TickerMsg::Delta {
            topic: String::from("tickers.BTCUSDT"),
            cs: Some(195377749067),
            ts: 1718995014034,
            data: TickerDeltaMsg {
                symbol: String::from("BTCUSDT"),
                tick_direction: Some(TickDirection::PlusTick),
                last_price: Some(63948.5),
                pre_open_price: None,
                pre_qty: None,
                cur_pre_listing_phase: None,
                prev_price24h: None,
                price24h_pcnt: Some(-0.015895),
                high_price24h: None,
                low_price24h: None,
                prev_price1h: None,
                mark_price: None,
                index_price: None,
                open_interest: None,
                open_interest_value: None,
                turnover24h: Some(6793884423.5518),
                volume24h: Some(105991.376),
                funding_rate: None,
                next_funding_time: None,
                bid1_price: Some(63948.4),
                bid1_size: Some(3.439),
                ask1_price: Some(63948.5),
                ask1_size: Some(2.566),
                delivery_time: None,
                basis_rate: None,
                delivery_fee_rate: None,
                predicted_delivery_price: None,
            },
        });
        assert_eq!(message, expected);
    }

    #[test]
    fn deserialize_incoming_message_ticker_snapshot() {
        // Category: linear.
        let json = r#"{
		    "topic": "tickers.BTCUSDT",
		    "type": "snapshot",
		    "data": {
                "symbol":"BTCUSDT",
                "tickDirection":"ZeroPlusTick",
                "price24hPcnt":"-0.044555",
                "lastPrice":"84594.40",
                "prevPrice24h":"88539.30",
                "highPrice24h":"89389.90",
                "lowPrice24h":"82055.60",
                "prevPrice1h":"84307.20",
                "markPrice":"84594.00",
                "indexPrice":"84650.47",
                "openInterest":"52903.75",
                "openInterestValue":"4475339827.50",
                "turnover24h":"17166562011.6514",
                "volume24h":"200176.9910",
                "nextFundingTime":"1740643200000",
                "fundingRate":"-0.00016974",
                "bid1Price":"84594.30",
                "bid1Size":"6.777",
                "ask1Price":"84594.40",
                "ask1Size":"0.660",
                "preOpenPrice":"",
                "preQty":"",
                "curPreListingPhase":""
		    },
		    "cs": 337149693308,
		    "ts": 1740622194359
		}"#;
        let message: IncomingMessage = deserialize_slice(json.as_bytes()).unwrap();
        let expected = IncomingMessage::Ticker(TickerMsg::Snapshot {
            topic: String::from("tickers.BTCUSDT"),
            cs: Some(337149693308),
            ts: 1740622194359,
            data: TickerSnapshotMsg {
                symbol: String::from("BTCUSDT"),
                tick_direction: TickDirection::ZeroPlusTick,
                last_price: 84594.40,
                pre_open_price: None,
                pre_qty: None,
                cur_pre_listing_phase: Some(String::from("")),
                prev_price24h: 88539.30,
                price24h_pcnt: -0.044555,
                high_price24h: 89389.90,
                low_price24h: 82055.60,
                prev_price1h: 84307.20,
                mark_price: 84594.00,
                index_price: 84650.47,
                open_interest: 52903.75,
                open_interest_value: 4475339827.50,
                turnover24h: 17166562011.6514,
                volume24h: 200176.9910,
                funding_rate: -0.00016974,
                next_funding_time: 1740643200000,
                bid1_price: 84594.30,
                bid1_size: 6.777,
                ask1_price: 84594.40,
                ask1_size: 0.660,
                delivery_time: None,
                basis_rate: None,
                delivery_fee_rate: None,
                predicted_delivery_price: None,
            },
        });
        assert_eq!(message, expected);
    }

    #[test]
    fn deserialize_incoming_message_trade_snapshot() {
        // Category: linear.
        let json = r#"{
            "topic":"publicTrade.BTCUSDT",
            "type":"snapshot",
            "ts":1741433245359,
            "data":[
                {
                    "T":1741433245357,
                    "s":"BTCUSDT",
                    "S":"Buy",
                    "v":"0.007",
                    "p":"85821.00",
                    "L":"PlusTick",
                    "i":"485eaa70-df6e-5260-bbef-4f7324e3c5d9",
                    "BT":false
                }
            ]
        }"#;
        let message: IncomingMessage = deserialize_slice(json.as_bytes()).unwrap();
        let expected = IncomingMessage::Trade(TradeMsg::Snapshot {
            id: None,
            topic: String::from("publicTrade.BTCUSDT"),
            ts: 1741433245359,
            data: vec![TradeSnapshotMsg {
                time: 1741433245357,
                symbol: String::from("BTCUSDT"),
                side: Side::Buy,
                size: 0.007,
                price: 85821.00,
                tick_direction: TickDirection::PlusTick,
                trade_id: String::from("485eaa70-df6e-5260-bbef-4f7324e3c5d9"),
                block_trade: false,
                rpi_trade: None,
                mark_price: None,
                index_price: None,
                mark_iv: None,
                iv: None,
            }],
        });
        assert_eq!(message, expected);
    }

    #[test]
    fn deserialize_incoming_message_all_liquidation_snapshot() {
        // Category: linear.
        let json = r#"{
            "topic":"allLiquidation.BTCUSDT",
            "type":"snapshot",
            "ts":1741450605553,
            "data":[
                {
                    "T":1741450605236,
                    "s":"BTCUSDT",
                    "S":"Buy",
                    "v":"0.001",
                    "p":"85823.60"
                }
            ]
        }"#;
        let message: IncomingMessage = deserialize_slice(json.as_bytes()).unwrap();
        let expected = IncomingMessage::AllLiquidation(AllLiquidationMsg::Snapshot {
            topic: String::from("allLiquidation.BTCUSDT"),
            ts: 1741450605553,
            data: vec![AllLiquidationSnapshotMsg {
                time: 1741450605236,
                symbol: String::from("BTCUSDT"),
                side: Side::Buy,
                size: 0.001,
                price: 85823.60,
            }],
        });
        assert_eq!(message, expected);
    }
}

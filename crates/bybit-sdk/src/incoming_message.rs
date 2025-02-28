use serde::Deserialize;
use serde_aux::prelude::{deserialize_number_from_string, deserialize_option_number_from_string};

use crate::TickDirection;

#[derive(PartialEq, Deserialize, Debug)]
#[serde(untagged)]
pub enum IncomingMessage {
    Command(CommandMsg),
    Ticker(TickerMsg),
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
        #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
        cs: Option<u64>,
        ts: u64,
        data: TickerSnapshotMsg,
    },
    #[serde(rename = "delta")]
    Delta {
        topic: String,
        #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
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
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub last_price: f64,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub pre_open_price: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub pre_qty: Option<f64>,
    pub cur_pre_listing_phase: Option<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub prev_price24h: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub price24h_pcnt: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub high_price24h: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub low_price24h: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub prev_price1h: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub mark_price: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub index_price: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub open_interest: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub open_interest_value: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub turnover24h: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub volume24h: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub funding_rate: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub next_funding_time: u64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub bid1_price: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub bid1_size: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub ask1_price: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub ask1_size: f64,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub delivery_time: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub basis_rate: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub delivery_fee_rate: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub predicted_delivery_price: Option<f64>,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TickerDeltaMsg {
    pub symbol: String,
    pub tick_direction: Option<TickDirection>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub last_price: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub pre_open_price: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub pre_qty: Option<f64>,
    pub cur_pre_listing_phase: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub prev_price24h: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub price24h_pcnt: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub high_price24h: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub low_price24h: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub prev_price1h: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub mark_price: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub index_price: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub open_interest: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub open_interest_value: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub turnover24h: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub volume24h: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub funding_rate: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub next_funding_time: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub bid1_price: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub bid1_size: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub ask1_price: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub ask1_size: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub delivery_time: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub basis_rate: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub delivery_fee_rate: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub predicted_delivery_price: Option<f64>,
}

#[inline]
pub fn deserialize_incoming_message_slice(message: &[u8]) -> serde_json::Result<IncomingMessage> {
    serde_json::from_slice(message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_incoming_message_command_subscribe() {
        let json = r#"{"success":true,"ret_msg":"","conn_id":"c0c928a4-daab-460d-b186-45e90a10a3d4","req_id":"","op":"subscribe"}"#;
        let message = deserialize_incoming_message_slice(json.as_bytes()).unwrap();
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
        let message = deserialize_incoming_message_slice(json.as_bytes()).unwrap();
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
        let message = deserialize_incoming_message_slice(json.as_bytes()).unwrap();
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
        let message = deserialize_incoming_message_slice(json.as_bytes()).unwrap();
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
}

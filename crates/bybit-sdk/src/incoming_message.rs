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
pub struct TickerSnapshotMsg {
    symbol: String,
    #[serde(rename = "tickDirection")]
    tick_direction: TickDirection,
    #[serde(
        rename = "lastPrice",
        deserialize_with = "deserialize_number_from_string"
    )]
    last_price: f64,
    #[serde(
        rename = "preOpenPrice",
        deserialize_with = "deserialize_number_from_string"
    )]
    pre_open_price: f64,
    #[serde(rename = "preQty", deserialize_with = "deserialize_number_from_string")]
    pre_qty: f64,
    #[serde(rename = "curPreListingPhase")]
    cur_pre_listing_phase: String,
    #[serde(
        rename = "prevPrice24h",
        deserialize_with = "deserialize_number_from_string"
    )]
    prev_price24h: f64,
    #[serde(
        rename = "price24hPcnt",
        deserialize_with = "deserialize_number_from_string"
    )]
    price24h_pcnt: f64,
    #[serde(
        rename = "highPrice24h",
        deserialize_with = "deserialize_number_from_string"
    )]
    high_price24h: f64,
    #[serde(
        rename = "lowPrice24h",
        deserialize_with = "deserialize_number_from_string"
    )]
    low_price24h: f64,
    #[serde(
        rename = "prevPrice1h",
        deserialize_with = "deserialize_number_from_string"
    )]
    prev_price1h: f64,
    #[serde(
        rename = "markPrice",
        deserialize_with = "deserialize_number_from_string"
    )]
    mark_price: f64,
    #[serde(
        rename = "indexPrice",
        deserialize_with = "deserialize_number_from_string"
    )]
    index_price: f64,
    #[serde(
        rename = "openInterest",
        deserialize_with = "deserialize_number_from_string"
    )]
    open_interest: f64,
    #[serde(
        rename = "openInterestValue",
        deserialize_with = "deserialize_number_from_string"
    )]
    open_interest_value: f64,
    #[serde(
        rename = "turnover24h",
        deserialize_with = "deserialize_number_from_string"
    )]
    turnover24h: f64,
    #[serde(
        rename = "volume24h",
        deserialize_with = "deserialize_number_from_string"
    )]
    volume24h: f64,
    #[serde(
        rename = "fundingRate",
        deserialize_with = "deserialize_number_from_string"
    )]
    funding_rate: f64,
    #[serde(
        rename = "nextFundingTime",
        deserialize_with = "deserialize_number_from_string"
    )]
    next_funding_time: u64,
    #[serde(
        rename = "bid1Price",
        deserialize_with = "deserialize_number_from_string"
    )]
    bid1_price: String,
    #[serde(
        rename = "bid1Size",
        deserialize_with = "deserialize_number_from_string"
    )]
    bid1_size: String,
    #[serde(
        rename = "ask1Price",
        deserialize_with = "deserialize_number_from_string"
    )]
    ask1_price: String,
    #[serde(
        rename = "ask1Size",
        deserialize_with = "deserialize_number_from_string"
    )]
    ask1_size: String,
    #[serde(
        rename = "deliveryTime",
        deserialize_with = "deserialize_number_from_string"
    )]
    delivery_time: u64,
    #[serde(
        rename = "basisRate",
        deserialize_with = "deserialize_number_from_string"
    )]
    basis_rate: f64,
    #[serde(
        rename = "deliveryFeeRate",
        deserialize_with = "deserialize_number_from_string"
    )]
    delivery_fee_rate: f64,
    #[serde(
        rename = "predictedDeliveryPrice",
        deserialize_with = "deserialize_number_from_string"
    )]
    predicted_delivery_price: f64,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TickerDeltaMsg {
    symbol: String,
    // #[serde(rename = "tickDirection")]
    tick_direction: Option<TickDirection>,
    #[serde(
		// rename = "lastPrice",
		default,
		deserialize_with = "deserialize_option_number_from_string"
	)]
    last_price: Option<f64>,
    #[serde(
        rename = "preOpenPrice",
        default,
        deserialize_with = "deserialize_option_number_from_string"
    )]
    pre_open_price: Option<f64>,
    #[serde(
        rename = "preQty",
        default,
        deserialize_with = "deserialize_option_number_from_string"
    )]
    pre_qty: Option<f64>,
    #[serde(rename = "curPreListingPhase")]
    cur_pre_listing_phase: Option<String>,
    #[serde(
        rename = "prevPrice24h",
        default,
        deserialize_with = "deserialize_option_number_from_string"
    )]
    prev_price24h: Option<f64>,
    #[serde(
        rename = "price24hPcnt",
        default,
        deserialize_with = "deserialize_option_number_from_string"
    )]
    price24h_pcnt: Option<f64>,
    #[serde(
        rename = "highPrice24h",
        default,
        deserialize_with = "deserialize_option_number_from_string"
    )]
    high_price24h: Option<f64>,
    #[serde(
        rename = "lowPrice24h",
        default,
        deserialize_with = "deserialize_option_number_from_string"
    )]
    low_price24h: Option<f64>,
    #[serde(
        rename = "prevPrice1h",
        default,
        deserialize_with = "deserialize_option_number_from_string"
    )]
    prev_price1h: Option<f64>,
    #[serde(
        rename = "markPrice",
        default,
        deserialize_with = "deserialize_option_number_from_string"
    )]
    mark_price: Option<f64>,
    #[serde(
        rename = "indexPrice",
        default,
        deserialize_with = "deserialize_option_number_from_string"
    )]
    index_price: Option<f64>,
    #[serde(
        rename = "openInterest",
        default,
        deserialize_with = "deserialize_option_number_from_string"
    )]
    open_interest: Option<f64>,
    #[serde(
        rename = "openInterestValue",
        default,
        deserialize_with = "deserialize_option_number_from_string"
    )]
    open_interest_value: Option<f64>,
    #[serde(
        rename = "turnover24h",
        default,
        deserialize_with = "deserialize_option_number_from_string"
    )]
    turnover24h: Option<f64>,
    #[serde(
        rename = "volume24h",
        default,
        deserialize_with = "deserialize_option_number_from_string"
    )]
    volume24h: Option<f64>,
    #[serde(
        rename = "fundingRate",
        default,
        deserialize_with = "deserialize_option_number_from_string"
    )]
    funding_rate: Option<f64>,
    #[serde(
        rename = "nextFundingTime",
        default,
        deserialize_with = "deserialize_option_number_from_string"
    )]
    next_funding_time: Option<u64>,
    #[serde(
        rename = "bid1Price",
        default,
        deserialize_with = "deserialize_option_number_from_string"
    )]
    bid1_price: Option<f64>,
    #[serde(
        rename = "bid1Size",
        default,
        deserialize_with = "deserialize_option_number_from_string"
    )]
    bid1_size: Option<f64>,
    #[serde(
        rename = "ask1Price",
        default,
        deserialize_with = "deserialize_option_number_from_string"
    )]
    ask1_price: Option<f64>,
    #[serde(
        rename = "ask1Size",
        default,
        deserialize_with = "deserialize_option_number_from_string"
    )]
    ask1_size: Option<f64>,
    #[serde(
        rename = "deliveryTime",
        default,
        deserialize_with = "deserialize_option_number_from_string"
    )]
    delivery_time: Option<u64>,
    #[serde(
        rename = "basisRate",
        default,
        deserialize_with = "deserialize_option_number_from_string"
    )]
    basis_rate: Option<f64>,
    #[serde(
        rename = "deliveryFeeRate",
        default,
        deserialize_with = "deserialize_option_number_from_string"
    )]
    delivery_fee_rate: Option<f64>,
    #[serde(
        rename = "predictedDeliveryPrice",
        default,
        deserialize_with = "deserialize_option_number_from_string"
    )]
    predicted_delivery_price: Option<f64>,
}

pub fn deserialize_incoming_message(message: &str) -> serde_json::Result<IncomingMessage> {
    serde_json::from_str(message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_incoming_message_command_subscribe() {
        let json = r#"{"success":true,"ret_msg":"","conn_id":"c0c928a4-daab-460d-b186-45e90a10a3d4","req_id":"","op":"subscribe"}"#;
        let message = deserialize_incoming_message(json).unwrap();
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
        let message = deserialize_incoming_message(json).unwrap();
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
        let message = deserialize_incoming_message(json).unwrap();
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
}

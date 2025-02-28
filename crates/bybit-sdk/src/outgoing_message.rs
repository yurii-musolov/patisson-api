use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(tag = "op")]
pub enum OutgoingMessage {
    #[serde(rename = "subscribe")]
    Subscribe {
        req_id: Option<String>,
        args: Vec<String>,
    },
    #[serde(rename = "unsubscribe")]
    Unsubscribe {
        req_id: Option<String>,
        args: Vec<String>,
    },
    #[serde(rename = "auth")]
    Auth {
        req_id: Option<String>,
        args: (String, i64, String),
    },
    #[serde(rename = "ping")]
    Ping { req_id: Option<String> },
    #[serde(rename = "pong")]
    Pong { req_id: Option<String> },
}

#[inline]
pub fn serialize_outgoing_message<T>(msg: &T) -> serde_json::Result<String>
where
    T: ?Sized + Serialize,
{
    serde_json::to_string(msg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_outgoing_message_subscribe() {
        let msg = OutgoingMessage::Subscribe {
            req_id: Some(String::from("request_id")),
            args: vec![String::from("tickers.BTCUSDT")],
        };
        let expected = r#"{"op":"subscribe","req_id":"request_id","args":["tickers.BTCUSDT"]}"#;
        let serialized = serialize_outgoing_message(&msg).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_serialize_outgoing_message_unsubscribe() {
        let msg = OutgoingMessage::Unsubscribe {
            req_id: Some(String::from("request_id")),
            args: vec![String::from("tickers.BTCUSDT")],
        };
        let expected = r#"{"op":"unsubscribe","req_id":"request_id","args":["tickers.BTCUSDT"]}"#;
        let serialized = serialize_outgoing_message(&msg).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_serialize_outgoing_message_auth() {
        let msg = OutgoingMessage::Auth {
            req_id: Some(String::from("request_id")),
            args: (
                String::from("api_key"),
                1662350400000,
                String::from("signature"),
            ),
        };
        let expected =
            r#"{"op":"auth","req_id":"request_id","args":["api_key",1662350400000,"signature"]}"#;
        let serialized = serialize_outgoing_message(&msg).unwrap();
        assert_eq!(serialized, expected);
    }
}

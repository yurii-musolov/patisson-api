use std::time::Duration;
use tokio::{self, time::sleep};

use bybit_sdk::{
    stream_async, topic_all_liquidation, topic_kline, topic_ticker, topic_trade, AllLiquidationMsg,
    CommandMsg, IncomingMessage, Interval, KLineMsg, OutgoingMessage, TickerMsg, TradeMsg,
    PATH_PUBLIC_LINEAR, URL_BASE_STREAM_MAINNET_1,
};

const WEBSOCKET_PING_INTERVAL: u64 = 10; // Sec.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{URL_BASE_STREAM_MAINNET_1}{PATH_PUBLIC_LINEAR}");
    let message_0001 = OutgoingMessage::Subscribe {
        req_id: Some(String::from("req-0001")),
        args: vec![topic_ticker("BTCUSDT")],
    };
    let message_0002 = OutgoingMessage::Unsubscribe {
        req_id: Some(String::from("req-0002")),
        args: vec![topic_ticker("BTCUSDT")],
    };
    let message_0003 = OutgoingMessage::Subscribe {
        req_id: Some(String::from("req-0003")),
        args: vec![topic_trade("BTCUSDT")],
    };
    let message_0004 = OutgoingMessage::Unsubscribe {
        req_id: Some(String::from("req-0004")),
        args: vec![topic_trade("BTCUSDT")],
    };
    let message_0005 = OutgoingMessage::Subscribe {
        req_id: Some(String::from("req-0005")),
        args: vec![topic_kline("BTCUSDT", Interval::Minute1)],
    };
    let message_0006 = OutgoingMessage::Unsubscribe {
        req_id: Some(String::from("req-0006")),
        args: vec![topic_kline("BTCUSDT", Interval::Minute1)],
    };

    let message_0007 = OutgoingMessage::Subscribe {
        req_id: Some(String::from("req-0007")),
        args: vec![topic_all_liquidation("BTCUSDT")],
    };
    let message_0008 = OutgoingMessage::Unsubscribe {
        req_id: Some(String::from("req-0008")),
        args: vec![topic_all_liquidation("BTCUSDT")],
    };

    let (tx, mut rx) = stream_async(&url, WEBSOCKET_PING_INTERVAL).await?;

    tokio::spawn(async move {
        if let Err(e) = tx.send(message_0001).await {
            println!("Send with error: {e}");
        };

        sleep(Duration::from_secs(5)).await;

        if let Err(e) = tx.send(message_0002).await {
            println!("Send with error: {e}");
        };

        sleep(Duration::from_secs(2)).await;

        if let Err(e) = tx.send(message_0003).await {
            println!("Send with error: {e}");
        };

        sleep(Duration::from_secs(5)).await;

        if let Err(e) = tx.send(message_0004).await {
            println!("Send with error: {e}");
        };

        sleep(Duration::from_secs(2)).await;

        if let Err(e) = tx.send(message_0005).await {
            println!("Send with error: {e}");
        };

        sleep(Duration::from_secs(5)).await;

        if let Err(e) = tx.send(message_0006).await {
            println!("Send with error: {e}");
        };

        sleep(Duration::from_secs(2)).await;

        if let Err(e) = tx.send(message_0007).await {
            println!("Send with error: {e}");
        };

        sleep(Duration::from_secs(3600)).await;

        if let Err(e) = tx.send(message_0008).await {
            println!("Send with error: {e}");
        };
    });

    while let Some(message) = rx.recv().await {
        print(message);
    }

    Ok(())
}

fn print(message: IncomingMessage) {
    match message {
        IncomingMessage::Command(message) => match message {
            CommandMsg::Pong {
                req_id: _,
                ret_msg: _,
                conn_id: _,
                args: _,
                success: _,
            } => {}
            CommandMsg::Ping {
                req_id: _,
                ret_msg: _,
                conn_id: _,
                args: _,
                success: _,
            } => {}
            _ => println!("{message:?}"),
        },
        IncomingMessage::Ticker(message) => match *message {
            TickerMsg::Snapshot {
                topic: _,
                cs: _,
                ts: _,
                data,
            } => println!("{data:?}"),
            TickerMsg::Delta {
                topic: _,
                cs: _,
                ts: _,
                data,
            } => println!("{data:?}"),
        },
        IncomingMessage::Trade(message) => match message {
            TradeMsg::Snapshot {
                id: _,
                topic: _,
                ts: _,
                data,
            } => println!("{data:?}"),
        },
        IncomingMessage::KLine(message) => match message {
            KLineMsg::Snapshot {
                topic: _,
                ts: _,
                data,
            } => println!("{data:?}"),
        },
        IncomingMessage::AllLiquidation(message) => match message {
            AllLiquidationMsg::Snapshot {
                topic: _,
                ts: _,
                data,
            } => println!("{data:?}"),
        },
    }
}

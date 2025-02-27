use bybit_sdk::{
    stream_async, IncomingMessage, OutgoingMessage, TickerMsg, PATH_PUBLIC_LINEAR,
    URL_BASE_STREAM_MAINNET,
};
use std::time::Duration;
use tokio::{self, time::sleep};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{URL_BASE_STREAM_MAINNET}{PATH_PUBLIC_LINEAR}");
    let (tx, mut rx) = stream_async(&url).await?;

    tokio::spawn(async move {
        let message = OutgoingMessage::Subscribe {
            req_id: Some(String::from("req-0001")),
            args: vec![String::from("tickers.BTCUSDT")],
        };
        if let Err(e) = tx.send(message).await {
            println!("Send with error: {e}");
        };

        sleep(Duration::from_secs(1)).await;

        let message = OutgoingMessage::Unsubscribe {
            req_id: Some(String::from("req-0002")),
            args: vec![String::from("tickers.BTCUSDT")],
        };
        if let Err(e) = tx.send(message).await {
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
        IncomingMessage::Command(message) => println!("{message:?}"),
        IncomingMessage::Ticker(message) => match message {
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
    }
}

use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio::{
    self,
    sync::mpsc::{channel, Receiver, Sender},
    time::sleep,
};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{protocol::Message, Utf8Bytes},
};

use crate::{
    common::{deserialize_slice, serialize},
    IncomingMessage, OutgoingMessage,
};

pub async fn stream_async(
    url: &str,
    ping_interval: u64,
) -> anyhow::Result<(Sender<OutgoingMessage>, Receiver<IncomingMessage>)> {
    let (incoming_tx, incoming_rx) = channel::<IncomingMessage>(1);
    let (outgoing_tx, mut outgoing_rx) = channel::<OutgoingMessage>(1);

    let (stream, _) = connect_async(url).await?;
    let (mut sender, mut receiver) = stream.split();

    let handshake = outgoing_tx.clone();
    tokio::spawn(async move {
        let mut count = 0_u64;
        loop {
            sleep(Duration::from_secs(ping_interval)).await;
            count += 1;
            let id = format!("ping-{count}");
            let message = OutgoingMessage::Ping { req_id: Some(id) };
            if let Err(e) = handshake.send(message).await {
                println!("Send ping error: {e}");
                break;
            };
        }
    });

    tokio::spawn(async move {
        while let Some(result) = receiver.next().await {
            match result {
                Ok(message) => match message {
                    Message::Text(slice) => {
                        match deserialize_slice(slice.as_ref()) {
                            Ok(message) => {
                                if let Err(e) = incoming_tx.send(message).await {
                                    println!("[bybit.stream.incoming] Send IncomingMessage failed with: {e}!");
                                }
                            }
                            Err(e) => {
                                println!("[bybit.stream.incoming] Deserialize IncomingMessage failed with: {e}!");
                                println!("[bybit.stream.incoming] DEBUG: IncomingMessage: {slice}");
                            }
                        };
                    }
                    Message::Binary(d) => println!(
                        "[bybit.stream.binary] Binary got {} bytes: {:?}",
                        d.len(),
                        d
                    ),
                    Message::Close(close_frame) => {
                        match close_frame {
                            Some(close_frame) => println!(
                                "[bybit.stream.close] Close got close with code {} and reason `{}`",
                                close_frame.code, close_frame.reason
                            ),
                            None => {
                                println!("[bybit.stream.close] Close somehow got close message without CloseFrame")
                            }
                        }

                        break;
                    }
                    Message::Pong(v) => println!("[bybit.stream.pong] Pong got pong with {v:?}."),
                    Message::Ping(v) => println!("[bybit.stream.ping] Ping got ping with {v:?}."),
                    Message::Frame(_) => {
                        unreachable!("Frame This is never supposed to happen.")
                    }
                },
                Err(e) => println!("[bybit.stream] Receive message failed with: {e}!"),
            }
        }
    });

    tokio::spawn(async move {
        while let Some(message) = outgoing_rx.recv().await {
            let message = match serialize(&message) {
                Ok(serialized) => Message::Text(Utf8Bytes::from(&serialized)),
                Err(e) => {
                    println!("[bybit.stream.outgoing] Serialize OutgoingMessage failed with {e}!");
                    continue;
                }
            };

            if let Err(e) = sender.send(message).await {
                println!("[bybit.stream.outgoing] Send OutgoingMessage failed with {e}!");
            };
        }
    });

    Ok((outgoing_tx, incoming_rx))
}

use futures_util::{SinkExt, StreamExt};
use tokio;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{protocol::Message, Utf8Bytes},
};

use crate::{
    deserialize_incoming_message_slice, serialize_outgoing_message, IncomingMessage,
    OutgoingMessage,
};

pub async fn stream_async(
    url: &str,
) -> anyhow::Result<(Sender<OutgoingMessage>, Receiver<IncomingMessage>)> {
    let (incoming_tx, incoming_rx) = channel::<IncomingMessage>(1);
    let (outgoing_tx, mut outgoing_rx) = channel::<OutgoingMessage>(1);

    let (stream, _) = connect_async(url).await?;
    let (mut sender, mut receiver) = stream.split();

    tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(slice) => {
                    match deserialize_incoming_message_slice(slice.as_ref()) {
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
                        None => 
                        println!("[bybit.stream.close] Close somehow got close message without CloseFrame"),
                    }

                    break;
                }
                Message::Pong(v) => println!("[bybit.stream.pong] Pong got pong with {v:?}"),
                Message::Ping(v) => println!("[bybit.stream.ping] Ping got ping with {v:?}"),
                Message::Frame(_) => {
                    unreachable!("[bybit.stream.frame] Frame This is never supposed to happen")
                }
            }
        }
    });

    tokio::spawn(async move {
        while let Some(message) = outgoing_rx.recv().await {
            let message = match serialize_outgoing_message(&message) {
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

mod api;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

use crate::{
    application::IApp,
    common::{deserialize_slice, serialize},
};
pub use api::{IngoingMessage, OutgoingMessage};

pub async fn websocket_handler<T>(
    ws: WebSocketUpgrade,
    State(state): State<Arc<T>>,
) -> impl IntoResponse
where
    T: IApp + std::marker::Send + std::marker::Sync + 'static,
{
    ws.on_upgrade(move |socket| websocket(socket, state))
}

pub async fn websocket<T>(socket: WebSocket, state: Arc<T>)
where
    T: IApp + std::marker::Send + std::marker::Sync + 'static,
{
    let (mut sender, mut receiver) = socket.split();
    let (in_tx, in_rx) = std::sync::mpsc::channel::<IngoingMessage>();
    let (out_tx, out_rx) = std::sync::mpsc::channel::<OutgoingMessage>();

    let cancel_token = CancellationToken::new();

    tracing::debug!("Connect");
    state.connect(cancel_token.clone(), in_rx, out_tx);

    let token = cancel_token.clone();
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = out_rx.recv() {
            if token.is_cancelled() {
                return;
            }

            let msg = serialize(&msg).unwrap();
            if sender.send(Message::text(msg)).await.is_err() {
                break;
            }
        }
    });

    let token = cancel_token.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(message))) = receiver.next().await {
            if token.is_cancelled() {
                return;
            }

            let message = deserialize_slice(message.as_bytes()).unwrap();
            in_tx.send(message).unwrap();
        }
    });

    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    };

    if !cancel_token.is_cancelled() {
        cancel_token.cancel();
    }
    tracing::debug!("Disconnect");
}

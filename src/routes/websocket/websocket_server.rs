use warp;
use warp::ws::{Message, WebSocket};
use futures::{SinkExt, StreamExt};
use futures::FutureExt;
use futures::stream::{SplitSink, SplitStream};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use crate::TryFutureExt;

pub(crate) async fn websocket_message(ws: WebSocket) {
    let (mut tx_client, mut rx_client) = ws.split();
    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);

    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            tx_client
                .send(message)
                .unwrap_or_else(|e| {
                    eprintln!("websocket send error: {}", e);
                })
                .await;
        }
    });

    while let Some(result) = rx_client.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error {}", e);
                break;
            }
        };
        message_interface(msg)
    }

}

fn message_interface(message: Message) {
    let message = if let Ok(a) = message.to_str()
    { a } else { return; };
}
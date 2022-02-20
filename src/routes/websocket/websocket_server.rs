use warp;
use warp::ws::{WebSocket};
use futures::StreamExt;
use futures::FutureExt;

pub(crate) async fn get_websocket_server(ws: WebSocket) {
    let (tx, rx) = ws.split();
    rx.forward(tx).map(|result| {
        if let Err(e) = result {
            eprintln!("websocket error: {:?}", e);
        }
    });
}
use crate::{AppState, Client, Clients};
use axum::extract::{
    ws::{Message, WebSocket, WebSocketUpgrade},
    State,
};
use axum::http::StatusCode;
use axum::response::Response;
use futures::{FutureExt, StreamExt};
use log::{error, info};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;

pub async fn handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> std::result::Result<Response, StatusCode> {
    let wsState = WsState::new("foo".into(), state.clients.clone());
    Ok(ws.on_upgrade(|socket| client_connection(socket, wsState)))
}

pub struct WsState {
    id: String,
    clients: Clients,
}
impl WsState {
    pub fn new(id: String, clients: Clients) -> Self {
        Self { id, clients }
    }
}
pub async fn client_connection(ws: WebSocket, state: WsState) {
    let id = state.id;
    let clients = state.clients;

    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();
    let client_rcv = UnboundedReceiverStream::new(client_rcv);

    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            error!("error sending websocket msg: {}", e);
        }
    }));

    tokio::spawn(async move {
        let mut val = 0;
        // let client = Client::new(Some(client_sender), Arc::new(RwLock::new(HashMap::new())));
        // let _ = client_sender.send(Ok(Message::Text("AllGood".into())));
        loop {
            val += 1;
            let _ = client_sender.send(Ok(Message::Text(val.to_string())));
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        }
    });
    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                error!("error receiving ws message: {}", e);
                break;
            }
        };
        client_msg(msg).await;
    }
    // info!("{} disconnected", &id);
}

async fn client_msg(msg: Message) {
    match msg {
        Message::Text(t) => {
            dbg!(t);
            // let proxy_response: ProxyResponse = serde_json::from_str(&t).unwrap();
            // let active_clients = clients.read().await;
            // let client = active_clients.get(&id);
            // client
            //     .unwrap()
            //     .write_response(proxy_response.guid.clone(), proxy_response)
            //     .await
        }
        Message::Binary(_) => info!("Binary"),
        Message::Ping(_) | Message::Pong(_) => info!("Pingpong"),
        Message::Close(frame) => {
            info!("Close: {frame:?}")
            // if let Some(f) = frame {
            //     info!("{} disconnect message: {}", id, f.reason);
            // }
        }
    }
}

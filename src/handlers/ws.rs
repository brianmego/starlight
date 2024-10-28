use crate::{models::location::Location, AppState, Client, Clients};
use serde::{Deserialize, Serialize};
use socketioxide::{
    extract::{AckSender, Bin, Data, SocketRef},
    SocketIo,
};
use serde_json::Value;
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

// pub async fn handler(
//     ws: WebSocketUpgrade,
//     State(state): State<Arc<AppState>>,
// ) -> std::result::Result<Response, StatusCode> {
//     let ws_state = WsState::new("foo".into(), state.clients.clone());
//     Ok(ws.on_upgrade(|socket| client_connection(socket, ws_state)))
// }

// pub struct WsState {
//     id: String,
//     clients: Clients,
// }
// impl WsState {
//     pub fn new(id: String, clients: Clients) -> Self {
//         Self { id, clients }
//     }
// }
// pub async fn client_connection(ws: WebSocket, state: WsState) {
//     let id = state.id;
//     let clients = state.clients;

//     let (client_ws_sender, mut client_ws_rcv) = ws.split();
//     let (client_sender, client_rcv) = mpsc::unbounded_channel();
//     let client_rcv = UnboundedReceiverStream::new(client_rcv);

//     tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
//         if let Err(e) = result {
//             error!("error sending websocket msg: {}", e);
//         }
//     }));

//     tokio::spawn(async move {
//         let mut val = 0;
//         // let client = Client::new(Some(client_sender), Arc::new(RwLock::new(HashMap::new())));
//         // let _ = client_sender.send(Ok(Message::Text("AllGood".into())));
//         loop {
//             val += 1;
//             let _ = client_sender.send(Ok(Message::Text(val.to_string())));
//             tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
//         }
//     });
//     while let Some(result) = client_ws_rcv.next().await {
//         let msg = match result {
//             Ok(msg) => msg,
//             Err(e) => {
//                 error!("error receiving ws message: {}", e);
//                 break;
//             }
//         };
//         client_msg(msg).await;
//     }
//     // info!("{} disconnected", &id);
// }

// async fn client_msg(msg: Message) {
//     match msg {
//         Message::Text(t) => {
//             dbg!(t);
//             // let proxy_response: ProxyResponse = serde_json::from_str(&t).unwrap();
//             // let active_clients = clients.read().await;
//             // let client = active_clients.get(&id);
//             // client
//             //     .unwrap()
//             //     .write_response(proxy_response.guid.clone(), proxy_response)
//             //     .await
//         }
//         Message::Binary(_) => info!("Binary"),
//         Message::Ping(_) | Message::Pong(_) => info!("Pingpong"),
//         Message::Close(frame) => {
//             info!("Close: {frame:?}")
//             // if let Some(f) = frame {
//             //     info!("{} disconnect message: {}", id, f.reason);
//             // }
//         }
//     }
// }

#[derive(Serialize, Clone, Default)]
struct LockedData {
    locations: Vec<Location>
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct ClientState {
    endpoint: String,
    value: String,
    jwt: String
}

pub fn on_connect(socket: SocketRef, Data(data): Data<Value>) {
    info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);
    socket.on(
        "message",
        |socket: SocketRef, Data::<ClientState>(data), Bin(bin)| {
            info!("Received event: {:?} - {:?}", socket.id, data);
            let locked_data = LockedData{locations: vec![Location::new(&data.value)]};
            socket.broadcast().emit("locked-data", locked_data).ok();
        },
    );

    // socket.on(
    //     "message-with-ack",
    //     |Data::<Value>(data), ack: AckSender, Bin(bin)| {
    //         info!("Received event: {:?} {:?}", data, bin);
    //         ack.bin(bin).send(data).ok();
    //     },
    // );
}

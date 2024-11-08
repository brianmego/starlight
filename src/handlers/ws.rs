use crate::{handlers::login::Claims, models::{dayofweek::DayOfWeek, location::Location, timeslot::TimeSlot}, AppState, Client, Clients};
use axum::extract::{
    ws::{Message, WebSocket, WebSocketUpgrade},
    State,
};
use axum::http::StatusCode;
use axum::response::Response;
use futures::{FutureExt, StreamExt};
use jsonwebtoken::{self, Algorithm, DecodingKey, Validation};
use log::{error, info, debug};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use socketioxide::{
    extract::{AckSender, Bin, Data, SocketRef},
    SocketIo,
};
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
    locations: Vec<Location>,
    days: Vec<DayOfWeek>,
    timeslots: Vec<TimeSlot>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum Endpoint {
    DayOfWeek,
    Location,
    Timeslot,
}
struct UnknownEndpointError;
impl TryFrom<String> for Endpoint {
    type Error = UnknownEndpointError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if &value == "dayofweek" {
            return Ok(Self::DayOfWeek);
        } else if &value == "location" {
            return Ok(Self::Location);
        } else if &value == "timeslot" {
            return Ok(Self::Timeslot);
        }
        Err(UnknownEndpointError)
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct AllSelections {
    location: Option<String>,
    day: Option<String>,
    timeslot: Option<String>,
}
impl AllSelections {
    fn reservable(self) -> bool {
        self.location.is_some() && self.day.is_some() && self.timeslot.is_some()
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ClientState {
    endpoint: String,
    value: String,
    jwt: String,
    all_selections: AllSelections,
}

pub fn on_connect(socket: SocketRef, Data(data): Data<Value>) {
    info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);
    socket.on(
        "message",
        |socket: SocketRef, Data::<ClientState>(data), Bin(bin)| {
            debug!("Received event: {:?} - {:?}", socket.id, data);
            let endpoint: Result<Endpoint, UnknownEndpointError> = data.endpoint.clone().try_into();
            match endpoint {
                Ok(ept) => match ept {
                    Endpoint::DayOfWeek => {
                        let locked_data = LockedData {
                            locations: vec![],
                            days: vec![DayOfWeek::new(&data.value)],
                            timeslots: vec![],
                        };
                        socket.broadcast().emit("locked-data", locked_data).ok();
                    },
                    Endpoint::Location => {
                        let locked_data = LockedData {
                            locations: vec![Location::new(&data.value)],
                            days: vec![],
                            timeslots: vec![],
                        };
                        socket.broadcast().emit("locked-data", locked_data).ok();
                    }
                    Endpoint::Timeslot => {
                        let locked_data = LockedData {
                            locations: vec![],
                            days: vec![],
                            timeslots: vec![TimeSlot::new(1, 3)],
                        };
                        socket.broadcast().emit("locked-data", locked_data).ok();
                        // info!("{:?}", data.all_selections);
                        // match data.all_selections.reservable() {
                        //     true => info!("Reservable!"),
                        //     false => info!("Not yet reservable..."),
                        // }
                    }
                },
                Err(err) => error!("Unknown endpoint: {}", data.endpoint),
            }
            // let claims = jsonwebtoken::decode::<Claims>(
            //     &data.jwt,
            //     &DecodingKey::from_secret("secret".as_ref()),
            //     &Validation::new(Algorithm::HS256),
            // );
        },
    );

    socket.on(
        "reserve",
        |socket: SocketRef, Data::<String>(data), Bin(bin)| {
            info!("Received event: {:?} - {:?}", socket.id, data);
        }
    );
}

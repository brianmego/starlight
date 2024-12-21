use crate::{
    handlers::login::{Claims, DbUser},
    models::{location::Location, reservation::Reservation},
    Client, Clients, DB,
};
use axum::extract::{
    ws::{Message, WebSocket, WebSocketUpgrade},
    State,
};
use axum::http::StatusCode;
use axum::response::Response;
use futures::{FutureExt, StreamExt};
use jsonwebtoken::{self, Algorithm, DecodingKey, Validation};
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use socketioxide::{
    extract::{AckSender, Bin, Data, SocketRef},
    SocketIo,
};
use std::{collections::HashMap, str::FromStr, sync::Arc};
use surrealdb::RecordId;
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;


#[derive(Clone, Debug, Serialize, Deserialize)]
enum Endpoint {
    Location,
    Reservation,
}
struct UnknownEndpointError;
impl TryFrom<String> for Endpoint {
    type Error = UnknownEndpointError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if &value == "location" {
            return Ok(Self::Location);
        } else if &value == "reservation" {
            return Ok(Self::Reservation);
        }
        Err(UnknownEndpointError)
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AllSelections {
    location: Option<String>,
    day: Option<String>,
    start_time: Option<String>,
    jwt: String,
}
impl AllSelections {
    fn reservable(&self) -> bool {
        self.location.is_some() && self.day.is_some() && self.start_time.is_some()
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ClientState {
    endpoint: String,
    value: String,
    jwt: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct DbReservation {
    id: RecordId,
    location: RecordId,
    start: u8,
    duration: u8,
    day_of_week: RecordId,
    reserved_by: Option<RecordId>,
}

pub fn on_connect(socket: SocketRef, Data(data): Data<Value>) {
    debug!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);
    socket.on(
        "reserve",
        |socket: SocketRef, Data::<AllSelections>(data), Bin(bin)| async move {
            info!("Received Reserve event: {:?} - {:?}", socket.id, data);
            match data.reservable() {
                true => {
                    let claims = jsonwebtoken::decode::<Claims>(
                        &data.jwt,
                        &DecodingKey::from_secret("secret".as_ref()),
                        &Validation::new(Algorithm::HS256),
                    );
                    match claims {
                        Ok(c) => {
                            let id = c.claims.id();
                            let day_id: i64 = data.day.unwrap().parse().unwrap();
                            let day_of_week = RecordId::from(("day_of_week", day_id));
                            let location = RecordId::from(("location", data.location.unwrap()));
                            let start: u8 = data.start_time.unwrap().parse().unwrap();
                            let mut response = DB
                                .query(
                                    "SELECT *
                            FROM reservation
                            WHERE day_of_week = $day_of_week
                              AND location = $location
                              AND start = $start",
                                )
                                .bind(("day_of_week", day_of_week))
                                .bind(("location", location))
                                .bind(("start", start))
                                .await
                                .unwrap();
                            let reservation: Option<DbReservation> = response.take(0).unwrap();
                            let mut reservation = reservation.unwrap();
                            reservation.reserved_by = Some(DbUser::new(&id).id());
                            let reservation_id = reservation.id.to_string();
                            let (table, reservation_id) = reservation_id.split_once(':').unwrap();
                            let updated_reservation: Option<DbReservation> = DB
                                .update((table, reservation_id))
                                .content(reservation)
                                .await
                                .unwrap();
                            socket.emit("message", "Reserved!").ok()
                        }
                        Err(_) => socket.emit("message", "Session Expired").ok(),
                    }
                }
                false => socket.emit("message", "This is not reservable").ok(),
            };
        },
    );
}

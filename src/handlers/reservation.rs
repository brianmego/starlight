use crate::handlers::login::Claims;
use crate::models::reservation::Reservation;
use crate::DB;
use axum::extract::Path;
use axum::http::{header::HeaderMap, StatusCode};
use axum::Json;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use surrealdb::RecordId;

#[derive(Debug, Deserialize, Serialize)]
pub struct ReservationDBResult {
    reservation_id: RecordId,
    day_of_week_id: RecordId,
    day_of_week_name: String,
    location_id: RecordId,
    location_name: String,
    start_time: i8,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ReservationResult {
    reservation_id: String,
    day_of_week_id: i8,
    day_of_week_name: String,
    location_id: String,
    location_name: String,
    start_time_id: i8,
    start_time_name: String,
}
impl From<ReservationDBResult> for ReservationResult {
    fn from(value: ReservationDBResult) -> Self {
        ReservationResult {
            reservation_id: value.reservation_id.key().to_string(),
            day_of_week_id: value
                .day_of_week_id
                .key()
                .to_string()
                .parse::<i8>()
                .unwrap(),
            day_of_week_name: value.day_of_week_name,
            location_id: value.location_id.key().to_string(),
            location_name: value.location_name,
            start_time_id: value.start_time,
            start_time_name: ClockTime(value.start_time).as_12_hour_time(),
        }
    }
}

fn get_hour_suffix(hour: i8) -> String {
    match hour.lt(&12) {
        true => "am",
        false => "pm",
    }
    .into()
}
struct ClockTime(i8);
impl ClockTime {
    fn as_12_hour_time(&self) -> String {
        let start_time = self.0;
        let end_time = self.0 + 2;
        format!(
            "{} {} - {} {}",
            start_time % 12,
            get_hour_suffix(start_time),
            end_time % 12,
            get_hour_suffix(end_time)
        )
    }
}

const RESERVATION_QUERY: &str = "
    SELECT
        id AS reservation_id,
        day_of_week AS day_of_week_id,
        day_of_week.name AS day_of_week_name,
        location AS location_id,
        location.name AS location_name,
        start AS start_time
    FROM reservation;
";

const USER_RESERVATION_QUERY: &str = "
    SELECT
        id AS reservation_id,
        day_of_week AS day_of_week_id,
        day_of_week.name AS day_of_week_name,
        location AS location_id,
        location.name AS location_name,
        start AS start_time
    FROM reservation
    WHERE reserved_by=$user;
";

pub async fn handler_get() -> Json<Vec<ReservationResult>> {
    info!("/reservation");
    let mut response = DB.query(RESERVATION_QUERY).await.unwrap();
    let reservation_db_list: Vec<ReservationDBResult> = response.take(0).unwrap();
    let reservation_list = reservation_db_list
        .into_iter()
        .map(|res| res.into())
        .collect();
    Json(reservation_list)
}

pub async fn handler_get_user_reservations(
    Path(user_id): Path<String>,
) -> Json<Vec<ReservationResult>> {
    info!("/reservation/{}", user_id);
    let user_record = RecordId::from(("user", &user_id));
    let mut response = DB
        .query(USER_RESERVATION_QUERY)
        .bind(("user", user_record))
        .await
        .unwrap();
    let reservation_db_list: Vec<ReservationDBResult> = response.take(0).unwrap();
    let reservation_list = reservation_db_list
        .into_iter()
        .map(|res| res.into())
        .collect();
    Json(reservation_list)
}

pub async fn handler_delete_reservation(
    Path(reservation_id): Path<String>,
    headers: HeaderMap,
) -> StatusCode {
    let auth_header = headers.get("Authorization");
    let jwt = auth_header
        .unwrap()
        .to_str()
        .unwrap()
        .split("Bearer ")
        .last()
        .unwrap();
    let claims = jsonwebtoken::decode::<Claims>(
        &jwt,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::new(Algorithm::HS256),
    );
    let id = claims.unwrap().claims.id();
    let reservation_id = RecordId::from(("reservation", reservation_id));
    dbg!(&reservation_id);
    let response = DB
        .query("UPDATE reservation SET reserved_by=None WHERE id = $reservation_id")
        .bind(("reservation_id", reservation_id))
        .await
        .unwrap();
    dbg!(response);
    StatusCode::OK
}

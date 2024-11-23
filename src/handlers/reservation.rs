use crate::models::reservation::Reservation;
use crate::DB;
use axum::http::StatusCode;
use axum::Json;
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
    }.into()
}
struct ClockTime(i8);
impl ClockTime {
    fn as_12_hour_time(&self) -> String {
        let start_time = self.0;
        let end_time = self.0 + 2;
        format!("{} {} - {} {}", start_time % 12, get_hour_suffix(start_time), end_time % 12, get_hour_suffix(end_time))
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

pub async fn handler_get() -> Json<Vec<ReservationResult>> {
    info!("/reservation");
    let mut response = DB.query(RESERVATION_QUERY).await.unwrap();
    let reservation_db_list: Vec<ReservationDBResult> = response.take(0).unwrap();
    let mut reservation_list = vec![];
    for res in reservation_db_list {
        reservation_list.push(res.into())
    }
    Json(reservation_list)
}

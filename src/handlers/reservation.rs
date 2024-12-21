use crate::handlers::login::Claims;
use crate::models::reservation::Reservation;
use crate::DB;
use axum::extract::Path;
use axum::http::{header::HeaderMap, StatusCode};
use axum::Json;
use chrono::{prelude::*, DateTime, TimeDelta, TimeZone};
use chrono_tz::{America::Chicago, Tz};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use surrealdb::sql::Datetime as SurrealDateTime;
use surrealdb::RecordId;

#[derive(Debug, Deserialize, Serialize)]
pub struct ReservationDBResult {
    date: String,
    day_of_week_id: i8,
    day_of_week_name: String,
    location_id: RecordId,
    location_name: String,
    reservation_id: RecordId,
    start_time: i8,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ReservationResult {
    reservation_id: String,
    date: String,
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
            date: value.date,
            day_of_week_id: value.day_of_week_id,
            day_of_week_name: value.day_of_week_name,
            location_id: value.location_id.key().to_string(),
            location_name: value.location_name,
            start_time_id: value.start_time,
            start_time_name: ClockTime(value.start_time).as_12_hour_time(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct ReservationRequest {
    reservation_id: String,
    user_id: String,
}

fn get_hour_suffix(hour: i8) -> String {
    match hour.lt(&12) {
        true => "am",
        false => match hour.ge(&24) {
            true => "am",
            false => "pm",
        },
    }
    .into()
}
struct ClockTime(i8);
impl ClockTime {
    fn as_12_hour_time(&self) -> String {
        let mut start_time = self.0;
        let mut end_time = self.0 + 2;
        let start_time_suffix = get_hour_suffix(start_time);
        let end_time_suffix = get_hour_suffix(end_time);
        start_time = start_time % 12;
        end_time = end_time % 12;
        if start_time == 0 {
            start_time = 12;
        }
        if end_time == 0 {
            end_time = 12;
        }
        format!(
            "{} {} - {} {}",
            start_time, start_time_suffix, end_time, end_time_suffix
        )
    }
}

const AVAILABLE_RESERVATIONS_QUERY: &str = "
    SELECT
        time::format(day - 6h, '%Y-%m-%d') as date,
        id AS reservation_id,
        fn::day_of_week(day - 6h).day AS day_of_week_id,
        fn::day_of_week(day - 6h).name AS day_of_week_name,
        location AS location_id,
        location.name AS location_name,
        time::hour(day - 6h) AS start_time
    FROM reservation
    WHERE reserved_by=None
      AND day > $start_time
      AND day < $end_time
";

const USER_RESERVATION_QUERY: &str = "
    SELECT
        id AS reservation_id,
        time::format(day - 6h, '%Y-%m-%d') as date,
        fn::day_of_week(day - 6h).day AS day_of_week_id,
        fn::day_of_week(day - 6h).name AS day_of_week_name,
        location AS location_id,
        location.name AS location_name,
        time::hour(day - 6h) AS start_time
    FROM reservation
    WHERE reserved_by=$user
    ORDER BY date;
";

const SET_RESERVATION_QUERY: &str = "
    UPDATE reservation
    SET reserved_by=$user
    WHERE id = $reservation_id
";

pub async fn handler_get() -> Json<Vec<ReservationResult>> {
    info!("GET /reservation");
    let start_time = Chicago.with_ymd_and_hms(2025, 1, 20, 22, 0, 0).unwrap();
    // let start_time = Utc::now();
    let registration_window = RegistrationWindow::new(start_time);
    let start_time = SurrealDateTime::from(registration_window.start().to_utc());
    let end_time = SurrealDateTime::from(registration_window.end().to_utc());
    dbg!(&registration_window);
    let mut response = DB
        .query(AVAILABLE_RESERVATIONS_QUERY)
        .bind(("start_time", start_time))
        .bind(("end_time", end_time))
        .await
        .unwrap();
    let reservation_db_list: Vec<ReservationDBResult> = response.take(0).unwrap();
    let reservation_list = reservation_db_list
        .into_iter()
        .map(|res| res.into())
        .collect();
    Json(reservation_list)
}

pub async fn handler_post(
    headers: HeaderMap,
    Path(reservation_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    info!("POST /reservation/{}", reservation_id);
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
    let user_id = claims.map_err(|x| StatusCode::UNAUTHORIZED)?.claims.id();
    let reservation_id = RecordId::from(("reservation", reservation_id));
    let user_record = RecordId::from(user_id.split_once(':').unwrap());
    let response = DB
        .query(SET_RESERVATION_QUERY)
        .bind(("reservation_id", reservation_id))
        .bind(("user", user_record))
        .await
        .unwrap();
    Ok(StatusCode::OK)
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
) -> Result<StatusCode, StatusCode> {
    // return Err(StatusCode::UNAUTHORIZED);
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
    let id = claims.map_err(|x| StatusCode::UNAUTHORIZED)?.claims.id();
    let reservation_id = RecordId::from(("reservation", reservation_id));
    let response = DB
        .query("UPDATE reservation SET reserved_by=None WHERE id = $reservation_id")
        .bind(("reservation_id", reservation_id))
        .await
        .unwrap();
    Ok(StatusCode::OK)
}

#[derive(Debug)]
struct RegistrationWindow<Tz: TimeZone> {
    start: DateTime<Tz>,
    end: DateTime<Tz>,
}

impl<Tz: TimeZone> RegistrationWindow<Tz> {
    fn new(start: DateTime<Tz>) -> Self {
        let days_to_add = match start.weekday() {
            Weekday::Mon => match start.hour() < 22 {
                true => 5,
                false => 12,
            },
            Weekday::Tue => 11,
            Weekday::Wed => 10,
            Weekday::Thu => 9,
            Weekday::Fri => 8,
            Weekday::Sat => 7,
            Weekday::Sun => 6,
        };
        let end = start.clone() + TimeDelta::days(days_to_add)
            - TimeDelta::hours(start.hour().into())
            - TimeDelta::minutes(start.minute().into())
            - TimeDelta::seconds(start.second().into());
        Self { start, end }
    }
    fn start(&self) -> DateTime<Tz> {
        self.start.clone()
    }
    fn end(&self) -> DateTime<Tz> {
        self.end.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(12, "12 pm - 2 pm")]
    #[test_case(10, "10 am - 12 pm")]
    #[test_case(11, "11 am - 1 pm")]
    #[test_case(23, "11 pm - 1 am")]
    fn test_clock_time(start: i8, expected: &str) {
        let actual = ClockTime(start).as_12_hour_time();
        let expected = expected.to_string();
        assert_eq!(actual, expected);
    }

    #[test_case(Chicago.with_ymd_and_hms(2025, 1, 18, 19, 0, 0).unwrap(), Chicago.with_ymd_and_hms(2025, 1, 25, 0, 0, 0).unwrap(); "18th")]
    #[test_case(Chicago.with_ymd_and_hms(2025, 1, 19, 19, 0, 0).unwrap(), Chicago.with_ymd_and_hms(2025, 1, 25, 0, 0, 0).unwrap(); "19th")]
    #[test_case(Chicago.with_ymd_and_hms(2025, 1, 20, 19, 0, 0).unwrap(), Chicago.with_ymd_and_hms(2025, 1, 25, 0, 0, 0).unwrap(); "20th before 10")]
    #[test_case(Chicago.with_ymd_and_hms(2025, 1, 20, 22, 0, 0).unwrap(), Chicago.with_ymd_and_hms(2025, 2, 1, 0, 0, 0).unwrap(); "20th after 10")]
    #[test_case(Chicago.with_ymd_and_hms(2025, 1, 21, 19, 0, 0).unwrap(), Chicago.with_ymd_and_hms(2025, 2, 1, 0, 0, 0).unwrap(); "21st")]
    fn test_active_registration_window(start_time: DateTime<Tz>, end_time: DateTime<Tz>) {
        let actual = RegistrationWindow::new(start_time);
        assert_eq!(actual.end(), end_time);
    }
}

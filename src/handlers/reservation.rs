use crate::handlers::login::Claims;
use crate::models::reservation::{Reservation, UnreservableReason};
use crate::{queries, AppState, DB};
use axum::extract::{Path, State};
use axum::http::{header::HeaderMap, StatusCode};
use axum::Json;
use cached::proc_macro::once;
use chrono::{prelude::*, DateTime, TimeDelta, TimeZone};
use chrono_tz::{America::Chicago, Tz};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use log::info;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime as SurrealDateTime;
use surrealdb::RecordId;

#[derive(Debug, Deserialize, Serialize)]
pub struct ReservationDBResult {
    date: String,
    day_of_week_id: i8,
    day_of_week_name: String,
    location_id: RecordId,
    location_name: String,
    location_address: String,
    location_notes: Option<String>,
    reservation_id: RecordId,
    start_time: i8,
    next_week: Option<bool>,
    passed: Option<bool>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReservationResult {
    reservation_id: String,
    date: String,
    day_of_week_id: i8,
    day_of_week_name: String,
    location_id: String,
    location_name: String,
    location_address: String,
    location_notes: Option<String>,
    start_time_id: i8,
    start_time_name: String,
    next_week: Option<bool>,
    passed: Option<bool>,
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
            location_address: value.location_address,
            location_notes: value.location_notes,
            start_time_id: value.start_time,
            start_time_name: ClockTime(value.start_time).as_12_hour_time(),
            next_week: value.next_week,
            passed: value.passed,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct ReservationRequest {
    reservation_id: String,
    user_id: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReservationListResult {
    time_until_next_unlock: i64,
    reservations: Vec<ReservationResult>,
}

impl ReservationListResult {
    pub fn new(time_until_next_unlock: i64, reservations: Vec<ReservationResult>) -> Self {
        Self {
            time_until_next_unlock,
            reservations,
        }
    }
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

pub fn now(offset: i64) -> DateTime<Tz> {
    Utc::now().with_timezone(&Chicago) + TimeDelta::seconds(offset)
}

#[once(time=3, sync_writes=true)]
async fn get_available_reservations(registration_window: &RegistrationWindow<Tz>) -> Vec<ReservationResult> {
    let start_time = SurrealDateTime::from(registration_window.now().to_utc());
    let end_time = SurrealDateTime::from(registration_window.end().to_utc());
    let next_week_start = SurrealDateTime::from(registration_window.next_week_start().to_utc());
    let mut response = DB
        .query(queries::AVAILABLE_RESERVATIONS_QUERY)
        .bind(("start_time", start_time))
        .bind(("end_time", end_time))
        .bind(("next_week_start", next_week_start))
        .await
        .unwrap();
    let reservation_db_list: Vec<ReservationDBResult> = response.take(0).unwrap();
    reservation_db_list
        .into_iter()
        .map(|res| res.into())
        .collect()
}

pub async fn handler_get(State(state): State<AppState>) -> Json<ReservationListResult> {
    info!("GET /api/reservation");
    let offset = state.time_offset;
    let registration_window = RegistrationWindow::new(now(offset));
    let reservation_list = get_available_reservations(&registration_window).await;
    Json(ReservationListResult::new(
        registration_window.time_until_next_unlock(),
        reservation_list,
    ))
}

pub async fn handler_post(
    headers: HeaderMap,
    Path(reservation_id): Path<String>,
    State(state): State<AppState>,
) -> Result<StatusCode, StatusCode> {
    let offset = state.time_offset;
    info!("POST /api/reservation/{}", reservation_id);
    let auth_header = headers.get("Authorization");
    let jwt = auth_header
        .unwrap()
        .to_str()
        .map_err(|_| StatusCode::UNAUTHORIZED)?
        .split("Bearer ")
        .last()
        .ok_or_else(|| StatusCode::UNAUTHORIZED)?;
    let decoded_jwt = jsonwebtoken::decode::<Claims>(
        &jwt,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;
    let registration_window = RegistrationWindow::new(now(offset));

    let claims = decoded_jwt.claims;
    let user_id = claims.id();
    let reservation_record = Reservation::get_by_id(&reservation_id).await.unwrap();
    let reservation_id = RecordId::from(("reservation", reservation_id));
    match reservation_record
        .is_reservable_by_user(&user_id, registration_window)
        .await
    {
        Ok(_) => {}
        Err(err) => match err {
            UnreservableReason::NotEnoughTokens => {
                println!("Not enough tokens");
                Err(StatusCode::PAYMENT_REQUIRED)?;
            }
            UnreservableReason::AlreadyReserved(uid) => {
                println!("Already reserved by user: {}", uid);
                Err(StatusCode::CONFLICT)?;
            }
        },
    }

    let user_record = RecordId::from(user_id.split_once(':').unwrap());
    DB.query(queries::SET_RESERVATION_QUERY)
        .bind(("reservation_id", reservation_id))
        .bind(("user", user_record))
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(StatusCode::OK)
}

pub async fn handler_get_user_reservations(
    Path(user_id): Path<String>,
    State(state): State<AppState>,
) -> Json<Vec<ReservationResult>> {
    info!("/api/reservation/{}", user_id);
    let offset = state.time_offset;
    let user_record = RecordId::from(("user", &user_id));
    let registration_window = RegistrationWindow::new(now(offset));
    let next_week_start = SurrealDateTime::from(registration_window.next_week_start().to_utc());
    let current_time = SurrealDateTime::from(registration_window.now().to_utc());
    let mut response = DB
        .query(queries::USER_RESERVATION_QUERY)
        .bind(("user", user_record))
        .bind(("next_week_start", next_week_start))
        .bind(("current_time", current_time))
        .await
        .unwrap();
    let reservation_db_list: Vec<ReservationDBResult> = response.take(0).unwrap();
    let reservation_list: Vec<ReservationResult> = reservation_db_list
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
    let _claims = jsonwebtoken::decode::<Claims>(
        &jwt,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let reservation_id = RecordId::from(("reservation", reservation_id));
    DB.query("UPDATE reservation SET reserved_by=None WHERE id = $reservation_id")
        .bind(("reservation_id", reservation_id))
        .await
        .unwrap();
    Ok(StatusCode::OK)
}

#[derive(Debug)]
pub struct RegistrationWindow<Tz: TimeZone> {
    now: DateTime<Tz>,
    start: DateTime<Tz>,
    end: DateTime<Tz>,
    next_week_start: DateTime<Tz>,
}

impl<Tz: TimeZone> RegistrationWindow<Tz> {
    pub fn new(now: DateTime<Tz>) -> Self {
        // This is the most complicated logic on the site.
        // You can see up to two Friday's in the future.
        // The next week doesn't unlock until Monday at 10PM
        // Therefore Monday at 9PM you can see 5 days of stuff (M-F),
        // but Monday at 10PM you can see 12 days of stuff (M-F-F)
        //
        // Also, the next week rolls over on Friday at noon.
        // Therefore, Friday at 11AM the current week will have a single day in it (Friday)
        // At 12PM the current week now has Friday-Friday.
        //
        // The unit tests validate this crazy logic
        let days_to_add = match now.weekday() {
            Weekday::Mon => match now.hour() < 22 {
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

        let start = now.clone()
            - TimeDelta::days(12 - days_to_add)
            - TimeDelta::hours(now.hour().into())
            - TimeDelta::minutes(now.minute().into())
            - TimeDelta::seconds(now.second().into());

        let end = now.clone() + TimeDelta::days(days_to_add)
            - TimeDelta::hours(now.hour().into())
            - TimeDelta::minutes(now.minute().into())
            - TimeDelta::seconds(now.second().into());
        let next_week_start = match now.weekday() {
            Weekday::Tue | Weekday::Wed | Weekday::Thu => end.clone() - TimeDelta::days(7),
            Weekday::Fri => match now.hour() < 12 {
                true => end.clone() - TimeDelta::days(7),
                false => end.clone(),
            },
            Weekday::Mon => match now.hour() < 22 {
                true => end.clone(),
                false => end.clone() - TimeDelta::days(7),
            },
            Weekday::Sat | Weekday::Sun => end.clone(),
        };
        Self {
            now,
            start,
            end,
            next_week_start,
        }
    }

    pub fn now(&self) -> DateTime<Tz> {
        self.now.clone()
    }

    fn end(&self) -> DateTime<Tz> {
        self.end.clone()
    }

    fn start(&self) -> DateTime<Tz> {
        self.start.clone()
    }

    pub fn next_week_start(&self) -> DateTime<Tz> {
        self.next_week_start.clone()
    }

    pub fn time_until_next_unlock(&self) -> i64 {
        let now = self.now();
        let time_until = if now.hour() < 12 {
            let future = Chicago.with_ymd_and_hms(now.year(), now.month(), now.day(), 12, 0, 0);
            future.single().unwrap().naive_local() - now.naive_local()
        } else if now.hour() < 22 {
            let future = Chicago.with_ymd_and_hms(now.year(), now.month(), now.day(), 22, 0, 0);
            future.single().unwrap().naive_local() - now.naive_local()
        } else {
            let future = Chicago.with_ymd_and_hms(now.year(), now.month(), now.day(), 22, 0, 0);
            (future.single().unwrap() + TimeDelta::days(1)).naive_local() - now.naive_local()
        };
        time_until.num_seconds()
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

    #[test_case(
        Chicago.with_ymd_and_hms(2025, 1, 18, 19, 0, 0).unwrap(),
        Chicago.with_ymd_and_hms(2025, 1, 13, 0, 0, 0).unwrap(),
        Chicago.with_ymd_and_hms(2025, 1, 25, 0, 0, 0).unwrap(),
        Chicago.with_ymd_and_hms(2025, 1, 25, 0, 0, 0).unwrap();
        "Saturday")]
    #[test_case(
        Chicago.with_ymd_and_hms(2025, 1, 19, 19, 0, 0).unwrap(),
        Chicago.with_ymd_and_hms(2025, 1, 13, 0, 0, 0).unwrap(),
        Chicago.with_ymd_and_hms(2025, 1, 25, 0, 0, 0).unwrap(),
        Chicago.with_ymd_and_hms(2025, 1, 25, 0, 0, 0).unwrap();
        "Sunday")]
    #[test_case(
        Chicago.with_ymd_and_hms(2025, 1, 20, 19, 0, 0).unwrap(),
        Chicago.with_ymd_and_hms(2025, 1, 13, 0, 0, 0).unwrap(),
        Chicago.with_ymd_and_hms(2025, 1, 25, 0, 0, 0).unwrap(),
        Chicago.with_ymd_and_hms(2025, 1, 25, 0, 0, 0).unwrap();
        "Monday before 10")]
    #[test_case(
        Chicago.with_ymd_and_hms(2025, 1, 20, 22, 0, 0).unwrap(),
        Chicago.with_ymd_and_hms(2025, 1, 20, 0, 0, 0).unwrap(),
        Chicago.with_ymd_and_hms(2025, 2, 1, 0, 0, 0).unwrap(),
        Chicago.with_ymd_and_hms(2025, 1, 25, 0, 0, 0).unwrap();
        "Monday after 10")]
    #[test_case(
        Chicago.with_ymd_and_hms(2025, 1, 21, 19, 0, 0).unwrap(),
        Chicago.with_ymd_and_hms(2025, 1, 20, 0, 0, 0).unwrap(),
        Chicago.with_ymd_and_hms(2025, 2, 1, 0, 0, 0).unwrap(),
        Chicago.with_ymd_and_hms(2025, 1, 25, 0, 0, 0).unwrap();
        "Tuesday")]
    #[test_case(
        Chicago.with_ymd_and_hms(2025, 1, 22, 19, 0, 0).unwrap(),
        Chicago.with_ymd_and_hms(2025, 1, 20, 0, 0, 0).unwrap(),
        Chicago.with_ymd_and_hms(2025, 2, 1, 0, 0, 0).unwrap(),
        Chicago.with_ymd_and_hms(2025, 1, 25, 0, 0, 0).unwrap();
        "Wednesday")]
    #[test_case(
        Chicago.with_ymd_and_hms(2025, 1, 23, 19, 0, 0).unwrap(),
        Chicago.with_ymd_and_hms(2025, 1, 20, 0, 0, 0).unwrap(),
        Chicago.with_ymd_and_hms(2025, 2, 1, 0, 0, 0).unwrap(),
        Chicago.with_ymd_and_hms(2025, 1, 25, 0, 0, 0).unwrap();
        "Thursday")]
    #[test_case(
        Chicago.with_ymd_and_hms(2025, 1, 24, 11, 0, 0).unwrap(),
        Chicago.with_ymd_and_hms(2025, 1, 20, 0, 0, 0).unwrap(),
        Chicago.with_ymd_and_hms(2025, 2, 1, 0, 0, 0).unwrap(),
        Chicago.with_ymd_and_hms(2025, 1, 25, 0, 0, 0).unwrap();
        "Friday before noon")]
    #[test_case(
        Chicago.with_ymd_and_hms(2025, 1, 24, 12, 0, 0).unwrap(),
        Chicago.with_ymd_and_hms(2025, 1, 20, 0, 0, 0).unwrap(),
        Chicago.with_ymd_and_hms(2025, 2, 1, 0, 0, 0).unwrap(),
        Chicago.with_ymd_and_hms(2025, 2, 1, 0, 0, 0).unwrap();
        "Friday after noon")]
    fn test_active_registration_window(
        now: DateTime<Tz>,
        start_time: DateTime<Tz>,
        end_time: DateTime<Tz>,
        next_week_start: DateTime<Tz>,
    ) {
        let actual = RegistrationWindow::new(now);
        assert_eq!(actual.now(), now);
        assert_eq!(actual.start(), start_time);
        assert_eq!(actual.end(), end_time);
        assert_eq!(actual.next_week_start(), next_week_start);
    }
}

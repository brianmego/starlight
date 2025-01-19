use chrono::{prelude::*, DateTime};
use chrono_tz::America::Chicago;
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use crate::handlers::reservation::ClockTime;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CurrentReservationDB {
    id: RecordId,
    date: DateTime<Utc>,
    username: String,
    location: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CurrentReservation {
    id: String,
    date: String,
    time: String,
    username: String,
    location: String,
}
impl From<CurrentReservationDB> for CurrentReservation {
    fn from(value: CurrentReservationDB) -> Self {
        let chicago_time = value.date.with_timezone(&Chicago);
        let clock_time = ClockTime(chicago_time.hour() as i8);
        Self {
            id: value.id.key().to_string(),
            date: format!(
                "{}/{}/{}",
                value.date.month(),
                value.date.day(),
                value.date.year(),
            ),
            time: clock_time.as_12_hour_time(),
            username: value.username,
            location: value.location,
        }
    }
}

use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Reservation {
    id: RecordId,
    day_of_week: RecordId,
    location: RecordId,
    start: u8,
    duration: u8,
}

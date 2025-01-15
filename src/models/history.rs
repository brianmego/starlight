use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReservationLog {
    id: RecordId,
    action: String,
    at: DateTime<Utc>,
    old_reserved_by: Option<RecordId>,
    new_reserved_by: Option<RecordId>,
    reservation_id: RecordId,
}

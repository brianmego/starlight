use crate::models::timeslot::TimeSlot;
use crate::DB;
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;

type TimeSlotResult = surrealdb::Result<Json<Vec<TimeSlot>>>;

pub async fn handler_get() -> Json<Vec<TimeSlot>> {
    let timeslots: Vec<TimeSlot> = DB.select("timeslot").await.unwrap();
    Json(timeslots)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_handler() {
        let result = handler_get().await;
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].name(), "Chuy's");
    }
}


use crate::models::dayofweek::DayOfWeek;
use crate::DB;
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;

type DayOfWeekResult = Json<Vec<DayOfWeek>>;

pub async fn handler_get() -> DayOfWeekResult {
    let mut response = DB.query("SELECT * FROM dayofweek").await.unwrap();
    let days: Vec<DayOfWeek> = DB.query("SELECT * FROM dayofweek").await.unwrap().take(0).expect("Data should exist");
    Json(days)
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

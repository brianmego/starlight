use crate::models::dayofweek::DayOfWeek;
use crate::DB;
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;

type DayOfWeekResult = surrealdb::Result<Json<Vec<DayOfWeek>>>;

pub async fn handler_get() -> Json<Vec<DayOfWeek>> {
    let days: Vec<DayOfWeek> = DB.select("dayofweek").await.unwrap();
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


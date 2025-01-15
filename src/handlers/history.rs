use crate::models::history::ReservationLog;
use crate::DB;
use axum::Json;

pub async fn handler_get() -> Json<Vec<ReservationLog>> {
    let history_rows: Vec<ReservationLog> = DB.select("reservation_log").await.unwrap();
    Json(history_rows)
}


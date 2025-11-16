use crate::DB;
use crate::Result;
use crate::models::history::{CurrentReservation, CurrentReservationDB};
use crate::queries;
use axum::Json;
use log::info;

pub async fn handler_get() -> Result<Json<Vec<CurrentReservation>>> {
    info!("GET /api/history");
    let db_rows: Vec<CurrentReservationDB> =
        DB.query(queries::CLAIMED_RESERVATIONS).await?.take(0)?;
    let current_reservations: Vec<CurrentReservation> =
        db_rows.into_iter().map(|r| r.into()).collect();
    Ok(Json(current_reservations))
}

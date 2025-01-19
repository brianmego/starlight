use crate::models::history::{ CurrentReservation, CurrentReservationDB };
use crate::queries;
use crate::Result;
use crate::DB;
use axum::Json;

pub async fn handler_get() -> Result<Json<Vec<CurrentReservation>>> {
    let db_rows: Vec<CurrentReservationDB> =
        DB.query(queries::CLAIMED_RESERVATIONS).await?.take(0)?;
    let current_reservations: Vec<CurrentReservation> = db_rows.into_iter().map(|r| r.into()).collect();
    Ok(Json(current_reservations))
}

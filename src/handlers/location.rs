use crate::models::location::Location;
use crate::DB;
use axum::Json;

pub async fn handler_get() -> Json<Vec<Location>> {
    let locations: Vec<Location> = DB.select("location").await.unwrap();
    Json(locations)
}

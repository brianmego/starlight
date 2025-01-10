use crate::models::location::Location;
use crate::DB;
use axum::Json;
use serde::Deserialize;

pub async fn handler_get() -> Json<Vec<Location>> {
    let locations: Vec<Location> = DB.select("location").await.unwrap();
    Json(locations)
}

#[derive(Debug, Deserialize)]
pub struct LocationPayload {
    name: String,
    address: String,
    notes: String
}

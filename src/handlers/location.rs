use crate::models::location::Location;
use crate::DB;
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;

pub async fn handler_get() -> Json<Vec<Location>> {
    let locations: Vec<Location> = DB.select("location").await.unwrap();
    Json(locations)
}

pub async fn handler_post(Json(payload): Json<LocationPayload>) -> StatusCode {
    let name = payload.name;
    let location: Result<Option<Location>, surrealdb::Error> = DB
        .create(("location", name.to_lowercase()))
        .content(Location::new(&name))
        .await;
    match location {
        Ok(l) => println!("Created: {l:?}"),
        Err(e) => println!("Error: {e}"),
    };
    StatusCode::CREATED
}

pub async fn handler_delete(Json(payload): Json<LocationPayload>) -> StatusCode {
    let name = payload.name;
    let location: Result<Option<Location>, surrealdb::Error> = DB
        .delete(("location", name.to_lowercase()))
        .await;
    match location {
        Ok(l) => println!("Removed: {l:?}"),
        Err(e) => println!("Error: {e}"),
    };
    StatusCode::OK
}

#[derive(Debug, Deserialize)]
pub struct LocationPayload {
    name: String,
}

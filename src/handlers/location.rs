use crate::models::location::Location;
use crate::models::user::User;
use crate::DB;
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use surrealdb::opt::auth::Scope;

type LocationResult = surrealdb::Result<Json<Vec<Location>>>;

pub async fn handler_get() -> Json<Vec<Location>> {
    // let jwt = DB
    //     .signin(Scope {
    //         namespace: "scouts",
    //         database: "scouts",
    //         scope: "user",
    //         params: User::new("Brian", "abc123"),
    //     })
    //     .await.unwrap();

    // dbg!(&jwt.as_insecure_token());
    // let valid = DB.authenticate(jwt).await.unwrap();
    let locations: Vec<Location> = DB.select("location").await.unwrap();
    // dbg!(&locations);
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

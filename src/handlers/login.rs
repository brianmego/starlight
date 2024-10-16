use crate::models::user::User;
use crate::{Result,Error, DB};
use axum::Json;
use serde::{Deserialize, Serialize};
use surrealdb::opt::auth::{Jwt, Record};

#[derive(Deserialize, Debug)]
pub struct Credentials {
    user: String,
    password: String,
}

pub async fn handler_post(Json(payload): Json<Credentials>) -> Result<Json<LoginResponse>> {
    dbg!("I am here");
    dbg!(&payload);
    let jwt = DB
        .signin(Record {
            namespace: "scouts",
            database: "scouts",
            access: "user",
            params: User::new(&payload.user, &payload.password),
        })
        .await?;

    dbg!(&jwt.as_insecure_token());
    println!("{} logged in!", &payload.user);
    let response = LoginResponse::new(jwt);
    Ok(Json(response))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    jwt: Jwt,
}

impl LoginResponse {
    fn new(jwt: Jwt) -> Self {
        Self { jwt }
    }
}

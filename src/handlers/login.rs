use crate::models::user::User;
use crate::{Result,Error, DB};
use axum::Json;
use serde::{Deserialize, Serialize};
use surrealdb::opt::auth::{Jwt, Scope};

#[derive(Deserialize, Debug)]
pub struct Credentials {
    user: String,
    password: String,
}

pub async fn handler_post(Json(payload): Json<Credentials>) -> Result<Json<LoginResponse>> {
    dbg!("I am here");
    dbg!("{}", &payload);
    let jwt = DB
        .signin(Scope {
            namespace: "scouts",
            database: "scouts",
            scope: "user",
            params: User::new(&payload.user, &payload.password),
        })
        .await?;

    dbg!(&jwt.as_insecure_token());
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

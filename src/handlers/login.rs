use crate::models::user::User;
use crate::{Error, Result, DB};
use chrono::Utc;
use axum::Json;
use serde::{Deserialize, Serialize};
use surrealdb::opt::auth::{Jwt, Record};
use jsonwebtoken::{encode, EncodingKey, Header};

#[derive(Deserialize, Debug)]
pub struct Credentials {
    user: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    ID: String,
    exp: i64
}

pub async fn handler_post(Json(payload): Json<Credentials>) -> Result<Json<LoginResponse>> {
    let _ = DB
        .signin(Record {
            namespace: "scouts",
            database: "scouts",
            access: "user",
            params: User::new(&payload.user, &payload.password),
        })
        .await?;
    let ts = Utc::now().timestamp();
    println!("{} logged in!", &payload.user);
    let claims = Claims{ID: payload.user, exp: ts};
    let jwt = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref()));
    let response = LoginResponse::new(jwt.unwrap());
    Ok(Json(response))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    jwt: String,
}

impl LoginResponse {
    fn new(jwt: String) -> Self {
        Self { jwt }
    }
}

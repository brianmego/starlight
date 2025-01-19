use log::warn;
use crate::{models::user::TroopType, Error, Result, DB};
use axum::Json;
use chrono::{Utc, TimeDelta};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Deserialize, Debug)]
pub struct Credentials {
    user: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    #[serde(rename = "ID")]
    id: String,
    trooptype: TroopType,
    is_admin: bool,
    exp: i64,
}
impl Claims {
    pub fn id(&self) -> String {
        self.id.clone()
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbUser {
    id: RecordId,
    trooptype: RecordId,
    is_admin: bool,
}

pub async fn handler_post(Json(payload): Json<Credentials>) -> Result<Json<LoginResponse>> {
    let username = payload.user.clone();
    let password = payload.password.clone();
    let user_id: Option<DbUser> = DB.query("SELECT id, is_admin, trooptype FROM user WHERE username = $username AND crypto::argon2::compare(password, $password)")
        .bind(("username", username))
        .bind(("password", password))
        .await?.take(0)?;
    let ts = (Utc::now() + TimeDelta::minutes(60)).timestamp();
    match user_id {
        Some(u) => {
            println!("{} logged in!", &payload.user);
            let claims = Claims {
                id: u.id.to_string(),
                trooptype: u.trooptype.into(),
                is_admin: u.is_admin,
                exp: ts,
            };
            let jwt = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret("secret".as_ref()),
            );
            let response = LoginResponse::new(jwt.unwrap());
            Ok(Json(response))
        }
        None => {
            warn!("Bad login attempt for {}", payload.user);
            Err(Error::LoginError(LoginError))
        },
    }
}
#[derive(Debug)]
pub struct LoginError;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    jwt: String,
}

impl LoginResponse {
    fn new(jwt: String) -> Self {
        Self { jwt }
    }
}

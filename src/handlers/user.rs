use crate::models::user::User;
use crate::AppState;
use axum::extract::{Path, State};
use axum::Json;
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};

use super::reservation::{now, RegistrationWindow};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserGetResponse {
    user: User,
    tokens_used: u32,
    total_tokens: u32,
    now: String,
    time_until_next_unlock: i64
}

impl UserGetResponse {
    async fn new(user: User, window: RegistrationWindow<Tz>) -> Self {
        let tokens_used = user.tokens_used(&window).await;
        let now = window.now().format("%m-%d-%Y %H:%M:%S").to_string();
        let total_tokens = user.total_tokens(&window);
        let time_until_next_unlock = window.time_until_next_unlock();
        Self {
            user,
            tokens_used,
            total_tokens,
            now,
            time_until_next_unlock
        }
    }
}

pub async fn handler_get(
    Path(user_id): Path<String>,
    State(state): State<AppState>,
) -> Json<Option<UserGetResponse>> {
    let offset = state.time_offset;
    let window = RegistrationWindow::new(now(offset));
    let user = User::get_by_id(&user_id).await;
    let resp = match user {
        Some(u) => Some(UserGetResponse::new(u, window).await),
        None => None,
    };
    Json(resp)
}

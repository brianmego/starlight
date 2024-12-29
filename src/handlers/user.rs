use crate::models::user::User;
use axum::extract::Path;
use axum::Json;
use serde::{Deserialize, Serialize};

use super::reservation::RegistrationWindow;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserGetResponse {
    user: User,
    tokens_used: u32,
    total_tokens: u32,
    now: String
}

impl UserGetResponse {
    async fn new(user: User) -> Self {
        let registration_window = RegistrationWindow::default();
        let tokens_used = user.tokens_used().await;
        let now = registration_window.now().format("%m-%d-%Y %H:%M").to_string();
        let total_tokens = user.total_tokens(Some(registration_window));
        Self {
            user,
            tokens_used,
            total_tokens,
            now
        }
    }
}

pub async fn handler_get(Path(user_id): Path<String>) -> Json<Option<UserGetResponse>> {
    let user = User::get_by_id(&user_id).await;
    let resp = match user {
        Some(u) => Some(UserGetResponse::new(u).await),
        None => None,
    };
    Json(resp)
}

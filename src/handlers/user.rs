use crate::models::user::User;
use axum::extract::Path;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserGetResponse {
    user: User,
    tokens_used: u32,
    total_tokens: u32,
}

impl UserGetResponse {
    async fn new(user: User) -> Self {
        let tokens_used = user.tokens_used().await;
        let total_tokens = user.total_tokens();
        Self {
            user,
            tokens_used,
            total_tokens,
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

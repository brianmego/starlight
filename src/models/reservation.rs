use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use crate::{
    models::user::User,
    DB,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Reservation {
    day: DateTime<Utc>,
    duration: u8,
    location: RecordId,
    id: RecordId,
}
impl Reservation {
    pub async fn get_by_id(id: &str) -> Option<Self> {
        DB.select(("reservation", id)).await.unwrap()
    }
    pub fn day(&self) -> DateTime<Utc> {
        self.day.clone()
    }
    pub async fn is_reservable_by_user(
        &self,
        user_id: &str,
        next_week_start: DateTime<Tz>,
    ) -> bool {
        let is_next_week = self.day() > next_week_start;
        match is_next_week {
            true => {
                let user = User::get_by_id(user_id).await.unwrap();
                let current_res_count = user.tokens_used().await;
                match user.total_tokens(None) > current_res_count {
                    true => true,
                    false => false,
                }
            }
            false => true,
        }
    }
}

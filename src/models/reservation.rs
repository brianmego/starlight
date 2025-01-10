use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use crate::{handlers::reservation::RegistrationWindow, models::user::User, DB};

pub enum UnreservableReason {
    NotEnoughTokens,
    AlreadyReserved(String),
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Reservation {
    day: DateTime<Utc>,
    duration: u8,
    location: RecordId,
    id: RecordId,
    reserved_by: Option<RecordId>,
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
        window: RegistrationWindow<Tz>
    ) -> Result<(), UnreservableReason> {
        match &self.reserved_by {
            Some(id) => {
                let key = id.key();
                return Err(UnreservableReason::AlreadyReserved(key.to_string()));
            }
            None => {
                let is_next_week = self.day() > window.next_week_start();
                match is_next_week {
                    true => {
                        let user = User::get_by_id(user_id).await.unwrap();
                        let current_res_count = user.tokens_used(&window).await;
                        match user.total_tokens(&window) > current_res_count {
                            true => Ok(()),
                            false => Err(UnreservableReason::NotEnoughTokens),
                        }
                    }
                    false => Ok(()),
                }
            }
        }
    }
}

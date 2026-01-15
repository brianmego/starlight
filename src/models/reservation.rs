use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use crate::{
    DB,
    handlers::reservation::RegistrationWindow,
    models::user::{TroopType, User},
};

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
        self.day
    }
    pub async fn is_reservable_by_user(
        &self,
        user_id: &str,
        window: &RegistrationWindow<Tz>,
        swapping_for_token: bool,
        troop_type: TroopType,
    ) -> Result<(), UnreservableReason> {
        if let Some(id) = &self.reserved_by {
            let key = id.key();
            Err(UnreservableReason::AlreadyReserved(key.to_string()))
        } else {
            let is_next_week = match troop_type {
                TroopType::FridayOnly => self.day() > window.last_week_start(),
                _ => self.day() > window.next_week_start(),
            };

            if is_next_week {
                let user = User::get_by_id(user_id).await.unwrap();
                let current_res_count = if swapping_for_token {
                    user.tokens_used(window).await - 1
                } else {
                    user.tokens_used(window).await
                };
                if user.total_tokens(window) > current_res_count {
                    Ok(())
                } else {
                    Err(UnreservableReason::NotEnoughTokens)
                }
            } else {
                Ok(())
            }
        }
    }
    pub fn will_cost_token(&self, registration_window: &RegistrationWindow<Tz>, troop_type: TroopType) -> bool {
        match troop_type {
            TroopType::FridayOnly => self.day() >= registration_window.last_week_start(),
            _ => self.day() >= registration_window.next_week_start()
        }
    }
}

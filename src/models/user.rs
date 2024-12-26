use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime as SurrealDateTime;
use surrealdb::RecordId;

use crate::{
    handlers::reservation::{now, RegistrationWindow},
    queries, DB,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TroopType {
    Level1,
    Level2,
}
impl From<RecordId> for TroopType {
    fn from(value: RecordId) -> Self {
        if value.key().to_string() == "level1" {
            TroopType::Level1
        } else if value.key().to_string() == "level2" {
            TroopType::Level2
        } else {
            unreachable!("Only 2 troop types in the DB")
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    id: String,
    trooptype: TroopType,
    username: String,
}

impl User {
    pub async fn get_by_id(id: &str) -> Option<Self> {
        let (_, id) = id.split_once(':')?;
        let row: UserDbRecord = DB.select(("user", id)).await.unwrap()?;
        Some(row.into())
    }
    pub fn record_id(&self) -> RecordId {
        RecordId::from(("user", &self.id))
    }
    pub async fn tokens_used(&self) -> u32 {
        let registration_window = RegistrationWindow::new(now());
        let next_week_start = SurrealDateTime::from(registration_window.next_week_start().to_utc());
        let mut response = DB
            .query(queries::USER_TOKEN_USAGE_COUNT)
            .bind(("user", self.record_id()))
            .bind(("next_week_start", next_week_start))
            .await
            .unwrap();
        let tokens_used: Option<i32> = response.take(0).unwrap();
        dbg!(self.record_id(), registration_window.next_week_start(), &tokens_used);
        tokens_used
            .unwrap_or_default()
            .try_into()
            .unwrap_or_default()
    }
    pub fn total_tokens(&self) -> u32 {
        match self.trooptype {
            TroopType::Level1 => 1,
            TroopType::Level2 => 2,
        }
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserDbRecord {
    id: RecordId,
    trooptype: RecordId,
    username: String,
}

impl From<UserDbRecord> for User {
    fn from(value: UserDbRecord) -> Self {
        Self {
            id: value.id.key().to_string(),
            trooptype: value.trooptype.into(),
            username: value.username,
        }
    }
}

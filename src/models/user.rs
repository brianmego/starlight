use chrono::{Datelike, Timelike};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;
use surrealdb::sql::Datetime as SurrealDateTime;

use crate::{DB, handlers::reservation::RegistrationWindow, queries};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TroopType {
    Level1, // M, W-F tokens
    Level2, // M-F tokens
    Level3, // infinite tokens
}
impl From<RecordId> for TroopType {
    fn from(value: RecordId) -> Self {
        if value.key().to_string() == "level1" {
            TroopType::Level1
        } else if value.key().to_string() == "level2" {
            TroopType::Level2
        } else if value.key().to_string() == "level3" {
            TroopType::Level3
        } else {
            unreachable!("Only 3 troop types in the DB")
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SwapReservationDBResult {
    id: RecordId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SwapReservationResult {
    id: String,
}
impl SwapReservationResult {
    pub fn id(&self) -> String {
        self.id.clone()
    }
}

impl From<SwapReservationDBResult> for SwapReservationResult {
    fn from(value: SwapReservationDBResult) -> Self {
        Self {
            id: value.id.key().to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    id: String,
    trooptype: TroopType,
    username: String,
    is_admin: bool,
}

impl User {
    #[cfg(test)]
    fn new(id: &str, troop_type: TroopType, username: &str, is_admin: bool) -> Self {
        User {
            id: id.into(),
            trooptype: troop_type,
            username: username.into(),
            is_admin,
        }
    }

    pub async fn get_by_id(id: &str) -> Option<Self> {
        let (_, id) = id.split_once(':')?;
        let row: UserDbRecord = DB.select(("user", id)).await.unwrap()?;
        Some(row.into())
    }
    pub fn record_id(&self) -> RecordId {
        RecordId::from(("user", &self.id))
    }
    pub async fn tokens_used(&self, window: &RegistrationWindow<Tz>) -> u32 {
        let next_week_start = SurrealDateTime::from(window.next_week_start().to_utc());
        let mut response = DB
            .query(queries::USER_TOKEN_USAGE_COUNT)
            .bind(("user", self.record_id()))
            .bind(("next_week_start", next_week_start))
            .await
            .unwrap();
        let tokens_used: Option<i32> = response.take(0).unwrap();
        tokens_used
            .unwrap_or_default()
            .try_into()
            .unwrap_or_default()
    }
    pub async fn get_swap_reservation(&self, window: &RegistrationWindow<Tz>) -> Option<SwapReservationResult> {
        let next_week_start = SurrealDateTime::from(window.next_week_start().to_utc());
        let query = DB
            .query(queries::USER_SWAP_RESERVATION)
            .bind(("user", self.record_id()))
            .bind(("next_week_start", next_week_start));
        let mut db_res: Option<SwapReservationDBResult> = query.await.unwrap().take(0).unwrap();
        db_res.take().map(SwapReservationResult::from)
    }

    pub fn total_tokens(&self, window: &RegistrationWindow<Tz>) -> u32 {
        // Level1
        // M T W R F S U
        // 1 0 1 1 1 0 0

        // Level2
        // M T W R F S U
        // 1 1 1 1 1 0 0
        match self.trooptype {
            TroopType::Level1 => match window.now().hour() < 22 {
                true => match window.now().weekday() {
                    chrono::Weekday::Mon => 0,
                    chrono::Weekday::Tue => 1,
                    chrono::Weekday::Wed => 1,
                    chrono::Weekday::Thu => 2,
                    chrono::Weekday::Fri => 3,
                    chrono::Weekday::Sat => 0,
                    chrono::Weekday::Sun => 0,
                },
                false => match window.now().weekday() {
                    chrono::Weekday::Mon => 1,
                    chrono::Weekday::Tue => 1,
                    chrono::Weekday::Wed => 2,
                    chrono::Weekday::Thu => 3,
                    chrono::Weekday::Fri => 0,
                    chrono::Weekday::Sat => 0,
                    chrono::Weekday::Sun => 0,
                },
            },
            TroopType::Level2 => match window.now().hour() < 22 {
                true => match window.now().weekday() {
                    chrono::Weekday::Mon => 0,
                    chrono::Weekday::Tue => 1,
                    chrono::Weekday::Wed => 2,
                    chrono::Weekday::Thu => 3,
                    chrono::Weekday::Fri => 4,
                    chrono::Weekday::Sat => 0,
                    chrono::Weekday::Sun => 0,
                },
                false => match window.now().weekday() {
                    chrono::Weekday::Mon => 1,
                    chrono::Weekday::Tue => 2,
                    chrono::Weekday::Wed => 3,
                    chrono::Weekday::Thu => 4,
                    chrono::Weekday::Fri => 5,
                    chrono::Weekday::Sat => 0,
                    chrono::Weekday::Sun => 0,
                },
            },
            TroopType::Level3 => 99, // 99 problems, but a booth ain't one
        }
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserDbRecord {
    id: RecordId,
    trooptype: RecordId,
    username: String,
    is_admin: bool,
}

impl From<UserDbRecord> for User {
    fn from(value: UserDbRecord) -> Self {
        Self {
            id: value.id.key().to_string(),
            trooptype: value.trooptype.into(),
            username: value.username,
            is_admin: value.is_admin,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use chrono_tz::America::Chicago;
    use test_case::test_case;

    #[test_case(
        User::new("95ophx5ryqhqku7qn93d", TroopType::Level1, "Name", false),
        RegistrationWindow::new(Chicago.with_ymd_and_hms(2025, 1, 18, 19, 0, 0).unwrap()),
        0; "Lvl1 - Saturday"
    )]
    #[test_case(
        User::new("95ophx5ryqhqku7qn93d", TroopType::Level1, "Name", false),
        RegistrationWindow::new(Chicago.with_ymd_and_hms(2025, 1, 19, 19, 0, 0).unwrap()),
        0; "Lvl1 - Sunday"
    )]
    #[test_case(
        User::new("95ophx5ryqhqku7qn93d", TroopType::Level1, "Name", false),
        RegistrationWindow::new(Chicago.with_ymd_and_hms(2025, 1, 20, 19, 0, 0).unwrap()),
        0; "Lvl1 - Monday before 10"
    )]
    #[test_case(
        User::new("95ophx5ryqhqku7qn93d", TroopType::Level1, "Name", false),
        RegistrationWindow::new(Chicago.with_ymd_and_hms(2025, 1, 20, 22, 0, 0).unwrap()),
        1; "Lvl1 - Monday after 10"
    )]
    #[test_case(
        User::new("95ophx5ryqhqku7qn93d", TroopType::Level1, "Name", false),
        RegistrationWindow::new(Chicago.with_ymd_and_hms(2025, 1, 21, 19, 0, 0).unwrap()),
        1; "Lvl1 - Tuesday before 10"
    )]
    #[test_case(
        User::new("95ophx5ryqhqku7qn93d", TroopType::Level1, "Name", false),
        RegistrationWindow::new(Chicago.with_ymd_and_hms(2025, 1, 21, 22, 0, 0).unwrap()),
        1; "Lvl1 - Tuesday after 10"
    )]
    #[test_case(
        User::new("95ophx5ryqhqku7qn93d", TroopType::Level1, "Name", false),
        RegistrationWindow::new(Chicago.with_ymd_and_hms(2025, 1, 22, 19, 0, 0).unwrap()),
        1; "Lvl1 - Wednesday before 10"
    )]
    #[test_case(
        User::new("95ophx5ryqhqku7qn93d", TroopType::Level1, "Name", false),
        RegistrationWindow::new(Chicago.with_ymd_and_hms(2025, 1, 22, 22, 0, 0).unwrap()),
        2; "Lvl1 - Wednesday after 10"
    )]
    #[test_case(
        User::new("95ophx5ryqhqku7qn93d", TroopType::Level1, "Name", false),
        RegistrationWindow::new(Chicago.with_ymd_and_hms(2025, 1, 23, 19, 0, 0).unwrap()),
        2; "Lvl1 - Thursday before 10"
    )]
    #[test_case(
        User::new("95ophx5ryqhqku7qn93d", TroopType::Level1, "Name", false),
        RegistrationWindow::new(Chicago.with_ymd_and_hms(2025, 1, 23, 22, 0, 0).unwrap()),
        3; "Lvl1 - Thursday after 10"
    )]
    #[test_case(
        User::new("95ophx5ryqhqku7qn93d", TroopType::Level1, "Name", false),
        RegistrationWindow::new(Chicago.with_ymd_and_hms(2025, 1, 24, 17, 0, 0).unwrap()),
        3; "Lvl1 - Friday before 10"
    )]
    #[test_case(
        User::new("95ophx5ryqhqku7qn93d", TroopType::Level1, "Name", false),
        RegistrationWindow::new(Chicago.with_ymd_and_hms(2025, 1, 24, 22, 0, 0).unwrap()),
        0; "Lvl1 - Friday after 10"
    )]
    #[test_case(
        User::new("95ophx5ryqhqku7qn93d", TroopType::Level2, "Name", false),
        RegistrationWindow::new(Chicago.with_ymd_and_hms(2025, 1, 18, 19, 0, 0).unwrap()),
        0; "Lvl2 - Saturday"
    )]
    #[test_case(
        User::new("95ophx5ryqhqku7qn93d", TroopType::Level2, "Name", false),
        RegistrationWindow::new(Chicago.with_ymd_and_hms(2025, 1, 19, 19, 0, 0).unwrap()),
        0; "Lvl2 - Sunday"
    )]
    #[test_case(
        User::new("95ophx5ryqhqku7qn93d", TroopType::Level2, "Name", false),
        RegistrationWindow::new(Chicago.with_ymd_and_hms(2025, 1, 20, 19, 0, 0).unwrap()),
        0; "Lvl2 - Monday before 10"
    )]
    #[test_case(
        User::new("95ophx5ryqhqku7qn93d", TroopType::Level2, "Name", false),
        RegistrationWindow::new(Chicago.with_ymd_and_hms(2025, 1, 20, 22, 0, 0).unwrap()),
        1; "Lvl2 - Monday after 10"
    )]
    #[test_case(
        User::new("95ophx5ryqhqku7qn93d", TroopType::Level2, "Name", false),
        RegistrationWindow::new(Chicago.with_ymd_and_hms(2025, 1, 21, 19, 0, 0).unwrap()),
        1; "Lvl2 - Tuesday before 10"
    )]
    #[test_case(
        User::new("95ophx5ryqhqku7qn93d", TroopType::Level2, "Name", false),
        RegistrationWindow::new(Chicago.with_ymd_and_hms(2025, 1, 21, 22, 0, 0).unwrap()),
        2; "Lvl2 - Tuesday after 10"
    )]
    #[test_case(
        User::new("95ophx5ryqhqku7qn93d", TroopType::Level2, "Name", false),
        RegistrationWindow::new(Chicago.with_ymd_and_hms(2025, 1, 22, 19, 0, 0).unwrap()),
        2; "Lvl2 - Wednesday before 10"
    )]
    #[test_case(
        User::new("95ophx5ryqhqku7qn93d", TroopType::Level2, "Name", false),
        RegistrationWindow::new(Chicago.with_ymd_and_hms(2025, 1, 22, 22, 0, 0).unwrap()),
        3; "Lvl2 - Wednesday after 10"
    )]
    #[test_case(
        User::new("95ophx5ryqhqku7qn93d", TroopType::Level2, "Name", false),
        RegistrationWindow::new(Chicago.with_ymd_and_hms(2025, 1, 23, 19, 0, 0).unwrap()),
        3; "Lvl2 - Thursday before 10"
    )]
    #[test_case(
        User::new("95ophx5ryqhqku7qn93d", TroopType::Level2, "Name", false),
        RegistrationWindow::new(Chicago.with_ymd_and_hms(2025, 1, 23, 22, 0, 0).unwrap()),
        4; "Lvl2 - Thursday after 10"
    )]
    #[test_case(
        User::new("95ophx5ryqhqku7qn93d", TroopType::Level2, "Name", false),
        RegistrationWindow::new(Chicago.with_ymd_and_hms(2025, 1, 24, 17, 0, 0).unwrap()),
        4; "Lvl2 - Friday before 10"
    )]
    #[test_case(
        User::new("95ophx5ryqhqku7qn93d", TroopType::Level2, "Name", false),
        RegistrationWindow::new(Chicago.with_ymd_and_hms(2025, 1, 24, 22, 0, 0).unwrap()),
        5; "Lvl2 - Friday after 10"
    )]
    #[test_case(
        User::new("95ophx5ryqhqku7qn93d", TroopType::Level3, "Name", false),
        RegistrationWindow::new(Chicago.with_ymd_and_hms(2025, 1, 24, 22, 0, 0).unwrap()),
        99; "Lvl3 - Always 99"
    )]
    fn test_user_total_tokens(
        user: User,
        registration_window: RegistrationWindow<Tz>,
        expected: u32,
    ) {
        let actual = user.total_tokens(&registration_window);
        assert_eq!(actual, expected);
    }
}

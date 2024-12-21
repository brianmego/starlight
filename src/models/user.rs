use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TroopType {
    Level1,
    Level2
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    username: String,
    trooptype: TroopType
}

// impl User {
//     pub fn new(username: &str, trooptype: TroopType) -> Self {
//         let username = username.to_string();
//         Self { username, trooptype }
//     }
// }



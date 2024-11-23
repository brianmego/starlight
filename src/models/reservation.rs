use std::str::FromStr;

use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Reservation {
    id: RecordId,
    day_of_week: RecordId,
    location: RecordId,
    start: u8,
    duration: u8,
}

impl Reservation {
    pub fn new(start: u8, duration: u8) -> Self {
        let day_of_week = RecordId::from_str("dayofweek:1").unwrap();
        let location = RecordId::from_str("location:chuys").unwrap();
        let id = RecordId::from_str("reservation:70srxtzl3f26nrrxlf6h").unwrap();
        Self { id, day_of_week, location, start, duration }
    }
    pub fn start(&self) -> u8 {
        self.start
    }
    pub fn end(&self) -> u8 {
        self.start + self.duration
    }
}



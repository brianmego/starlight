use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TimeSlot {
    start: u8,
    end: u8
}

impl TimeSlot {
    pub fn new(start: u8, end: u8) -> Self {
        Self { start, end }
    }
    pub fn start(&self) -> u8 {
        self.start
    }
    pub fn end(&self) -> u8 {
        self.start
    }
}



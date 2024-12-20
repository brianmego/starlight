use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DayOfWeek {
    name: String,
}

impl DayOfWeek {
    pub fn new(name: &str) -> Self {
        let name = name.to_string();
        Self { name }
    }
}


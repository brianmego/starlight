use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Location {
    name: String,
    address: String,
    notes: Option<String>,
    enabled: bool
}

impl Location {
    pub fn new(name: &str, address: &str, notes: Option<&str>, enabled: bool) -> Self {
        let name = name.to_string();
        let address = address.to_string();
        let notes = match notes {
            Some(s) => Some(s.to_string()),
            None => None,
        };
        Self { name, address, notes, enabled }
    }
}


use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Location {
    name: String,
    address: String,
    notes: String
}

impl Location {
    pub fn new(name: &str, address: &str, notes: &str) -> Self {
        let name = name.to_string();
        let address = address.to_string();
        let notes = notes.to_string();
        Self { name, address, notes }
    }
}


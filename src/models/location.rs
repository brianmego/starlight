use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Location {
    name: String,
}

impl Location {
    pub fn new(name: &str) -> Self {
        let name = name.to_string();
        Self { name }
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
}


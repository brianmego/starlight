use serde::Serialize;
use axum::Json;

pub async fn handler() -> Json<Vec<Location>> {
    Json(vec![
        Location::new("Chuy's"),
        Location::new("Jardin Corona"),
        Location::new("Randall's"),
    ])
}

#[derive(Debug, Serialize)]
pub struct Location {
    name: String,
}

impl Location {
    fn new(name: &str) -> Self {
        let name = name.to_string();
        Self { name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_handler(){
        let result = handler().await;
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].name, "Chuy's");
    }
}

pub async fn handler() -> &'static str {
    "Ok"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_handler(){
        let result = handler().await;
        assert_eq!(result,"Ok");
    }
}

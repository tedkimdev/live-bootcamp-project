use std::collections::{HashSet};

use crate::domain::{BannedTokenStore, BannedTokenStoreError};


#[derive(Default)]
pub struct HashsetBannedTokenStore {
    pub tokens: HashSet<String>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn add_token(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
        self.tokens.insert(token);
        Ok(())
    }
    async fn contains_token(&self, token: &str) -> Result<bool, BannedTokenStoreError> {
        Ok(self.tokens.contains(token))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_token() {
        let mut banned_token_store = HashsetBannedTokenStore::default();

        let token = "token";
        
        let result = banned_token_store.add_token(token.to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_contains_token() {
        let mut banned_token_store = HashsetBannedTokenStore::default();

        let token = "token";
        
        let result = banned_token_store.add_token(token.to_string()).await;
        assert!(result.is_ok());

        let result = banned_token_store.contains_token(token).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
        
        let unlisted_token = "unlisted_token";
        let result = banned_token_store.contains_token(unlisted_token).await;
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }
}
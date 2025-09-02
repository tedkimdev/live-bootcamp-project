use std::collections::{HashSet};

use crate::domain::{BannedTokenStore, BannedTokenStoreError};


#[derive(Default)]
pub struct HashsetBannedTokenStore {
    pub tokens: HashSet<String>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn add_banned_token(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
        if self.tokens.insert(token) {
            Ok(())
        } else {
            Err(BannedTokenStoreError::UnexpectedError)
        }
    }
    async fn get_banned_token(&self, token: String) -> Result<String, BannedTokenStoreError> {
        if self.tokens.contains(&token) {
            Ok(token)
        } else {
            Err(BannedTokenStoreError::TokenNotFound)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_banned_token() {
        let mut banned_token_store = HashsetBannedTokenStore::default();

        let token = "token";
        
        let result = banned_token_store.add_banned_token(token.to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_banned_token() {
        let mut banned_token_store = HashsetBannedTokenStore::default();

        let token = "token";
        
        let result = banned_token_store.add_banned_token(token.to_string()).await;
        assert!(result.is_ok());

        let result = banned_token_store.get_banned_token(token.to_string()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), token);
        
        let unlisted_token = "unlisted_token";
        let result = banned_token_store.get_banned_token(unlisted_token.to_string()).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), BannedTokenStoreError::TokenNotFound);

    }
}
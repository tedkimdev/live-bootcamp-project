use std::collections::HashMap;

use crate::domain::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Default)]
pub struct HashmapUserStore {
    pub users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        match self.users.get(&user.email) {
            Some(_u) => Err(UserStoreError::UserAlreadyExists),
            None => {
                self.users.insert(user.email.clone(), user);
                Ok(())
            }
        }
    }

    pub fn get_user(&self, email: &str) -> Result<&User, UserStoreError> {
        match self.users.get(email) {
            Some(u) => Ok(u),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        if let Some(u) = self.users.get(email) {
            if u.password == password {
                return Ok(());
            } else {
                return Err(UserStoreError::InvalidCredentials);
            }
        }
        Err(UserStoreError::UserNotFound)
    }
}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            


#[cfg(test)]
mod tests{
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut user_store = HashmapUserStore {
            users: HashMap::new(),
        };
        let email = "dev.ted.kim@gmail.com";
        let user = User::new(email.to_owned(), "password".to_owned(), true);

        let result = user_store.add_user(user);
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_user() {
        let mut user_store = HashmapUserStore {
            users: HashMap::new(),
        };
        let email = "dev.ted.kim@gmail.com";
        let password = "password";
        let require_2fa = true;
        let user = User::new(email.to_owned(), password.to_owned(), require_2fa);

        let result = user_store.add_user(user);
        assert!(result.is_ok());

        let result = user_store.get_user(email);
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.email, email.to_string());
        assert_eq!(user.password, password);
        assert_eq!(user.require_2fa, require_2fa);
    }
    #[tokio::test]
    async fn test_validate_user() {
        let mut user_store = HashmapUserStore {
            users: HashMap::new(),
        };
        let email = "dev.ted.kim@gmail.com";
        let password = "password";
        let require_2fa = true;
        let user = User::new(email.to_owned(), password.to_owned(), require_2fa);

        let result = user_store.add_user(user);
        assert!(result.is_ok());

        let result = user_store.validate_user(email, "wrong_password");
        assert!(result.is_err());
        let err = result.unwrap_err();
        
        assert_eq!(err, UserStoreError::InvalidCredentials);
    }
    
}
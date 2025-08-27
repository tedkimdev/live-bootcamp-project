use std::collections::HashMap;

use crate::domain::{Email, Password, User, UserStore, UserStoreError};

#[derive(Default)]
pub struct HashmapUserStore {
    pub users: HashMap<Email, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        match self.users.get(&user.email) {
            Some(_u) => Err(UserStoreError::UserAlreadyExists),
            None => {
                self.users.insert(user.email.clone(), user);
                Ok(())
            }
        }
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(u) => Ok(u.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    async fn validate_user(&self, email: &Email, password: &Password) -> Result<(), UserStoreError> {
        if let Some(u) = self.users.get(email) {
            if u.password == *password {
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
        let email = Email::parse("dev.ted.kim@gmail.com".to_string()).unwrap();
        let password = Password::parse("password".to_string()).unwrap();
        
        let user = User::new(
            email,
            password,
            true,
        );

        let result = user_store.add_user(user).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_user() {
        let mut user_store = HashmapUserStore {
            users: HashMap::new(),
        };
        let email = Email::parse("dev.ted.kim@gmail.com".to_string()).unwrap();
        let password = Password::parse("password".to_string()).unwrap();
        let require_2fa = true;
        let user = User::new(email.clone(), password.clone(), require_2fa);

        let result = user_store.add_user(user).await;
        assert!(result.is_ok());

        let result = user_store.get_user(&email).await;
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.email, email);
        assert_eq!(user.password, password);
        assert_eq!(user.require_2fa, require_2fa);
    }
    #[tokio::test]
    async fn test_validate_user() {
        let mut user_store = HashmapUserStore {
            users: HashMap::new(),
        };
        
        let email = Email::parse("dev.ted.kim@gmail.com".to_string()).unwrap();
        let password = Password::parse("password".to_string()).unwrap();
        let wrong_password = Password::parse("wrong_password".to_string()).unwrap();
        let require_2fa = true;
        
        let user = User::new(email.clone(), password, require_2fa);

        let result = user_store.add_user(user).await;
        assert!(result.is_ok());

        let result = user_store.validate_user(&email, &wrong_password).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        
        assert_eq!(err, UserStoreError::InvalidCredentials);
    }
    
}
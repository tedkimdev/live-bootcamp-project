use std::collections::HashMap;

use crate::domain::{Email, Password, User, UserStore, UserStoreError};

#[derive(Default)]
pub struct HashmapUserStore {
    pub users: HashMap<Email, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // match self.users.get(&user.email) {
        //     Some(_u) => Err(UserStoreError::UserAlreadyExists),
        //     None => {
        //         self.users.insert(user.email.clone(), user);
        //         Ok(())
        //     }
        // }
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(u) => Ok(u.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    async fn validate_user(&self, email: &Email, password: &Password) -> Result<(), UserStoreError> {
        match self.users.get(email) {
            Some(user) => {
                if user.password.eq(password) {
                    Ok(())
                } else {
                    Err(UserStoreError::InvalidCredentials)
                }
            }
            None => Err(UserStoreError::UserNotFound),
        }
    }

    async fn delete_user(&mut self, email: &Email, password: &Password) -> Result<(), UserStoreError> {
        if self.validate_user(email, password).await.is_ok() {
            match self.users.remove(email) {
                Some(_u) => return Ok(()),
                None => {
                    
                    return Err(UserStoreError::UserNotFound);
                },
            }
        };

        Err(UserStoreError::InvalidCredentials)
    }
}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            


#[cfg(test)]
mod tests{
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut user_store = HashmapUserStore::default();
        let email = Email::parse("dev.ted.kim@gmail.com".to_string()).unwrap();
        let password = Password::parse("password".to_string()).unwrap();
        
        let user = User::new(
            email,
            password,
            false,
        );

        // Test adding a new user
        let result = user_store.add_user(user.clone()).await;
        assert!(result.is_ok());

        // Test adding an existing user
        let result = user_store.add_user(user).await;
        assert_eq!(result, Err(UserStoreError::UserAlreadyExists));
    }
    #[tokio::test]
    async fn test_get_user() {
        let mut user_store = HashmapUserStore::default();
        let email = Email::parse("dev.ted.kim@gmail.com".to_string()).unwrap();
        let password = Password::parse("password".to_string()).unwrap();
        let require_2fa = false;
        let user = User::new(email.clone(), password.clone(), require_2fa);

        // Test getting a user that exists
        user_store.users.insert(email.clone(), user.clone());
        let result = user_store.get_user(&email).await;
        assert_eq!(result, Ok(user));

        // Test getting a user that doesn't exist
        let result = user_store
            .get_user(&Email::parse("nonexistent@example.com".to_owned()).unwrap())
            .await;

        assert_eq!(result, Err(UserStoreError::UserNotFound));
    }
    #[tokio::test]
    async fn test_validate_user() {
        let mut user_store = HashmapUserStore::default();
        
        let email = Email::parse("dev.ted.kim@gmail.com".to_string()).unwrap();
        let password = Password::parse("password".to_string()).unwrap();
        let require_2fa = false;
        
        let user = User::new(email.clone(), password.clone(), require_2fa);

        // Test validating a user that exists with correct password
        user_store.users.insert(email.clone(), user.clone());
        let result = user_store.validate_user(&email, &password).await;
        assert_eq!(result, Ok(()));

        // Test validating a user that exists with incorrect password
        let wrong_password = Password::parse("wrong_password".to_string()).unwrap();
        let result = user_store.validate_user(&email, &wrong_password).await;
        assert_eq!(result, Err(UserStoreError::InvalidCredentials));

        // Test validating a user that doesn't exist
        let result = user_store
            .validate_user(
                &Email::parse("nonexistent@example.com".to_string()).unwrap(),
                &password,
            )
            .await;

        assert_eq!(result, Err(UserStoreError::UserNotFound));
    }

    #[tokio::test]
    async fn test_delete_user() {
        let mut user_store = HashmapUserStore {
            users: HashMap::new(),
        };
        
        let email = Email::parse("dev.ted.kim@gmail.com".to_string()).unwrap();
        let password = Password::parse("password".to_string()).unwrap();
        let wrong_password = Password::parse("wrong_password".to_string()).unwrap();
        let require_2fa = true;

        let user = User::new(email.clone(), password.clone(), require_2fa);

        let result = user_store.add_user(user).await;
        assert!(result.is_ok());

        let result = user_store.delete_user(&email, &wrong_password).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        
        assert_eq!(err, UserStoreError::InvalidCredentials);

        let result = user_store.delete_user(&email, &password).await;
        assert!(result.is_ok());
    }
    
}
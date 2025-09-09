use std::collections::HashMap;

use crate::domain::{Email, LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError};


#[derive(Default)]
pub struct HashmapTwoFACodeStore {
    codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        self.codes.insert(email, (login_attempt_id, code));
        Ok(())
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        self.codes.remove(email);
        Ok(())
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        match self.codes.get(email) {
            Some(v) => Ok(v.clone()),
            None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_code() {
        let mut two_fa_code_store = HashmapTwoFACodeStore::default();
        let email = Email::parse("dev.ted.kim@gmail.com".to_string()).unwrap();
        let login_attempt_id = LoginAttemptId::parse("4dce63c8-2031-4e79-ad59-145fef4bd15b".to_string()).unwrap();
        let code = TwoFACode::parse("123456".to_string()).unwrap();
        

        let result = two_fa_code_store.add_code(email, login_attempt_id, code).await;

        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn test_remove_code() {
        let mut two_fa_code_store = HashmapTwoFACodeStore::default();
        let email = Email::parse("dev.ted.kim@gmail.com".to_string()).unwrap();
        let login_attempt_id = LoginAttemptId::parse("4dce63c8-2031-4e79-ad59-145fef4bd15b".to_string()).unwrap();
        let code = TwoFACode::parse("123456".to_string()).unwrap();
        

        let result = two_fa_code_store.add_code(email.clone(), login_attempt_id, code).await;

        assert!(result.is_ok());

        let result = two_fa_code_store.remove_code(&email).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_code() {
        let mut two_fa_code_store = HashmapTwoFACodeStore::default();
        let email = Email::parse("dev.ted.kim@gmail.com".to_string()).unwrap();
        let login_attempt_id = LoginAttemptId::parse("4dce63c8-2031-4e79-ad59-145fef4bd15b".to_string()).unwrap();
        let code = TwoFACode::parse("123456".to_string()).unwrap();
        

        let result = two_fa_code_store.add_code(email.clone(), login_attempt_id.clone(), code.clone()).await;

        assert!(result.is_ok());

        let result = two_fa_code_store.get_code(&email).await;
        assert!(result.is_ok());
        assert_eq!(result.as_ref().unwrap().0, login_attempt_id);
        assert_eq!(result.unwrap().1, code);
    }
}
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Email(pub String);

impl Email {
    pub fn parse(email: String) -> Result<Self, EmailError> {
        if !validator::validate_email(email.clone()) {
            return Err(EmailError::InvalidEmail);
        }
        Ok(Email(email))
    }
}

#[derive(Debug)]
pub enum EmailError {
    InvalidEmail,
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::{faker, Fake};
    use quickcheck::quickcheck;

    #[test]
    fn test_email() {
        let result = Email::parse("dev.ted.kim@gmail.com".to_string());

        assert!(result.is_ok());
    }
    quickcheck! {
        fn prop_random_email_valid() -> bool {
            let email: String = faker::internet::en::FreeEmail().fake();

            Email::parse(email).is_ok()
        }

        fn prop_random_strings_invalid(s: String) -> bool {
            match Email::parse(s.clone()) {
                Ok(_) => validator::validate_email(&s),
                Err(_) => !validator::validate_email(&s),
            }
        }
    }
}

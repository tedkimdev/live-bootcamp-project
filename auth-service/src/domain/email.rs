#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Email(pub String);

impl Email {
    pub fn parse(s: String) -> Result<Self, String> {
        if !validator::validate_email(s.clone()) {
            return Err(format!("{} is not a valid email.", s));
        }
        Ok(Email(s))
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::{faker::{self, internet::en::SafeEmail}, Fake};
    use quickcheck::quickcheck;

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert!(Email::parse(email).is_err());
    }
    #[test]
    fn test_email() {
        let result = Email::parse("dev.ted.kim@gmail.com".to_string());

        assert!(result.is_ok());
    }
    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com".to_string();
        assert!(Email::parse(email).is_err());
    }
    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();
        assert!(Email::parse(email).is_err());
    }

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let email = SafeEmail().fake_with_rng(g);
            Self(email)
        }
    }
    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        Email::parse(valid_email.0).is_ok()
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

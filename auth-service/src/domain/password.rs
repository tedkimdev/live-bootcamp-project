#[derive(Debug, Clone, PartialEq)]
pub struct Password(pub String);

impl Password {
    pub fn parse(s: String) -> Result<Password, String> {
        if validate_password(&s) {
            return Ok(Self(s));
        }
        Err("Failed to parse string to a Password type".to_owned())
    }
}

fn validate_password(s: &str) -> bool {
    s.len() >= 8
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::faker::internet::en::Password as FakePassword;
    use fake::Fake;

    use quickcheck::quickcheck;

    #[test]
    fn empty_string_is_rejected() {
        let password = "".to_owned();
        assert!(Password::parse(password).is_err());
    }

    #[test]
    fn string_less_than_8_characters_is_rejected() {
        let password = "1234567".to_owned();
        assert!(Password::parse(password).is_err());
    }

    #[derive(Debug, Clone)]
    struct ValidPasswordFixture(pub String);

    impl quickcheck::Arbitrary for ValidPasswordFixture {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let password = FakePassword(8..30).fake_with_rng(g);
            Self(password)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_passwords_are_parsed_successfully(valid_password: ValidPasswordFixture) -> bool {
        Password::parse(valid_password.0).is_ok()
    }

    quickcheck! {
        fn prop_parse_passwords(s: String) -> bool {
            match Password::parse(s.clone()) {
                Ok(_) => s.len() >= 8,
                Err(_) => s.len() < 8,
            }
        }
    }
}

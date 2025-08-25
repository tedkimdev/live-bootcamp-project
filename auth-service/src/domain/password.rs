#[derive(Debug, Clone, PartialEq)]
pub struct Password(pub String);

impl Password {
    pub fn parse(password: String) -> Result<Password, PasswordError> {
        if password.len() < 8 {
            return Err(PasswordError::InvalidPassword);
        }
        Ok(Password(password))
    }
}

#[derive(Debug)]
pub enum PasswordError {
    InvalidPassword,
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::quickcheck;

    quickcheck! {
        fn prop_parse_passwords(s: String) -> bool {
            match Password::parse(s.clone()) {
                Ok(_) => s.len() >= 8,
                Err(_) => s.len() < 8,
            }
        }
    }
}

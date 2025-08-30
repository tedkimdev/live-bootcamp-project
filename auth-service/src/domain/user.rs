use crate::domain::{Email, Password};

#[derive(Clone, Debug, PartialEq)]
pub struct User {
    pub email: Email,
    pub password: Password,
    pub require_2fa: bool,
}

impl User {
    pub fn new(email: Email, password: Password, require_2fa: bool) -> Self {
        Self {
            email,
            password,
            require_2fa,
        }
    }
}

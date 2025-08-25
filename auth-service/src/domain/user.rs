// The User struct should contain 3 fields. email, which is a String; 
// password, which is also a String; and requires_2fa, which is a boolean. 
pub struct User {
    pub email: String,
    pub password: String,
    pub require_2fa: bool,
}

impl User {
    pub fn new(email: String, password: String, require_2fa: bool) -> Self {
        Self {
            email,
            password,
            require_2fa,
        }
    }
}
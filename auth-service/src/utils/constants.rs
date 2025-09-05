use axum::http;
use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env as std_env;

// Define a lazily evaluated static. lazy_static is needed because std_env::var is not a const function.
lazy_static! {
    pub static ref JWT_SECRET: String = set_token();
    pub static ref ALLOWED_ORIGINS: Vec<http::HeaderValue> = set_allowed_origins();
}

fn set_token() -> String {
    dotenv().ok(); // Load environment variables
    let secret = std_env::var(env::JWT_SECRET_ENV_VAR).expect("JWT_SCRET must be set.");
    if secret.is_empty() {
        panic!("JWT_SECRET must not be empty.");
    }
    secret
}

fn set_allowed_origins() -> Vec<http::HeaderValue> {
    dotenv().ok();
    let origins = std_env::var(env::ALLOWED_ORIGINS_VAR).unwrap_or("".to_string());

    let allowed: Vec<http::HeaderValue> = origins
            .split(',')
            .filter_map(|o| o.trim().parse::<http::HeaderValue>().ok())
            .collect();
    allowed
}

pub mod env {
    pub const JWT_SECRET_ENV_VAR: &str =  "JWT_SECRET";
    pub const ALLOWED_ORIGINS_VAR: &str = "ALLOWED_ORIGINS";
}

pub const JWT_COOKIE_NAME: &str = "jwt";
pub const JWT_REFRESH_COOKIE_NAME: &str = "jwt_refresh";

pub mod prod {
    pub const APP_ADDRESS: &str = "0.0.0.0:3000";
}

pub mod test {
    pub const APP_ADDRESS: &str = "127.0.0.1:0";
}
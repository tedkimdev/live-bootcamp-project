use axum::{http, response::IntoResponse};
use axum_extra::extract::{cookie::Cookie, CookieJar};

use crate::{domain::AuthAPIError, utils::{validate_token, JWT_COOKIE_NAME}};

pub async fn logout(jar: CookieJar) ->(CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let cookie = match jar.get(JWT_COOKIE_NAME) {
        Some(cookie) => cookie,
        None => return (jar, Err(AuthAPIError::MissingToken)),
    };

    let token = cookie.value().to_owned();
    
    if validate_token(&token).await.is_err() {
        return (jar, Err(AuthAPIError::InvalidToken));
    };
    
    let jar = jar.remove(Cookie::from(JWT_COOKIE_NAME));

    (jar, Ok(http::StatusCode::OK))
}

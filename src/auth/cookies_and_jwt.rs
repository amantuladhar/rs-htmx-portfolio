use chrono::Duration;
use chrono::Utc;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use jwt::VerifyWithKey;
use serde::Deserialize;
use serde::Serialize;
use sha2::Sha256;

use crate::auth::decode_jwt_token_middleware::AUTH_TOKEN_KEY;

// TODO(Aman): add to env file
pub const TOKEN_EXPIRY_TIME_IN_SECONDS: i64 = 60 * 60;
const SECRET: &str = "some-secret";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoggedInUser {
    pub user_id: i32,
    pub username: String,
    pub exp: i64,
}

pub fn create_token(user_id: i32, username: &str) -> anyhow::Result<String> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(SECRET.as_bytes())?;
    let exp = Utc::now()
        .checked_add_signed(Duration::seconds(TOKEN_EXPIRY_TIME_IN_SECONDS))
        .expect("valid timestamp")
        .timestamp();
    let payload = LoggedInUser {
        user_id,
        username: username.to_string(),
        exp,
    };
    Ok(payload.sign_with_key(&key)?)
}

pub fn decode_token(token: &str) -> anyhow::Result<LoggedInUser> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(SECRET.as_bytes())?;
    let claim: LoggedInUser = token.verify_with_key(&key)?;
    Ok(claim)
}

pub fn create_cookie(value: String) -> String {
    create_cookie_with_exp_time(value, TOKEN_EXPIRY_TIME_IN_SECONDS)
}

pub fn create_cookie_with_exp_time(value: String, expiry_seconds: i64) -> String {
    let expiry_time = Utc::now()
        .checked_add_signed(Duration::seconds(expiry_seconds))
        .expect("should be able to add time")
        .format("%a, %d %b %Y %H:%M:%S GMT")
        .to_string();
    format!(
        "{}={}; Secure; HttpOnly; SameSite=Strict; Expires={}",
        AUTH_TOKEN_KEY, value, expiry_time
    )
}

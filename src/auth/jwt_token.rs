use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use jwt::VerifyWithKey;
use serde::Deserialize;
use serde::Serialize;
use sha2::Sha256;

#[derive(Serialize, Deserialize)]
pub struct TokenPayload {
    user_id: i32,
    username: String,
}

// TODO(Aman): add to env file
const SECRET: &str = "some-secret";

pub fn create_token(user_id: i32, username: &str) -> anyhow::Result<String> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(SECRET.as_bytes())?;
    let payload = TokenPayload {
        user_id,
        username: username.to_string(),
    };
    Ok(payload.sign_with_key(&key)?)
}

pub fn decode_token(token: &str) -> anyhow::Result<TokenPayload> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(SECRET.as_bytes())?;
    let claim: TokenPayload = token.verify_with_key(&key)?;
    Ok(claim)
}

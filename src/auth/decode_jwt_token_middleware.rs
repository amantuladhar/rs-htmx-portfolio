use axum::{
    body::Body,
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::extract::cookie::CookieJar;
use chrono::Utc;

use crate::auth::cookies_and_jwt::decode_token;

pub const AUTH_TOKEN_KEY: &str = "AUTH_TOKEN";

pub async fn decode_jwt_token(
    // State(pool): State<PgPool>,
    mut req: Request,
    next: Next,
) -> Result<Response, impl IntoResponse> {
    let headers = req.headers();
    let cookies_jar = CookieJar::from_headers(headers);
    tracing::debug!("cookies_jar: {:?}", cookies_jar);
    if let Some(auth_header) = cookies_jar.get(AUTH_TOKEN_KEY) {
        tracing::debug!("auth_header value: {}", auth_header.value());
        if let Ok(token_payload) = decode_token(auth_header.value()) {
            if token_payload.exp > Utc::now().timestamp() {
                req.extensions_mut().insert(token_payload);
            }
        }
    }
    Ok::<Response<Body>, ()>(next.run(req).await)
}

// Reference: https://docs.rs/axum/latest/axum/middleware/index.html#passing-state-from-middleware-to-handlers
// Rejection: https://github.com/tokio-rs/axum/blob/main/examples/customize-extractor-error/README.md

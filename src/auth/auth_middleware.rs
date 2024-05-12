use std::collections::HashMap;

use axum::{
    body::Body,
    extract::Request,
    http::{self, HeaderMap},
    middleware::Next,
    response::{IntoResponse, Response},
};
use chrono::Utc;

use crate::auth::cookies_and_jwt::{create_cookie_with_exp_time, decode_token};

pub const AUTH_TOKEN_KEY: &str = "AUTH_TOKEN";

pub async fn auth_or_401(
    // State(pool): State<PgPool>,
    mut req: Request,
    next: Next,
) -> Result<Response, impl IntoResponse> {
    // let auth_header = req
    //     .headers()
    //     .get(http::header::AUTHORIZATION)
    //     .and_then(|header| header.to_str().ok());
    let cookies = req
        .headers()
        .get_all(http::header::COOKIE)
        .into_iter()
        .map(|cookie| cookie.to_str().unwrap())
        .map(|cookie| {
            let split = cookie.split("=").collect::<Vec<&str>>();
            (split[0], split[1])
        })
        .collect::<HashMap<&str, &str>>();

    // let mut header_unauthorized = HeaderMap::new();
    // header_unauthorized.insert("hx-redirect", "/login".parse().unwrap());
    // header_unauthorized.insert(
    //     "Set-Cookie",
    //     create_cookie_with_exp_time("this-cookie-was-deleted".to_string(), 0)
    //         .parse()
    //         .unwrap(),
    // );
    if let Some(auth_header) = cookies.get(AUTH_TOKEN_KEY) {
        if let Ok(token_payload) = decode_token(auth_header) {
            if token_payload.exp > Utc::now().timestamp() {
                req.extensions_mut().insert(token_payload);
            }
        }
    }
    Ok::<Response<Body>, ()>(next.run(req).await)
}

// Reference: https://docs.rs/axum/latest/axum/middleware/index.html#passing-state-from-middleware-to-handlers
// Rejection: https://github.com/tokio-rs/axum/blob/main/examples/customize-extractor-error/README.md

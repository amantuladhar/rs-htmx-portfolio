use std::collections::HashMap;

use axum::{
    extract::{Request, State},
    http::{self, StatusCode},
    middleware::Next,
    response::Response,
};
use sqlx::PgPool;

#[derive(Clone)]
pub struct LoggedInUser {}

pub async fn auth(
    State(pool): State<PgPool>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
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
    tracing::info!(?cookies, "Auth token");
    let auth_header =  cookies.get("TOKEN");

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    // if let Some(current_user) = authorize_current_user(auth_header).await {
    //     // insert the current user into a request extension so the handler can
    //     // extract it
    //     req.extensions_mut().insert(current_user);
    //     Ok(next.run(req).await)
    // } else {
    //     Err(StatusCode::UNAUTHORIZED)
    // }
    Ok(next.run(req).await)
}

async fn authorize_current_user(auth_token: &str) -> Option<LoggedInUser> {
    todo!();
}

// Reference: https://docs.rs/axum/latest/axum/middleware/index.html#passing-state-from-middleware-to-handlers

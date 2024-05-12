use axum::{
    extract::rejection::ExtensionRejection,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::json;
use thiserror::Error;

use crate::auth::cookies_and_jwt::create_cookie_with_exp_time;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("🤬 Unauthorized")]
    Unauthorized(#[from] ExtensionRejection),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let mut header = HeaderMap::new();
        let (status, message) = match self {
            ApiError::Unauthorized(_) => {
                header.insert("hx-redirect", "/login".parse().unwrap());
                header.insert(
                    "Set-Cookie",
                    create_cookie_with_exp_time("this-cookie-was-deleted".to_string(), 0)
                        .parse()
                        .unwrap(),
                );
                (StatusCode::UNAUTHORIZED, self.to_string())
            }
        };
        let payload = json!({ "message": message});
        (status, header, Json(payload)).into_response()
    }
}

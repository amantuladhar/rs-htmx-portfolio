use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

// Make our own error that wraps `anyhow::Error`.
pub enum AppError {
    ErrStatusCodeOnly(StatusCode),
    ErrStatusCodeAndString(StatusCode, String),
}
// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        use AppError::*;
        match self {
            ErrStatusCodeAndString(code, msg) => return (code, msg).into_response(),
            ErrStatusCodeOnly(s) => return (s).into_response(),
        }
    }
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("🤬 Unauthorized")]
    Unauthorized(#[from] ExtensionRejection),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ApiError::Unauthorized(_) => (StatusCode::UNAUTHORIZED, self.to_string()),
        };
        let payload = json!({ "message": message});
        (status, Json(payload)).into_response()
    }
}

use axum::http::StatusCode;
use tracing::{subscriber::set_global_default, Level};

pub mod static_file_handler;

pub fn setup_log() {
    let subscriber = tracing_subscriber::fmt()
        // Use a more compact, abbreviated log format
        .compact()
        // .without_time()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(false)
        // Build the subscriber
        .with_max_level(Level::TRACE)
        .finish();
    set_global_default(subscriber).expect("Unable to set global default for tracing subscriber");
}

pub fn bad_request<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::BAD_REQUEST, err.to_string())
}
pub fn anyhow_400(err: anyhow::Error) -> (StatusCode, String) {
    bad_request(&*err)
}

pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

pub fn anyhow_err(err: anyhow::Error) -> (StatusCode, String) {
    internal_error(&*err)
}

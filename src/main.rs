use axum::{
    http::{header, HeaderMap, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Router,
};
use rust_embed::RustEmbed;
use shtml::{html, Component};
use templates::pages::{
    about_page::about_page,
    login_page::{login_page, submit_login},
    root_page::root_page,
};

mod templates;

#[derive(RustEmbed)]
#[folder = "public"]
struct Asset;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root_page))
        .route("/about", get(about_page))
        .route("/login", get(login_page))
        .route("/login", post(submit_login))
        .route("/public/*file", get(static_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// We use a wildcard matcher ("/dist/*file") to match against everything
// within our defined assets directory. This is the directory on our Asset
// struct below, where folder = "examples/public/".
async fn static_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();

    if path.starts_with("public/") {
        path = path.replace("public/", "");
    }

    StaticFile(path)
}
pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();

        match Asset::get(path.as_str()) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
            }
            None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
        }
    }
}

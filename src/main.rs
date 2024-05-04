use axum::{
    http::{header, HeaderMap},
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use shtml::{html, Component};
use templates::pages::root_page::root_page;
use tower_http::services::ServeDir;

mod templates;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/css", ServeDir::new("public/css"))
        .route("/", get(root_page))
        .route("/clicked", post(clicked_path));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
async fn clicked_path() -> impl IntoResponse {
    let html = html! {
        <div class="text-3xl bg-green-500">Returned from server</div>
    }
    .to_string();
    let mut headers = HeaderMap::new();
    headers.insert(header::SET_COOKIE, "NAME=AMAN".parse().unwrap());
    (headers, Html(html))
}

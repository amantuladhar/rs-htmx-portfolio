use auth_middleware::auth;
use axum::{
    extract::State, http::StatusCode, middleware, response::IntoResponse, routing::get, Router,
};
use db_config::setup_db;
use shtml::{html, Component, Render};
use sqlx::PgPool;
use static_file_handler::static_handler;
use templates::pages::{
    about_page::about_page,
    login_page::{login_page, submit_login},
    root_page::root_page,
    signup_page::{signup_page, signup_post_handler},
};
use utils::setup_log;

mod auth_middleware;
mod db_config;
mod error_handling;
mod static_file_handler;
mod templates;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().expect("dotenvy setup failed");
    setup_log();
    let pool = setup_db().await;

    let app = Router::new()
        .route("/", get(root_page))
        .route("/about", get(about_page))
        .route("/login", get(login_page).post(submit_login))
        .route("/signup", get(signup_page).post(signup_post_handler))
        .route("/test", get(test_get_handler))
        .route("/public/*file", get(static_handler))
        .route_layer(middleware::from_fn_with_state(pool.clone(), auth))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

pub struct User {
    id: i32,
    username: String,
    password: String,
    created_at: chrono::DateTime<chrono::Utc>,
}
pub async fn test_get_handler(State(pool): State<PgPool>) -> impl IntoResponse {
    let users = sqlx::query_as!(User, "SELECT * FROM rs_portfolio_user")
        .fetch_all(&pool)
        .await
        .unwrap();
    let html = html! {
        <div>
            {
                users
                  .iter()
                  .map(|user| html! {
                    <div>Test: {user.username.clone()}</div>
                  })
                  .collect::<Vec<_>>()
            }
        </div>
    };
    (StatusCode::OK, html.to_string())
}

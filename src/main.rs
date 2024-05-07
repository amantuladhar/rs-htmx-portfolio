use auth::auth_middleware::auth;
use axum::{
    extract::State, http::StatusCode, middleware, response::IntoResponse, routing::get, Router,
};
use config::db_config::setup_db;
use shtml::{html, Component, Render};
use sqlx::PgPool;
use templates::pages::{
    about_page::about_page,
    login_page::{login_page, login_post_handler},
    root_page::root_page,
    signup_page::{signup_page, signup_post_handler},
};
use tower_http::trace::TraceLayer;
use utils::{setup_log, static_file_handler::static_handler};

mod auth;
mod config;
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
        .route("/login", get(login_page).post(login_post_handler))
        .route("/signup", get(signup_page).post(signup_post_handler))
        .route("/test", get(test_get_handler))
        .route("/public/*file", get(static_handler))
        .route_layer(middleware::from_fn_with_state(pool.clone(), auth))
        .layer(TraceLayer::new_for_http())
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
                    <ul>
                        <li>Id: {user.id.clone()}</li>
                        <li>Username: {user.username.clone()}</li>
                        <li>Password: {user.password.clone()}</li>
                        <li>CreatedAt: {user.created_at.clone().to_rfc3339()}</li>
                    </ul>
                  })
                  .collect::<Vec<_>>()
            }
        </div>
    };
    (StatusCode::OK, html.to_string())
}

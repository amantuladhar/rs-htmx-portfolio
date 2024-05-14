use auth::decode_jwt_token_middleware::decode_jwt_token;
use axum::{
    middleware::from_fn_with_state,
    routing::{get, post},
    Router,
};
use config::db_config::setup_db;
use templates::pages::{
    login_page::{login_page, login_post_handler},
    signup_page::{signup_page, signup_post_handler},
    update_portfolio::{
        experience::{
            edit_experience_dialog::{add_experience_dialog, edit_experience_dialog},
            edit_experience_post_handler::edit_experience_post_handler,
        },
        update_portfolio_page::update_portfolio_page,
    },
};
use tower_http::trace::TraceLayer;
use utils::{setup_log, static_file_handler::static_handler};

mod auth;
mod config;
mod errors;
mod repository;
mod templates;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    utils::env::EnvVars::init();
    setup_log();
    let pool = setup_db().await;

    let app = Router::new()
        // auth
        .route("/login", get(login_page).post(login_post_handler))
        .route("/signup", get(signup_page).post(signup_post_handler))
        // root
        .route("/", get(update_portfolio_page))
        // experiences
        .route("/experiences", post(edit_experience_post_handler))
        .route("/experiences/:experience_id", get(edit_experience_dialog))
        .route("/experiences/new", get(add_experience_dialog))
        // global routes and middlewares
        .route("/public/*file", get(static_handler))
        .route_layer(from_fn_with_state(pool.clone(), decode_jwt_token))
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

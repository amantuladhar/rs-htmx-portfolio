use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::utils::env::EnvVars;

pub async fn setup_db() -> Pool<Postgres> {
    let db_url = EnvVars::db_url();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_url)
        .await
        .expect("can't connect to database");
    pool
}

// // we can also write a custom extractor that grabs a connection from the pool
// // which setup is appropriate depends on your application
// #[async_trait]
// impl<S> FromRequestParts<S> for DbPool
// where
//     PgPool: FromRef<S>,
//     S: Send + Sync,
// {
//     type Rejection = (StatusCode, String);

//     async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
//         let pool = PgPool::from_ref(state);
//         Ok(Self(pool))
//     }
// }
// we can extract the connection pool with `State`
// async fn using_connection_pool_extractor(
//     State(pool): State<PgPool>,
// ) -> Result<String, (StatusCode, String)> {
//     sqlx::query_scalar("select 'hello world from pg'")
//         .fetch_one(&pool)
//         .await
//         .map_err(internal_error)
// }

// async fn using_connection_extractor(
//     DbPool(mut conn): DbPool,
// ) -> Result<String, (StatusCode, String)> {
//     sqlx::query_scalar("select 'hello world from pg'")
//         .fetch_one(&mut *conn)
//         .await
//         .map_err(internal_error)
// }

// Reference https://github.com/tokio-rs/axum/blob/main/examples/sqlx-postgres/src/main.rs

use axum::{
    routing::{get, post}
    Router,
};
use towe_http::cors::{AllowOrigin, CorsLayer};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

mod spatial;
use spatial::get_spatial;

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .min_connections(1)
        .acquire_timeout(std::time::Duration(5))
        .connect(&)
        .await.expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        run(&pool)
        .await
        .expect("Failed run database migrations");

    let app = Router::new()
        .route("/", get(get_spatial_handler));

    let listener = tokio::net::TcpListener::bind().await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

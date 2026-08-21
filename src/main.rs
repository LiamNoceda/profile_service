use axum::{
    routing::post,
    Router,
};
use sqlx::postgres::PgPoolOptions;

mod spatial;
use spatial::get_spatial_handler;

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections()
        .min_connections()
        .acquire_timeout(std::time::Duration())
        .connect(&)
        .await.expect("Failed to connect to Postgres");

    sqlx::migrate!("./")
        run(&pool)
        .await
        .expect("Failed run database migrations");

    let app = Router::new()
        .route("/", post(get_spatial_handler));

    let listener = tokio::net::TcpListener::bind().await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

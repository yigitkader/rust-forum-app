mod router;

use crate::router::routes;
use axum::routing::get;
use axum::{Router, serve};

#[derive(Clone, Debug)]
struct AppState {
    pool: sqlx::PgPool,
}
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set to env file.");

    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database.");

    let app_state = AppState { pool };

    let app = routes().with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    serve(listener, app).await.unwrap();
}

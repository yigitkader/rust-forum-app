use crate::AppState;
use axum::Router;
use axum::response::IntoResponse;
use axum::routing::get;

pub fn posts_router() -> Router<AppState> {
    Router::new().route("/", get(home_page))
}

async fn home_page() -> impl IntoResponse {
    "posts home page"
}

use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use crate::AppState;

pub fn users_router() -> Router<AppState>{
    Router::new().route("/login", get(login_page))
}

async fn login_page() -> impl IntoResponse {
    "login page"
}
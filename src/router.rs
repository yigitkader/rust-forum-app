use crate::AppState;
use crate::handlers::posts_handler::posts_router;
use crate::handlers::users_handler::users_router;
use axum::Router;

pub fn routes() -> Router<AppState> {
    Router::new().merge(posts_router()).merge(users_router())
}

use axum::{Router, routing::get};

pub mod health;
pub mod root;
pub mod user_dummy;

pub fn router() -> Router<crate::state::AppState> {
    Router::new()
        .route("/", get(root::handler))
        .route("/health", get(health::handler))
        .nest("/users-dummy", user_dummy::router())
}

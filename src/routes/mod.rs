use axum::{Router, routing::get};

pub mod health;
pub mod root;

pub fn router() -> Router<crate::state::AppState> {
    Router::new()
        .route("/", get(root::handler))
        .route("/health", get(health::handler))
}

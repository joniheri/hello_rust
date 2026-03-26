use axum::Router;
use tracing_subscriber::EnvFilter;

use crate::{config::AppConfig, routes, state::AppState};

pub fn build_router(config: AppConfig) -> Router {
    let state = AppState { config };
    routes::router().with_state(state)
}

pub fn init_tracing(log_filter: &str) {
    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(log_filter))
        .expect("invalid log filter configuration");

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .compact()
        .init();
}

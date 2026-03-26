mod app;
mod config;
mod error;
mod models;
mod response;
mod routes;
mod state;

use tracing::info;

#[tokio::main]
async fn main() {
    let config = config::AppConfig::from_env();
    app::init_tracing(&config.log_filter);
    let app = app::build_router(config.clone());

    let listener = tokio::net::TcpListener::bind(config.socket_addr())
        .await
        .expect("failed to bind TCP listener");

    info!(
        service = %config.service_name,
        address = %config.socket_addr(),
        "server listening"
    );

    axum::serve(listener, app).await.expect("axum server error");
}

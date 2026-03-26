use axum::{Json, extract::State};
use serde::Serialize;

use crate::{error::AppResult, state::AppState};

#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
    service: String,
}

pub async fn handler(State(state): State<AppState>) -> AppResult<Json<HealthResponse>> {
    if state.config.service_name.trim().is_empty() {
        return Err(crate::error::AppError::Internal(
            "service name configuration is empty",
        ));
    }

    Ok(Json(HealthResponse {
        status: "ok",
        service: state.config.service_name,
    }))
}

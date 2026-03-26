use axum::{Json, extract::State};
use serde::Serialize;

use crate::{error::AppResult, response::ApiSuccessResponse, state::AppState};

#[derive(Serialize)]
pub struct HealthResponse {
    health: &'static str,
    service: String,
}

pub async fn handler(
    State(state): State<AppState>,
) -> AppResult<Json<ApiSuccessResponse<HealthResponse>>> {
    if state.config.service_name.trim().is_empty() {
        return Err(crate::error::AppError::Internal(
            "service name configuration is empty".to_string(),
        ));
    }

    Ok(crate::response::success(HealthResponse {
        health: "ok",
        service: state.config.service_name,
    }))
}

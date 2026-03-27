use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::{
    error::AppResult,
    models::user_dummy::{CreateUserRequest, UpdateUserRequest, User},
    response::{ApiSuccessResponse, success},
    state::AppState,
};

#[derive(Serialize)]
struct DeleteUserResponse {
    message: &'static str,
}

pub async fn list_users(
    State(state): State<AppState>,
) -> AppResult<Json<ApiSuccessResponse<Vec<User>>>> {
    let users = state.user_dummy_service.list_users()?;
    Ok(success(users))
}

pub async fn get_user(
    Path(id): Path<u64>,
    State(state): State<AppState>,
) -> AppResult<Json<ApiSuccessResponse<User>>> {
    let user = state.user_dummy_service.get_user(id)?;
    Ok(success(user))
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> AppResult<impl IntoResponse> {
    let user = state.user_dummy_service.create_user(payload)?;
    Ok((StatusCode::CREATED, success(user)))
}

pub async fn update_user(
    Path(id): Path<u64>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateUserRequest>,
) -> AppResult<Json<ApiSuccessResponse<User>>> {
    let user = state.user_dummy_service.update_user(id, payload)?;
    Ok(success(user))
}

pub async fn delete_user(
    Path(id): Path<u64>,
    State(state): State<AppState>,
) -> AppResult<Response> {
    state.user_dummy_service.delete_user(id)?;
    Ok(success(DeleteUserResponse {
        message: "user deleted",
    })
    .into_response())
}


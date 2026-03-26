use axum::{
    Json, Router,
    extract::{Path, State},
    response::{IntoResponse, Response},
    routing::get,
};
use bcrypt::{DEFAULT_COST, hash};
use serde::Serialize;

use crate::{
    error::{AppError, AppResult},
    models::user::{CreateUserRequest, UpdateUserRequest, User},
    response::{ApiSuccessResponse, success},
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/{id}", get(get_user).put(update_user).delete(delete_user))
}

#[derive(Serialize)]
struct DeleteUserResponse {
    message: &'static str,
}

async fn list_users(
    State(state): State<AppState>,
) -> AppResult<Json<ApiSuccessResponse<Vec<User>>>> {
    let users = state
        .users
        .read()
        .map_err(|_| AppError::Internal("failed to acquire read lock".to_string()))?;

    let mut list: Vec<User> = users.values().cloned().collect();
    list.sort_by_key(|u| u.id);
    Ok(success(list))
}

async fn get_user(
    Path(id): Path<u64>,
    State(state): State<AppState>,
) -> AppResult<Json<ApiSuccessResponse<User>>> {
    let users = state
        .users
        .read()
        .map_err(|_| AppError::Internal("failed to acquire read lock".to_string()))?;

    let user = users
        .get(&id)
        .cloned()
        .ok_or_else(|| AppError::NotFound(format!("user with id {} not found", id)))?;

    Ok(success(user))
}

async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> AppResult<impl IntoResponse> {
    validate_email(&payload.email)?;
    validate_username(&payload.username)?;

    let mut users = state
        .users
        .write()
        .map_err(|_| AppError::Internal("failed to acquire write lock".to_string()))?;

    if users.values().any(|u| u.email == payload.email) {
        return Err(AppError::BadRequest(format!(
            "email: {} is already in use",
            payload.email
        )));
    }
    if users.values().any(|u| u.username == payload.username) {
        return Err(AppError::BadRequest(format!(
            "username: {} is already in use",
            payload.username
        )));
    }

    let password_hash = match payload.password.as_deref() {
        Some(password) => Some(hash_password(password)?),
        None => None,
    };

    let user = User {
        id: state.allocate_user_id(),
        email: payload.email.trim().to_string(),
        username: payload.username.trim().to_string(),
        password_hash,
        fullname: payload.fullname.unwrap_or_default().trim().to_string(),
    };
    users.insert(user.id, user.clone());

    Ok((axum::http::StatusCode::CREATED, success(user)))
}

async fn update_user(
    Path(id): Path<u64>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateUserRequest>,
) -> AppResult<Json<ApiSuccessResponse<User>>> {
    if payload.username.is_none() && payload.fullname.is_none() && payload.email.is_none() {
        return Err(AppError::BadRequest(
            "at least one field (username, fullname, or email) must be provided".to_string(),
        ));
    }

    if let Some(email) = &payload.email {
        validate_email(email)?;
    }
    if let Some(username) = &payload.username {
        validate_username(username)?;
    }

    let mut users = state
        .users
        .write()
        .map_err(|_| AppError::Internal("failed to acquire write lock".to_string()))?;

    let user = users
        .get_mut(&id)
        .ok_or_else(|| AppError::NotFound(format!("user with id {} not found", id)))?;

    if let Some(email) = payload.email {
        user.email = email;
    }
    if let Some(username) = payload.username {
        user.username = username;
    }
    if let Some(fullname) = payload.fullname {
        user.fullname = fullname;
    }

    Ok(success(user.clone()))
}

async fn delete_user(Path(id): Path<u64>, State(state): State<AppState>) -> AppResult<Response> {
    let mut users = state
        .users
        .write()
        .map_err(|_| AppError::Internal("failed to acquire write lock".to_string()))?;

    if users.remove(&id).is_none() {
        return Err(AppError::NotFound(format!("user with id {} not found", id)));
    }

    Ok(success(DeleteUserResponse {
        message: "user deleted",
    })
    .into_response())
}

fn validate_email(email: &str) -> AppResult<()> {
    let email = email.trim();
    if email.is_empty() {
        return Err(AppError::BadRequest("email is required".to_string()));
    }
    if email.len() < 15 {
        return Err(AppError::BadRequest(
            "email must be at least 15 characters".to_string(),
        ));
    }
    if !email.contains('@') {
        return Err(AppError::BadRequest(
            "email must be a valid email address".to_string(),
        ));
    }
    Ok(())
}

fn validate_username(username: &str) -> AppResult<()> {
    let username = username.trim();
    if username.is_empty() {
        return Err(AppError::BadRequest("username is required".to_string()));
    }
    if username.len() < 5 {
        return Err(AppError::BadRequest(
            "username must be at least 5 characters".to_string(),
        ));
    }
    Ok(())
}

fn hash_password(password: &str) -> AppResult<String> {
    let password = password.trim();
    if password.is_empty() {
        return Err(AppError::BadRequest(
            "password must not be empty when provided".to_string(),
        ));
    }
    hash(password, DEFAULT_COST).map_err(|_| AppError::Internal("failed to hash password".to_string()))
}

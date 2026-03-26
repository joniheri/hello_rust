use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};

use crate::{
    error::{AppError, AppResult},
    models::user::{CreateUserRequest, UpdateUserRequest, User},
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/{id}", get(get_user).put(update_user).delete(delete_user))
}

async fn list_users(State(state): State<AppState>) -> AppResult<Json<Vec<User>>> {
    let users = state
        .users
        .read()
        .map_err(|_| AppError::Internal("failed to acquire read lock".to_string()))?;

    let mut list: Vec<User> = users.values().cloned().collect();
    list.sort_by_key(|u| u.id);
    Ok(Json(list))
}

async fn get_user(Path(id): Path<u64>, State(state): State<AppState>) -> AppResult<Json<User>> {
    let users = state
        .users
        .read()
        .map_err(|_| AppError::Internal("failed to acquire read lock".to_string()))?;

    let user = users
        .get(&id)
        .cloned()
        .ok_or_else(|| AppError::NotFound(format!("user with id {} not found", id)))?;

    Ok(Json(user))
}

async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> AppResult<impl IntoResponse> {
    validate_email(&payload.email)?;
    validate_username(&payload.username)?;
    validate_fullname(&payload.fullname)?;

    let user = User {
        id: state.allocate_user_id(),
        email: payload.email,
        username: payload.username,
        fullname: payload.fullname,
    };

    let mut users = state
        .users
        .write()
        .map_err(|_| AppError::Internal("failed to acquire write lock".to_string()))?;
    users.insert(user.id, user.clone());

    Ok((StatusCode::CREATED, Json(user)))
}

async fn update_user(
    Path(id): Path<u64>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateUserRequest>,
) -> AppResult<Json<User>> {
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
    if let Some(fullname) = &payload.fullname {
        validate_fullname(fullname)?;
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

    Ok(Json(user.clone()))
}

async fn delete_user(Path(id): Path<u64>, State(state): State<AppState>) -> AppResult<StatusCode> {
    let mut users = state
        .users
        .write()
        .map_err(|_| AppError::Internal("failed to acquire write lock".to_string()))?;

    if users.remove(&id).is_none() {
        return Err(AppError::NotFound(format!("user with id {} not found", id)));
    }

    Ok(StatusCode::NO_CONTENT)
}

fn validate_email(email: &str) -> AppResult<()> {
    if email.trim().is_empty() || !email.contains('@') {
        return Err(AppError::BadRequest(
            "email must be a valid email address".to_string(),
        ));
    }
    Ok(())
}

fn validate_username(username: &str) -> AppResult<()> {
    if username.trim().is_empty() {
        return Err(AppError::BadRequest(
            "username must not be empty".to_string(),
        ));
    }
    Ok(())
}

fn validate_fullname(name: &str) -> AppResult<()> {
    if name.trim().is_empty() {
        return Err(AppError::BadRequest(
            "fullname must not be empty".to_string(),
        ));
    }
    Ok(())
}

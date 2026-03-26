use std::sync::Arc;

use bcrypt::{DEFAULT_COST, hash};

use crate::{
    error::{AppError, AppResult},
    models::user::{CreateUserRequest, UpdateUserRequest, User},
    repositories::user_repository::UserRepository,
};

pub struct UserService {
    user_repository: Arc<UserRepository>,
}

impl UserService {
    pub fn new(user_repository: Arc<UserRepository>) -> Self {
        Self { user_repository }
    }

    pub fn list_users(&self) -> AppResult<Vec<User>> {
        self.user_repository.list()
    }

    pub fn get_user(&self, id: u64) -> AppResult<User> {
        self.user_repository
            .find_by_id(id)?
            .ok_or_else(|| AppError::NotFound(format!("user with id {} not found", id)))
    }

    pub fn create_user(&self, payload: CreateUserRequest) -> AppResult<User> {
        validate_email(&payload.email)?;
        validate_username(&payload.username)?;

        let email = payload.email.trim().to_string();
        let username = payload.username.trim().to_string();

        if self.user_repository.exists_by_email(&email)? {
            return Err(AppError::BadRequest(format!(
                "email: {} is already in use",
                email
            )));
        }
        if self.user_repository.exists_by_username(&username)? {
            return Err(AppError::BadRequest(format!(
                "username: {} is already in use",
                username
            )));
        }

        let password_hash = payload.password.as_deref().map(hash_password).transpose()?;

        self.user_repository.create(User {
            id: 0,
            email,
            username,
            password_hash,
            fullname: payload.fullname.unwrap_or_default().trim().to_string(),
        })
    }

    pub fn update_user(&self, id: u64, payload: UpdateUserRequest) -> AppResult<User> {
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

        let updated = self.user_repository.update(
            id,
            payload.email.map(|v| v.trim().to_string()),
            payload.username.map(|v| v.trim().to_string()),
            payload.fullname.map(|v| v.trim().to_string()),
        )?;

        updated.ok_or_else(|| AppError::NotFound(format!("user with id {} not found", id)))
    }

    pub fn delete_user(&self, id: u64) -> AppResult<()> {
        let deleted = self.user_repository.delete(id)?;
        if !deleted {
            return Err(AppError::NotFound(format!("user with id {} not found", id)));
        }
        Ok(())
    }
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

    hash(password, DEFAULT_COST)
        .map_err(|_| AppError::Internal("failed to hash password".to_string()))
}

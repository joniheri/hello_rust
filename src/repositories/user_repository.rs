use std::{
    collections::HashMap,
    sync::{
        RwLock,
        atomic::{AtomicU64, Ordering},
    },
};

use crate::{error::AppError, models::user::User};

pub struct UserRepository {
    users: RwLock<HashMap<u64, User>>,
    next_user_id: AtomicU64,
}

impl UserRepository {
    pub fn new() -> Self {
        Self {
            users: RwLock::new(HashMap::new()),
            next_user_id: AtomicU64::new(1),
        }
    }

    pub fn list(&self) -> Result<Vec<User>, AppError> {
        let users = self
            .users
            .read()
            .map_err(|_| AppError::Internal("failed to acquire read lock".to_string()))?;

        let mut list: Vec<User> = users.values().cloned().collect();
        list.sort_by_key(|u| u.id);
        Ok(list)
    }

    pub fn find_by_id(&self, id: u64) -> Result<Option<User>, AppError> {
        let users = self
            .users
            .read()
            .map_err(|_| AppError::Internal("failed to acquire read lock".to_string()))?;
        Ok(users.get(&id).cloned())
    }

    pub fn exists_by_email(&self, email: &str) -> Result<bool, AppError> {
        let users = self
            .users
            .read()
            .map_err(|_| AppError::Internal("failed to acquire read lock".to_string()))?;
        Ok(users.values().any(|u| u.email == email))
    }

    pub fn exists_by_username(&self, username: &str) -> Result<bool, AppError> {
        let users = self
            .users
            .read()
            .map_err(|_| AppError::Internal("failed to acquire read lock".to_string()))?;
        Ok(users.values().any(|u| u.username == username))
    }

    pub fn create(&self, mut user: User) -> Result<User, AppError> {
        let mut users = self
            .users
            .write()
            .map_err(|_| AppError::Internal("failed to acquire write lock".to_string()))?;

        user.id = self.next_user_id.fetch_add(1, Ordering::Relaxed);
        users.insert(user.id, user.clone());
        Ok(user)
    }

    pub fn update(
        &self,
        id: u64,
        email: Option<String>,
        username: Option<String>,
        fullname: Option<String>,
    ) -> Result<Option<User>, AppError> {
        let mut users = self
            .users
            .write()
            .map_err(|_| AppError::Internal("failed to acquire write lock".to_string()))?;

        let Some(user) = users.get_mut(&id) else {
            return Ok(None);
        };

        if let Some(email) = email {
            user.email = email;
        }
        if let Some(username) = username {
            user.username = username;
        }
        if let Some(fullname) = fullname {
            user.fullname = fullname;
        }

        Ok(Some(user.clone()))
    }

    pub fn delete(&self, id: u64) -> Result<bool, AppError> {
        let mut users = self
            .users
            .write()
            .map_err(|_| AppError::Internal("failed to acquire write lock".to_string()))?;
        Ok(users.remove(&id).is_some())
    }
}

use std::{
    collections::HashMap,
    sync::{
        Arc, RwLock,
        atomic::{AtomicU64, Ordering},
    },
};

use crate::config::AppConfig;
use crate::models::user::User;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub users: Arc<RwLock<HashMap<u64, User>>>,
    pub next_user_id: Arc<AtomicU64>,
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config,
            users: Arc::new(RwLock::new(HashMap::new())),
            next_user_id: Arc::new(AtomicU64::new(1)),
        }
    }

    pub fn allocate_user_id(&self) -> u64 {
        self.next_user_id.fetch_add(1, Ordering::Relaxed)
    }
}

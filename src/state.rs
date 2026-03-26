use std::sync::Arc;

use crate::config::AppConfig;
use crate::repositories::user_repository::UserRepository;
use crate::services::user_service::UserService;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub user_service: Arc<UserService>,
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        let user_repository = Arc::new(UserRepository::new());
        let user_service = Arc::new(UserService::new(user_repository));

        Self {
            config,
            user_service,
        }
    }
}

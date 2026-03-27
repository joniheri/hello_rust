use std::sync::Arc;

use crate::config::AppConfig;
use crate::repositories::user_dummy_repository::UserDummyRepository;
use crate::services::user_dummy_service::UserDummyService;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub user_dummy_service: Arc<UserDummyService>,
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        let user_dummy_repository = Arc::new(UserDummyRepository::new());
        let user_dummy_service = Arc::new(UserDummyService::new(user_dummy_repository));

        Self {
            config,
            user_dummy_service,
        }
    }
}


use axum::{Router, routing::get};

use crate::{controllers::user_dummy_controller, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(user_dummy_controller::list_users).post(user_dummy_controller::create_user),
        )
        .route(
            "/{id}",
            get(user_dummy_controller::get_user)
                .put(user_dummy_controller::update_user)
                .delete(user_dummy_controller::delete_user),
        )
}



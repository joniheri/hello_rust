use axum::{Router, routing::get};

use crate::{controllers::user_controller, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(user_controller::list_users).post(user_controller::create_user),
        )
        .route(
            "/{id}",
            get(user_controller::get_user)
                .put(user_controller::update_user)
                .delete(user_controller::delete_user),
        )
}

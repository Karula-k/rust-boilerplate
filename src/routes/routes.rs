use std::sync::Arc;

use axum::{Router, routing::{get, post}};

use crate::{AppState, handlers::users_handler};



pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/users", get(users_handler::list_users))
}
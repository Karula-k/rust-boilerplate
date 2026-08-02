use std::sync::Arc;

use axum::{Router, routing::{get, post}};

use crate::{AppState, handlers::{auth_handler, users_handler}};



pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
    .route("/register", post(auth_handler::register))
   .route("/login", post(auth_handler::login))
   .route("/users", get(users_handler::list_users))
}
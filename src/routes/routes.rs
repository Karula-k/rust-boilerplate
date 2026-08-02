use std::sync::Arc;

use axum::{Router, routing::{get, post}};

use crate::{AppState, handlers::{auth_handler, users_handler}, middleware::middleware::auth_middleware};



pub fn routes(state: &Arc<AppState>) -> Router<Arc<AppState>> {
    let public_routes = Router::new()
        .route("/register", post(auth_handler::register))
        .route("/login", post(auth_handler::login));

    let protected_routes = Router::new()
        .route("/users", get(users_handler::list_users))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    public_routes
        .merge(protected_routes)
}
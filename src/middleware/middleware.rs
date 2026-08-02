use std::sync::Arc;

use axum::{extract::{Request, State}, http::StatusCode, middleware::Next, response::Response};

use crate::{AppState, middleware::jwt_strategy::verify_token};

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = req
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;
 
    // Validate token against your state
     verify_token(
        token.to_string(),
        &state.env,
    )
    .await
    .map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    Ok(next.run(req).await)
}
 
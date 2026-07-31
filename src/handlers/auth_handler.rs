use std::sync::Arc;

use axum::{Json, extract::State};
use cornucopia::queries::{users};


use crate::{AppState, models::login_models::LoginForm, utils::{error::AppError, response::ApiResponse}};

pub async fn login(State( states,): State<Arc<AppState>>, Json(payload): Json<LoginForm>,)->Result<ApiResponse<users::GetUserByUsername>,AppError>{
  
    let user:users::GetUserByUsername = users::get_user_by_username()
        .bind(&states.db_pool, &payload.username).one().await?;

 Ok(ApiResponse::JsonData(user))
}

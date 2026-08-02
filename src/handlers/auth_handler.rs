use std::sync::Arc;

use argon2::Config;
use axum::{Json, extract::State};
use cornucopia::queries::{users};
use rand::RngExt;


use crate::{AppState, models::login_models::LoginForm, utils::{error::AppError, response::ApiResponse}};

pub async fn login(State( states,): State<Arc<AppState>>, Json(payload): Json<LoginForm>,)->Result<ApiResponse<users::GetUserByUsername>,AppError>{
  
    let user:users::GetUserByUsername = users::get_user_by_username()
        .bind(&states.db_pool, &payload.username).one().await?;

    let matches = argon2::verify_encoded(&user.password, &payload.password.as_bytes()).unwrap();
    if !matches {
        return  Err(AppError::Unauthorized);
    }   
 Ok(ApiResponse::JsonData(user))
}

pub async fn register(State(states): State<Arc<AppState>>, Json(payload): Json<LoginForm>)->Result<ApiResponse<()>,AppError>{
    let mut salt = [0u8; 16];
    rand::rng().fill(&mut salt);
    let config = Config::default();
    let hash = argon2::hash_encoded(&payload.password.into_bytes(), &salt, &config).unwrap();

    users::create_users().bind(&states.db_pool, &payload.username, &hash).one().await?;

    Ok(ApiResponse::JsonData(()) )
}
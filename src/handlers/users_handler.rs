use std::sync::Arc;

use axum::{extract::State};
use cornucopia::queries::{users};


use crate::{AppState, utils::error::AppError, utils::response::{ApiResponse}};

pub async fn list_users(State( states): State<Arc<AppState>>,)->Result<ApiResponse<Vec<users::ListUsers>>,AppError>{
  
    let rows: Vec<users::ListUsers> = users::list_users()
        .bind(&states.db_pool, &10, &0)
        .all().await?;

 Ok(ApiResponse::JsonData(rows))
}

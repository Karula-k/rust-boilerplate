use std::sync::Arc;

use axum::{extract::State};
use cornucopia::queries::{users};


use crate::{AppState, error::AppError, response::{ApiResponse, Message}};

pub async fn list_users(State( states): State<Arc<AppState>>,)->Result<ApiResponse,AppError>{
  
    let rows = users::list_users()
        .bind(&states.db_pool, &10, &5)
        .all().await?;

  Ok(ApiResponse::JsonData(
        rows.into_iter()
            .map(|row| Message {
                message: row.username,
            })
            .collect(),
    ))
}

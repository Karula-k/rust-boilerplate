use axum::{response::{IntoResponse, Response}, http::StatusCode, Json};
 
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    Unauthorized,
     Internal(String),
}
 
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".into()),
            AppError::Internal(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg.into())
            }
        };
 
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}

impl From<tokio_postgres::Error> for AppError {
    fn from(err: tokio_postgres::Error) -> Self {
        match err {
           _ => AppError::Internal(err.to_string()),
        }
    }
}
use axum::{ Json, http::StatusCode, response::{IntoResponse, Response}};
use serde::Serialize;



pub enum ApiResponse<T>
where
    T: Serialize,
{
    OK,
    Created,
    JsonData(T),
}


impl <T> IntoResponse for ApiResponse<T>  where T: Serialize,{
    fn into_response(self) -> Response {
        match self{
            Self::OK=>(StatusCode::OK).into_response(),
            Self::Created=>(StatusCode::CREATED).into_response(),
            Self::JsonData(data)=>(StatusCode::OK, Json(data)).into_response()
        }
    }
}
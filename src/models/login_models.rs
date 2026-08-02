use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginForm {
   pub username: String,
   pub password: String,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct LoginResponse {
    pub id: uuid::Uuid,
    pub username: String,
    pub access_token: String,
    pub refresh_token: String,
}

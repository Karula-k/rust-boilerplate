use chrono::Utc;
use cornucopia::queries::users;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::utils::{config::Config, error::AppError};


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  //user_id     
    pub username: String,
    pub exp: usize,    
    pub iat: usize, 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessAndRefreshToken {
    pub access_token: String,    
    pub refresh_token: String, 
}
pub fn generate_token(user:&users::GetUserByUsername,env: &Config,expires_in: usize,)->Result<String,AppError>{
    let now = Utc::now().timestamp() as usize;
    let user_clams = Claims  {
        sub: user.id.to_string(),
        username: user.username.to_string(),
        exp: (now + expires_in) as usize, // 1 hour
        iat: now,
    };

    let token = encode(&Header::default(), &user_clams, &EncodingKey::from_secret(&env.secret.as_bytes()))?;

    Ok(token)
}


pub async fn verify_token(token:String,env: &Config)-> Result<Claims,AppError>{
    let user_clams = decode(&token, &DecodingKey::from_secret(env.secret.as_bytes()), &Validation::default())?.claims;

    Ok(user_clams)
}

pub async fn generated_access_and_refresh_token(user:&users::GetUserByUsername, env: &Config)->Result<AccessAndRefreshToken,AppError>{
    let access_token = generate_token(&user, &env, 60*60)?;  // 1 hour
    let refresh_token = generate_token(&user, &env, 60*60*24*7)?; // 1 week

    
    Ok(AccessAndRefreshToken {
        access_token,
        refresh_token,
    })
}
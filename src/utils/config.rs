use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config{
    pub database_url: String,
    pub host:String,
    pub port:String,
    pub secret: String,
}

impl Config {
    pub fn from_env()->Self{
        dotenv().ok();
        Self { 
        database_url: env::var("DATABASE_URL").expect("Check Database env"),
        host:env::var("HOST").unwrap_or_else(|_|"0.0.0.0".to_string()),
        port:env::var("PORT").unwrap_or_else(|_|"3000".to_string()).parse().expect("Port must be valid u16"), 
        secret:env::var("JWT_SECRET").unwrap_or_else(|_|"SECRET".to_string()),

    }
    }
    pub  fn address(&self)->String{
        format!("{}:{}",self.host, self.port )
    }
}
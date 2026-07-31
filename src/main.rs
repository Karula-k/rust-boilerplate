// will remove when implementation all Api response and error response
#![allow(dead_code)]
mod routes;
mod utils;
mod handlers;
mod models;
use std::sync::Arc;

use tokio_postgres::{ Client, NoTls};
use crate::{routes::routes::routes, utils::config::Config};
struct AppState {
    db_pool: Client,
}
 

#[tokio::main]
async fn main() {
      let config: Config = Config::from_env();
     let (client, connection) = tokio_postgres::connect(&config.database_url, NoTls).await.unwrap();
        tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

     let app_state = Arc::new(AppState{
        db_pool: client,
     });
      let app = routes().with_state(app_state);
      let listener = tokio::net::TcpListener::bind(config.address()).await.unwrap();
      axum::serve(listener, app).await.unwrap();
}

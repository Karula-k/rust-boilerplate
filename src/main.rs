// will remove when implementation cornucopia
#![allow(dead_code)]

mod config;

use std::sync::Arc;

use axum::{Router, routing::get};
use tokio_postgres::{Client, NoTls};

use crate::config::Config;

struct AppState {
    db: Client,
}

#[tokio::main]
async fn main() {
      let config = Config::from_env();
     let (client, connection) = tokio_postgres::connect(&config.database_url, NoTls).await.unwrap();
        tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

     let app_state = Arc::new(AppState { db: client });
      let app = Router::new().route("/", get(|| async { "Hello, World!" })).with_state(app_state);
      let listener = tokio::net::TcpListener::bind(config.address()).await.unwrap();
      axum::serve(listener, app).await.unwrap();
}

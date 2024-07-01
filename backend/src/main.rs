use std::env;
use std::error::Error;
use std::num::ParseIntError;
use std::process::exit;
use std::str::FromStr;

use axum::routing::{get, post};
use axum::Router;
use dotenv::dotenv;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::ConnectOptions;
use tracing::{error, warn};

use crate::api::get::get_post::get_post;
use crate::api::get::get_post_list::get_post_list;
use crate::api::post::refresh::refresh;

mod api;
mod constants;
mod logger;
mod markdown;
mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    match dotenv() {
        Ok(_) => {
            println!("[Info] env loaded, starting up...")
        }
        Err(e) => {
            panic!("[Fatal] failed to load env var: {}", e)
        }
    };

    let _guards = logger::init();

    let port: u32 = match env::var("BACKEND_PORT") {
        Ok(value) => value.parse().unwrap_or_else(|e: ParseIntError| {
            warn!("Failed to parse BACKEND_PORT: {}", e.to_string());
            3001
        }),
        Err(_) => 3001,
    };

    let db_uri: String = env::var("DB_CONNECTION").unwrap_or_else(|e| {
        error!(
            "Invalid or missing env var: DB_CONNECTION: {}",
            e.to_string()
        );
        exit(1);
    });
    let db_connect_option = PgConnectOptions::from_str(&db_uri)
        .unwrap()
        .disable_statement_logging();

    let db_pool = PgPoolOptions::new()
        .connect_with(db_connect_option)
        .await
        .unwrap_or_else(|e| {
            error!("Failed to load database: {}", e.to_string());
            exit(1);
        });

    let app = Router::new()
        .route("/refresh_posts", post(refresh))
        .route("/get_post_list", get(get_post_list))
        .route("/get_post/:post_id", get(get_post))
        .with_state(db_pool);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap_or_else(|e| {
            error!("Failed to build TCP listener: {}", e.to_string());
            exit(1);
        });

    axum::serve(listener, app).await.unwrap_or_else(|e| {
        error!("Failed to start axum: {}", e.to_string());
        exit(1);
    });

    Ok(())
}

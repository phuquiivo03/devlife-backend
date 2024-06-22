mod app;
mod auth;
mod constants;
mod controllers;
mod db;
mod error;
mod helpers;
mod models;
mod routes;
mod services;
mod utils;
mod socket;

use std::env;

use dotenv::dotenv;
use error::{Error, Result};

use app::get_app;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let app = get_app().await?;

    let port = env::var("PORT").map_err(|e| Error::Other(e.to_string()))?;

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    println!("{}", format!("Server running on port {port}"));

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

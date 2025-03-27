mod adapters;
mod app_modules;
mod config;
mod domain;
mod server;

use std::net::TcpListener;

use config::database;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let db_connection_pool = database::get_db_connection_pool().await;
    // Create TCP listener
    let listener = TcpListener::bind("0.0.0.0:8080")?;

    // Create and run server
    let server = server::WebServer::new(listener, db_connection_pool.clone());
    server.run().await?;

    Ok(())
}

mod app_state;

use dotenvy::dotenv;

use app_state::AppState;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app_state = AppState::new().await;

    // Fetch a connection
    match app_state.db_pool.get().await {
        Ok(conn) => {
            println!("Database connection acquired!");
            let _ = conn.execute("SELECT 1", &[]).await;
        }
        Err(e) => eprintln!("Error getting connection: {}", e),
    }
}

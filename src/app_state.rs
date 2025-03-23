// Application wide state management
mod app_config;
pub mod db;
mod db_config;
mod defaults;

use app_config::AppConfig;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

#[derive(Clone, Debug)]
pub struct AppState {
    pub config: &'static AppConfig,
    pub db_pool: &'static Pool<PostgresConnectionManager<NoTls>>,
}

impl AppState {
    pub async fn new() -> Self {
        let config = AppConfig::get_config().await;
        let db_pool = db::get_db_pool(&config.db_config).await;

        AppState { db_pool, config }
    }
}

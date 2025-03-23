/*
This module contains code for a Global database Singleton instance
*/

use super::db_config::DBConfig;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio::sync::OnceCell;
use tokio_postgres::NoTls;

pub type PgPool = Pool<PostgresConnectionManager<NoTls>>;

// Global database instance (Singleton)
static DB_INSTANCE: tokio::sync::OnceCell<PgPool> = OnceCell::const_new();

pub async fn get_db_pool(config: &DBConfig) -> &'static PgPool {
    DB_INSTANCE
        .get_or_init(|| async {
            println!("Initializing database connection...");
            let manager =
                PostgresConnectionManager::new_from_stringlike(config.database_url.clone(), NoTls)
                    .unwrap();
            Pool::builder()
                .max_size(config.max_db_connections)
                .build(manager)
                .await
                .unwrap()
        })
        .await
}

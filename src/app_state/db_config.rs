/*
This module contains config settings for the database connection.
*/
use std::env;

use super::defaults;

#[derive(Clone, Debug)]
pub struct DBConfig {
    pub database_url: String,
    pub max_db_connections: u32,
}

impl DBConfig {
    pub fn load() -> Self {
        let db_host = env::var("DB_HOST").expect("DB_HOST must be set");
        let db_port: u16 = env::var("DB_PORT")
            .expect("DB_PORT must be set")
            .parse()
            .expect("DB_PORT must be a number");
        let db_username = env::var("DB_USERNAME").expect("DB_USERNAME must be set");
        let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
        let db_name = env::var("DB_NAME").expect("DB_NAME must be set");
        let max_db_connections: u32 = env::var("MAX_DB_CONNECTIONS")
            .unwrap_or_else(|_| defaults::MAX_DB_CONNECTIONS.to_string())
            .parse()
            .expect("MAX_DB_CONNECTIONS must be a number");

        DBConfig {
            database_url: format!(
                "postgres://{db_username}:{db_password}@{db_host}:{db_port}/{db_name}"
            ),
            max_db_connections,
        }
    }
}

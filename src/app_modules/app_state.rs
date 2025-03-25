use std::sync::Arc;

use crate::config::database::PgPool;
use crate::domain::services::UserService;

// Configuration struct to hold application state
pub struct AppState {
    db_pool: Arc<PgPool>,
    pub user_service: UserService,
    // Add other services or configuration as needed
}
impl AppState {
    pub fn new(pool: PgPool) -> AppState {
        let db_pool = Arc::new(pool);
        let user_service = UserService::new(db_pool.clone());

        AppState {
            db_pool,
            user_service,
        }
    }
}

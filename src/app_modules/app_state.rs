use std::sync::Arc;

use crate::config::database::PgPool;
use crate::domain::services::AuthService;
use crate::domain::services::UserService;

// Configuration struct to hold application state
pub struct AppState {
    db_pool: Arc<PgPool>,
    pub user_service: Arc<UserService>,
    pub auth_service: Arc<AuthService>,
    // Add other services or configuration as needed
}
impl AppState {
    pub fn new(pool: PgPool) -> AppState {
        let db_pool = Arc::new(pool);
        let user_service = Arc::new(UserService::new(db_pool.clone()));
        let auth_service = Arc::new(AuthService::new(user_service.clone()));

        AppState {
            db_pool,
            user_service,
            auth_service,
        }
    }
}

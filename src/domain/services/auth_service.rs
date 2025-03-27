use std::collections::HashMap;
use std::sync::Arc;

use crate::{
    app_modules::auth::{AuthMethod, AuthStrategy},
    domain::services::UserService,
};

use crate::app_modules::auth::configure_auth_strategies;

pub struct AuthService {
    pub strategies: HashMap<AuthMethod, Box<dyn AuthStrategy + Send + Sync>>,
}

impl AuthService {
    pub fn new(user_service: Arc<UserService>) -> Self {
        let strategies = configure_auth_strategies(user_service);

        Self { strategies }
    }
}

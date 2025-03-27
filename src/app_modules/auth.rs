mod auth_strategies;

pub use auth_strategies::AuthStrategy;

pub use crate::domain::errors::UserError;

use crate::domain::services::EmailService;
use crate::domain::services::UserService;
use auth_strategies::EmailPasswordAuthStrategy;
use std::collections::HashMap;
use std::sync::Arc;

// Authentication Methods Enum
#[derive(Hash, Eq, PartialEq)]
pub enum AuthMethod {
    EmailPassword,
    Google,
    Facebook,
    // Other providers can be added
}

pub struct PasswordHasher;
impl PasswordHasher {
    pub fn new() -> Self {
        Self
    }

    pub fn hash_password(&self, password: &str) -> Result<String, UserError> {
        // TODO: Implement password hashing logic here
        Ok("hashed_password".to_string())
    }
}

// Configuration for authentication strategies
pub fn configure_auth_strategies(
    user_service: Arc<UserService>,
    email_service: Arc<EmailService>,
    password_hasher: Arc<PasswordHasher>,
) -> HashMap<AuthMethod, Box<dyn AuthStrategy + Send + Sync>> {
    let mut strategies = HashMap::new();

    strategies.insert(
        AuthMethod::EmailPassword,
        Box::new(EmailPasswordAuthStrategy::new(
            user_service,
            email_service,
            password_hasher,
        )) as Box<dyn AuthStrategy + Send + Sync>,
    );

    // When ready to add Google Auth
    // strategies.insert(
    //     AuthMethod::Google,
    //     Box::new(GoogleAuthStrategy::new(...))
    // );

    strategies
}

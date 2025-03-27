use crate::adapters::dtos::RegisteredUserDto;
use crate::adapters::dtos::RegistrationDto;
use crate::domain::errors::UserError;
use crate::domain::models::AuthProvider;
use crate::domain::services::UserService;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{error, info};

// Authentication Methods Enum
#[derive(Hash, Eq, PartialEq)]
pub enum AuthMethod {
    EmailPassword,
    Google,
    Facebook,
    // Other providers can be added
}

// Authentication Strategy Trait
#[async_trait::async_trait]
pub trait AuthStrategy {
    async fn register(
        &self,
        registration_data: RegistrationDto,
    ) -> Result<RegisteredUserDto, UserError>;
}

// Email/Password Registration Strategy
pub struct EmailPasswordAuthStrategy {
    user_service: Arc<UserService>,
    password_hasher: PasswordHasher,
    email_validator: EmailValidator,
}

impl EmailPasswordAuthStrategy {
    pub fn new(
        user_service: Arc<UserService>,
        password_hasher: PasswordHasher,
        email_validator: EmailValidator,
    ) -> Self {
        Self {
            user_service,
            password_hasher,
            email_validator,
        }
    }
}

#[async_trait::async_trait]
impl AuthStrategy for EmailPasswordAuthStrategy {
    async fn register(
        &self,
        registration_data: RegistrationDto,
    ) -> Result<RegisteredUserDto, UserError> {
        // Validate email format
        self.email_validator
            .validate(&registration_data.email)
            .map_err(|e| {
                error!("Invalid email format: {}", e);
                UserError::InvalidEmail
            })?;

        // Check if user already exists
        if self
            .user_service
            .user_exists(&registration_data.email)
            .await?
        {
            return Err(UserError::UserAlreadyExists);
        }

        // Create user with defaults
        let mut new_user = self
            .user_service
            .create_user_with_defaults(&registration_data.email);

        // Hash password if provided
        if let Some(password) = &registration_data.password {
            new_user.password_hash =
                Some(self.password_hasher.hash_password(password).map_err(|e| {
                    error!("Password hashing failed: {}", e);
                    UserError::PasswordHashingError
                })?);
        }

        new_user.auth_provider = AuthProvider::Local;

        let saved_user = self.user_service.create_user(new_user).await?;

        // Generate verification token
        let verification_token = self
            .user_service
            .generate_email_verification_token(&saved_user.user_id)
            .await?;

        // Send verification email asynchronously
        let email_clone = saved_user.email.clone();
        tokio::spawn(async move {
            let email_service = EmailService::new();
            if let Err(e) = email_service
                .send_verification_email(email_clone, verification_token)
                .await
            {
                error!("Failed to send verification email: {}", e);
            }
        });

        Ok(RegisteredUserDto {
            id: saved_user.user_id,
            email: saved_user.email,
            auth_provider: saved_user.auth_provider.to_string(),
        })
    }
}

pub struct PasswordHasher;
pub struct EmailValidator;

impl PasswordHasher {
    fn new() -> Self {
        Self
    }

    fn hash_password(&self, password: &str) -> Result<String, UserError> {
        // TODO: Implement password hashing logic here
        Ok("hashed_password".to_string())
    }
}

impl EmailValidator {
    fn new() -> Self {
        Self
    }

    fn validate(&self, email: &str) -> Result<bool, UserError> {
        // TODO: Implement email validation logic here
        Ok(true)
    }
}

pub struct EmailService;
impl EmailService {
    pub fn new() -> Self {
        Self
    }

    pub async fn send_verification_email(
        &self,
        email: String,
        token: String,
    ) -> Result<(), UserError> {
        // TODO: Implement email sending logic here
        info!(
            "Sending verification email to {} with token {}",
            email, token
        );
        Ok(())
    }
}

// Configuration Example
pub fn configure_auth_strategies(
    user_service: Arc<UserService>,
) -> HashMap<AuthMethod, Box<dyn AuthStrategy + Send + Sync>> {
    let mut strategies = HashMap::new();

    strategies.insert(
        AuthMethod::EmailPassword,
        Box::new(EmailPasswordAuthStrategy::new(
            user_service,
            PasswordHasher::new(),
            EmailValidator::new(),
        )) as Box<dyn AuthStrategy + Send + Sync>,
    );

    // When ready to add Google Auth
    // strategies.insert(
    //     AuthMethod::Google,
    //     Box::new(GoogleAuthStrategy::new(...))
    // );

    strategies
}

/*
This module contains config settings for authentication.
It reads the following environment variables:
- JWT_SECRET
- JWT_EXPIRATION
- REFRESH_TOKEN_EXPIRATION
- ACCESS_TOKEN_EXPIRATION
- PASSWORD_RESET_EXPIRATION
- VERIFICATION_CODE_EXPIRATION
- MAX_FAILED_LOGIN_ATTEMPTS
- ACCOUNT_LOCKOUT_DURATION
- SESSION_TIMEOUT

and sets default values for any missing environment variables.
The default values are defined in the defaults module.
*/

use std::env;
use std::sync::OnceLock;

use super::defaults;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub jwt_secret: String,
    pub jwt_expiration: u32,              // in minutes
    pub refresh_token_expiration: u8,     // in days
    pub access_token_expiration: u8,      // in minutes
    pub password_reset_expiration: u8,    // in hours
    pub verification_code_expiration: u8, // in hours
    pub max_failed_login_attempts: u8,
    pub account_lockout_duration: u8, // in minutes
    pub session_timeout: u8,          // in minutes
}

impl AppConfig {
    fn load() -> Self {
        println!("Loading app config ... ... ...");
        Self {
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            jwt_expiration: env::var("JWT_EXPIRATION")
                .unwrap_or_else(|_| defaults::JWT_EXPIRATION.to_string())
                .parse()
                .expect("JWT_EXPIRATION must be a number"),
            refresh_token_expiration: env::var("REFRESH_TOKEN_EXPIRATION")
                .unwrap_or_else(|_| defaults::REFRESH_TOKEN_EXPIRATION.to_string())
                .parse()
                .expect("REFRESH_TOKEN_EXPIRATION must be a number"),
            access_token_expiration: env::var("ACCESS_TOKEN_EXPIRATION")
                .unwrap_or_else(|_| defaults::ACCESS_TOKEN_EXPIRATION.to_string())
                .parse()
                .expect("ACCESS_TOKEN_EXPIRATION must be a number"),
            password_reset_expiration: env::var("PASSWORD_RESET_EXPIRATION")
                .unwrap_or_else(|_| defaults::PASSWORD_RESET_EXPIRATION.to_string())
                .parse()
                .expect("PASSWORD_RESET_EXPIRATION must be a number"),
            verification_code_expiration: env::var("VERIFICATION_CODE_EXPIRATION")
                .unwrap_or_else(|_| defaults::VERIFICATION_CODE_EXPIRATION.to_string())
                .parse()
                .expect("VERIFICATION_CODE_EXPIRATION must be a number"),
            max_failed_login_attempts: env::var("MAX_FAILED_LOGIN_ATTEMPTS")
                .unwrap_or_else(|_| defaults::MAX_FAILED_LOGIN_ATTEMPTS.to_string())
                .parse()
                .expect("MAX_FAILED_LOGIN_ATTEMPTS to be a number"),
            account_lockout_duration: env::var("ACCOUNT_LOCKOUT_DURATION")
                .unwrap_or_else(|_| defaults::ACCOUNT_LOCKOUT_DURATION.to_string())
                .parse()
                .expect("ACCOUNT_LOCKOUT_DURATION must be a number"),
            session_timeout: env::var("SESSION_TIMEOUT")
                .unwrap_or_else(|_| defaults::SESSION_TIMEOUT.to_string())
                .parse()
                .expect("SESSION_TIMEOUT must be a number"),
        }
    }
}

// Global config instance
static CONFIG_INSTANCE: OnceLock<AppConfig> = OnceLock::new();

pub async fn get_config() -> &'static AppConfig {
    CONFIG_INSTANCE.get_or_init(|| AppConfig::load())
}

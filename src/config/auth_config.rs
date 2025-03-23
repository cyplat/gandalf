// Configuration for the auth service
#[derive(Clone, Debug)]
pub struct AuthConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiration: i64,    // in minutes
    pub refresh_token_expiration: i64, // in days
    pub access_token_expiration: i64,  // in minutes
    pub password_reset_expiration: i64, // in hours
    pub verification_code_expiration: i64, // in hours
    pub max_failed_login_attempts: i32,
    pub account_lockout_duration: i64, // in minutes
    pub session_timeout: i64,  // in minutes
}

impl AuthConfig {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            jwt_secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set"),
            jwt_expiration: env::var("JWT_EXPIRATION")
                .unwrap_or_else(|_| "60".to_string())
                .parse()
                .unwrap_or(60),
            refresh_token_expiration: env::var("REFRESH_TOKEN_EXPIRATION")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
            access_token_expiration: env::var("ACCESS_TOKEN_EXPIRATION")
                .unwrap_or_else(|_| "15".to_string())
                .parse()
                .unwrap_or(15),
            password_reset_expiration: env::var("PASSWORD_RESET_EXPIRATION")
                .unwrap_or_else(|_| "24".to_string())
                .parse()
                .unwrap_or(24),
            verification_code_expiration: env::var("VERIFICATION_CODE_EXPIRATION")
                .unwrap_or_else(|_| "24".to_string())
                .parse()
                .unwrap_or(24),
            max_failed_login_attempts: env::var("MAX_FAILED_LOGIN_ATTEMPTS")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .unwrap_or(5),
            account_lockout_duration: env::var("ACCOUNT_LOCKOUT_DURATION")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
            session_timeout: env::var("SESSION_TIMEOUT")
                .unwrap_or_else(|_| "120".to_string())
                .parse()
                .unwrap_or(120),
        }
    }
}
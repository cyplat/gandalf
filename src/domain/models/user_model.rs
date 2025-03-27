/*
This module holds the user model
*/

use chrono::{DateTime, Utc};
use std::net::IpAddr;
use uuid::Uuid;

use super::auth_provider_model::AuthProvider;
#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub external_id: Option<String>,
    pub username: Option<String>,
    pub email: String,
    pub password_hash: Option<String>,
    pub password_updated_at: Option<DateTime<Utc>>,
    pub password_reset_required: bool,
    pub failed_login_attempts: i32,
    pub last_failed_attempt: Option<DateTime<Utc>>,
    pub account_locked: bool,
    pub account_locked_until: Option<DateTime<Utc>>,
    pub account_enabled: bool,
    pub email_verified: bool,
    pub email_verification_token: Option<String>,
    pub email_verification_sent_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub requires_mfa: bool,
    pub auth_provider: AuthProvider,
    pub user_state: UserState,
    pub last_login_ip: Option<IpAddr>,
    pub last_user_agent: Option<String>,
    pub data_region: String,
    pub deletion_scheduled_at: Option<DateTime<Utc>>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            external_id: None,
            username: None,
            email: String::new(),
            password_hash: None,
            password_updated_at: None,
            password_reset_required: false,
            failed_login_attempts: 0,
            last_failed_attempt: None,
            account_locked: false,
            account_locked_until: None,
            account_enabled: true,
            email_verified: false,
            email_verification_token: None,
            email_verification_sent_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_login_at: None,
            requires_mfa: false,
            auth_provider: AuthProvider::default(),
            user_state: UserState::default(),
            last_login_ip: None,
            last_user_agent: None,
            data_region: "us-east".to_string(),
            deletion_scheduled_at: None,
        }
    }
}

#[derive(Debug)]
pub enum UserState {
    Registered,
    Verified,
    Active,
    Incomplete,
    Disabled,
    Locked,
    Deleted,
}
impl Default for UserState {
    fn default() -> Self {
        UserState::Registered
    }
}

impl std::str::FromStr for UserState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "registered" => Ok(UserState::Registered),
            "verified" => Ok(UserState::Verified),
            "active" => Ok(UserState::Active),
            "incomplete" => Ok(UserState::Incomplete),
            "disabled" => Ok(UserState::Disabled),
            "locked" => Ok(UserState::Locked),
            "deleted" => Ok(UserState::Deleted),
            _ => Err(format!("Invalid user state: {}", s)),
        }
    }
}

impl ToString for UserState {
    fn to_string(&self) -> String {
        match self {
            UserState::Registered => "registered".to_string(),
            UserState::Verified => "verified".to_string(),
            UserState::Active => "active".to_string(),
            UserState::Incomplete => "incomplete".to_string(),
            UserState::Disabled => "disabled".to_string(),
            UserState::Locked => "locked".to_string(),
            UserState::Deleted => "deleted".to_string(),
        }
    }
}

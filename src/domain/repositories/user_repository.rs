/*
This module holds user repository
*/
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::errors::DomainError;
use crate::domain::models::AuthProvider;
use crate::domain::models::User;

use super::base_repository::{BaseRepository, PgPool, RepositoryTrait};

type Result<T> = std::result::Result<T, DomainError>;

// Create User Repository
pub struct UserRepository {
    base: BaseRepository,
}

impl UserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self {
            base: BaseRepository::new(pool),
        }
    }
}

#[async_trait]
impl RepositoryTrait<User, Uuid> for UserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let conn = self.base.get_conn().await?;

        let query = "
            SELECT 
                user_id, external_id, username, email, password_hash, 
                password_updated_at, password_reset_required, failed_login_attempts,
                last_failed_attempt, account_locked, account_locked_until,
                account_enabled, email_verified, email_verification_token,
                email_verification_sent_at, created_at, updated_at, 
                last_login_at, requires_mfa, auth_provider,
                last_login_ip, last_user_agent, data_region, deletion_scheduled_at
            FROM auth.users
            WHERE user_id = $1
        ";

        let row = conn
            .query_opt(query, &[&id])
            .await
            .map_err(DomainError::DatabaseError)?;

        match row {
            Some(row) => Ok(Some(User::from_row(&row))),
            None => Ok(None),
        }
    }
}

// Helper functions for converting database rows to domain models
impl User {
    fn from_row(row: &tokio_postgres::Row) -> Self {
        User {
            user_id: row.get("user_id"),
            external_id: row.get("external_id"),
            username: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            password_updated_at: row.get("password_updated_at"),
            password_reset_required: row.get("password_reset_required"),
            failed_login_attempts: row.get("failed_login_attempts"),
            last_failed_attempt: row.get("last_failed_attempt"),
            account_locked: row.get("account_locked"),
            account_locked_until: row.get("account_locked_until"),
            account_enabled: row.get("account_enabled"),
            email_verified: row.get("email_verified"),
            email_verification_token: row.get("email_verification_token"),
            email_verification_sent_at: row.get("email_verification_sent_at"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            last_login_at: row.get("last_login_at"),
            requires_mfa: row.get("requires_mfa"),
            auth_provider: AuthProvider::Custom,
            last_login_ip: row.get("last_login_ip"),
            last_user_agent: row.get("last_user_agent"),
            data_region: row.get("data_region"),
            deletion_scheduled_at: row.get("deletion_scheduled_at"),
        }
    }
}

/*
This module holds user repository
*/
use async_trait::async_trait;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::errors::DomainError;
use crate::domain::models::AuthProvider;
use crate::domain::models::{User, UserState};

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
    pub async fn create(&self, user: &User) -> Result<User> {
        let conn = self.base.get_conn().await?;

        let query = "
            INSERT INTO auth.users (
                external_id, username, email, password_hash,
                password_updated_at, password_reset_required, failed_login_attempts,
                last_failed_attempt, account_locked, account_locked_until,
                account_enabled, email_verified, email_verification_token,
                email_verification_sent_at, created_at, updated_at,
                last_login_at, requires_mfa, auth_provider,user_state,
                last_login_ip, last_user_agent, data_region, deletion_scheduled_at
            )
            VALUES (
                $1, $2, $3, $4,
                $5, $6, $7,
                $8, $9, $10,
                $11, $12, $13,
                $14, $15, $16,
                $17, $18, $19,
                $20, $21, $22, $23
            )
            RETURNING *
        ";

        let row = conn
            .query_one(
                query,
                &[
                    &user.external_id,
                    &user.username,
                    &user.email,
                    &user.password_hash,
                    &user.password_updated_at,
                    &user.password_reset_required,
                    &user.failed_login_attempts,
                    &user.last_failed_attempt,
                    &user.account_locked,
                    &user.account_locked_until,
                    &user.account_enabled,
                    &user.email_verified,
                    &user.email_verification_token,
                    &user.email_verification_sent_at,
                    &user.created_at,
                    &user.updated_at,
                    &user.last_login_at,
                    &user.requires_mfa,
                    &user.auth_provider.to_string(),
                    &user.user_state.to_string(),
                    &user.last_login_ip,
                    &user.last_user_agent,
                    &user.data_region,
                    &user.deletion_scheduled_at,
                ],
            )
            .await
            .map_err(DomainError::DatabaseError)?;

        Ok(User::from_row(&row))
    }
    

    pub async fn email_exists(&self, email: &str) -> Result<bool> {
        let conn = self.base.get_conn().await?;

        let query = "
            SELECT EXISTS (
                SELECT 1
                FROM auth.users
                WHERE email = $1
            )
        ";

        let row = conn
            .query_one(query, &[&email])
            .await
            .map_err(DomainError::DatabaseError)?;

        let exists: bool = row.get(0);

        Ok(exists)
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
                last_login_at, requires_mfa, auth_provider,user_state,
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
            auth_provider: AuthProvider::from_str(row.get("auth_provider"))
                .expect("auth_provider should not be missing"),
            user_state: UserState::from_str(row.get("user_state"))
                .expect("user_state should not be missing"),
            last_login_ip: row.get("last_login_ip"),
            last_user_agent: row.get("last_user_agent"),
            data_region: row.get("data_region"),
            deletion_scheduled_at: row.get("deletion_scheduled_at"),
        }
    }
}

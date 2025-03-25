use std::sync::Arc;
use uuid::Uuid;

use crate::config::database::PgPool;
use crate::domain::errors::DomainError;
use crate::domain::models::User;
use crate::domain::repositories::{RepositoryTrait, UserRepository};

type Result<T> = std::result::Result<T, DomainError>;

pub struct UserService {
    user_repo: UserRepository,
}

impl UserService {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self {
            user_repo: UserRepository::new(db_pool),
        }
    }

    pub async fn get_user(&self, user_id: Uuid) -> Result<Option<User>> {
        let user = self.user_repo.find_by_id(user_id).await?;
        Ok(user)
    }
}

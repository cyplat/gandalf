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

    pub async fn save(&self, user: User) -> Result<User> {
        let user = self.user_repo.create(&user).await?;
        Ok(user)
    }

    // takes a user object and saves it.
    pub fn create_user(&self, email: String) -> User {
        let mut user = User::default();
        user.email = email;
        user
    }

    pub async fn user_exists(&self, email: &str) -> Result<bool> {
        let exists = self.user_repo.email_exists(email).await?;
        Ok(exists)
    }

    pub async fn generate_email_verification_token(&self, user_id: &Uuid) -> Result<String> {
        // Todo: generate a token and save it to the database
        // let token = Uuid::new_v4().to_string();
        Ok("some very long verification token".to_string())
    }
}

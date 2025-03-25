// Custom error type for domain

use tokio_postgres::Error as PgError;

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Entity not found")]
    NotFound,

    #[error("Database error: {0}")]
    DatabaseError(#[from] PgError),

    #[error("Unique constraint violation: {0}")]
    UniqueViolation(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

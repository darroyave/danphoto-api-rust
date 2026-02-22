use crate::domain::{AuthRepository, AuthUser, DomainError};
use async_trait::async_trait;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
struct AuthUserRow {
    id: Uuid,
    email: String,
    password_hash: String,
}

impl From<AuthUserRow> for AuthUser {
    fn from(row: AuthUserRow) -> Self {
        AuthUser {
            id: row.id,
            email: row.email,
            password_hash: row.password_hash,
        }
    }
}

pub struct AuthRepositoryImpl {
    pool: sqlx::PgPool,
}

impl AuthRepositoryImpl {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuthRepository for AuthRepositoryImpl {
    async fn get_by_email(&self, email: &str) -> Result<Option<AuthUser>, DomainError> {
        let row = sqlx::query_as::<_, AuthUserRow>(
            "SELECT id, email, password_hash FROM usuarios WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(row.map(AuthUser::from))
    }
}

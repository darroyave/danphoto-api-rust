use crate::domain::{DomainError, Usuario, UsuariosRepository};
use async_trait::async_trait;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct UsuarioRow {
    pub id: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
    pub url: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<UsuarioRow> for Usuario {
    fn from(row: UsuarioRow) -> Self {
        Usuario {
            id: row.id,
            name: row.name,
            email: row.email,
            url: row.url,
            created_at: row.created_at,
        }
    }
}

pub struct UsuariosRepositoryImpl {
    pool: sqlx::PgPool,
}

impl UsuariosRepositoryImpl {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UsuariosRepository for UsuariosRepositoryImpl {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Usuario>, DomainError> {
        let row = sqlx::query_as::<_, UsuarioRow>(
            "SELECT id, name, email, url, created_at FROM usuarios WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(row.map(Usuario::from))
    }

    async fn update_name(&self, id: Uuid, name: Option<&str>) -> Result<Option<Usuario>, DomainError> {
        let row = sqlx::query_as::<_, UsuarioRow>(
            r#"
            UPDATE usuarios SET name = $2 WHERE id = $1
            RETURNING id, name, email, url, created_at
            "#,
        )
        .bind(id)
        .bind(name)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(row.map(Usuario::from))
    }

    async fn update_avatar(&self, id: Uuid, url: &str) -> Result<Option<Usuario>, DomainError> {
        let row = sqlx::query_as::<_, UsuarioRow>(
            r#"
            UPDATE usuarios SET url = $2 WHERE id = $1
            RETURNING id, name, email, url, created_at
            "#,
        )
        .bind(id)
        .bind(url)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(row.map(Usuario::from))
    }
}

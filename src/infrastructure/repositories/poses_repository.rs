use crate::domain::{DomainError, Pose, PosesRepository};
use async_trait::async_trait;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct PoseRow {
    pub id: Uuid,
    pub name: Option<String>,
    pub url: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<PoseRow> for Pose {
    fn from(row: PoseRow) -> Self {
        Pose {
            id: row.id,
            name: row.name,
            url: row.url,
            created_at: row.created_at,
        }
    }
}

pub struct PosesRepositoryImpl {
    pool: sqlx::PgPool,
}

impl PosesRepositoryImpl {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PosesRepository for PosesRepositoryImpl {
    async fn get_all(&self) -> Result<Vec<Pose>, DomainError> {
        let rows = sqlx::query_as::<_, PoseRow>(
            "SELECT id, name, url, created_at FROM poses ORDER BY created_at DESC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(rows.into_iter().map(Pose::from).collect())
    }

    async fn get_paginated(&self, page: u32, limit: u32) -> Result<Vec<Pose>, DomainError> {
        let offset = page.saturating_mul(limit);
        let rows = sqlx::query_as::<_, PoseRow>(
            "SELECT id, name, url, created_at FROM poses ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        )
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(rows.into_iter().map(Pose::from).collect())
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Pose>, DomainError> {
        let row = sqlx::query_as::<_, PoseRow>(
            "SELECT id, name, url, created_at FROM poses WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(row.map(Pose::from))
    }

    async fn create_with_id(&self, id: Uuid, name: Option<&str>, url: &str) -> Result<Pose, DomainError> {
        let row = sqlx::query_as::<_, PoseRow>(
            r#"
            INSERT INTO poses (id, name, url)
            VALUES ($1, $2, $3)
            RETURNING id, name, url, created_at
            "#,
        )
        .bind(id)
        .bind(name)
        .bind(url)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(Pose::from(row))
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM poses WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(())
    }
}

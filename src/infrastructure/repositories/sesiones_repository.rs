use crate::domain::{DomainError, Pose, Sesion, SesionesRepository};
use async_trait::async_trait;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct SesionRow {
    pub id: Uuid,
    pub name: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub cover_url: String,
}

impl From<SesionRow> for Sesion {
    fn from(row: SesionRow) -> Self {
        Sesion {
            id: row.id,
            name: row.name,
            created_at: row.created_at,
            cover_url: row.cover_url,
        }
    }
}

#[derive(FromRow)]
struct PoseRow {
    id: Uuid,
    name: Option<String>,
    url: String,
    created_at: Option<chrono::DateTime<chrono::Utc>>,
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

pub struct SesionesRepositoryImpl {
    pool: sqlx::PgPool,
}

impl SesionesRepositoryImpl {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SesionesRepository for SesionesRepositoryImpl {
    async fn get_all(&self) -> Result<Vec<Sesion>, DomainError> {
        let rows = sqlx::query_as::<_, SesionRow>(
            "SELECT id, name, created_at, cover_url FROM sesiones ORDER BY created_at DESC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(rows.into_iter().map(Sesion::from).collect())
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Sesion>, DomainError> {
        let row = sqlx::query_as::<_, SesionRow>(
            "SELECT id, name, created_at, cover_url FROM sesiones WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(row.map(Sesion::from))
    }

    async fn get_poses_by_sesion(&self, sesion_id: Uuid) -> Result<Vec<Pose>, DomainError> {
        let rows = sqlx::query_as::<_, PoseRow>(
            r#"
            SELECT p.id, p.name, p.url, p.created_at
            FROM poses p
            INNER JOIN sesion_image si ON si.pose_id = p.id
            WHERE si.sesion_id = $1
            ORDER BY si.created_at ASC
            "#,
        )
        .bind(sesion_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(rows.into_iter().map(Pose::from).collect())
    }

    async fn create(&self, name: &str) -> Result<Sesion, DomainError> {
        let row = sqlx::query_as::<_, SesionRow>(
            r#"
            INSERT INTO sesiones (name)
            VALUES ($1)
            RETURNING id, name, created_at, cover_url
            "#,
        )
        .bind(name)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(Sesion::from(row))
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM sesiones WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(())
    }

    async fn add_poses_to_sesion(
        &self,
        sesion_id: Uuid,
        pose_ids: &[Uuid],
    ) -> Result<(), DomainError> {
        if pose_ids.is_empty() {
            return Ok(());
        }
        for pose_id in pose_ids {
            sqlx::query(
                r#"
                INSERT INTO sesion_image (sesion_id, pose_id)
                VALUES ($1, $2)
                ON CONFLICT (sesion_id, pose_id) DO NOTHING
                "#,
            )
            .bind(sesion_id)
            .bind(pose_id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        }
        Ok(())
    }

    async fn remove_pose_from_sesion(
        &self,
        sesion_id: Uuid,
        pose_id: Uuid,
    ) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM sesion_image WHERE sesion_id = $1 AND pose_id = $2")
            .bind(sesion_id)
            .bind(pose_id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(())
    }

    async fn update_cover(
        &self,
        sesion_id: Uuid,
        cover_url: &str,
    ) -> Result<Option<Sesion>, DomainError> {
        let row = sqlx::query_as::<_, SesionRow>(
            r#"
            UPDATE sesiones SET cover_url = $2 WHERE id = $1
            RETURNING id, name, created_at, cover_url
            "#,
        )
        .bind(sesion_id)
        .bind(cover_url)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(row.map(Sesion::from))
    }
}

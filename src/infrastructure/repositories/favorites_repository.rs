use crate::domain::{DomainError, FavoritesRepository, Pose};
use async_trait::async_trait;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct PoseRow {
    pub id: Uuid,
    pub url: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<PoseRow> for Pose {
    fn from(row: PoseRow) -> Self {
        Pose {
            id: row.id,
            url: row.url,
            created_at: row.created_at,
        }
    }
}

pub struct FavoritesRepositoryImpl {
    pool: sqlx::PgPool,
}

impl FavoritesRepositoryImpl {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl FavoritesRepository for FavoritesRepositoryImpl {
    async fn is_pose_favorite(&self, user_id: Uuid, pose_id: Uuid) -> Result<bool, DomainError> {
        let row: Option<(i64,)> = sqlx::query_as(
            "SELECT 1 FROM favoritos WHERE user_id = $1 AND pose_id = $2 LIMIT 1",
        )
        .bind(user_id)
        .bind(pose_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(row.is_some())
    }

    async fn add_pose_to_favorites(
        &self,
        user_id: Uuid,
        pose_id: Uuid,
    ) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            INSERT INTO favoritos (user_id, pose_id)
            VALUES ($1, $2)
            ON CONFLICT (pose_id, user_id) DO NOTHING
            "#,
        )
        .bind(user_id)
        .bind(pose_id)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(())
    }

    async fn remove_pose_from_favorites(
        &self,
        user_id: Uuid,
        pose_id: Uuid,
    ) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM favoritos WHERE user_id = $1 AND pose_id = $2")
            .bind(user_id)
            .bind(pose_id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(())
    }

    async fn remove_poses_from_favorites(
        &self,
        user_id: Uuid,
        pose_ids: &[Uuid],
    ) -> Result<(), DomainError> {
        if pose_ids.is_empty() {
            return Ok(());
        }
        // DELETE FROM favoritos WHERE user_id = $1 AND pose_id = ANY($2)
        let mut q = sqlx::query("DELETE FROM favoritos WHERE user_id = $1 AND pose_id = ANY($2)");
        q = q.bind(user_id).bind(pose_ids);
        q.execute(&self.pool)
            .await
            .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(())
    }

    async fn get_favorite_poses(&self, user_id: Uuid) -> Result<Vec<Pose>, DomainError> {
        let rows = sqlx::query_as::<_, PoseRow>(
            r#"
            SELECT p.id, p.url, p.created_at
            FROM poses p
            INNER JOIN favoritos f ON f.pose_id = p.id
            WHERE f.user_id = $1
            ORDER BY f.created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(rows.into_iter().map(Pose::from).collect())
    }
}

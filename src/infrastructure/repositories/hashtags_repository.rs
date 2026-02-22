use crate::domain::{DomainError, Hashtag, HashtagsRepository, Pose};
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

#[derive(FromRow)]
pub struct HashtagRow {
    pub id: Uuid,
    pub name: String,
}

impl From<HashtagRow> for Hashtag {
    fn from(row: HashtagRow) -> Self {
        Hashtag {
            id: row.id,
            name: row.name,
        }
    }
}

pub struct HashtagsRepositoryImpl {
    pool: sqlx::PgPool,
}

impl HashtagsRepositoryImpl {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HashtagsRepository for HashtagsRepositoryImpl {
    async fn get_all(&self) -> Result<Vec<Hashtag>, DomainError> {
        let rows = sqlx::query_as::<_, HashtagRow>(
            "SELECT id, name FROM hashtags ORDER BY name ASC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(rows.into_iter().map(Hashtag::from).collect())
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Hashtag>, DomainError> {
        let row = sqlx::query_as::<_, HashtagRow>("SELECT id, name FROM hashtags WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(row.map(Hashtag::from))
    }

    async fn create(&self, name: &str) -> Result<Hashtag, DomainError> {
        let row = sqlx::query_as::<_, HashtagRow>(
            r#"
            INSERT INTO hashtags (name)
            VALUES ($1)
            RETURNING id, name
            "#,
        )
        .bind(name)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(Hashtag::from(row))
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM hashtags WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(())
    }

    async fn get_hashtags_by_pose(&self, pose_id: Uuid) -> Result<Vec<Hashtag>, DomainError> {
        let rows = sqlx::query_as::<_, HashtagRow>(
            r#"
            SELECT h.id, h.name
            FROM hashtags h
            INNER JOIN hashtag_image hi ON hi.hashtag_id = h.id
            WHERE hi.pose_id = $1
            ORDER BY hi.created_at DESC
            "#,
        )
        .bind(pose_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(rows.into_iter().map(Hashtag::from).collect())
    }

    async fn add_hashtags_to_post(
        &self,
        post_id: Uuid,
        hashtag_ids: &[Uuid],
    ) -> Result<(), DomainError> {
        for &hashtag_id in hashtag_ids {
            sqlx::query(
                r#"
                INSERT INTO hashtag_post (post_id, hashtag_id)
                VALUES ($1, $2)
                ON CONFLICT (post_id, hashtag_id) DO NOTHING
                "#,
            )
            .bind(post_id)
            .bind(hashtag_id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        }
        Ok(())
    }

    async fn add_hashtag_to_pose(
        &self,
        pose_id: Uuid,
        hashtag_id: Uuid,
    ) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            INSERT INTO hashtag_image (pose_id, hashtag_id)
            VALUES ($1, $2)
            ON CONFLICT (hashtag_id, pose_id) DO NOTHING
            "#,
        )
        .bind(pose_id)
        .bind(hashtag_id)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(())
    }

    async fn remove_hashtag_from_pose(
        &self,
        pose_id: Uuid,
        hashtag_id: Uuid,
    ) -> Result<(), DomainError> {
        sqlx::query(
            "DELETE FROM hashtag_image WHERE pose_id = $1 AND hashtag_id = $2",
        )
        .bind(pose_id)
        .bind(hashtag_id)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(())
    }

    async fn remove_all_hashtags_from_pose(&self, pose_id: Uuid) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM hashtag_image WHERE pose_id = $1")
            .bind(pose_id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(())
    }

    async fn get_poses_by_hashtag(&self, hashtag_id: Uuid) -> Result<Vec<Pose>, DomainError> {
        let rows = sqlx::query_as::<_, PoseRow>(
            r#"
            SELECT p.id, p.name, p.url, p.created_at
            FROM poses p
            INNER JOIN hashtag_image hi ON hi.pose_id = p.id
            WHERE hi.hashtag_id = $1
            ORDER BY hi.created_at DESC
            "#,
        )
        .bind(hashtag_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(rows.into_iter().map(Pose::from).collect())
    }

    async fn get_poses_by_hashtag_paginated(
        &self,
        hashtag_id: Uuid,
        page: u32,
        limit: u32,
    ) -> Result<Vec<Pose>, DomainError> {
        let offset = page.saturating_mul(limit);
        let rows = sqlx::query_as::<_, PoseRow>(
            r#"
            SELECT p.id, p.name, p.url, p.created_at
            FROM poses p
            INNER JOIN hashtag_image hi ON hi.pose_id = p.id
            WHERE hi.hashtag_id = $1
            ORDER BY hi.created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(hashtag_id)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(rows.into_iter().map(Pose::from).collect())
    }
}

use crate::domain::{DomainError, Post, PostsRepository};
use async_trait::async_trait;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct PostRow {
    pub id: Uuid,
    pub description: Option<String>,
    pub url: Option<String>,
    pub user_id: Option<Uuid>,
    pub theme_of_the_day_id: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<PostRow> for Post {
    fn from(row: PostRow) -> Self {
        Post {
            id: row.id,
            description: row.description,
            url: row.url,
            user_id: row.user_id,
            theme_of_the_day_id: row.theme_of_the_day_id,
            created_at: row.created_at,
        }
    }
}

pub struct PostsRepositoryImpl {
    pool: sqlx::PgPool,
}

impl PostsRepositoryImpl {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PostsRepository for PostsRepositoryImpl {
    async fn get_all(&self) -> Result<Vec<Post>, DomainError> {
        let rows = sqlx::query_as::<_, PostRow>(
            "SELECT id, description, url, user_id, theme_of_the_day_id, created_at FROM posts ORDER BY created_at DESC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(rows.into_iter().map(Post::from).collect())
    }

    async fn get_paginated(&self, page: u32, limit: u32) -> Result<Vec<Post>, DomainError> {
        let offset = page.saturating_mul(limit);
        let rows = sqlx::query_as::<_, PostRow>(
            "SELECT id, description, url, user_id, theme_of_the_day_id, created_at FROM posts ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        )
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(rows.into_iter().map(Post::from).collect())
    }

    async fn get_by_theme_of_the_day_id(
        &self,
        theme_of_the_day_id: &str,
    ) -> Result<Vec<Post>, DomainError> {
        let rows = sqlx::query_as::<_, PostRow>(
            "SELECT id, description, url, user_id, theme_of_the_day_id, created_at FROM posts WHERE theme_of_the_day_id = $1 ORDER BY created_at DESC",
        )
        .bind(theme_of_the_day_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(rows.into_iter().map(Post::from).collect())
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Post>, DomainError> {
        let row = sqlx::query_as::<_, PostRow>(
            "SELECT id, description, url, user_id, theme_of_the_day_id, created_at FROM posts WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(row.map(Post::from))
    }

    async fn create_with_id(
        &self,
        id: Uuid,
        description: Option<&str>,
        url: Option<&str>,
        user_id: Option<Uuid>,
    ) -> Result<Post, DomainError> {
        let row = sqlx::query_as::<_, PostRow>(
            r#"
            INSERT INTO posts (id, description, url, user_id)
            VALUES ($1, $2, $3, $4)
            RETURNING id, description, url, user_id, theme_of_the_day_id, created_at
            "#,
        )
        .bind(id)
        .bind(description)
        .bind(url)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(Post::from(row))
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM posts WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(())
    }
}

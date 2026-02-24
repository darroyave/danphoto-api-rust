use crate::domain::{
    DomainError, PortfolioCategory, PortfolioImage, PortfolioRepository,
};
use async_trait::async_trait;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct PortfolioCategoryRow {
    pub id: Uuid,
    pub name: String,
    pub cover_url: String,
}

impl From<PortfolioCategoryRow> for PortfolioCategory {
    fn from(row: PortfolioCategoryRow) -> Self {
        PortfolioCategory {
            id: row.id,
            name: row.name,
            cover_url: row.cover_url,
        }
    }
}

#[derive(FromRow)]
pub struct PortfolioImageRow {
    pub id: Uuid,
    pub portfolio_category_id: Uuid,
    pub url: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<PortfolioImageRow> for PortfolioImage {
    fn from(row: PortfolioImageRow) -> Self {
        PortfolioImage {
            id: row.id,
            portfolio_category_id: row.portfolio_category_id,
            url: row.url,
            created_at: row.created_at,
        }
    }
}

pub struct PortfolioRepositoryImpl {
    pool: sqlx::PgPool,
}

impl PortfolioRepositoryImpl {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PortfolioRepository for PortfolioRepositoryImpl {
    async fn get_categories(&self) -> Result<Vec<PortfolioCategory>, DomainError> {
        let rows = sqlx::query_as::<_, PortfolioCategoryRow>(
            "SELECT id, name, cover_url FROM portfolio_category ORDER BY name ASC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(rows.into_iter().map(PortfolioCategory::from).collect())
    }

    async fn get_images_by_category(
        &self,
        category_id: Uuid,
        page: u32,
        limit: u32,
    ) -> Result<Vec<PortfolioImage>, DomainError> {
        let offset = page.saturating_mul(limit);
        let rows = sqlx::query_as::<_, PortfolioImageRow>(
            "SELECT id, portfolio_category_id, url, created_at FROM portfolio_image WHERE portfolio_category_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
        )
        .bind(category_id)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(rows.into_iter().map(PortfolioImage::from).collect())
    }

    async fn count_images_by_category(&self, category_id: Uuid) -> Result<u64, DomainError> {
        let row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM portfolio_image WHERE portfolio_category_id = $1",
        )
        .bind(category_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(row.0 as u64)
    }

    async fn create_category(&self, name: &str) -> Result<PortfolioCategory, DomainError> {
        let row = sqlx::query_as::<_, PortfolioCategoryRow>(
            r#"
            INSERT INTO portfolio_category (name, cover_url)
            VALUES ($1, '')
            RETURNING id, name, cover_url
            "#,
        )
        .bind(name)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(PortfolioCategory::from(row))
    }

    async fn update_category(
        &self,
        id: Uuid,
        name: &str,
    ) -> Result<Option<PortfolioCategory>, DomainError> {
        let row = sqlx::query_as::<_, PortfolioCategoryRow>(
            r#"
            UPDATE portfolio_category SET name = $2 WHERE id = $1
            RETURNING id, name, cover_url
            "#,
        )
        .bind(id)
        .bind(name)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(row.map(PortfolioCategory::from))
    }

    async fn delete_category(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM portfolio_category WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(())
    }

    async fn add_image_with_id(
        &self,
        id: Uuid,
        category_id: Uuid,
        url: &str,
    ) -> Result<PortfolioImage, DomainError> {
        let row = sqlx::query_as::<_, PortfolioImageRow>(
            r#"
            INSERT INTO portfolio_image (id, portfolio_category_id, url)
            VALUES ($1, $2, $3)
            RETURNING id, portfolio_category_id, url, created_at
            "#,
        )
        .bind(id)
        .bind(category_id)
        .bind(url)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            let msg = format!(
                "portfolio_image INSERT falló: {} (comprueba que la tabla exista con columnas id UUID, portfolio_category_id UUID, url TEXT, created_at TIMESTAMPTZ DEFAULT now())",
                e
            );
            DomainError::Repository(anyhow::Error::msg(msg))
        })?;
        Ok(PortfolioImage::from(row))
    }

    async fn delete_image(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM portfolio_image WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(())
    }
}

use crate::domain::{DomainError, ThemeOfTheDay, ThemeOfTheDayRepository};
use async_trait::async_trait;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct ThemeOfTheDayRow {
    pub id: String,
    pub name: String,
    pub url: String,
}

impl From<ThemeOfTheDayRow> for ThemeOfTheDay {
    fn from(row: ThemeOfTheDayRow) -> Self {
        ThemeOfTheDay {
            id: row.id,
            name: row.name,
            url: row.url,
        }
    }
}

pub struct ThemeOfTheDayRepositoryImpl {
    pool: sqlx::PgPool,
}

impl ThemeOfTheDayRepositoryImpl {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ThemeOfTheDayRepository for ThemeOfTheDayRepositoryImpl {
    async fn get_all(&self) -> Result<Vec<ThemeOfTheDay>, DomainError> {
        let rows = sqlx::query_as::<_, ThemeOfTheDayRow>(
            "SELECT id, name, url FROM theme_of_the_day ORDER BY id ASC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(rows.into_iter().map(ThemeOfTheDay::from).collect())
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<ThemeOfTheDay>, DomainError> {
        let row = sqlx::query_as::<_, ThemeOfTheDayRow>(
            "SELECT id, name, url FROM theme_of_the_day WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(row.map(ThemeOfTheDay::from))
    }

    async fn create(
        &self,
        id: &str,
        name: &str,
        url: &str,
    ) -> Result<ThemeOfTheDay, DomainError> {
        let row = sqlx::query_as::<_, ThemeOfTheDayRow>(
            r#"
            INSERT INTO theme_of_the_day (id, name, url)
            VALUES ($1, $2, $3)
            RETURNING id, name, url
            "#,
        )
        .bind(id)
        .bind(name)
        .bind(url)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(ThemeOfTheDay::from(row))
    }

    async fn update(
        &self,
        id: &str,
        name: Option<&str>,
        url: Option<&str>,
    ) -> Result<Option<ThemeOfTheDay>, DomainError> {
        let row = sqlx::query_as::<_, ThemeOfTheDayRow>(
            r#"
            UPDATE theme_of_the_day
            SET
                name = COALESCE($2, name),
                url = COALESCE($3, url)
            WHERE id = $1
            RETURNING id, name, url
            "#,
        )
        .bind(id)
        .bind(name)
        .bind(url)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(row.map(ThemeOfTheDay::from))
    }

    async fn delete(&self, id: &str) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM theme_of_the_day WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(())
    }
}

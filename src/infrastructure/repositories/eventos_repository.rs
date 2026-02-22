use crate::domain::{DomainError, Evento, EventosRepository};
use async_trait::async_trait;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct EventoRow {
    pub id: Uuid,
    pub name: String,
    pub place: String,
    pub mmdd: String,
    pub url: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<EventoRow> for Evento {
    fn from(row: EventoRow) -> Self {
        Evento {
            id: row.id,
            name: row.name,
            place: row.place,
            mmdd: row.mmdd,
            url: row.url,
            created_at: row.created_at,
        }
    }
}

pub struct EventosRepositoryImpl {
    pool: sqlx::PgPool,
}

impl EventosRepositoryImpl {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EventosRepository for EventosRepositoryImpl {
    async fn get_all(&self) -> Result<Vec<Evento>, DomainError> {
        let rows = sqlx::query_as::<_, EventoRow>("SELECT id, name, place, mmdd, url, created_at FROM eventos ORDER BY id ASC")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(rows.into_iter().map(Evento::from).collect())
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Evento>, DomainError> {
        let row = sqlx::query_as::<_, EventoRow>(
            "SELECT id, name, place, mmdd, url, created_at FROM eventos WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(row.map(Evento::from))
    }

    async fn create(
        &self,
        name: &str,
        place: &str,
        url: &str,
        mmdd: &str,
    ) -> Result<Evento, DomainError> {
        let id = Uuid::new_v4();
        let row = sqlx::query_as::<_, EventoRow>(
            r#"
            INSERT INTO eventos (id, name, place, url, mmdd)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, name, place, mmdd, url, created_at
            "#,
        )
        .bind(id)
        .bind(name)
        .bind(place)
        .bind(url)
        .bind(mmdd)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(Evento::from(row))
    }

    async fn update(
        &self,
        id: Uuid,
        name: Option<&str>,
        place: Option<&str>,
        url: Option<&str>,
        mmdd: Option<&str>,
    ) -> Result<Option<Evento>, DomainError> {
        // Construir UPDATE dinámico según campos presentes
        let row = sqlx::query_as::<_, EventoRow>(
            r#"
            UPDATE eventos
            SET
                name = COALESCE($2, name),
                place = COALESCE($3, place),
                url = COALESCE($4, url),
                mmdd = COALESCE($5, mmdd)
            WHERE id = $1
            RETURNING id, name, place, mmdd, url, created_at
            "#,
        )
        .bind(id)
        .bind(name)
        .bind(place)
        .bind(url)
        .bind(mmdd)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(row.map(Evento::from))
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM eventos WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(())
    }
}

use crate::domain::{DomainError, Place, PlacesRepository};
use async_trait::async_trait;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct PlaceRow {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub address: String,
    pub location: String,
    pub latitude: f64,
    pub longitude: f64,
    pub instagram: Option<String>,
    pub website: Option<String>,
    pub url: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<PlaceRow> for Place {
    fn from(row: PlaceRow) -> Self {
        Place {
            id: row.id,
            name: row.name,
            description: row.description,
            address: row.address,
            location: row.location,
            latitude: row.latitude,
            longitude: row.longitude,
            instagram: row.instagram,
            website: row.website,
            url: row.url,
            created_at: row.created_at,
        }
    }
}

pub struct PlacesRepositoryImpl {
    pool: sqlx::PgPool,
}

impl PlacesRepositoryImpl {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PlacesRepository for PlacesRepositoryImpl {
    async fn get_all(&self) -> Result<Vec<Place>, DomainError> {
        let rows = sqlx::query_as::<_, PlaceRow>(
            "SELECT id, name, description, address, location, latitude, longitude, instagram, website, url, created_at FROM places ORDER BY created_at DESC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(rows.into_iter().map(Place::from).collect())
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Place>, DomainError> {
        let row = sqlx::query_as::<_, PlaceRow>(
            "SELECT id, name, description, address, location, latitude, longitude, instagram, website, url, created_at FROM places WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(row.map(Place::from))
    }

    async fn create(
        &self,
        name: &str,
        description: &str,
        address: &str,
        location: &str,
        latitude: f64,
        longitude: f64,
        url: &str,
        instagram: Option<&str>,
        website: Option<&str>,
    ) -> Result<Place, DomainError> {
        let row = sqlx::query_as::<_, PlaceRow>(
            r#"
            INSERT INTO places (name, description, address, location, latitude, longitude, url, instagram, website)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, name, description, address, location, latitude, longitude, instagram, website, url, created_at
            "#,
        )
        .bind(name)
        .bind(description)
        .bind(address)
        .bind(location)
        .bind(latitude)
        .bind(longitude)
        .bind(url)
        .bind(instagram)
        .bind(website)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(Place::from(row))
    }

    async fn update(
        &self,
        id: Uuid,
        name: Option<&str>,
        description: Option<&str>,
        address: Option<&str>,
        location: Option<&str>,
        latitude: Option<f64>,
        longitude: Option<f64>,
        url: Option<&str>,
        instagram: Option<&str>,
        website: Option<&str>,
    ) -> Result<Option<Place>, DomainError> {
        // Build dynamic update: only non-None fields
        let row = sqlx::query_as::<_, PlaceRow>(
            r#"
            UPDATE places SET
                name = COALESCE($2, name),
                description = COALESCE($3, description),
                address = COALESCE($4, address),
                location = COALESCE($5, location),
                latitude = COALESCE($6, latitude),
                longitude = COALESCE($7, longitude),
                url = COALESCE($8, url),
                instagram = $9,
                website = $10
            WHERE id = $1
            RETURNING id, name, description, address, location, latitude, longitude, instagram, website, url, created_at
            "#,
        )
        .bind(id)
        .bind(name)
        .bind(description)
        .bind(address)
        .bind(location)
        .bind(latitude)
        .bind(longitude)
        .bind(url)
        .bind(instagram)
        .bind(website)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(row.map(Place::from))
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM places WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::Repository(anyhow::Error::from(e)))?;
        Ok(())
    }
}

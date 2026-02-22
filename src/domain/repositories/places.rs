// Contrato del repositorio de lugares (places)

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::Place;

use super::error::DomainError;

#[async_trait]
pub trait PlacesRepository: Send + Sync {
    async fn get_all(&self) -> Result<Vec<Place>, DomainError>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Place>, DomainError>;
    /// Crea un lugar con id conocido (para guardar la imagen como {id}.{ext}).
    async fn create_with_id(
        &self,
        id: Uuid,
        name: &str,
        description: &str,
        address: &str,
        location: &str,
        latitude: f64,
        longitude: f64,
        url: &str,
        instagram: Option<&str>,
        website: Option<&str>,
    ) -> Result<Place, DomainError>;
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
    ) -> Result<Option<Place>, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}

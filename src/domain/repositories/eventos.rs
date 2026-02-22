// Contrato del repositorio de eventos

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::Evento;

use super::error::DomainError;

#[async_trait]
pub trait EventosRepository: Send + Sync {
    async fn get_all(&self) -> Result<Vec<Evento>, DomainError>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Evento>, DomainError>;
    async fn create(
        &self,
        name: &str,
        place: &str,
        url: &str,
        mmdd: &str,
    ) -> Result<Evento, DomainError>;
    async fn update(
        &self,
        id: Uuid,
        name: Option<&str>,
        place: Option<&str>,
        url: Option<&str>,
        mmdd: Option<&str>,
    ) -> Result<Option<Evento>, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}

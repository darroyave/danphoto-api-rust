// Contrato del repositorio de usuarios (perfil: get/update, sin password)

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::Usuario;

use super::error::DomainError;

#[async_trait]
pub trait UsuariosRepository: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Usuario>, DomainError>;
    async fn update_name(&self, id: Uuid, name: Option<&str>) -> Result<Option<Usuario>, DomainError>;
    async fn update_avatar(&self, id: Uuid, url: &str) -> Result<Option<Usuario>, DomainError>;
}

// Contrato del repositorio de sesiones (agrupaciones de poses)

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{Pose, Sesion};

use super::error::DomainError;

#[async_trait]
pub trait SesionesRepository: Send + Sync {
    async fn get_all(&self) -> Result<Vec<Sesion>, DomainError>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Sesion>, DomainError>;
    async fn get_poses_by_sesion(&self, sesion_id: Uuid) -> Result<Vec<Pose>, DomainError>;
    async fn create(&self, name: &str) -> Result<Sesion, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
    /// Añade poses a una sesión (tabla sesion_image). Idempotente.
    async fn add_poses_to_sesion(
        &self,
        sesion_id: Uuid,
        pose_ids: &[Uuid],
    ) -> Result<(), DomainError>;
    async fn remove_pose_from_sesion(
        &self,
        sesion_id: Uuid,
        pose_id: Uuid,
    ) -> Result<(), DomainError>;
    async fn update_cover(&self, sesion_id: Uuid, cover_url: &str)
        -> Result<Option<Sesion>, DomainError>;
}

// Contrato del repositorio de poses

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::Pose;

use super::error::DomainError;

#[async_trait]
pub trait PosesRepository: Send + Sync {
    async fn get_all(&self) -> Result<Vec<Pose>, DomainError>;
    async fn get_paginated(&self, page: u32, limit: u32) -> Result<Vec<Pose>, DomainError>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Pose>, DomainError>;
    /// Crea una pose con id conocido (para guardar la imagen con ese id como nombre de archivo).
    async fn create_with_id(&self, id: Uuid, url: &str) -> Result<Pose, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}

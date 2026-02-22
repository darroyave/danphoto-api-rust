// Contrato del repositorio de portfolio (categorías e imágenes)

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{PortfolioCategory, PortfolioImage};

use super::error::DomainError;

#[async_trait]
pub trait PortfolioRepository: Send + Sync {
    async fn get_categories(&self) -> Result<Vec<PortfolioCategory>, DomainError>;
    async fn get_images_by_category(
        &self,
        category_id: Uuid,
        page: u32,
        limit: u32,
    ) -> Result<Vec<PortfolioImage>, DomainError>;
    async fn create_category(&self, name: &str) -> Result<PortfolioCategory, DomainError>;
    async fn update_category(&self, id: Uuid, name: &str) -> Result<Option<PortfolioCategory>, DomainError>;
    async fn delete_category(&self, id: Uuid) -> Result<(), DomainError>;
    /// Añade una imagen con id conocido (para guardar el archivo como {id}.{ext}).
    async fn add_image_with_id(
        &self,
        id: Uuid,
        category_id: Uuid,
        url: &str,
    ) -> Result<PortfolioImage, DomainError>;
    async fn delete_image(&self, id: Uuid) -> Result<(), DomainError>;
}

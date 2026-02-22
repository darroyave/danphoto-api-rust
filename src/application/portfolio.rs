// Casos de uso de Portfolio (Kotlin domain/cases/portfolio)

use crate::domain::{
    DomainError, PortfolioCategory, PortfolioImage, PortfolioRepository,
};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct GetPortfolioCategoriesUseCase {
    repo: Arc<dyn PortfolioRepository>,
}

impl GetPortfolioCategoriesUseCase {
    pub fn new(repo: Arc<dyn PortfolioRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self) -> Result<Vec<PortfolioCategory>, DomainError> {
        self.repo.get_categories().await
    }
}

#[derive(Clone)]
pub struct GetPortfolioImagesByCategoryUseCase {
    repo: Arc<dyn PortfolioRepository>,
}

impl GetPortfolioImagesByCategoryUseCase {
    pub fn new(repo: Arc<dyn PortfolioRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        category_id: Uuid,
        page: u32,
        limit: u32,
    ) -> Result<Vec<PortfolioImage>, DomainError> {
        self.repo.get_images_by_category(category_id, page, limit).await
    }
}

#[derive(Clone)]
pub struct CreatePortfolioCategoryUseCase {
    repo: Arc<dyn PortfolioRepository>,
}

impl CreatePortfolioCategoryUseCase {
    pub fn new(repo: Arc<dyn PortfolioRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, name: &str) -> Result<PortfolioCategory, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::Validation("El nombre es requerido".to_string()));
        }
        self.repo.create_category(name).await
    }
}

#[derive(Clone)]
pub struct UpdatePortfolioCategoryUseCase {
    repo: Arc<dyn PortfolioRepository>,
}

impl UpdatePortfolioCategoryUseCase {
    pub fn new(repo: Arc<dyn PortfolioRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid, name: &str) -> Result<PortfolioCategory, DomainError> {
        self.repo
            .update_category(id, name)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Categoría no encontrada: {}", id)))
    }
}

#[derive(Clone)]
pub struct DeletePortfolioCategoryUseCase {
    repo: Arc<dyn PortfolioRepository>,
}

impl DeletePortfolioCategoryUseCase {
    pub fn new(repo: Arc<dyn PortfolioRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid) -> Result<(), DomainError> {
        self.repo.delete_category(id).await
    }
}

#[derive(Clone)]
pub struct AddPortfolioImageUseCase {
    repo: Arc<dyn PortfolioRepository>,
}

impl AddPortfolioImageUseCase {
    pub fn new(repo: Arc<dyn PortfolioRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        category_id: Uuid,
        url: &str,
    ) -> Result<PortfolioImage, DomainError> {
        if url.trim().is_empty() {
            return Err(DomainError::Validation("La URL es requerida".to_string()));
        }
        self.repo.add_image(category_id, url).await
    }
}

#[derive(Clone)]
pub struct DeletePortfolioImageUseCase {
    repo: Arc<dyn PortfolioRepository>,
}

impl DeletePortfolioImageUseCase {
    pub fn new(repo: Arc<dyn PortfolioRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid) -> Result<(), DomainError> {
        self.repo.delete_image(id).await
    }
}

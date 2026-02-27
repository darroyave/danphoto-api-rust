use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePortfolioCategoryRequest {
    pub name: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdatePortfolioCategoryRequest {
    pub name: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdatePortfolioCoverRequest {
    pub cover_url: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PortfolioCategoryResponse {
    pub id: Uuid,
    pub name: String,
    pub cover_url: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AddPortfolioImageRequest {
    /// Imagen en base64 (acepta prefijo `data:image/xxx;base64,` o solo el payload).
    pub image_base64: String,
}

/// Respuesta de una imagen del portfolio.
#[derive(Debug, Serialize, ToSchema)]
pub struct PortfolioImageResponse {
    pub id: Uuid,
    pub portfolio_category_id: Uuid,
    pub url: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Respuesta paginada de imágenes de una categoría del portfolio (GET /api/portfolio/categories/{category_id}/images).
#[derive(Debug, Serialize, ToSchema)]
pub struct PortfolioImagesPaginatedResponse {
    pub items: Vec<PortfolioImageResponse>,
    /// Total de imágenes en la categoría.
    pub count: u64,
    /// Página actual (0-based).
    pub page: u32,
    /// Tamaño de página usado.
    pub limit: u32,
    /// Total de páginas.
    pub total_pages: u32,
}

impl From<crate::domain::PortfolioCategory> for PortfolioCategoryResponse {
    fn from(c: crate::domain::PortfolioCategory) -> Self {
        PortfolioCategoryResponse {
            id: c.id,
            name: c.name,
            cover_url: c.cover_url,
        }
    }
}

impl From<crate::domain::PortfolioImage> for PortfolioImageResponse {
    fn from(i: crate::domain::PortfolioImage) -> Self {
        PortfolioImageResponse {
            id: i.id,
            portfolio_category_id: i.portfolio_category_id,
            url: i.url,
            created_at: i.created_at,
        }
    }
}

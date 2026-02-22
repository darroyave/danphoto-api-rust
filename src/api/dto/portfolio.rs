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

#[derive(Debug, Serialize, ToSchema)]
pub struct PortfolioCategoryResponse {
    pub id: Uuid,
    pub name: String,
    pub cover_url: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AddPortfolioImageRequest {
    pub url: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PortfolioImageResponse {
    pub id: Uuid,
    pub portfolio_category_id: Uuid,
    pub url: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
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

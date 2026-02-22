use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioCategory {
    pub id: Uuid,
    pub name: String,
    pub cover_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioImage {
    pub id: Uuid,
    pub portfolio_category_id: Uuid,
    pub url: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

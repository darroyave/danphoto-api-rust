use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Place {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub address: String,
    pub location: String,
    pub latitude: f64,
    pub longitude: f64,
    pub instagram: Option<String>,
    pub website: Option<String>,
    pub url: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

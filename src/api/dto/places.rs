// DTOs de places (lugares)

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePlaceRequest {
    pub name: String,
    pub description: String,
    pub address: String,
    pub location: String,
    pub latitude: f64,
    pub longitude: f64,
    pub url: String,
    pub instagram: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdatePlaceRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub address: Option<String>,
    pub location: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub url: Option<String>,
    pub instagram: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PlaceResponse {
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

impl From<crate::domain::Place> for PlaceResponse {
    fn from(p: crate::domain::Place) -> Self {
        PlaceResponse {
            id: p.id,
            name: p.name,
            description: p.description,
            address: p.address,
            location: p.location,
            latitude: p.latitude,
            longitude: p.longitude,
            instagram: p.instagram,
            website: p.website,
            url: p.url,
            created_at: p.created_at,
        }
    }
}

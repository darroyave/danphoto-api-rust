// DTOs de sesiones (agrupaciones de poses)

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateSesionRequest {
    pub name: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AddPosesToSesionRequest {
    pub pose_ids: Vec<Uuid>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateSesionFromFavoritesRequest {
    pub name: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateSesionCoverRequest {
    pub cover_url: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SesionResponse {
    pub id: Uuid,
    pub name: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub cover_url: String,
}

impl From<crate::domain::Sesion> for SesionResponse {
    fn from(s: crate::domain::Sesion) -> Self {
        SesionResponse {
            id: s.id,
            name: s.name,
            created_at: s.created_at,
            cover_url: s.cover_url,
        }
    }
}

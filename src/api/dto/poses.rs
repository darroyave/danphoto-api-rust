use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePoseRequest {
    /// Imagen en base64 (acepta prefijo `data:image/xxx;base64,` o solo el payload).
    pub image_base64: String,
    /// IDs de hashtags a asociar a la pose (se insertan en la tabla de relación pose-hashtag).
    #[serde(default)]
    pub hashtag_ids: Option<Vec<Uuid>>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PoseResponse {
    pub id: Uuid,
    pub url: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Respuesta paginada de poses (GET /api/poses/paginated y GET /api/hashtags/{hashtag_id}/poses/paginated).
#[derive(Debug, Serialize, ToSchema)]
pub struct PosesPaginatedResponse {
    pub items: Vec<PoseResponse>,
    pub count: u64,
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdatePoseHashtagsRequest {
    pub hashtag_ids: Vec<Uuid>,
}

impl From<crate::domain::Pose> for PoseResponse {
    fn from(p: crate::domain::Pose) -> Self {
        PoseResponse {
            id: p.id,
            url: p.url,
            created_at: p.created_at,
        }
    }
}

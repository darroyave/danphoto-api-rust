use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePoseRequest {
    /// Imagen en base64 (acepta prefijo `data:image/xxx;base64,` o solo el payload).
    pub image_base64: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PoseResponse {
    pub id: Uuid,
    pub url: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
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

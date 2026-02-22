use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePoseRequest {
    pub name: Option<String>,
    pub url: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PoseResponse {
    pub id: Uuid,
    pub name: Option<String>,
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
            name: p.name,
            url: p.url,
            created_at: p.created_at,
        }
    }
}

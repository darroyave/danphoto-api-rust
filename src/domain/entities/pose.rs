use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pose {
    pub id: Uuid,
    pub url: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Favorito {
    pub id: Uuid,
    pub pose_id: Uuid,
    pub user_id: Uuid,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

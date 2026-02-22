use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sesion {
    pub id: Uuid,
    pub name: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub cover_url: String,
}

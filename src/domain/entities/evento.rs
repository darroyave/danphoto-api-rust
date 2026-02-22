use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evento {
    pub id: Uuid,
    pub name: String,
    pub place: String,
    pub mmdd: String,
    pub url: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub description: Option<String>,
    pub url: Option<String>,
    pub user_id: Option<Uuid>,
    pub theme_of_the_day_id: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePostRequest {
    pub description: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PostResponse {
    pub id: Uuid,
    pub description: Option<String>,
    pub url: Option<String>,
    pub user_id: Option<Uuid>,
    pub theme_of_the_day_id: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<crate::domain::Post> for PostResponse {
    fn from(p: crate::domain::Post) -> Self {
        PostResponse {
            id: p.id,
            description: p.description,
            url: p.url,
            user_id: p.user_id,
            theme_of_the_day_id: p.theme_of_the_day_id,
            created_at: p.created_at,
        }
    }
}

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePostRequest {
    pub description: Option<String>,
    /// Imagen en base64 (acepta prefijo `data:image/xxx;base64,` o solo el payload). Si se envía, la URL del post será /api/posts/{id}/image.
    pub image_base64: String,
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

/// Respuesta paginada de posts (GET /api/posts/paginated).
#[derive(Debug, Serialize, ToSchema)]
pub struct PostsPaginatedResponse {
    pub items: Vec<PostResponse>,
    pub count: u64,
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
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

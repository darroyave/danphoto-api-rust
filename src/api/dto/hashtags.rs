// DTOs de hashtags

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateHashtagRequest {
    pub name: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct HashtagResponse {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AddHashtagsToPostRequest {
    pub hashtag_ids: Vec<Uuid>,
}

impl From<crate::domain::Hashtag> for HashtagResponse {
    fn from(h: crate::domain::Hashtag) -> Self {
        HashtagResponse {
            id: h.id,
            name: h.name,
        }
    }
}

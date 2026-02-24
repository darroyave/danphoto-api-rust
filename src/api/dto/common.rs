// DTOs compartidos (error, etc.)

use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
}

/// GET /api/favorites/poses/{pose_id}: indica si la pose está en la tabla de favoritos del usuario.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct IsPoseFavoriteResponse {
    pub is_favorite: bool,
}

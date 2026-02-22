// DTOs de usuario/perfil (sin password)

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateUsuarioRequest {
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateUsuarioAvatarRequest {
    /// Imagen del avatar en base64 (acepta prefijo `data:image/xxx;base64,` o solo el payload).
    pub image_base64: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UsuarioResponse {
    pub id: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
    pub url: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<crate::domain::Usuario> for UsuarioResponse {
    fn from(u: crate::domain::Usuario) -> Self {
        UsuarioResponse {
            id: u.id,
            name: u.name,
            email: u.email,
            url: u.url,
            created_at: u.created_at,
        }
    }
}

// DTOs de eventos

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateEventoRequest {
    pub name: String,
    pub place: String,
    pub url: String,
    /// Fecha en formato MMdd (ej: "1024")
    pub mmdd: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateEventoRequest {
    pub name: Option<String>,
    pub place: Option<String>,
    pub url: Option<String>,
    pub mmdd: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EventoResponse {
    pub id: Uuid,
    pub name: String,
    pub place: String,
    pub mmdd: String,
    pub url: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<crate::domain::Evento> for EventoResponse {
    fn from(e: crate::domain::Evento) -> Self {
        EventoResponse {
            id: e.id,
            name: e.name,
            place: e.place,
            mmdd: e.mmdd,
            url: e.url,
            created_at: e.created_at,
        }
    }
}

// DTOs de theme of the day

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateThemeOfTheDayRequest {
    /// Id en formato MMdd (4 caracteres, ej: "1024")
    pub id: String,
    pub name: String,
    /// Imagen en base64 (puede incluir prefijo data:image/png;base64, o solo los datos).
    pub image_base64: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateThemeOfTheDayRequest {
    pub name: Option<String>,
    /// Si se envía, reemplaza la imagen (base64).
    pub image_base64: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ThemeOfTheDayResponse {
    pub id: String,
    pub name: String,
    pub url: String,
}

impl From<crate::domain::ThemeOfTheDay> for ThemeOfTheDayResponse {
    fn from(t: crate::domain::ThemeOfTheDay) -> Self {
        ThemeOfTheDayResponse {
            id: t.id,
            name: t.name,
            url: t.url,
        }
    }
}

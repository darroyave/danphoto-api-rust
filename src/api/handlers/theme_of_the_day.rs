use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use base64::Engine;
use std::path::Path as StdPath;
use std::sync::Arc;

use crate::api::{
    dto::{
        CreateThemeOfTheDayRequest, ErrorResponse, ThemeOfTheDayResponse,
        UpdateThemeOfTheDayRequest,
    },
    state::AppState,
    ApiError,
};
use crate::application::{
    CreateThemeOfTheDayUseCase, DeleteThemeOfTheDayUseCase, GetThemeOfTheDayAllUseCase,
    GetThemeOfTheDayByIdUseCase, GetThemeOfTheDayTodayUseCase, UpdateThemeOfTheDayUseCase,
};

/// Decodifica imagen base64 (acepta prefijo data:image/xxx;base64,) y la guarda en dir/{id}.{ext}.
/// Devuelve la URL que debe guardarse en BD: /api/theme-of-the-day/{id}/image.
fn save_theme_image_base64(
    dir: &str,
    id: &str,
    image_base64: &str,
) -> Result<String, ApiError> {
    let (payload, ext) = if let Some(rest) = image_base64.strip_prefix("data:") {
        let (mime, b64) = rest
            .split_once(";base64,")
            .ok_or_else(|| ApiError(crate::domain::DomainError::Validation("formato base64 inválido: se esperaba data:image/...;base64,...".to_string())))?;
        let ext = if mime.trim().to_lowercase().starts_with("image/png") {
            "png"
        } else {
            "jpg"
        };
        (b64.trim(), ext)
    } else {
        (image_base64.trim(), "jpg")
    };

    let bytes = base64::engine::general_purpose::STANDARD
        .decode(payload)
        .map_err(|e| ApiError(crate::domain::DomainError::Validation(format!("base64 inválido: {}", e))))?;
    if bytes.is_empty() {
        return Err(ApiError(crate::domain::DomainError::Validation("imagen vacía".to_string())));
    }

    std::fs::create_dir_all(dir).map_err(|e| ApiError(crate::domain::DomainError::Repository(anyhow::Error::from(e))))?;
    let filename = format!("{}.{}", id, ext);
    let path = StdPath::new(dir).join(&filename);
    std::fs::write(&path, &bytes).map_err(|e| ApiError(crate::domain::DomainError::Repository(anyhow::Error::from(e))))?;

    Ok(format!("/api/theme-of-the-day/{}/image", id))
}

/// Obtiene el tema del día de hoy (id = MMdd de la fecha actual). Equivalente a Kotlin getThemeOfTheDay().
#[utoipa::path(
    get,
    path = "/api/theme-of-the-day/today",
    tag = "theme_of_the_day",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Tema del día de hoy", body = ThemeOfTheDayResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 404, description = "No hay tema definido para hoy", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn get_theme_of_the_day_today(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
) -> Result<Json<ThemeOfTheDayResponse>, ApiError> {
    let uc = GetThemeOfTheDayTodayUseCase::new(Arc::clone(&state.theme_of_the_day_repo));
    let item = uc.execute().await?;
    Ok(Json(ThemeOfTheDayResponse::from(item)))
}

/// Lista todos los temas del día (requiere Bearer token).
#[utoipa::path(
    get,
    path = "/api/theme-of-the-day",
    tag = "theme_of_the_day",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Lista de temas del día", body = [ThemeOfTheDayResponse]),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn list_theme_of_the_day(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
) -> Result<Json<Vec<ThemeOfTheDayResponse>>, ApiError> {
    let uc = GetThemeOfTheDayAllUseCase::new(Arc::clone(&state.theme_of_the_day_repo));
    let items = uc.execute().await?;
    Ok(Json(items.into_iter().map(ThemeOfTheDayResponse::from).collect()))
}

/// Obtiene un tema del día por id (MMdd) (requiere Bearer token).
#[utoipa::path(
    get,
    path = "/api/theme-of-the-day/{id}",
    tag = "theme_of_the_day",
    security(("bearer_auth" = [])),
    params(("id" = String, Path, description = "Id del tema (MMdd, 4 caracteres)")),
    responses(
        (status = 200, description = "Tema del día encontrado", body = ThemeOfTheDayResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 404, description = "Tema no encontrado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn get_theme_of_the_day(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ThemeOfTheDayResponse>, ApiError> {
    let uc = GetThemeOfTheDayByIdUseCase::new(Arc::clone(&state.theme_of_the_day_repo));
    let item = uc.execute(&id).await?;
    Ok(Json(ThemeOfTheDayResponse::from(item)))
}

/// Crea un nuevo tema del día con imagen en base64 (requiere Bearer token).
#[utoipa::path(
    post,
    path = "/api/theme-of-the-day",
    tag = "theme_of_the_day",
    security(("bearer_auth" = [])),
    request_body = CreateThemeOfTheDayRequest,
    responses(
        (status = 200, description = "Tema del día creado", body = ThemeOfTheDayResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 400, description = "Validación fallida (id debe ser 4 caracteres MMdd, base64 inválido)", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn create_theme_of_the_day(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Json(body): Json<CreateThemeOfTheDayRequest>,
) -> Result<Json<ThemeOfTheDayResponse>, ApiError> {
    let url = save_theme_image_base64(
        &state.theme_of_the_day_images_dir,
        &body.id,
        &body.image_base64,
    )?;
    let uc = CreateThemeOfTheDayUseCase::new(Arc::clone(&state.theme_of_the_day_repo));
    let item = uc.execute(&body.id, &body.name, &url).await?;
    Ok(Json(ThemeOfTheDayResponse::from(item)))
}

/// Actualiza un tema del día existente (requiere Bearer token).
#[utoipa::path(
    put,
    path = "/api/theme-of-the-day/{id}",
    tag = "theme_of_the_day",
    security(("bearer_auth" = [])),
    params(("id" = String, Path, description = "Id del tema (MMdd)")),
    request_body = UpdateThemeOfTheDayRequest,
    responses(
        (status = 200, description = "Tema del día actualizado", body = ThemeOfTheDayResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 404, description = "Tema no encontrado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn update_theme_of_the_day(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<UpdateThemeOfTheDayRequest>,
) -> Result<Json<ThemeOfTheDayResponse>, ApiError> {
    let url = if let Some(ref img) = body.image_base64 {
        Some(save_theme_image_base64(
            &state.theme_of_the_day_images_dir,
            &id,
            img,
        )?)
    } else {
        None
    };
    let uc = UpdateThemeOfTheDayUseCase::new(Arc::clone(&state.theme_of_the_day_repo));
    let item = uc
        .execute(&id, body.name.as_deref(), url.as_deref())
        .await?;
    Ok(Json(ThemeOfTheDayResponse::from(item)))
}

/// Sirve la imagen del tema del día (público para que el front pueda usar la url del response).
#[utoipa::path(
    get,
    path = "/api/theme-of-the-day/{id}/image",
    tag = "theme_of_the_day",
    params(("id" = String, Path, description = "Id del tema (MMdd)")),
    responses(
        (status = 200, description = "Imagen del tema", content_type = "image/*"),
        (status = 404, description = "Imagen no encontrada"),
    ),
)]
pub async fn get_theme_of_the_day_image(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let dir = StdPath::new(&state.theme_of_the_day_images_dir);
    for ext in ["png", "jpg", "jpeg"] {
        let path = dir.join(format!("{}.{}", id, ext));
        if path.exists() {
            let bytes = std::fs::read(&path)
                .map_err(|e| ApiError(crate::domain::DomainError::Repository(anyhow::Error::from(e))))?;
            let content_type = if ext == "png" {
                "image/png"
            } else {
                "image/jpeg"
            };
            return Ok((
                StatusCode::OK,
                [(header::CONTENT_TYPE, content_type)],
                Body::from(bytes),
            ));
        }
    }
    Err(ApiError(crate::domain::DomainError::NotFound(format!(
        "Imagen no encontrada para el tema {}",
        id
    ))))
}

/// Elimina un tema del día (requiere Bearer token).
#[utoipa::path(
    delete,
    path = "/api/theme-of-the-day/{id}",
    tag = "theme_of_the_day",
    security(("bearer_auth" = [])),
    params(("id" = String, Path, description = "Id del tema (MMdd)")),
    responses(
        (status = 204, description = "Tema del día eliminado"),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 404, description = "Tema no encontrado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn delete_theme_of_the_day(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<axum::http::StatusCode, ApiError> {
    let uc = DeleteThemeOfTheDayUseCase::new(Arc::clone(&state.theme_of_the_day_repo));
    uc.execute(&id).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

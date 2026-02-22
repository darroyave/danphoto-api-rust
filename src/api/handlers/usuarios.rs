// Handlers de Usuario/Perfil. user_id desde JWT (solo el propio perfil).

use axum::{
    body::Body,
    extract::State,
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use base64::Engine;
use std::path::Path as StdPath;
use std::sync::Arc;

use crate::api::{
    dto::{ErrorResponse, UpdateUsuarioAvatarRequest, UpdateUsuarioRequest, UsuarioResponse},
    state::AppState,
    ApiError,
};
use crate::api::auth::{user_id_from_auth, BearerAuth};
use crate::application::{
    GetProfileUseCase, UpdateUsuarioAvatarUseCase, UpdateUsuarioUseCase,
};

/// Obtiene el perfil del usuario autenticado (datos sin password).
#[utoipa::path(
    get,
    path = "/api/profile",
    tag = "usuario",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Perfil del usuario", body = UsuarioResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 404, description = "Usuario no encontrado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn get_profile(
    auth: BearerAuth,
    State(state): State<AppState>,
) -> Result<Json<UsuarioResponse>, ApiError> {
    let user_id = user_id_from_auth(&state, &auth.0).await?;
    let uc = GetProfileUseCase::new(Arc::clone(&state.usuarios_repo));
    let user = uc.execute(user_id).await?;
    let user = user.ok_or_else(|| {
        ApiError(crate::domain::DomainError::NotFound(
            "Usuario no encontrado".to_string(),
        ))
    })?;
    Ok(Json(UsuarioResponse::from(user)))
}

/// Actualiza el nombre del usuario autenticado.
#[utoipa::path(
    put,
    path = "/api/profile",
    tag = "usuario",
    security(("bearer_auth" = [])),
    request_body = UpdateUsuarioRequest,
    responses(
        (status = 200, description = "Perfil actualizado", body = UsuarioResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 404, description = "Usuario no encontrado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn update_profile(
    auth: BearerAuth,
    State(state): State<AppState>,
    Json(body): Json<UpdateUsuarioRequest>,
) -> Result<Json<UsuarioResponse>, ApiError> {
    let user_id = user_id_from_auth(&state, &auth.0).await?;
    let uc = UpdateUsuarioUseCase::new(Arc::clone(&state.usuarios_repo));
    let user = uc.execute(user_id, body.name.as_deref()).await?;
    let user = user.ok_or_else(|| {
        ApiError(crate::domain::DomainError::NotFound(
            "Usuario no encontrado".to_string(),
        ))
    })?;
    Ok(Json(UsuarioResponse::from(user)))
}

/// Guarda avatar en base64 como {user_id}.{ext} y actualiza la URL del usuario a /api/profile/avatar.
fn save_profile_avatar_base64(
    dir: &str,
    user_id: &uuid::Uuid,
    image_base64: &str,
) -> Result<(), ApiError> {
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
    let filename = format!("{}.{}", user_id, ext);
    let path = StdPath::new(dir).join(&filename);
    std::fs::write(&path, &bytes).map_err(|e| ApiError(crate::domain::DomainError::Repository(anyhow::Error::from(e))))?;
    Ok(())
}

/// Actualiza el avatar (imagen base64) del usuario autenticado. Se guarda en disco; GET /api/profile/avatar sirve la imagen.
#[utoipa::path(
    put,
    path = "/api/profile/avatar",
    tag = "usuario",
    security(("bearer_auth" = [])),
    request_body = UpdateUsuarioAvatarRequest,
    responses(
        (status = 200, description = "Avatar actualizado", body = UsuarioResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 400, description = "Imagen base64 vacía o inválida", body = ErrorResponse),
        (status = 404, description = "Usuario no encontrado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn update_profile_avatar(
    auth: BearerAuth,
    State(state): State<AppState>,
    Json(body): Json<UpdateUsuarioAvatarRequest>,
) -> Result<Json<UsuarioResponse>, ApiError> {
    if body.image_base64.trim().is_empty() {
        return Err(ApiError(crate::domain::DomainError::Validation(
            "image_base64 es requerido".to_string(),
        )));
    }
    let user_id = user_id_from_auth(&state, &auth.0).await?;
    save_profile_avatar_base64(&state.profile_avatars_dir, &user_id, &body.image_base64)?;
    let uc = UpdateUsuarioAvatarUseCase::new(Arc::clone(&state.usuarios_repo));
    let user = uc.execute(user_id, "/api/profile/avatar").await?;
    let user = user.ok_or_else(|| {
        ApiError(crate::domain::DomainError::NotFound(
            "Usuario no encontrado".to_string(),
        ))
    })?;
    Ok(Json(UsuarioResponse::from(user)))
}

/// Sirve el avatar del usuario autenticado (imagen guardada como {user_id}.{ext}).
#[utoipa::path(
    get,
    path = "/api/profile/avatar",
    tag = "usuario",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Avatar del usuario", content_type = "image/*"),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 404, description = "Avatar no encontrado", body = ErrorResponse),
    ),
)]
pub async fn get_profile_avatar(
    auth: BearerAuth,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    let user_id = user_id_from_auth(&state, &auth.0).await?;
    let dir = StdPath::new(&state.profile_avatars_dir);
    for ext in ["png", "jpg", "jpeg"] {
        let path = dir.join(format!("{}.{}", user_id, ext));
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
    Err(ApiError(crate::domain::DomainError::NotFound(
        "Avatar no encontrado".to_string(),
    )))
}

// Handlers de Usuario/Perfil. user_id desde JWT (solo el propio perfil).

use axum::{extract::State, Json};
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

/// Actualiza el avatar (url) del usuario autenticado.
#[utoipa::path(
    put,
    path = "/api/profile/avatar",
    tag = "usuario",
    security(("bearer_auth" = [])),
    request_body = UpdateUsuarioAvatarRequest,
    responses(
        (status = 200, description = "Avatar actualizado", body = UsuarioResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 404, description = "Usuario no encontrado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn update_profile_avatar(
    auth: BearerAuth,
    State(state): State<AppState>,
    Json(body): Json<UpdateUsuarioAvatarRequest>,
) -> Result<Json<UsuarioResponse>, ApiError> {
    let user_id = user_id_from_auth(&state, &auth.0).await?;
    let uc = UpdateUsuarioAvatarUseCase::new(Arc::clone(&state.usuarios_repo));
    let user = uc.execute(user_id, &body.url).await?;
    let user = user.ok_or_else(|| {
        ApiError(crate::domain::DomainError::NotFound(
            "Usuario no encontrado".to_string(),
        ))
    })?;
    Ok(Json(UsuarioResponse::from(user)))
}

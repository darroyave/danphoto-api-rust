// Handlers de Sesiones. user_id desde JWT para AddFavoritesToSesion y CreateSesionFromFavorites.

use axum::{
    extract::{Path, State},
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::api::{
    dto::{
        AddPosesToSesionRequest, CreateSesionFromFavoritesRequest, CreateSesionRequest,
        ErrorResponse, PoseResponse, SesionResponse, UpdateSesionCoverRequest,
    },
    state::AppState,
    ApiError,
};
use crate::application::{
    AddFavoritesToSesionUseCase, AddPosesToSesionUseCase, CreateSesionFromFavoritesUseCase,
    CreateSesionUseCase, DeleteSesionUseCase, GetPosesBySesionUseCase, GetSesionByIdUseCase,
    GetSesionesUseCase, RemovePoseFromSesionUseCase, UpdateSesionCoverUseCase,
};
use crate::api::auth::{user_id_from_auth, BearerAuth};

/// Lista todas las sesiones.
#[utoipa::path(
    get,
    path = "/api/sesiones",
    tag = "sesiones",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Lista de sesiones", body = [SesionResponse]),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn list_sesiones(
    _auth: BearerAuth,
    State(state): State<AppState>,
) -> Result<Json<Vec<SesionResponse>>, ApiError> {
    let uc = GetSesionesUseCase::new(Arc::clone(&state.sesiones_repo));
    let items = uc.execute().await?;
    Ok(Json(items.into_iter().map(SesionResponse::from).collect()))
}

/// Obtiene una sesión por ID.
#[utoipa::path(
    get,
    path = "/api/sesiones/{id}",
    tag = "sesiones",
    security(("bearer_auth" = [])),
    params(("id" = Uuid, Path, description = "UUID de la sesión")),
    responses(
        (status = 200, description = "Sesión encontrada", body = SesionResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 404, description = "Sesión no encontrada", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn get_sesion(
    _auth: BearerAuth,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<SesionResponse>, ApiError> {
    let uc = GetSesionByIdUseCase::new(Arc::clone(&state.sesiones_repo));
    let sesion = uc.execute(id).await?;
    let sesion = sesion.ok_or_else(|| {
        ApiError(crate::domain::DomainError::NotFound(
            "Sesión no encontrada".to_string(),
        ))
    })?;
    Ok(Json(SesionResponse::from(sesion)))
}

/// Lista las poses de una sesión.
#[utoipa::path(
    get,
    path = "/api/sesiones/{id}/poses",
    tag = "sesiones",
    security(("bearer_auth" = [])),
    params(("id" = Uuid, Path, description = "UUID de la sesión")),
    responses(
        (status = 200, description = "Poses de la sesión", body = [PoseResponse]),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn get_poses_by_sesion(
    _auth: BearerAuth,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<PoseResponse>>, ApiError> {
    let uc = GetPosesBySesionUseCase::new(Arc::clone(&state.sesiones_repo));
    let poses = uc.execute(id).await?;
    Ok(Json(poses.into_iter().map(PoseResponse::from).collect()))
}

/// Crea una sesión nueva (vacía).
#[utoipa::path(
    post,
    path = "/api/sesiones",
    tag = "sesiones",
    security(("bearer_auth" = [])),
    request_body = CreateSesionRequest,
    responses(
        (status = 200, description = "Sesión creada", body = SesionResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn create_sesion(
    _auth: BearerAuth,
    State(state): State<AppState>,
    Json(body): Json<CreateSesionRequest>,
) -> Result<Json<SesionResponse>, ApiError> {
    let uc = CreateSesionUseCase::new(Arc::clone(&state.sesiones_repo));
    let sesion = uc.execute(&body.name).await?;
    Ok(Json(SesionResponse::from(sesion)))
}

/// Crea una sesión con el nombre dado y mueve las poses favoritas del usuario a ella.
#[utoipa::path(
    post,
    path = "/api/sesiones/from-favorites",
    tag = "sesiones",
    security(("bearer_auth" = [])),
    request_body = CreateSesionFromFavoritesRequest,
    responses(
        (status = 200, description = "Sesión creada con favoritos", body = SesionResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn create_sesion_from_favorites(
    auth: BearerAuth,
    State(state): State<AppState>,
    Json(body): Json<CreateSesionFromFavoritesRequest>,
) -> Result<Json<SesionResponse>, ApiError> {
    let user_id = user_id_from_auth(&state, &auth.0).await?;
    let uc = CreateSesionFromFavoritesUseCase::new(
        Arc::clone(&state.sesiones_repo),
        Arc::clone(&state.favorites_repo),
    );
    let sesion = uc.execute(user_id, &body.name).await?;
    Ok(Json(SesionResponse::from(sesion)))
}

/// Añade poses a una sesión (por IDs).
#[utoipa::path(
    post,
    path = "/api/sesiones/{id}/poses",
    tag = "sesiones",
    security(("bearer_auth" = [])),
    params(("id" = Uuid, Path, description = "UUID de la sesión")),
    request_body = AddPosesToSesionRequest,
    responses(
        (status = 204, description = "Poses añadidas"),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 404, description = "Sesión no encontrada", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn add_poses_to_sesion(
    _auth: BearerAuth,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<AddPosesToSesionRequest>,
) -> Result<axum::http::StatusCode, ApiError> {
    let uc = AddPosesToSesionUseCase::new(Arc::clone(&state.sesiones_repo));
    uc.execute(id, &body.pose_ids).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

/// Mueve los favoritos del usuario a una sesión existente (y los quita de favoritos).
#[utoipa::path(
    post,
    path = "/api/sesiones/{id}/add-favorites",
    tag = "sesiones",
    security(("bearer_auth" = [])),
    params(("id" = Uuid, Path, description = "UUID de la sesión")),
    responses(
        (status = 204, description = "Favoritos añadidos a la sesión"),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 404, description = "Sesión no encontrada", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn add_favorites_to_sesion(
    auth: BearerAuth,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, ApiError> {
    let user_id = user_id_from_auth(&state, &auth.0).await?;
    let uc = AddFavoritesToSesionUseCase::new(
        Arc::clone(&state.sesiones_repo),
        Arc::clone(&state.favorites_repo),
    );
    uc.execute(user_id, id).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

/// Quita una pose de una sesión.
#[utoipa::path(
    delete,
    path = "/api/sesiones/{id}/poses/{pose_id}",
    tag = "sesiones",
    security(("bearer_auth" = [])),
    params(("id" = Uuid, Path, description = "UUID de la sesión"), ("pose_id" = Uuid, Path, description = "UUID de la pose")),
    responses(
        (status = 204, description = "Pose quitada de la sesión"),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn remove_pose_from_sesion(
    _auth: BearerAuth,
    State(state): State<AppState>,
    Path((id, pose_id)): Path<(Uuid, Uuid)>,
) -> Result<axum::http::StatusCode, ApiError> {
    let uc = RemovePoseFromSesionUseCase::new(Arc::clone(&state.sesiones_repo));
    uc.execute(id, pose_id).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

/// Actualiza la portada de una sesión.
#[utoipa::path(
    put,
    path = "/api/sesiones/{id}/cover",
    tag = "sesiones",
    security(("bearer_auth" = [])),
    params(("id" = Uuid, Path, description = "UUID de la sesión")),
    request_body = UpdateSesionCoverRequest,
    responses(
        (status = 200, description = "Portada actualizada", body = SesionResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 404, description = "Sesión no encontrada", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn update_sesion_cover(
    _auth: BearerAuth,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateSesionCoverRequest>,
) -> Result<Json<SesionResponse>, ApiError> {
    let uc = UpdateSesionCoverUseCase::new(Arc::clone(&state.sesiones_repo));
    let sesion = uc.execute(id, &body.cover_url).await?;
    let sesion = sesion.ok_or_else(|| {
        ApiError(crate::domain::DomainError::NotFound(
            "Sesión no encontrada".to_string(),
        ))
    })?;
    Ok(Json(SesionResponse::from(sesion)))
}

/// Elimina una sesión.
#[utoipa::path(
    delete,
    path = "/api/sesiones/{id}",
    tag = "sesiones",
    security(("bearer_auth" = [])),
    params(("id" = Uuid, Path, description = "UUID de la sesión")),
    responses(
        (status = 204, description = "Sesión eliminada"),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn delete_sesion(
    _auth: BearerAuth,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, ApiError> {
    let uc = DeleteSesionUseCase::new(Arc::clone(&state.sesiones_repo));
    uc.execute(id).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

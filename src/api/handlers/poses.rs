// Handlers de Poses (Kotlin domain/cases/poses)

use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use base64::Engine;
use std::path::Path as StdPath;
use std::sync::Arc;
use uuid::Uuid;

use crate::api::{
    dto::{CreatePoseRequest, ErrorResponse, PoseResponse, UpdatePoseHashtagsRequest},
    state::AppState,
    ApiError,
};
use crate::application::{
    CreatePoseUseCase, DeletePoseUseCase, GetPoseByIdUseCase, GetPosesByHashtagPaginatedUseCase,
    GetPosesByHashtagUseCase, GetPosesPaginatedUseCase, GetPosesUseCase, UpdatePoseHashtagsUseCase,
};

#[derive(Debug, serde::Deserialize, utoipa::IntoParams)]
pub struct PaginationQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

/// Decodifica imagen base64 (acepta prefijo data:image/xxx;base64,) y la guarda en dir/{id}.{ext}.
/// Devuelve la URL que debe guardarse en BD: /api/poses/{id}/image.
fn save_pose_image_base64(
    dir: &str,
    id: &Uuid,
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

    Ok(format!("/api/poses/{}/image", id))
}

/// Lista todas las poses.
#[utoipa::path(
    get,
    path = "/api/poses",
    tag = "poses",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Lista de poses", body = [PoseResponse]),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn list_poses(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
) -> Result<Json<Vec<PoseResponse>>, ApiError> {
    let uc = GetPosesUseCase::new(Arc::clone(&state.poses_repo));
    let items = uc.execute().await?;
    Ok(Json(items.into_iter().map(PoseResponse::from).collect()))
}

/// Lista poses paginado (?page=0&limit=20).
#[utoipa::path(
    get,
    path = "/api/poses/paginated",
    tag = "poses",
    security(("bearer_auth" = [])),
    params(PaginationQuery),
    responses(
        (status = 200, description = "Lista de poses", body = [PoseResponse]),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn list_poses_paginated(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Query(q): Query<PaginationQuery>,
) -> Result<Json<Vec<PoseResponse>>, ApiError> {
    let page = q.page.unwrap_or(0);
    let limit = q.limit.unwrap_or(20).min(100);
    let uc = GetPosesPaginatedUseCase::new(Arc::clone(&state.poses_repo));
    let items = uc.execute(page, limit).await?;
    Ok(Json(items.into_iter().map(PoseResponse::from).collect()))
}

/// Obtiene una pose por id.
#[utoipa::path(
    get,
    path = "/api/poses/{id}",
    tag = "poses",
    security(("bearer_auth" = [])),
    params(("id" = Uuid, Path, description = "UUID de la pose")),
    responses(
        (status = 200, description = "Pose encontrada", body = PoseResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 404, description = "Pose no encontrada", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn get_pose(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<PoseResponse>, ApiError> {
    let uc = GetPoseByIdUseCase::new(Arc::clone(&state.poses_repo));
    let item = uc.execute(id).await?;
    Ok(Json(PoseResponse::from(item)))
}

/// Crea una pose (name opcional, image_base64 requerida). La imagen se guarda en disco y la URL devuelta es /api/poses/{id}/image.
#[utoipa::path(
    post,
    path = "/api/poses",
    tag = "poses",
    security(("bearer_auth" = [])),
    request_body = CreatePoseRequest,
    responses(
        (status = 200, description = "Pose creada", body = PoseResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 400, description = "Imagen base64 vacía o inválida", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn create_pose(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Json(body): Json<CreatePoseRequest>,
) -> Result<Json<PoseResponse>, ApiError> {
    if body.image_base64.trim().is_empty() {
        return Err(ApiError(crate::domain::DomainError::Validation(
            "image_base64 es requerido".to_string(),
        )));
    }
    let id = Uuid::new_v4();
    let url = save_pose_image_base64(&state.poses_images_dir, &id, &body.image_base64)?;
    let uc = CreatePoseUseCase::new(Arc::clone(&state.poses_repo));
    let item = uc
        .execute_with_id(id, &url)
        .await?;
    Ok(Json(PoseResponse::from(item)))
}

/// Sirve la imagen de una pose (público para que el front pueda usar la url del response).
#[utoipa::path(
    get,
    path = "/api/poses/{id}/image",
    tag = "poses",
    params(("id" = Uuid, Path, description = "UUID de la pose")),
    responses(
        (status = 200, description = "Imagen de la pose", content_type = "image/*"),
        (status = 404, description = "Imagen no encontrada"),
    ),
)]
pub async fn get_pose_image(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let dir = StdPath::new(&state.poses_images_dir);
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
        "Imagen no encontrada para la pose {}",
        id
    ))))
}

/// Elimina una pose (y sus relaciones con hashtags).
#[utoipa::path(
    delete,
    path = "/api/poses/{id}",
    tag = "poses",
    security(("bearer_auth" = [])),
    params(("id" = Uuid, Path, description = "UUID de la pose")),
    responses(
        (status = 204, description = "Pose eliminada"),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 404, description = "Pose no encontrada", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn delete_pose(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, ApiError> {
    let uc = DeletePoseUseCase::new(
        Arc::clone(&state.poses_repo),
        Arc::clone(&state.hashtags_repo),
    );
    uc.execute(id).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

/// Poses etiquetadas con un hashtag.
#[utoipa::path(
    get,
    path = "/api/hashtags/{hashtag_id}/poses",
    tag = "poses",
    security(("bearer_auth" = [])),
    params(("hashtag_id" = Uuid, Path, description = "UUID del hashtag")),
    responses(
        (status = 200, description = "Lista de poses", body = [PoseResponse]),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn get_poses_by_hashtag(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Path(hashtag_id): Path<Uuid>,
) -> Result<Json<Vec<PoseResponse>>, ApiError> {
    let uc = GetPosesByHashtagUseCase::new(Arc::clone(&state.hashtags_repo));
    let items = uc.execute(hashtag_id).await?;
    Ok(Json(items.into_iter().map(PoseResponse::from).collect()))
}

/// Poses etiquetadas con un hashtag (paginado).
#[utoipa::path(
    get,
    path = "/api/hashtags/{hashtag_id}/poses/paginated",
    tag = "poses",
    security(("bearer_auth" = [])),
    params(("hashtag_id" = Uuid, Path), PaginationQuery),
    responses(
        (status = 200, description = "Lista de poses", body = [PoseResponse]),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn get_poses_by_hashtag_paginated(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Path(hashtag_id): Path<Uuid>,
    Query(q): Query<PaginationQuery>,
) -> Result<Json<Vec<PoseResponse>>, ApiError> {
    let page = q.page.unwrap_or(0);
    let limit = q.limit.unwrap_or(20).min(100);
    let uc = GetPosesByHashtagPaginatedUseCase::new(Arc::clone(&state.hashtags_repo));
    let items = uc.execute(hashtag_id, page, limit).await?;
    Ok(Json(items.into_iter().map(PoseResponse::from).collect()))
}

/// Actualiza los hashtags de una pose (reemplaza la lista).
#[utoipa::path(
    put,
    path = "/api/poses/{pose_id}/hashtags",
    tag = "poses",
    security(("bearer_auth" = [])),
    params(("pose_id" = Uuid, Path, description = "UUID de la pose")),
    request_body = UpdatePoseHashtagsRequest,
    responses(
        (status = 204, description = "Hashtags actualizados"),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn update_pose_hashtags(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Path(pose_id): Path<Uuid>,
    Json(body): Json<UpdatePoseHashtagsRequest>,
) -> Result<axum::http::StatusCode, ApiError> {
    let uc = UpdatePoseHashtagsUseCase::new(Arc::clone(&state.hashtags_repo));
    uc.execute(pose_id, &body.hashtag_ids).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

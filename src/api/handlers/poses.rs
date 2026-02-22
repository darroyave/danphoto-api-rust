// Handlers de Poses (Kotlin domain/cases/poses)

use axum::{
    extract::{Path, Query, State},
    Json,
};
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

/// Crea una pose (name opcional, url requerida).
#[utoipa::path(
    post,
    path = "/api/poses",
    tag = "poses",
    security(("bearer_auth" = [])),
    request_body = CreatePoseRequest,
    responses(
        (status = 200, description = "Pose creada", body = PoseResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 400, description = "URL vacía", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn create_pose(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Json(body): Json<CreatePoseRequest>,
) -> Result<Json<PoseResponse>, ApiError> {
    let uc = CreatePoseUseCase::new(Arc::clone(&state.poses_repo));
    let item = uc
        .execute(body.name.as_deref(), &body.url)
        .await?;
    Ok(Json(PoseResponse::from(item)))
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

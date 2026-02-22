// Handlers de Favoritos (Kotlin domain/cases/favorites). user_id desde JWT (email -> auth_repository).

use axum::{
    extract::{Path, State},
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::api::{
    dto::{ErrorResponse, PoseResponse},
    state::AppState,
    ApiError,
};
use crate::api::auth::user_id_from_auth;
use crate::application::{
    AddPoseToFavoritesUseCase, GetFavoritePosesUseCase, IsPoseFavoriteUseCase,
    RemovePoseFromFavoritesUseCase,
};

/// Lista poses favoritas del usuario (JWT).
#[utoipa::path(
    get,
    path = "/api/favorites/poses",
    tag = "favorites",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Lista de poses favoritas", body = [PoseResponse]),
        (status = 401, description = "No autorizado / Usuario no encontrado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn get_favorite_poses(
    auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
) -> Result<Json<Vec<PoseResponse>>, ApiError> {
    let user_id = user_id_from_auth(&state, &auth.0).await?;
    let uc = GetFavoritePosesUseCase::new(Arc::clone(&state.favorites_repo));
    let items = uc.execute(user_id).await?;
    Ok(Json(items.into_iter().map(PoseResponse::from).collect()))
}

/// Indica si una pose es favorita del usuario.
#[utoipa::path(
    get,
    path = "/api/favorites/poses/{pose_id}",
    tag = "favorites",
    security(("bearer_auth" = [])),
    params(("pose_id" = Uuid, Path, description = "UUID de la pose")),
    responses(
        (status = 200, description = "JSON: { is_favorite: boolean }"),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn is_pose_favorite(
    auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Path(pose_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let user_id = user_id_from_auth(&state, &auth.0).await?;
    let uc = IsPoseFavoriteUseCase::new(Arc::clone(&state.favorites_repo));
    let is_fav = uc.execute(user_id, pose_id).await?;
    Ok(Json(serde_json::json!({ "is_favorite": is_fav })))
}

/// Añade una pose a favoritos del usuario.
#[utoipa::path(
    post,
    path = "/api/favorites/poses/{pose_id}",
    tag = "favorites",
    security(("bearer_auth" = [])),
    params(("pose_id" = Uuid, Path, description = "UUID de la pose")),
    responses(
        (status = 204, description = "Añadida a favoritos"),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn add_pose_to_favorites(
    auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Path(pose_id): Path<Uuid>,
) -> Result<axum::http::StatusCode, ApiError> {
    let user_id = user_id_from_auth(&state, &auth.0).await?;
    let uc = AddPoseToFavoritesUseCase::new(Arc::clone(&state.favorites_repo));
    uc.execute(user_id, pose_id).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

/// Quita una pose de favoritos del usuario.
#[utoipa::path(
    delete,
    path = "/api/favorites/poses/{pose_id}",
    tag = "favorites",
    security(("bearer_auth" = [])),
    params(("pose_id" = Uuid, Path, description = "UUID de la pose")),
    responses(
        (status = 204, description = "Quitada de favoritos"),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn remove_pose_from_favorites(
    auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Path(pose_id): Path<Uuid>,
) -> Result<axum::http::StatusCode, ApiError> {
    let user_id = user_id_from_auth(&state, &auth.0).await?;
    let uc = RemovePoseFromFavoritesUseCase::new(Arc::clone(&state.favorites_repo));
    uc.execute(user_id, pose_id).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

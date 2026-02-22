// Handlers de hashtags (alineados con Kotlin domain/cases/hashtags)

use axum::{
    extract::{Path, State},
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::api::{
    dto::{AddHashtagsToPostRequest, CreateHashtagRequest, ErrorResponse, HashtagResponse},
    state::AppState,
    ApiError,
};
use crate::application::{
    AddHashtagsToPostUseCase, CreateHashtagUseCase, DeleteHashtagUseCase, GetHashtagByIdUseCase,
    GetHashtagsByPoseUseCase, GetHashtagsUseCase,
};

/// Lista todos los hashtags (GetHashtagsUseCase).
#[utoipa::path(
    get,
    path = "/api/hashtags",
    tag = "hashtags",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Lista de hashtags", body = [HashtagResponse]),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn list_hashtags(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
) -> Result<Json<Vec<HashtagResponse>>, ApiError> {
    let uc = GetHashtagsUseCase::new(Arc::clone(&state.hashtags_repo));
    let items = uc.execute().await?;
    Ok(Json(items.into_iter().map(HashtagResponse::from).collect()))
}

/// Obtiene un hashtag por id.
#[utoipa::path(
    get,
    path = "/api/hashtags/{id}",
    tag = "hashtags",
    security(("bearer_auth" = [])),
    params(("id" = Uuid, Path, description = "UUID del hashtag")),
    responses(
        (status = 200, description = "Hashtag encontrado", body = HashtagResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 404, description = "Hashtag no encontrado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn get_hashtag(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<HashtagResponse>, ApiError> {
    let uc = GetHashtagByIdUseCase::new(Arc::clone(&state.hashtags_repo));
    let item = uc.execute(id).await?;
    Ok(Json(HashtagResponse::from(item)))
}

/// Crea un hashtag (CreateHashtagUseCase).
#[utoipa::path(
    post,
    path = "/api/hashtags",
    tag = "hashtags",
    security(("bearer_auth" = [])),
    request_body = CreateHashtagRequest,
    responses(
        (status = 200, description = "Hashtag creado", body = HashtagResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 400, description = "Nombre vacío o duplicado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn create_hashtag(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Json(body): Json<CreateHashtagRequest>,
) -> Result<Json<HashtagResponse>, ApiError> {
    let uc = CreateHashtagUseCase::new(Arc::clone(&state.hashtags_repo));
    let item = uc.execute(&body.name).await?;
    Ok(Json(HashtagResponse::from(item)))
}

/// Elimina un hashtag (DeleteHashtagUseCase).
#[utoipa::path(
    delete,
    path = "/api/hashtags/{id}",
    tag = "hashtags",
    security(("bearer_auth" = [])),
    params(("id" = Uuid, Path, description = "UUID del hashtag")),
    responses(
        (status = 204, description = "Hashtag eliminado"),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 404, description = "Hashtag no encontrado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn delete_hashtag(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, ApiError> {
    let uc = DeleteHashtagUseCase::new(Arc::clone(&state.hashtags_repo));
    uc.execute(id).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

/// Hashtags asociados a una pose (GetHashtagsByPoseUseCase).
#[utoipa::path(
    get,
    path = "/api/poses/{pose_id}/hashtags",
    tag = "hashtags",
    security(("bearer_auth" = [])),
    params(("pose_id" = Uuid, Path, description = "UUID de la pose")),
    responses(
        (status = 200, description = "Lista de hashtags de la pose", body = [HashtagResponse]),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn get_hashtags_by_pose(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Path(pose_id): Path<Uuid>,
) -> Result<Json<Vec<HashtagResponse>>, ApiError> {
    let uc = GetHashtagsByPoseUseCase::new(Arc::clone(&state.hashtags_repo));
    let items = uc.execute(pose_id).await?;
    Ok(Json(items.into_iter().map(HashtagResponse::from).collect()))
}

/// Añade hashtags a un post (AddHashtagsToPostUseCase).
#[utoipa::path(
    post,
    path = "/api/posts/{post_id}/hashtags",
    tag = "hashtags",
    security(("bearer_auth" = [])),
    params(("post_id" = Uuid, Path, description = "UUID del post")),
    request_body = AddHashtagsToPostRequest,
    responses(
        (status = 204, description = "Hashtags añadidos al post"),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn add_hashtags_to_post(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Path(post_id): Path<Uuid>,
    Json(body): Json<AddHashtagsToPostRequest>,
) -> Result<axum::http::StatusCode, ApiError> {
    let uc = AddHashtagsToPostUseCase::new(Arc::clone(&state.hashtags_repo));
    uc.execute(post_id, &body.hashtag_ids).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

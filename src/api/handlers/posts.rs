// Handlers de Posts (Kotlin domain/cases/posts)

use axum::{
    extract::{Path, Query, State},
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::api::{
    dto::{CreatePostRequest, ErrorResponse, PostResponse},
    state::AppState,
    ApiError,
};
use crate::application::{
    CreatePostUseCase, DeletePostUseCase, GetPostByIdUseCase, GetPostsByThemeOfTheDayIdUseCase,
    GetPostsPaginatedUseCase, GetPostsUseCase,
};

#[derive(Debug, serde::Deserialize, utoipa::IntoParams)]
pub struct PaginationQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

/// Lista todos los posts.
#[utoipa::path(
    get,
    path = "/api/posts",
    tag = "posts",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Lista de posts", body = [PostResponse]),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn list_posts(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
) -> Result<Json<Vec<PostResponse>>, ApiError> {
    let uc = GetPostsUseCase::new(Arc::clone(&state.posts_repo));
    let items = uc.execute().await?;
    Ok(Json(items.into_iter().map(PostResponse::from).collect()))
}

/// Lista posts paginado (?page=0&limit=20).
#[utoipa::path(
    get,
    path = "/api/posts/paginated",
    tag = "posts",
    security(("bearer_auth" = [])),
    params(PaginationQuery),
    responses(
        (status = 200, description = "Lista de posts", body = [PostResponse]),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn list_posts_paginated(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Query(q): Query<PaginationQuery>,
) -> Result<Json<Vec<PostResponse>>, ApiError> {
    let page = q.page.unwrap_or(0);
    let limit = q.limit.unwrap_or(20).min(100);
    let uc = GetPostsPaginatedUseCase::new(Arc::clone(&state.posts_repo));
    let items = uc.execute(page, limit).await?;
    Ok(Json(items.into_iter().map(PostResponse::from).collect()))
}

/// Posts por tema del día (MMdd).
#[utoipa::path(
    get,
    path = "/api/posts/theme-of-the-day/{theme_of_the_day_id}",
    tag = "posts",
    security(("bearer_auth" = [])),
    params(("theme_of_the_day_id" = String, Path, description = "Id del tema (MMdd)")),
    responses(
        (status = 200, description = "Lista de posts", body = [PostResponse]),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn get_posts_by_theme_of_the_day(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Path(theme_of_the_day_id): Path<String>,
) -> Result<Json<Vec<PostResponse>>, ApiError> {
    let uc = GetPostsByThemeOfTheDayIdUseCase::new(Arc::clone(&state.posts_repo));
    let items = uc.execute(&theme_of_the_day_id).await?;
    Ok(Json(items.into_iter().map(PostResponse::from).collect()))
}

/// Obtiene un post por id.
#[utoipa::path(
    get,
    path = "/api/posts/{id}",
    tag = "posts",
    security(("bearer_auth" = [])),
    params(("id" = Uuid, Path, description = "UUID del post")),
    responses(
        (status = 200, description = "Post encontrado", body = PostResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 404, description = "Post no encontrado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn get_post(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<PostResponse>, ApiError> {
    let uc = GetPostByIdUseCase::new(Arc::clone(&state.posts_repo));
    let item = uc.execute(id).await?;
    Ok(Json(PostResponse::from(item)))
}

/// Crea un post (user_id opcional, desde JWT si está autenticado).
#[utoipa::path(
    post,
    path = "/api/posts",
    tag = "posts",
    security(("bearer_auth" = [])),
    request_body = CreatePostRequest,
    responses(
        (status = 200, description = "Post creado", body = PostResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn create_post(
    auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Json(body): Json<CreatePostRequest>,
) -> Result<Json<PostResponse>, ApiError> {
    let user = state
        .auth_repository
        .get_by_email(&auth.0)
        .await
        .map_err(ApiError::from)?;
    let user_id = user.map(|u| u.id);
    let uc = CreatePostUseCase::new(Arc::clone(&state.posts_repo));
    let item = uc
        .execute(
            body.description.as_deref(),
            body.url.as_deref(),
            user_id,
        )
        .await?;
    Ok(Json(PostResponse::from(item)))
}

/// Elimina un post.
#[utoipa::path(
    delete,
    path = "/api/posts/{id}",
    tag = "posts",
    security(("bearer_auth" = [])),
    params(("id" = Uuid, Path, description = "UUID del post")),
    responses(
        (status = 204, description = "Post eliminado"),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 404, description = "Post no encontrado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn delete_post(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, ApiError> {
    let uc = DeletePostUseCase::new(Arc::clone(&state.posts_repo));
    uc.execute(id).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

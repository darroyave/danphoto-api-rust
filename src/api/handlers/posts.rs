// Handlers de Posts (Kotlin domain/cases/posts)

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
    dto::{CreatePostRequest, ErrorResponse, PostResponse, PostsPaginatedResponse},
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

/// Decodifica imagen base64 y la guarda en dir/{id}.{ext}. Devuelve la URL: /api/posts/{id}/image.
fn save_post_image_base64(
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

    Ok(format!("/api/posts/{}/image", id))
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

/// Lista posts paginado (?page=0&limit=20). Devuelve items, count, page, limit y total_pages.
#[utoipa::path(
    get,
    path = "/api/posts/paginated",
    tag = "posts",
    security(("bearer_auth" = [])),
    params(PaginationQuery),
    responses(
        (status = 200, description = "Lista paginada de posts (items, count, page, limit, total_pages)", body = PostsPaginatedResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn list_posts_paginated(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Query(q): Query<PaginationQuery>,
) -> Result<Json<PostsPaginatedResponse>, ApiError> {
    let page = q.page.unwrap_or(0);
    let limit = q.limit.unwrap_or(20).min(100);
    let uc = GetPostsPaginatedUseCase::new(Arc::clone(&state.posts_repo));
    let (items, count) = uc.execute(page, limit).await?;
    let total_pages = if count == 0 {
        0
    } else {
        ((count as u32) + limit - 1) / limit
    };
    Ok(Json(PostsPaginatedResponse {
        items: items.into_iter().map(PostResponse::from).collect(),
        count,
        page,
        limit,
        total_pages,
    }))
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

/// Crea un post con imagen en base64 (user_id desde JWT si está autenticado). La imagen se guarda en disco y la URL es /api/posts/{id}/image.
#[utoipa::path(
    post,
    path = "/api/posts",
    tag = "posts",
    security(("bearer_auth" = [])),
    request_body = CreatePostRequest,
    responses(
        (status = 200, description = "Post creado", body = PostResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 400, description = "Imagen base64 vacía o inválida", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn create_post(
    auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Json(body): Json<CreatePostRequest>,
) -> Result<Json<PostResponse>, ApiError> {
    if body.image_base64.trim().is_empty() {
        return Err(ApiError(crate::domain::DomainError::Validation(
            "image_base64 es requerido".to_string(),
        )));
    }
    if body.theme_of_the_day_id.trim().is_empty() {
        return Err(ApiError(crate::domain::DomainError::Validation(
            "theme_of_the_day_id es requerido".to_string(),
        )));
    }
    let user = state
        .auth_repository
        .get_by_email(&auth.0)
        .await
        .map_err(ApiError::from)?;
    let user_id = user.map(|u| u.id);
    let id = Uuid::new_v4();
    let url = save_post_image_base64(&state.posts_images_dir, &id, &body.image_base64)?;
    let uc = CreatePostUseCase::new(Arc::clone(&state.posts_repo));
    let item = uc
        .execute_with_id(
            id,
            body.description.as_deref(),
            Some(&url),
            user_id,
            body.theme_of_the_day_id.trim(),
        )
        .await?;
    Ok(Json(PostResponse::from(item)))
}

/// Sirve la imagen de un post (público).
#[utoipa::path(
    get,
    path = "/api/posts/{id}/image",
    tag = "posts",
    params(("id" = Uuid, Path, description = "UUID del post")),
    responses(
        (status = 200, description = "Imagen del post", content_type = "image/*"),
        (status = 404, description = "Imagen no encontrada"),
    ),
)]
pub async fn get_post_image(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let dir = StdPath::new(&state.posts_images_dir);
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
        "Imagen no encontrada para el post {}",
        id
    ))))
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

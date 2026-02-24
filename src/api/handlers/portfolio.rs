// Handlers de Portfolio (Kotlin domain/cases/portfolio)

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
    dto::{
        AddPortfolioImageRequest, CreatePortfolioCategoryRequest,
        PortfolioCategoryResponse, PortfolioImageResponse, PortfolioImagesPaginatedResponse,
        UpdatePortfolioCategoryRequest,
    },
    state::AppState,
    ApiError,
};
use crate::application::{
    AddPortfolioImageUseCase, CreatePortfolioCategoryUseCase, DeletePortfolioCategoryUseCase,
    DeletePortfolioImageUseCase, GetPortfolioCategoriesUseCase,
    GetPortfolioImagesByCategoryUseCase, UpdatePortfolioCategoryUseCase,
};

#[derive(Debug, serde::Deserialize, utoipa::IntoParams)]
pub struct PaginationQuery {
    /// Página (0-based). Por defecto 0.
    pub page: Option<u32>,
    /// Tamaño de página (máx. 100). Por defecto 20.
    pub limit: Option<u32>,
}

/// Decodifica imagen base64 y la guarda en dir/{id}.{ext}. Devuelve la URL: /api/portfolio/images/{id}/image.
fn save_portfolio_image_base64(
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

    Ok(format!("/api/portfolio/images/{}/image", id))
}

/// Lista categorías del portfolio.
#[utoipa::path(
    get,
    path = "/api/portfolio/categories",
    tag = "portfolio",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Lista de categorías", body = [PortfolioCategoryResponse]),
        (status = 401, description = "No autorizado", body = crate::api::dto::ErrorResponse),
        (status = 500, description = "Error interno", body = crate::api::dto::ErrorResponse),
    ),
)]
pub async fn list_portfolio_categories(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
) -> Result<Json<Vec<PortfolioCategoryResponse>>, ApiError> {
    let uc = GetPortfolioCategoriesUseCase::new(Arc::clone(&state.portfolio_repo));
    let items = uc.execute().await?;
    Ok(Json(items.into_iter().map(PortfolioCategoryResponse::from).collect()))
}

/// Imágenes de una categoría del portfolio (paginado). Query: ?page=0&limit=20. Devuelve items, count, page, limit y total_pages.
#[utoipa::path(
    get,
    path = "/api/portfolio/categories/{category_id}/images",
    tag = "portfolio",
    security(("bearer_auth" = [])),
    params(
        ("category_id" = Uuid, Path, description = "UUID de la categoría"),
        PaginationQuery
    ),
    responses(
        (status = 200, description = "Lista paginada de imágenes (items, count, page, limit, total_pages)", body = PortfolioImagesPaginatedResponse),
        (status = 401, description = "No autorizado", body = crate::api::dto::ErrorResponse),
        (status = 500, description = "Error interno", body = crate::api::dto::ErrorResponse),
    ),
)]
pub async fn get_portfolio_images(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Path(category_id): Path<Uuid>,
    Query(q): Query<PaginationQuery>,
) -> Result<Json<PortfolioImagesPaginatedResponse>, ApiError> {
    let page = q.page.unwrap_or(0);
    let limit = q.limit.unwrap_or(20).min(100);
    let uc = GetPortfolioImagesByCategoryUseCase::new(Arc::clone(&state.portfolio_repo));
    let (items, count) = uc.execute(category_id, page, limit).await?;
    let total_pages = if count == 0 {
        0
    } else {
        ((count as u32) + limit - 1) / limit
    };
    Ok(Json(PortfolioImagesPaginatedResponse {
        items: items.into_iter().map(PortfolioImageResponse::from).collect(),
        count,
        page,
        limit,
        total_pages,
    }))
}

/// Crea una categoría del portfolio.
#[utoipa::path(
    post,
    path = "/api/portfolio/categories",
    tag = "portfolio",
    security(("bearer_auth" = [])),
    request_body = CreatePortfolioCategoryRequest,
    responses(
        (status = 200, description = "Categoría creada", body = PortfolioCategoryResponse),
        (status = 401, description = "No autorizado", body = crate::api::dto::ErrorResponse),
        (status = 400, description = "Nombre vacío", body = crate::api::dto::ErrorResponse),
        (status = 500, description = "Error interno", body = crate::api::dto::ErrorResponse),
    ),
)]
pub async fn create_portfolio_category(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Json(body): Json<CreatePortfolioCategoryRequest>,
) -> Result<Json<PortfolioCategoryResponse>, ApiError> {
    let uc = CreatePortfolioCategoryUseCase::new(Arc::clone(&state.portfolio_repo));
    let item = uc.execute(&body.name).await?;
    Ok(Json(PortfolioCategoryResponse::from(item)))
}

/// Actualiza una categoría del portfolio.
#[utoipa::path(
    put,
    path = "/api/portfolio/categories/{id}",
    tag = "portfolio",
    security(("bearer_auth" = [])),
    params(("id" = Uuid, Path, description = "UUID de la categoría")),
    request_body = UpdatePortfolioCategoryRequest,
    responses(
        (status = 200, description = "Categoría actualizada", body = PortfolioCategoryResponse),
        (status = 401, description = "No autorizado", body = crate::api::dto::ErrorResponse),
        (status = 404, description = "Categoría no encontrada", body = crate::api::dto::ErrorResponse),
        (status = 500, description = "Error interno", body = crate::api::dto::ErrorResponse),
    ),
)]
pub async fn update_portfolio_category(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdatePortfolioCategoryRequest>,
) -> Result<Json<PortfolioCategoryResponse>, ApiError> {
    let uc = UpdatePortfolioCategoryUseCase::new(Arc::clone(&state.portfolio_repo));
    let item = uc.execute(id, &body.name).await?;
    Ok(Json(PortfolioCategoryResponse::from(item)))
}

/// Elimina una categoría del portfolio.
#[utoipa::path(
    delete,
    path = "/api/portfolio/categories/{id}",
    tag = "portfolio",
    security(("bearer_auth" = [])),
    params(("id" = Uuid, Path, description = "UUID de la categoría")),
    responses(
        (status = 204, description = "Categoría eliminada"),
        (status = 401, description = "No autorizado", body = crate::api::dto::ErrorResponse),
        (status = 500, description = "Error interno", body = crate::api::dto::ErrorResponse),
    ),
)]
pub async fn delete_portfolio_category(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, ApiError> {
    let uc = DeletePortfolioCategoryUseCase::new(Arc::clone(&state.portfolio_repo));
    uc.execute(id).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

/// Añade una imagen (base64) a una categoría del portfolio. La imagen se guarda en disco; la URL será /api/portfolio/images/{id}/image.
#[utoipa::path(
    post,
    path = "/api/portfolio/categories/{category_id}/images",
    tag = "portfolio",
    security(("bearer_auth" = [])),
    params(("category_id" = Uuid, Path, description = "UUID de la categoría")),
    request_body = AddPortfolioImageRequest,
    responses(
        (status = 200, description = "Imagen añadida", body = PortfolioImageResponse),
        (status = 401, description = "No autorizado", body = crate::api::dto::ErrorResponse),
        (status = 400, description = "Imagen base64 vacía o inválida", body = crate::api::dto::ErrorResponse),
        (status = 500, description = "Error interno", body = crate::api::dto::ErrorResponse),
    ),
)]
pub async fn add_portfolio_image(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Path(category_id): Path<Uuid>,
    Json(body): Json<AddPortfolioImageRequest>,
) -> Result<Json<PortfolioImageResponse>, ApiError> {
    if body.image_base64.trim().is_empty() {
        return Err(ApiError(crate::domain::DomainError::Validation(
            "image_base64 es requerido".to_string(),
        )));
    }
    let id = Uuid::new_v4();
    let dir = &state.portfolio_images_dir;
    let url = match save_portfolio_image_base64(dir, &id, &body.image_base64) {
        Ok(u) => u,
        Err(e) => return Err(e),
    };
    let uc = AddPortfolioImageUseCase::new(Arc::clone(&state.portfolio_repo));
    match uc.execute_with_id(id, category_id, &url).await {
        Ok(item) => Ok(Json(PortfolioImageResponse::from(item))),
        Err(e) => {
            // Borrar el archivo recién guardado si el INSERT falla (evitar huérfanos)
            for ext in ["png", "jpg", "jpeg"] {
                let path = StdPath::new(dir).join(format!("{}.{}", id, ext));
                let _ = std::fs::remove_file(&path);
            }
            Err(ApiError(e))
        }
    }
}

/// Sirve la imagen de un ítem del portfolio (público).
#[utoipa::path(
    get,
    path = "/api/portfolio/images/{id}/image",
    tag = "portfolio",
    params(("id" = Uuid, Path, description = "UUID de la imagen")),
    responses(
        (status = 200, description = "Imagen del portfolio", content_type = "image/*"),
        (status = 404, description = "Imagen no encontrada"),
    ),
)]
pub async fn get_portfolio_image(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let dir = StdPath::new(&state.portfolio_images_dir);
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
        "Imagen no encontrada para el portfolio {}",
        id
    ))))
}

/// Elimina una imagen del portfolio.
#[utoipa::path(
    delete,
    path = "/api/portfolio/images/{id}",
    tag = "portfolio",
    security(("bearer_auth" = [])),
    params(("id" = Uuid, Path, description = "UUID de la imagen")),
    responses(
        (status = 204, description = "Imagen eliminada"),
        (status = 401, description = "No autorizado", body = crate::api::dto::ErrorResponse),
        (status = 500, description = "Error interno", body = crate::api::dto::ErrorResponse),
    ),
)]
pub async fn delete_portfolio_image(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, ApiError> {
    let uc = DeletePortfolioImageUseCase::new(Arc::clone(&state.portfolio_repo));
    uc.execute(id).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

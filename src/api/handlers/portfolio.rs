// Handlers de Portfolio (Kotlin domain/cases/portfolio)

use axum::{
    extract::{Path, Query, State},
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::api::{
    dto::{
        AddPortfolioImageRequest, CreatePortfolioCategoryRequest,
        PortfolioCategoryResponse, PortfolioImageResponse, UpdatePortfolioCategoryRequest,
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
    pub page: Option<u32>,
    pub limit: Option<u32>,
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

/// Imágenes de una categoría (paginado).
#[utoipa::path(
    get,
    path = "/api/portfolio/categories/{category_id}/images",
    tag = "portfolio",
    security(("bearer_auth" = [])),
    params(("category_id" = Uuid, Path), PaginationQuery),
    responses(
        (status = 200, description = "Lista de imágenes", body = [PortfolioImageResponse]),
        (status = 401, description = "No autorizado", body = crate::api::dto::ErrorResponse),
        (status = 500, description = "Error interno", body = crate::api::dto::ErrorResponse),
    ),
)]
pub async fn get_portfolio_images(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Path(category_id): Path<Uuid>,
    Query(q): Query<PaginationQuery>,
) -> Result<Json<Vec<PortfolioImageResponse>>, ApiError> {
    let page = q.page.unwrap_or(0);
    let limit = q.limit.unwrap_or(20).min(100);
    let uc = GetPortfolioImagesByCategoryUseCase::new(Arc::clone(&state.portfolio_repo));
    let items = uc.execute(category_id, page, limit).await?;
    Ok(Json(items.into_iter().map(PortfolioImageResponse::from).collect()))
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

/// Añade una imagen a una categoría del portfolio.
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
        (status = 400, description = "URL vacía", body = crate::api::dto::ErrorResponse),
        (status = 500, description = "Error interno", body = crate::api::dto::ErrorResponse),
    ),
)]
pub async fn add_portfolio_image(
    _auth: crate::api::auth::BearerAuth,
    State(state): State<AppState>,
    Path(category_id): Path<Uuid>,
    Json(body): Json<AddPortfolioImageRequest>,
) -> Result<Json<PortfolioImageResponse>, ApiError> {
    let uc = AddPortfolioImageUseCase::new(Arc::clone(&state.portfolio_repo));
    let item = uc.execute(category_id, &body.url).await?;
    Ok(Json(PortfolioImageResponse::from(item)))
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

// Handlers de Places (lugares). Requieren Bearer token.

use axum::{
    extract::{Path, State},
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::api::{
    dto::{CreatePlaceRequest, ErrorResponse, PlaceResponse, UpdatePlaceRequest},
    state::AppState,
    ApiError,
};
use crate::application::{
    CreatePlaceUseCase, DeletePlaceUseCase, GetPlaceByIdUseCase, GetPlacesUseCase,
    UpdatePlaceUseCase,
};
use crate::api::auth::BearerAuth;

/// Lista todos los lugares.
#[utoipa::path(
    get,
    path = "/api/places",
    tag = "places",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Lista de lugares", body = [PlaceResponse]),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn list_places(
    _auth: BearerAuth,
    State(state): State<AppState>,
) -> Result<Json<Vec<PlaceResponse>>, ApiError> {
    let uc = GetPlacesUseCase::new(Arc::clone(&state.places_repo));
    let items = uc.execute().await?;
    Ok(Json(items.into_iter().map(PlaceResponse::from).collect()))
}

/// Obtiene un lugar por ID.
#[utoipa::path(
    get,
    path = "/api/places/{id}",
    tag = "places",
    security(("bearer_auth" = [])),
    params(("id" = Uuid, Path, description = "UUID del lugar")),
    responses(
        (status = 200, description = "Lugar encontrado", body = PlaceResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 404, description = "Lugar no encontrado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn get_place(
    _auth: BearerAuth,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<PlaceResponse>, ApiError> {
    let uc = GetPlaceByIdUseCase::new(Arc::clone(&state.places_repo));
    let place = uc.execute(id).await?;
    let place = place.ok_or_else(|| ApiError(crate::domain::DomainError::NotFound("Lugar no encontrado".to_string())))?;
    Ok(Json(PlaceResponse::from(place)))
}

/// Crea un nuevo lugar.
#[utoipa::path(
    post,
    path = "/api/places",
    tag = "places",
    security(("bearer_auth" = [])),
    request_body = CreatePlaceRequest,
    responses(
        (status = 200, description = "Lugar creado", body = PlaceResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn create_place(
    _auth: BearerAuth,
    State(state): State<AppState>,
    Json(body): Json<CreatePlaceRequest>,
) -> Result<Json<PlaceResponse>, ApiError> {
    let uc = CreatePlaceUseCase::new(Arc::clone(&state.places_repo));
    let place = uc
        .execute(
            &body.name,
            &body.description,
            &body.address,
            &body.location,
            body.latitude,
            body.longitude,
            &body.url,
            body.instagram.as_deref(),
            body.website.as_deref(),
        )
        .await?;
    Ok(Json(PlaceResponse::from(place)))
}

/// Actualiza un lugar existente.
#[utoipa::path(
    put,
    path = "/api/places/{id}",
    tag = "places",
    security(("bearer_auth" = [])),
    params(("id" = Uuid, Path, description = "UUID del lugar")),
    request_body = UpdatePlaceRequest,
    responses(
        (status = 200, description = "Lugar actualizado", body = PlaceResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 404, description = "Lugar no encontrado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn update_place(
    _auth: BearerAuth,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdatePlaceRequest>,
) -> Result<Json<PlaceResponse>, ApiError> {
    let uc = UpdatePlaceUseCase::new(Arc::clone(&state.places_repo));
    let place = uc
        .execute(
            id,
            body.name.as_deref(),
            body.description.as_deref(),
            body.address.as_deref(),
            body.location.as_deref(),
            body.latitude,
            body.longitude,
            body.url.as_deref(),
            body.instagram.as_deref(),
            body.website.as_deref(),
        )
        .await?;
    let place = place.ok_or_else(|| ApiError(crate::domain::DomainError::NotFound("Lugar no encontrado".to_string())))?;
    Ok(Json(PlaceResponse::from(place)))
}

/// Elimina un lugar.
#[utoipa::path(
    delete,
    path = "/api/places/{id}",
    tag = "places",
    security(("bearer_auth" = [])),
    params(("id" = Uuid, Path, description = "UUID del lugar")),
    responses(
        (status = 204, description = "Lugar eliminado"),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 404, description = "Lugar no encontrado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn delete_place(
    _auth: BearerAuth,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, ApiError> {
    let uc = DeletePlaceUseCase::new(Arc::clone(&state.places_repo));
    uc.execute(id).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

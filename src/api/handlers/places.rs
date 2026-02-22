// Handlers de Places (lugares). Requieren Bearer token.

use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use base64::Engine;
use std::path::Path as StdPath;
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

/// Decodifica imagen base64 y la guarda en dir/{id}.{ext}. Devuelve la URL: /api/places/{id}/image.
fn save_place_image_base64(
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

    Ok(format!("/api/places/{}/image", id))
}

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

/// Crea un nuevo lugar con imagen en base64. La URL será /api/places/{id}/image.
#[utoipa::path(
    post,
    path = "/api/places",
    tag = "places",
    security(("bearer_auth" = [])),
    request_body = CreatePlaceRequest,
    responses(
        (status = 200, description = "Lugar creado", body = PlaceResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 400, description = "Imagen base64 vacía o inválida", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn create_place(
    _auth: BearerAuth,
    State(state): State<AppState>,
    Json(body): Json<CreatePlaceRequest>,
) -> Result<Json<PlaceResponse>, ApiError> {
    if body.image_base64.trim().is_empty() {
        return Err(ApiError(crate::domain::DomainError::Validation(
            "image_base64 es requerido".to_string(),
        )));
    }
    let id = Uuid::new_v4();
    let url = save_place_image_base64(&state.places_images_dir, &id, &body.image_base64)?;
    let uc = CreatePlaceUseCase::new(Arc::clone(&state.places_repo));
    let place = uc
        .execute_with_id(
            id,
            &body.name,
            &body.description,
            &body.address,
            &body.location,
            body.latitude,
            body.longitude,
            &url,
            body.instagram.as_deref(),
            body.website.as_deref(),
        )
        .await?;
    Ok(Json(PlaceResponse::from(place)))
}

/// Actualiza un lugar existente. Si se envía image_base64, reemplaza la imagen.
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
    let url = if let Some(ref b64) = body.image_base64 {
        if b64.trim().is_empty() {
            None
        } else {
            Some(save_place_image_base64(&state.places_images_dir, &id, b64)?)
        }
    } else {
        None
    };
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
            url.as_deref(),
            body.instagram.as_deref(),
            body.website.as_deref(),
        )
        .await?;
    let place = place.ok_or_else(|| ApiError(crate::domain::DomainError::NotFound("Lugar no encontrado".to_string())))?;
    Ok(Json(PlaceResponse::from(place)))
}

/// Sirve la imagen de un lugar (público).
#[utoipa::path(
    get,
    path = "/api/places/{id}/image",
    tag = "places",
    params(("id" = Uuid, Path, description = "UUID del lugar")),
    responses(
        (status = 200, description = "Imagen del lugar", content_type = "image/*"),
        (status = 404, description = "Imagen no encontrada"),
    ),
)]
pub async fn get_place_image(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let dir = StdPath::new(&state.places_images_dir);
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
        "Imagen no encontrada para el lugar {}",
        id
    ))))
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

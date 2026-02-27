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
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

use crate::api::auth::{BearerAuth, LoginRequest, LoginResponse};
use crate::api::{
    dto::{CreateEventoRequest, ErrorResponse, EventoResponse, UpdateEventoRequest},
    state::AppState,
    ApiError,
};
use crate::application::{
    CreateEventoUseCase, DeleteEventoUseCase, GetEventoByIdUseCase, GetEventosUseCase,
    UpdateEventoUseCase,
};

/// Añade el esquema de seguridad Bearer JWT al OpenAPI.
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .description(Some("Token obtenido en POST /api/auth/login (body: email + password)"))
                        .build(),
                ),
            )
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    paths(
        crate::api::auth::login,
        list_eventos,
        get_evento,
        get_evento_image,
        create_evento,
        update_evento,
        delete_evento,
        crate::api::handlers::theme_of_the_day::list_theme_of_the_day,
        crate::api::handlers::theme_of_the_day::get_theme_of_the_day_today,
        crate::api::handlers::theme_of_the_day::get_theme_of_the_day,
        crate::api::handlers::theme_of_the_day::get_theme_of_the_day_image,
        crate::api::handlers::theme_of_the_day::create_theme_of_the_day,
        crate::api::handlers::theme_of_the_day::update_theme_of_the_day,
        crate::api::handlers::theme_of_the_day::delete_theme_of_the_day,
        crate::api::handlers::hashtags::list_hashtags,
        crate::api::handlers::hashtags::get_hashtag,
        crate::api::handlers::hashtags::create_hashtag,
        crate::api::handlers::hashtags::delete_hashtag,
        crate::api::handlers::hashtags::get_hashtags_by_pose,
        crate::api::handlers::hashtags::add_hashtags_to_post,
        crate::api::handlers::poses::list_poses,
        crate::api::handlers::poses::list_poses_paginated,
        crate::api::handlers::poses::get_pose,
        crate::api::handlers::poses::get_pose_image,
        crate::api::handlers::poses::create_pose,
        crate::api::handlers::poses::delete_pose,
        crate::api::handlers::poses::get_poses_by_hashtag,
        crate::api::handlers::poses::get_poses_by_hashtag_paginated,
        crate::api::handlers::poses::update_pose_hashtags,
        crate::api::handlers::posts::list_posts,
        crate::api::handlers::posts::list_posts_paginated,
        crate::api::handlers::posts::get_posts_by_theme_of_the_day,
        crate::api::handlers::posts::get_post,
        crate::api::handlers::posts::get_post_image,
        crate::api::handlers::posts::create_post,
        crate::api::handlers::posts::delete_post,
        crate::api::handlers::portfolio::list_portfolio_categories,
        crate::api::handlers::portfolio::get_portfolio_images,
        crate::api::handlers::portfolio::get_portfolio_image,
        crate::api::handlers::portfolio::create_portfolio_category,
        crate::api::handlers::portfolio::update_portfolio_category,
        crate::api::handlers::portfolio::update_portfolio_cover,
        crate::api::handlers::portfolio::delete_portfolio_category,
        crate::api::handlers::portfolio::add_portfolio_image,
        crate::api::handlers::portfolio::delete_portfolio_image,
        crate::api::handlers::favorites::get_favorite_poses,
        crate::api::handlers::favorites::is_pose_favorite,
        crate::api::handlers::favorites::add_pose_to_favorites,
        crate::api::handlers::favorites::remove_pose_from_favorites,
        crate::api::handlers::places::list_places,
        crate::api::handlers::places::get_place,
        crate::api::handlers::places::get_place_image,
        crate::api::handlers::places::create_place,
        crate::api::handlers::places::update_place,
        crate::api::handlers::places::delete_place,
        crate::api::handlers::sesiones::list_sesiones,
        crate::api::handlers::sesiones::get_sesion,
        crate::api::handlers::sesiones::get_poses_by_sesion,
        crate::api::handlers::sesiones::create_sesion,
        crate::api::handlers::sesiones::create_sesion_from_favorites,
        crate::api::handlers::sesiones::add_poses_to_sesion,
        crate::api::handlers::sesiones::add_favorites_to_sesion,
        crate::api::handlers::sesiones::remove_pose_from_sesion,
        crate::api::handlers::sesiones::update_sesion_cover,
        crate::api::handlers::sesiones::delete_sesion,
        crate::api::handlers::usuarios::get_profile,
        crate::api::handlers::usuarios::update_profile,
        crate::api::handlers::usuarios::get_profile_avatar,
        crate::api::handlers::usuarios::update_profile_avatar,
    ),
    components(schemas(
        LoginRequest,
        LoginResponse,
        EventoResponse,
        CreateEventoRequest,
        UpdateEventoRequest,
        ErrorResponse,
        crate::api::dto::ThemeOfTheDayResponse,
        crate::api::dto::CreateThemeOfTheDayRequest,
        crate::api::dto::UpdateThemeOfTheDayRequest,
        crate::api::dto::HashtagResponse,
        crate::api::dto::CreateHashtagRequest,
        crate::api::dto::AddHashtagsToPostRequest,
        crate::api::dto::PoseResponse,
        crate::api::dto::CreatePoseRequest,
        crate::api::dto::UpdatePoseHashtagsRequest,
        crate::api::dto::PostResponse,
        crate::api::dto::CreatePostRequest,
        crate::api::dto::PortfolioCategoryResponse,
        crate::api::dto::PortfolioImageResponse,
        crate::api::dto::CreatePortfolioCategoryRequest,
        crate::api::dto::UpdatePortfolioCategoryRequest,
        crate::api::dto::AddPortfolioImageRequest,
        crate::api::dto::PlaceResponse,
        crate::api::dto::CreatePlaceRequest,
        crate::api::dto::UpdatePlaceRequest,
        crate::api::dto::SesionResponse,
        crate::api::dto::CreateSesionRequest,
        crate::api::dto::AddPosesToSesionRequest,
        crate::api::dto::CreateSesionFromFavoritesRequest,
        crate::api::dto::UpdateSesionCoverRequest,
        crate::api::dto::UsuarioResponse,
        crate::api::dto::UpdateUsuarioRequest,
        crate::api::dto::UpdateUsuarioAvatarRequest,
    )),
    tags(
        (name = "auth", description = "Autenticación JWT"),
        (name = "eventos", description = "CRUD de eventos (requieren Bearer token)"),
        (name = "theme_of_the_day", description = "CRUD tema del día (requieren Bearer token)"),
        (name = "hashtags", description = "Hashtags y relación con poses/posts (requieren Bearer token)"),
        (name = "poses", description = "Poses e imágenes (requieren Bearer token)"),
        (name = "posts", description = "Posts (requieren Bearer token)"),
        (name = "portfolio", description = "Portfolio: categorías (requieren Bearer token)"),
        (name = "portfolio_images", description = "Portfolio: imágenes por categoría (requieren Bearer token; GET imagen es público)"),
        (name = "favorites", description = "Favoritos del usuario (requieren Bearer token)"),
        (name = "places", description = "Lugares (requieren Bearer token)"),
        (name = "sesiones", description = "Sesiones de poses (requieren Bearer token)"),
        (name = "usuario", description = "Perfil del usuario (requieren Bearer token)"),
    ),
)]
pub struct ApiDoc;

/// Decodifica imagen base64 y la guarda en dir/{id}.{ext}. Devuelve la URL: /api/eventos/{id}/image.
fn save_evento_image_base64(
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

    Ok(format!("/api/eventos/{}/image", id))
}

/// Lista todos los eventos (requiere Bearer token).
#[utoipa::path(
    get,
    path = "/api/eventos",
    tag = "eventos",
    responses(
        (status = 200, description = "Lista de eventos", body = [EventoResponse]),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn list_eventos(
    _auth: BearerAuth,
    State(state): State<AppState>,
) -> Result<Json<Vec<EventoResponse>>, ApiError> {
    let uc = GetEventosUseCase::new(Arc::clone(&state.eventos_repo));
    let eventos = uc.execute().await?;
    Ok(Json(
        eventos.into_iter().map(EventoResponse::from).collect(),
    ))
}

/// Obtiene un evento por ID (requiere Bearer token).
#[utoipa::path(
    get,
    path = "/api/eventos/{id}",
    tag = "eventos",
    security(("bearer_auth" = [])),
    params(("id" = Uuid, Path, description = "UUID del evento")),
    responses(
        (status = 200, description = "Evento encontrado", body = EventoResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 404, description = "Evento no encontrado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn get_evento(
    _auth: BearerAuth,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<EventoResponse>, ApiError> {
    let uc = GetEventoByIdUseCase::new(Arc::clone(&state.eventos_repo));
    let evento = uc.execute(id).await?;
    Ok(Json(EventoResponse::from(evento)))
}

/// Crea un nuevo evento con imagen en base64 (requiere Bearer token). La URL será /api/eventos/{id}/image.
#[utoipa::path(
    post,
    path = "/api/eventos",
    tag = "eventos",
    security(("bearer_auth" = [])),
    request_body = CreateEventoRequest,
    responses(
        (status = 200, description = "Evento creado", body = EventoResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 400, description = "Validación fallida (mmdd vacío o imagen base64 inválida)", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn create_evento(
    _auth: BearerAuth,
    State(state): State<AppState>,
    Json(body): Json<CreateEventoRequest>,
) -> Result<Json<EventoResponse>, ApiError> {
    if body.image_base64.trim().is_empty() {
        return Err(ApiError(crate::domain::DomainError::Validation(
            "image_base64 es requerido".to_string(),
        )));
    }
    let id = Uuid::new_v4();
    let url = save_evento_image_base64(&state.eventos_images_dir, &id, &body.image_base64)?;
    let uc = CreateEventoUseCase::new(Arc::clone(&state.eventos_repo));
    let evento = uc
        .execute_with_id(id, &body.name, &body.place, &url, &body.mmdd)
        .await?;
    Ok(Json(EventoResponse::from(evento)))
}

/// Actualiza un evento existente (requiere Bearer token). Si se envía image_base64, reemplaza la imagen.
#[utoipa::path(
    put,
    path = "/api/eventos/{id}",
    tag = "eventos",
    security(("bearer_auth" = [])),
    params(("id" = Uuid, Path, description = "UUID del evento")),
    request_body = UpdateEventoRequest,
    responses(
        (status = 200, description = "Evento actualizado", body = EventoResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 404, description = "Evento no encontrado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn update_evento(
    _auth: BearerAuth,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateEventoRequest>,
) -> Result<Json<EventoResponse>, ApiError> {
    let url = if let Some(ref b64) = body.image_base64 {
        if b64.trim().is_empty() {
            None
        } else {
            Some(save_evento_image_base64(&state.eventos_images_dir, &id, b64)?)
        }
    } else {
        None
    };
    let uc = UpdateEventoUseCase::new(Arc::clone(&state.eventos_repo));
    let evento = uc
        .execute(
            id,
            body.name.as_deref(),
            body.place.as_deref(),
            url.as_deref(),
            body.mmdd.as_deref(),
        )
        .await?;
    Ok(Json(EventoResponse::from(evento)))
}

/// Sirve la imagen de un evento (público).
#[utoipa::path(
    get,
    path = "/api/eventos/{id}/image",
    tag = "eventos",
    params(("id" = Uuid, Path, description = "UUID del evento")),
    responses(
        (status = 200, description = "Imagen del evento", content_type = "image/*"),
        (status = 404, description = "Imagen no encontrada"),
    ),
)]
pub async fn get_evento_image(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let dir = StdPath::new(&state.eventos_images_dir);
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
        "Imagen no encontrada para el evento {}",
        id
    ))))
}

/// Elimina un evento (requiere Bearer token).
#[utoipa::path(
    delete,
    path = "/api/eventos/{id}",
    tag = "eventos",
    security(("bearer_auth" = [])),
    params(("id" = Uuid, Path, description = "UUID del evento")),
    responses(
        (status = 204, description = "Evento eliminado"),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 404, description = "Evento no encontrado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn delete_evento(
    _auth: BearerAuth,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, ApiError> {
    let uc = DeleteEventoUseCase::new(Arc::clone(&state.eventos_repo));
    uc.execute(id).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

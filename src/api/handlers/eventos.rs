use axum::{
    extract::{Path, State},
    Json,
};
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
        crate::api::handlers::portfolio::delete_portfolio_category,
        crate::api::handlers::portfolio::add_portfolio_image,
        crate::api::handlers::portfolio::delete_portfolio_image,
        crate::api::handlers::favorites::get_favorite_poses,
        crate::api::handlers::favorites::is_pose_favorite,
        crate::api::handlers::favorites::add_pose_to_favorites,
        crate::api::handlers::favorites::remove_pose_from_favorites,
        crate::api::handlers::places::list_places,
        crate::api::handlers::places::get_place,
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
        (name = "portfolio", description = "Portfolio: categorías e imágenes (requieren Bearer token)"),
        (name = "favorites", description = "Favoritos del usuario (requieren Bearer token)"),
        (name = "places", description = "Lugares (requieren Bearer token)"),
        (name = "sesiones", description = "Sesiones de poses (requieren Bearer token)"),
        (name = "usuario", description = "Perfil del usuario (requieren Bearer token)"),
    ),
)]
pub struct ApiDoc;

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

/// Crea un nuevo evento (requiere Bearer token).
#[utoipa::path(
    post,
    path = "/api/eventos",
    tag = "eventos",
    security(("bearer_auth" = [])),
    request_body = CreateEventoRequest,
    responses(
        (status = 200, description = "Evento creado", body = EventoResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 400, description = "Validación fallida (ej: mmdd vacío)", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse),
    ),
)]
pub async fn create_evento(
    _auth: BearerAuth,
    State(state): State<AppState>,
    Json(body): Json<CreateEventoRequest>,
) -> Result<Json<EventoResponse>, ApiError> {
    let uc = CreateEventoUseCase::new(Arc::clone(&state.eventos_repo));
    let evento = uc
        .execute(&body.name, &body.place, &body.url, &body.mmdd)
        .await?;
    Ok(Json(EventoResponse::from(evento)))
}

/// Actualiza un evento existente (requiere Bearer token).
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
    let uc = UpdateEventoUseCase::new(Arc::clone(&state.eventos_repo));
    let evento = uc
        .execute(
            id,
            body.name.as_deref(),
            body.place.as_deref(),
            body.url.as_deref(),
            body.mmdd.as_deref(),
        )
        .await?;
    Ok(Json(EventoResponse::from(evento)))
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

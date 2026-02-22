#[allow(unused_imports)]
use axum::{
    extract::State,
    routing::{delete, get, post, put},
    Router,
};
use axum::http::HeaderValue;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use tower_http::cors::{AllowOrigin, Any, CorsLayer};

use super::auth::login;
use super::handlers::eventos::{
    create_evento, delete_evento, get_evento, list_eventos, update_evento,
};
use super::handlers::favorites::{
    add_pose_to_favorites, get_favorite_poses, is_pose_favorite, remove_pose_from_favorites,
};
use super::handlers::hashtags::{
    add_hashtags_to_post, create_hashtag, delete_hashtag, get_hashtag, get_hashtags_by_pose,
    list_hashtags,
};
use super::handlers::portfolio::{
    add_portfolio_image, create_portfolio_category, delete_portfolio_category,
    delete_portfolio_image, get_portfolio_image, get_portfolio_images,
    list_portfolio_categories, update_portfolio_category,
};
use super::handlers::places::{
    create_place, delete_place, get_place, list_places, update_place,
};
use super::handlers::poses::{
    create_pose, delete_pose, get_pose, get_pose_image, get_poses_by_hashtag,
    get_poses_by_hashtag_paginated, list_poses, list_poses_paginated, update_pose_hashtags,
};
use super::handlers::posts::{
    create_post, delete_post, get_post, get_post_image, get_posts_by_theme_of_the_day,
    list_posts, list_posts_paginated,
};
use super::handlers::sesiones::{
    add_favorites_to_sesion, add_poses_to_sesion, create_sesion, create_sesion_from_favorites,
    delete_sesion, get_poses_by_sesion, get_sesion, list_sesiones, remove_pose_from_sesion,
    update_sesion_cover,
};
use super::handlers::theme_of_the_day::{
    create_theme_of_the_day, delete_theme_of_the_day, get_theme_of_the_day,
    get_theme_of_the_day_image, get_theme_of_the_day_today, list_theme_of_the_day,
    update_theme_of_the_day,
};
use super::handlers::usuarios::{get_profile, update_profile, update_profile_avatar};
use super::state::AppState;
use super::swagger::{
    serve_index_css, serve_openapi_json, serve_swagger_initializer_js, serve_swagger_ui,
    serve_swagger_ui_bundle_js, serve_swagger_ui_css, serve_swagger_ui_standalone_preset_js,
};

/// Construye la capa CORS: si `CORS_ALLOWED_ORIGINS` está definido, solo esos orígenes; si no, cualquier origen (desarrollo).
fn cors_layer_from_config(config: &crate::config::Config) -> CorsLayer {
    let origins: Vec<HeaderValue> = config
        .cors_allowed_origins
        .iter()
        .filter_map(|s| HeaderValue::try_from(s.as_str()).ok())
        .collect();
    if origins.is_empty() {
        CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any)
    } else {
        CorsLayer::new()
            .allow_origin(AllowOrigin::list(origins))
            .allow_methods(Any)
            .allow_headers(Any)
    }
}

/// Crea el router de la API (incluye Swagger UI en /swagger-ui).
/// Si `config.rate_limit_login_per_minute` > 0, aplica rate limiting por IP al login.
pub fn create_router(state: AppState, config: &crate::config::Config) -> Router {
    let cors = cors_layer_from_config(config);
    let rest_routes = Router::new()
        .route("/api/eventos", get(list_eventos).post(create_evento))
        .route(
            "/api/eventos/{id}",
            get(get_evento).put(update_evento).delete(delete_evento),
        )
        .route(
            "/api/theme-of-the-day",
            get(list_theme_of_the_day).post(create_theme_of_the_day),
        )
        .route(
            "/api/theme-of-the-day/today",
            get(get_theme_of_the_day_today),
        )
        .route("/api/theme-of-the-day/{id}/image", get(get_theme_of_the_day_image))
        .route(
            "/api/theme-of-the-day/{id}",
            get(get_theme_of_the_day)
                .put(update_theme_of_the_day)
                .delete(delete_theme_of_the_day),
        )
        .route("/api/hashtags", get(list_hashtags).post(create_hashtag))
        .route("/api/hashtags/{id}", get(get_hashtag).delete(delete_hashtag))
        .route("/api/poses/{pose_id}/hashtags", get(get_hashtags_by_pose).put(update_pose_hashtags))
        .route("/api/posts/{post_id}/hashtags", post(add_hashtags_to_post))
        .route("/api/poses", get(list_poses).post(create_pose))
        .route("/api/poses/paginated", get(list_poses_paginated))
        .route("/api/poses/{id}/image", get(get_pose_image))
        .route("/api/poses/{id}", get(get_pose).delete(delete_pose))
        .route("/api/hashtags/{hashtag_id}/poses", get(get_poses_by_hashtag))
        .route("/api/hashtags/{hashtag_id}/poses/paginated", get(get_poses_by_hashtag_paginated))
        .route("/api/posts", get(list_posts).post(create_post))
        .route("/api/posts/paginated", get(list_posts_paginated))
        .route("/api/posts/theme-of-the-day/{theme_of_the_day_id}", get(get_posts_by_theme_of_the_day))
        .route("/api/posts/{id}/image", get(get_post_image))
        .route("/api/posts/{id}", get(get_post).delete(delete_post))
        .route("/api/portfolio/categories", get(list_portfolio_categories).post(create_portfolio_category))
        .route("/api/portfolio/categories/{id}", put(update_portfolio_category).delete(delete_portfolio_category))
        .route("/api/portfolio/categories/{category_id}/images", get(get_portfolio_images).post(add_portfolio_image))
        .route("/api/portfolio/images/{id}/image", get(get_portfolio_image))
        .route("/api/portfolio/images/{id}", delete(delete_portfolio_image))
        .route("/api/favorites/poses", get(get_favorite_poses))
        .route("/api/favorites/poses/{pose_id}", get(is_pose_favorite).post(add_pose_to_favorites).delete(remove_pose_from_favorites))
        .route("/api/places", get(list_places).post(create_place))
        .route("/api/places/{id}", get(get_place).put(update_place).delete(delete_place))
        .route("/api/sesiones", get(list_sesiones).post(create_sesion))
        .route("/api/sesiones/from-favorites", post(create_sesion_from_favorites))
        .route("/api/sesiones/{id}", get(get_sesion).delete(delete_sesion))
        .route("/api/sesiones/{id}/poses", get(get_poses_by_sesion).post(add_poses_to_sesion))
        .route("/api/sesiones/{id}/add-favorites", post(add_favorites_to_sesion))
        .route("/api/sesiones/{id}/poses/{pose_id}", delete(remove_pose_from_sesion))
        .route("/api/sesiones/{id}/cover", put(update_sesion_cover))
        .route("/api/profile", get(get_profile).put(update_profile))
        .route("/api/profile/avatar", put(update_profile_avatar))
        .route("/api/health", get(|| async { "ok" }))
        .route("/api-docs/openapi.json", get(serve_openapi_json))
        .route("/swagger-ui", get(serve_swagger_ui_root))
        .route("/swagger-ui/{*path}", get(serve_swagger_ui))
        // Assets que el HTML de Swagger UI pide en la raíz (evitan 404)
        .route("/swagger-ui.css", get(serve_swagger_ui_css))
        .route("/index.css", get(serve_index_css))
        .route("/swagger-ui-bundle.js", get(serve_swagger_ui_bundle_js))
        .route("/swagger-ui-standalone-preset.js", get(serve_swagger_ui_standalone_preset_js))
        .route("/swagger-initializer.js", get(serve_swagger_initializer_js));

    let app = if config.rate_limit_login_per_minute > 0 {
        let period_secs = (60 / config.rate_limit_login_per_minute).max(1) as u64;
        let governor_conf = GovernorConfigBuilder::default()
            .per_second(period_secs)
            .burst_size(config.rate_limit_login_per_minute)
            .use_headers()
            .finish()
            .expect("rate limit config inválido");
        let login_router = Router::new()
            .route("/api/auth/login", post(login))
            .layer(GovernorLayer::new(governor_conf))
            .with_state(state.clone());
        login_router
            .merge(rest_routes.with_state(state))
            .layer(cors)
    } else {
        rest_routes
            .route("/api/auth/login", post(login))
            .with_state(state)
            .layer(cors)
    };

    app
}

/// Redirige /swagger-ui al index (path vacío).
async fn serve_swagger_ui_root(State(s): State<AppState>) -> axum::response::Response {
    serve_swagger_ui(axum::extract::Path(String::new()), State(s)).await
}

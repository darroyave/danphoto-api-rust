// Sirve OpenAPI JSON y Swagger UI usando el mismo AppState que el resto de la API.

use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    Json,
};

use super::handlers::eventos::ApiDoc;
use super::state::AppState;
use std::sync::OnceLock;
use utoipa::OpenApi;
use utoipa_swagger_ui::Config;

static SWAGGER_CONFIG: OnceLock<std::sync::Arc<Config<'static>>> = OnceLock::new();

fn get_config() -> std::sync::Arc<Config<'static>> {
    SWAGGER_CONFIG
        .get_or_init(|| std::sync::Arc::new(Config::from("/api-docs/openapi.json")))
        .clone()
}

/// Sirve el JSON de OpenAPI (para que Swagger UI lo cargue).
pub async fn serve_openapi_json(State(_): State<AppState>) -> impl IntoResponse {
    Json(ApiDoc::openapi())
}

/// Sirve un archivo estático de Swagger UI (path vacío = index, sino el archivo pedido).
pub async fn serve_swagger_ui(
    Path(path): Path<String>,
    State(_): State<AppState>,
) -> Response {
    let path = path.trim().trim_start_matches('/');
    let path = if path.is_empty() { "/" } else { path };
    serve_swagger_file(path).await
}

async fn serve_swagger_file(path: &str) -> Response {
    let config = get_config();
    match utoipa_swagger_ui::serve(path, config) {
        Ok(Some(file)) => (
            [(
                axum::http::header::CONTENT_TYPE,
                axum::http::HeaderValue::from_str(&file.content_type)
                    .unwrap_or_else(|_| axum::http::HeaderValue::from_static("application/octet-stream")),
            )],
            file.bytes.to_vec(),
        )
            .into_response(),
        Ok(None) => axum::http::StatusCode::NOT_FOUND.into_response(),
        Err(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// Handlers para assets que el HTML de Swagger UI pide en la raíz (/swagger-ui.css, etc.).
pub async fn serve_swagger_ui_css(State(_): State<AppState>) -> Response {
    serve_swagger_file("swagger-ui.css").await
}
pub async fn serve_index_css(State(_): State<AppState>) -> Response {
    serve_swagger_file("index.css").await
}
pub async fn serve_swagger_ui_bundle_js(State(_): State<AppState>) -> Response {
    serve_swagger_file("swagger-ui-bundle.js").await
}
pub async fn serve_swagger_ui_standalone_preset_js(State(_): State<AppState>) -> Response {
    serve_swagger_file("swagger-ui-standalone-preset.js").await
}
pub async fn serve_swagger_initializer_js(State(_): State<AppState>) -> Response {
    serve_swagger_file("swagger-initializer.js").await
}

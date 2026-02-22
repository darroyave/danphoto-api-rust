// Autenticación JWT: login contra tabla usuarios y extractor Bearer para proteger rutas.

use axum::{
    extract::{FromRef, FromRequestParts, State},
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use uuid::Uuid;

use super::error::ApiError;
use super::state::AppState;

/// Resuelve el user_id del usuario autenticado a partir del email (sub del JWT).
/// Usado por handlers que necesitan el id (favoritos, sesiones desde favoritos, perfil).
pub async fn user_id_from_auth(state: &AppState, email: &str) -> Result<Uuid, ApiError> {
    let user = state
        .auth_repository
        .get_by_email(email)
        .await
        .map_err(ApiError::from)?;
    user.map(|u| u.id).ok_or_else(|| {
        ApiError(crate::domain::DomainError::NotFound(
            "Usuario no encontrado".to_string(),
        ))
    })
}

/// Claims del JWT (sub = email del usuario).
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}

/// Request de login (email + password).
#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Response con el token Bearer.
#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
    pub token_type: String,
}

/// Extractor que exige `Authorization: Bearer <token>` válido (el String es el email del token).
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct BearerAuth(pub String);

impl BearerAuth {
    fn from_header_and_secret(
        auth_header: Option<&axum::http::HeaderValue>,
        secret: &[u8],
    ) -> Result<Self, AuthError> {
        let header = auth_header.ok_or(AuthError::Missing)?;
        let s = header.to_str().map_err(|_| AuthError::Invalid)?;
        let token = s.strip_prefix("Bearer ").ok_or(AuthError::Invalid)?;
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret),
            &Validation::default(),
        )
        .map_err(|_| AuthError::Invalid)?;
        Ok(BearerAuth(token_data.claims.sub))
    }
}

#[derive(Debug)]
pub enum AuthError {
    Missing,
    Invalid,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            AuthError::Missing => (StatusCode::UNAUTHORIZED, "Authorization header missing"),
            AuthError::Invalid => (StatusCode::UNAUTHORIZED, "Invalid or expired token"),
        };
        (status, Json(serde_json::json!({ "error": msg }))).into_response()
    }
}

impl<S> FromRequestParts<S> for BearerAuth
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = Response;

    fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        let secret = AppState::from_ref(state).jwt_secret.clone();
        let auth = parts.headers.get(AUTHORIZATION).cloned();
        async move {
            BearerAuth::from_header_and_secret(auth.as_ref(), secret.as_bytes())
                .map_err(IntoResponse::into_response)
        }
    }
}

/// Genera un JWT para el usuario dado (sub = email).
fn create_token(email: &str, secret: &[u8], exp_secs: i64) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = chrono::Utc::now().timestamp() + exp_secs;
    let claims = Claims {
        sub: email.to_string(),
        exp,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
}

/// Login: busca usuario en tabla usuarios, verifica password y devuelve JWT.
#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Token JWT", body = LoginResponse),
        (status = 401, description = "Usuario o contraseña incorrectos"),
    ),
)]
pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<serde_json::Value>)> {
    let email = body.email.trim();
    let user: Option<crate::domain::AuthUser> = state
        .auth_repository
        .get_by_email(email)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Error al buscar usuario" })),
            )
        })?;

    let user = user.ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({ "error": "Usuario o contraseña incorrectos" })),
        )
    })?;

    let ok = bcrypt::verify(&body.password, &user.password_hash).unwrap_or(false);
    if !ok {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({ "error": "Usuario o contraseña incorrectos" })),
        ));
    }

    let token = create_token(&user.email, state.jwt_secret.as_bytes(), 24 * 3600)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Error generando token" })),
            )
        })?;

    Ok(Json(LoginResponse {
        token,
        token_type: "Bearer".to_string(),
    }))
}


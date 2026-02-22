use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use crate::domain::DomainError;

#[derive(Debug)]
pub struct ApiError(pub DomainError);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self.0 {
            DomainError::NotFound(_) => (StatusCode::NOT_FOUND, self.0.to_string()),
            DomainError::Validation(_) => (StatusCode::BAD_REQUEST, self.0.to_string()),
            DomainError::Repository(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error interno del servidor".to_string(),
            ),
        };
        (
            status,
            Json(json!({ "error": message })),
        )
            .into_response()
    }
}

impl From<DomainError> for ApiError {
    fn from(e: DomainError) -> Self {
        ApiError(e)
    }
}

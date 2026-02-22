// Capa de presentación: REST API (Axum)

pub mod auth;
pub mod dto;
pub mod error;
pub mod handlers;
pub mod routes;
pub mod state;
pub mod swagger;

pub use error::ApiError;
pub use routes::create_router;
pub use state::AppState;
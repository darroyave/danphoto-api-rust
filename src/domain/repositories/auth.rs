// Contrato de autenticación (login por email)

use async_trait::async_trait;
use uuid::Uuid;

use super::error::DomainError;

/// Usuario para autenticación (login): id, email y hash de contraseña.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AuthUser {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
}

#[async_trait]
pub trait AuthRepository: Send + Sync {
    /// Busca un usuario por email (para login).
    async fn get_by_email(&self, email: &str) -> Result<Option<AuthUser>, DomainError>;
}

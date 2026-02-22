// Casos de uso de Usuario/Perfil (Kotlin domain/cases/usuario). user_id viene del JWT.

use crate::domain::{DomainError, Usuario, UsuariosRepository};
use std::sync::Arc;
use uuid::Uuid;

/// Perfil del usuario autenticado (datos sin password).
#[derive(Clone)]
pub struct GetProfileUseCase {
    repo: Arc<dyn UsuariosRepository>,
}

impl GetProfileUseCase {
    pub fn new(repo: Arc<dyn UsuariosRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, user_id: Uuid) -> Result<Option<Usuario>, DomainError> {
        self.repo.get_by_id(user_id).await
    }
}

#[derive(Clone)]
pub struct UpdateUsuarioUseCase {
    repo: Arc<dyn UsuariosRepository>,
}

impl UpdateUsuarioUseCase {
    pub fn new(repo: Arc<dyn UsuariosRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid, name: Option<&str>) -> Result<Option<Usuario>, DomainError> {
        self.repo.update_name(id, name).await
    }
}

#[derive(Clone)]
pub struct UpdateUsuarioAvatarUseCase {
    repo: Arc<dyn UsuariosRepository>,
}

impl UpdateUsuarioAvatarUseCase {
    pub fn new(repo: Arc<dyn UsuariosRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid, url: &str) -> Result<Option<Usuario>, DomainError> {
        self.repo.update_avatar(id, url).await
    }
}

// Casos de uso de Eventos (orquestan el repositorio)

use crate::domain::{DomainError, Evento, EventosRepository};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct GetEventosUseCase {
    repo: Arc<dyn EventosRepository>,
}

impl GetEventosUseCase {
    pub fn new(repo: Arc<dyn EventosRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self) -> Result<Vec<Evento>, DomainError> {
        self.repo.get_all().await
    }
}

#[derive(Clone)]
pub struct GetEventoByIdUseCase {
    repo: Arc<dyn EventosRepository>,
}

impl GetEventoByIdUseCase {
    pub fn new(repo: Arc<dyn EventosRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid) -> Result<Evento, DomainError> {
        self.repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Evento no encontrado: {}", id)))
    }
}

#[derive(Clone)]
pub struct CreateEventoUseCase {
    repo: Arc<dyn EventosRepository>,
}

impl CreateEventoUseCase {
    pub fn new(repo: Arc<dyn EventosRepository>) -> Self {
        Self { repo }
    }

    /// Crea un evento con id conocido (imagen guardada como {id}.{ext}).
    pub async fn execute_with_id(
        &self,
        id: Uuid,
        name: &str,
        place: &str,
        url: &str,
        mmdd: &str,
    ) -> Result<Evento, DomainError> {
        if mmdd.trim().is_empty() {
            return Err(DomainError::Validation(
                "El campo fecha (MMdd) es requerido".to_string(),
            ));
        }
        self.repo.create_with_id(id, name, place, url, mmdd).await
    }
}

#[derive(Clone)]
pub struct UpdateEventoUseCase {
    repo: Arc<dyn EventosRepository>,
}

impl UpdateEventoUseCase {
    pub fn new(repo: Arc<dyn EventosRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        id: Uuid,
        name: Option<&str>,
        place: Option<&str>,
        url: Option<&str>,
        mmdd: Option<&str>,
    ) -> Result<Evento, DomainError> {
        self.repo
            .update(id, name, place, url, mmdd)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Evento no encontrado: {}", id)))
    }
}

#[derive(Clone)]
pub struct DeleteEventoUseCase {
    repo: Arc<dyn EventosRepository>,
}

impl DeleteEventoUseCase {
    pub fn new(repo: Arc<dyn EventosRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid) -> Result<(), DomainError> {
        self.repo.delete(id).await
    }
}

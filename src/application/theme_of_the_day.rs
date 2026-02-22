// Casos de uso de Theme of the Day (orquestan el repositorio)

use crate::domain::{DomainError, ThemeOfTheDay, ThemeOfTheDayRepository};
use std::sync::Arc;

#[derive(Clone)]
pub struct GetThemeOfTheDayAllUseCase {
    repo: Arc<dyn ThemeOfTheDayRepository>,
}

impl GetThemeOfTheDayAllUseCase {
    pub fn new(repo: Arc<dyn ThemeOfTheDayRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self) -> Result<Vec<ThemeOfTheDay>, DomainError> {
        self.repo.get_all().await
    }
}

/// Obtiene el tema del día de hoy (id = MMdd de la fecha actual). Equivalente a Kotlin getThemeOfTheDay().
#[derive(Clone)]
pub struct GetThemeOfTheDayTodayUseCase {
    repo: Arc<dyn ThemeOfTheDayRepository>,
}

impl GetThemeOfTheDayTodayUseCase {
    pub fn new(repo: Arc<dyn ThemeOfTheDayRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self) -> Result<ThemeOfTheDay, DomainError> {
        let mmdd = chrono::Utc::now().format("%m%d").to_string();
        self.repo
            .get_by_id(&mmdd)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("No hay tema del día para hoy ({}).", mmdd)))
    }
}

#[derive(Clone)]
pub struct GetThemeOfTheDayByIdUseCase {
    repo: Arc<dyn ThemeOfTheDayRepository>,
}

impl GetThemeOfTheDayByIdUseCase {
    pub fn new(repo: Arc<dyn ThemeOfTheDayRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: &str) -> Result<ThemeOfTheDay, DomainError> {
        self.repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Tema del día no encontrado: {}", id)))
    }
}

#[derive(Clone)]
pub struct CreateThemeOfTheDayUseCase {
    repo: Arc<dyn ThemeOfTheDayRepository>,
}

impl CreateThemeOfTheDayUseCase {
    pub fn new(repo: Arc<dyn ThemeOfTheDayRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: &str, name: &str, url: &str) -> Result<ThemeOfTheDay, DomainError> {
        if id.trim().is_empty() {
            return Err(DomainError::Validation(
                "El campo id (MMdd) es requerido".to_string(),
            ));
        }
        if id.len() != 4 {
            return Err(DomainError::Validation(
                "El id debe tener exactamente 4 caracteres (formato MMdd)".to_string(),
            ));
        }
        self.repo.create(id, name, url).await
    }
}

#[derive(Clone)]
pub struct UpdateThemeOfTheDayUseCase {
    repo: Arc<dyn ThemeOfTheDayRepository>,
}

impl UpdateThemeOfTheDayUseCase {
    pub fn new(repo: Arc<dyn ThemeOfTheDayRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        id: &str,
        name: Option<&str>,
        url: Option<&str>,
    ) -> Result<ThemeOfTheDay, DomainError> {
        self.repo
            .update(id, name, url)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Tema del día no encontrado: {}", id)))
    }
}

#[derive(Clone)]
pub struct DeleteThemeOfTheDayUseCase {
    repo: Arc<dyn ThemeOfTheDayRepository>,
}

impl DeleteThemeOfTheDayUseCase {
    pub fn new(repo: Arc<dyn ThemeOfTheDayRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: &str) -> Result<(), DomainError> {
        self.repo.delete(id).await
    }
}

// Contrato del repositorio de tema del día

use async_trait::async_trait;

use crate::domain::ThemeOfTheDay;

use super::error::DomainError;

#[async_trait]
pub trait ThemeOfTheDayRepository: Send + Sync {
    async fn get_all(&self) -> Result<Vec<ThemeOfTheDay>, DomainError>;
    async fn get_by_id(&self, id: &str) -> Result<Option<ThemeOfTheDay>, DomainError>;
    async fn create(&self, id: &str, name: &str, url: &str) -> Result<ThemeOfTheDay, DomainError>;
    async fn update(
        &self,
        id: &str,
        name: Option<&str>,
        url: Option<&str>,
    ) -> Result<Option<ThemeOfTheDay>, DomainError>;
    async fn delete(&self, id: &str) -> Result<(), DomainError>;
}

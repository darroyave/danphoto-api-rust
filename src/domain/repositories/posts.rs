// Contrato del repositorio de posts

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::Post;

use super::error::DomainError;

#[async_trait]
pub trait PostsRepository: Send + Sync {
    async fn get_all(&self) -> Result<Vec<Post>, DomainError>;
    async fn get_paginated(&self, page: u32, limit: u32) -> Result<Vec<Post>, DomainError>;
    async fn get_by_theme_of_the_day_id(
        &self,
        theme_of_the_day_id: &str,
    ) -> Result<Vec<Post>, DomainError>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Post>, DomainError>;
    async fn create(
        &self,
        description: Option<&str>,
        url: Option<&str>,
        user_id: Option<Uuid>,
    ) -> Result<Post, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}

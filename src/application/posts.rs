// Casos de uso de Posts (Kotlin domain/cases/posts)

use crate::domain::{DomainError, Post, PostsRepository};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct GetPostsUseCase {
    repo: Arc<dyn PostsRepository>,
}

impl GetPostsUseCase {
    pub fn new(repo: Arc<dyn PostsRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self) -> Result<Vec<Post>, DomainError> {
        self.repo.get_all().await
    }
}

#[derive(Clone)]
pub struct GetPostsPaginatedUseCase {
    repo: Arc<dyn PostsRepository>,
}

impl GetPostsPaginatedUseCase {
    pub fn new(repo: Arc<dyn PostsRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, page: u32, limit: u32) -> Result<Vec<Post>, DomainError> {
        self.repo.get_paginated(page, limit).await
    }
}

#[derive(Clone)]
pub struct GetPostsByThemeOfTheDayIdUseCase {
    repo: Arc<dyn PostsRepository>,
}

impl GetPostsByThemeOfTheDayIdUseCase {
    pub fn new(repo: Arc<dyn PostsRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, theme_of_the_day_id: &str) -> Result<Vec<Post>, DomainError> {
        self.repo.get_by_theme_of_the_day_id(theme_of_the_day_id).await
    }
}

#[derive(Clone)]
pub struct GetPostByIdUseCase {
    repo: Arc<dyn PostsRepository>,
}

impl GetPostByIdUseCase {
    pub fn new(repo: Arc<dyn PostsRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid) -> Result<Post, DomainError> {
        self.repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Post no encontrado: {}", id)))
    }
}

#[derive(Clone)]
pub struct CreatePostUseCase {
    repo: Arc<dyn PostsRepository>,
}

impl CreatePostUseCase {
    pub fn new(repo: Arc<dyn PostsRepository>) -> Self {
        Self { repo }
    }

    /// Crea un post con id conocido (para imágenes guardadas como {id}.{ext}).
    pub async fn execute_with_id(
        &self,
        id: Uuid,
        description: Option<&str>,
        url: Option<&str>,
        user_id: Option<Uuid>,
    ) -> Result<Post, DomainError> {
        self.repo.create_with_id(id, description, url, user_id).await
    }
}

#[derive(Clone)]
pub struct DeletePostUseCase {
    repo: Arc<dyn PostsRepository>,
}

impl DeletePostUseCase {
    pub fn new(repo: Arc<dyn PostsRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid) -> Result<(), DomainError> {
        self.repo.delete(id).await
    }
}

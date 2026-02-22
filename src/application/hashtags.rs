// Casos de uso de Hashtags (alineados con Kotlin domain/cases/hashtags)

use crate::domain::{DomainError, Hashtag, HashtagsRepository};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct GetHashtagsUseCase {
    repo: Arc<dyn HashtagsRepository>,
}

impl GetHashtagsUseCase {
    pub fn new(repo: Arc<dyn HashtagsRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self) -> Result<Vec<Hashtag>, DomainError> {
        self.repo.get_all().await
    }
}

#[derive(Clone)]
pub struct GetHashtagByIdUseCase {
    repo: Arc<dyn HashtagsRepository>,
}

impl GetHashtagByIdUseCase {
    pub fn new(repo: Arc<dyn HashtagsRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid) -> Result<Hashtag, DomainError> {
        self.repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Hashtag no encontrado: {}", id)))
    }
}

#[derive(Clone)]
pub struct CreateHashtagUseCase {
    repo: Arc<dyn HashtagsRepository>,
}

impl CreateHashtagUseCase {
    pub fn new(repo: Arc<dyn HashtagsRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, name: &str) -> Result<Hashtag, DomainError> {
        let name = name.trim();
        if name.is_empty() {
            return Err(DomainError::Validation("El nombre del hashtag es requerido".to_string()));
        }
        self.repo.create(name).await
    }
}

#[derive(Clone)]
pub struct DeleteHashtagUseCase {
    repo: Arc<dyn HashtagsRepository>,
}

impl DeleteHashtagUseCase {
    pub fn new(repo: Arc<dyn HashtagsRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid) -> Result<(), DomainError> {
        self.repo.delete(id).await
    }
}

/// GetHashtagsByPoseUseCase - hashtags asociados a una pose (Kotlin GetHashtagsByPoseUseCase).
#[derive(Clone)]
pub struct GetHashtagsByPoseUseCase {
    repo: Arc<dyn HashtagsRepository>,
}

impl GetHashtagsByPoseUseCase {
    pub fn new(repo: Arc<dyn HashtagsRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, pose_id: Uuid) -> Result<Vec<Hashtag>, DomainError> {
        self.repo.get_hashtags_by_pose(pose_id).await
    }
}

/// AddHashtagsToPostUseCase - añade hashtags a un post (Kotlin AddHashtagsToPostUseCase).
#[derive(Clone)]
pub struct AddHashtagsToPostUseCase {
    repo: Arc<dyn HashtagsRepository>,
}

impl AddHashtagsToPostUseCase {
    pub fn new(repo: Arc<dyn HashtagsRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, post_id: Uuid, hashtag_ids: &[Uuid]) -> Result<(), DomainError> {
        self.repo.add_hashtags_to_post(post_id, hashtag_ids).await
    }
}

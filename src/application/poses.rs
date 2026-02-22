// Casos de uso de Poses (Kotlin domain/cases/poses)

use crate::domain::{DomainError, HashtagsRepository, Pose, PosesRepository};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct GetPosesUseCase {
    repo: Arc<dyn PosesRepository>,
}

impl GetPosesUseCase {
    pub fn new(repo: Arc<dyn PosesRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self) -> Result<Vec<Pose>, DomainError> {
        self.repo.get_all().await
    }
}

#[derive(Clone)]
pub struct GetPosesPaginatedUseCase {
    repo: Arc<dyn PosesRepository>,
}

impl GetPosesPaginatedUseCase {
    pub fn new(repo: Arc<dyn PosesRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, page: u32, limit: u32) -> Result<Vec<Pose>, DomainError> {
        self.repo.get_paginated(page, limit).await
    }
}

#[derive(Clone)]
pub struct GetPoseByIdUseCase {
    repo: Arc<dyn PosesRepository>,
}

impl GetPoseByIdUseCase {
    pub fn new(repo: Arc<dyn PosesRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid) -> Result<Pose, DomainError> {
        self.repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Pose no encontrada: {}", id)))
    }
}

#[derive(Clone)]
pub struct CreatePoseUseCase {
    repo: Arc<dyn PosesRepository>,
}

impl CreatePoseUseCase {
    pub fn new(repo: Arc<dyn PosesRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, name: Option<&str>, url: &str) -> Result<Pose, DomainError> {
        if url.trim().is_empty() {
            return Err(DomainError::Validation("La URL es requerida".to_string()));
        }
        self.repo.create(name, url).await
    }

    /// Crea una pose con id conocido (para imágenes guardadas como {id}.{ext}).
    pub async fn execute_with_id(&self, id: Uuid, name: Option<&str>, url: &str) -> Result<Pose, DomainError> {
        if url.trim().is_empty() {
            return Err(DomainError::Validation("La URL es requerida".to_string()));
        }
        self.repo.create_with_id(id, name, url).await
    }
}

#[derive(Clone)]
pub struct DeletePoseUseCase {
    poses_repo: Arc<dyn PosesRepository>,
    hashtags_repo: Arc<dyn crate::domain::HashtagsRepository>,
}

impl DeletePoseUseCase {
    pub fn new(
        poses_repo: Arc<dyn PosesRepository>,
        hashtags_repo: Arc<dyn crate::domain::HashtagsRepository>,
    ) -> Self {
        Self {
            poses_repo,
            hashtags_repo,
        }
    }

    pub async fn execute(&self, id: Uuid) -> Result<(), DomainError> {
        let _ = self.hashtags_repo.remove_all_hashtags_from_pose(id).await;
        self.poses_repo.delete(id).await
    }
}

#[derive(Clone)]
pub struct GetPosesByHashtagUseCase {
    repo: Arc<dyn HashtagsRepository>,
}

impl GetPosesByHashtagUseCase {
    pub fn new(repo: Arc<dyn HashtagsRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, hashtag_id: Uuid) -> Result<Vec<Pose>, DomainError> {
        self.repo.get_poses_by_hashtag(hashtag_id).await
    }
}

#[derive(Clone)]
pub struct GetPosesByHashtagPaginatedUseCase {
    repo: Arc<dyn HashtagsRepository>,
}

impl GetPosesByHashtagPaginatedUseCase {
    pub fn new(repo: Arc<dyn HashtagsRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        hashtag_id: Uuid,
        page: u32,
        limit: u32,
    ) -> Result<Vec<Pose>, DomainError> {
        self.repo
            .get_poses_by_hashtag_paginated(hashtag_id, page, limit)
            .await
    }
}

#[derive(Clone)]
pub struct UpdatePoseHashtagsUseCase {
    repo: Arc<dyn HashtagsRepository>,
}

impl UpdatePoseHashtagsUseCase {
    pub fn new(repo: Arc<dyn HashtagsRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        pose_id: Uuid,
        hashtag_ids: &[Uuid],
    ) -> Result<(), DomainError> {
        let current = self.repo.get_hashtags_by_pose(pose_id).await?;
        let current_ids: std::collections::HashSet<Uuid> =
            current.into_iter().map(|h| h.id).collect();
        let new_ids: std::collections::HashSet<Uuid> =
            hashtag_ids.iter().copied().collect();
        for id in &new_ids {
            if !current_ids.contains(id) {
                let _ = self.repo.add_hashtag_to_pose(pose_id, *id).await;
            }
        }
        for id in &current_ids {
            if !new_ids.contains(id) {
                let _ = self.repo.remove_hashtag_from_pose(pose_id, *id).await;
            }
        }
        Ok(())
    }
}

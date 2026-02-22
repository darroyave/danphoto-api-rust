// Casos de uso de Favoritos (Kotlin domain/cases/favorites). Requieren user_id (del JWT).

use crate::domain::{DomainError, FavoritesRepository, Pose};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct GetFavoritePosesUseCase {
    repo: Arc<dyn FavoritesRepository>,
}

impl GetFavoritePosesUseCase {
    pub fn new(repo: Arc<dyn FavoritesRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, user_id: Uuid) -> Result<Vec<Pose>, DomainError> {
        self.repo.get_favorite_poses(user_id).await
    }
}

#[derive(Clone)]
pub struct AddPoseToFavoritesUseCase {
    repo: Arc<dyn FavoritesRepository>,
}

impl AddPoseToFavoritesUseCase {
    pub fn new(repo: Arc<dyn FavoritesRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, user_id: Uuid, pose_id: Uuid) -> Result<(), DomainError> {
        self.repo.add_pose_to_favorites(user_id, pose_id).await
    }
}

#[derive(Clone)]
pub struct RemovePoseFromFavoritesUseCase {
    repo: Arc<dyn FavoritesRepository>,
}

impl RemovePoseFromFavoritesUseCase {
    pub fn new(repo: Arc<dyn FavoritesRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, user_id: Uuid, pose_id: Uuid) -> Result<(), DomainError> {
        self.repo.remove_pose_from_favorites(user_id, pose_id).await
    }
}

#[derive(Clone)]
pub struct IsPoseFavoriteUseCase {
    repo: Arc<dyn FavoritesRepository>,
}

impl IsPoseFavoriteUseCase {
    pub fn new(repo: Arc<dyn FavoritesRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, user_id: Uuid, pose_id: Uuid) -> Result<bool, DomainError> {
        self.repo.is_pose_favorite(user_id, pose_id).await
    }
}

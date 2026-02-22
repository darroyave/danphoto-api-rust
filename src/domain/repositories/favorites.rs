// Contrato del repositorio de favoritos (por usuario autenticado)

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::Pose;

use super::error::DomainError;

#[async_trait]
pub trait FavoritesRepository: Send + Sync {
    async fn is_pose_favorite(&self, user_id: Uuid, pose_id: Uuid) -> Result<bool, DomainError>;
    async fn add_pose_to_favorites(
        &self,
        user_id: Uuid,
        pose_id: Uuid,
    ) -> Result<(), DomainError>;
    async fn remove_pose_from_favorites(
        &self,
        user_id: Uuid,
        pose_id: Uuid,
    ) -> Result<(), DomainError>;
    /// Quita varias poses de favoritos del usuario (para AddFavoritesToSesion / CreateSesionFromFavorites).
    async fn remove_poses_from_favorites(
        &self,
        user_id: Uuid,
        pose_ids: &[Uuid],
    ) -> Result<(), DomainError>;
    async fn get_favorite_poses(&self, user_id: Uuid) -> Result<Vec<Pose>, DomainError>;
}

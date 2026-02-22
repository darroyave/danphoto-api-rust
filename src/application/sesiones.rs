// Casos de uso de Sesiones (Kotlin domain/cases/sesiones)

use crate::domain::{DomainError, FavoritesRepository, Pose, Sesion, SesionesRepository};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct GetSesionesUseCase {
    repo: Arc<dyn SesionesRepository>,
}

impl GetSesionesUseCase {
    pub fn new(repo: Arc<dyn SesionesRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self) -> Result<Vec<Sesion>, DomainError> {
        self.repo.get_all().await
    }
}

#[derive(Clone)]
pub struct GetSesionByIdUseCase {
    repo: Arc<dyn SesionesRepository>,
}

impl GetSesionByIdUseCase {
    pub fn new(repo: Arc<dyn SesionesRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid) -> Result<Option<Sesion>, DomainError> {
        self.repo.get_by_id(id).await
    }
}

#[derive(Clone)]
pub struct GetPosesBySesionUseCase {
    repo: Arc<dyn SesionesRepository>,
}

impl GetPosesBySesionUseCase {
    pub fn new(repo: Arc<dyn SesionesRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, sesion_id: Uuid) -> Result<Vec<Pose>, DomainError> {
        self.repo.get_poses_by_sesion(sesion_id).await
    }
}

#[derive(Clone)]
pub struct CreateSesionUseCase {
    repo: Arc<dyn SesionesRepository>,
}

impl CreateSesionUseCase {
    pub fn new(repo: Arc<dyn SesionesRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, name: &str) -> Result<Sesion, DomainError> {
        self.repo.create(name).await
    }
}

#[derive(Clone)]
pub struct DeleteSesionUseCase {
    repo: Arc<dyn SesionesRepository>,
}

impl DeleteSesionUseCase {
    pub fn new(repo: Arc<dyn SesionesRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid) -> Result<(), DomainError> {
        self.repo.delete(id).await
    }
}

#[derive(Clone)]
pub struct AddPosesToSesionUseCase {
    repo: Arc<dyn SesionesRepository>,
}

impl AddPosesToSesionUseCase {
    pub fn new(repo: Arc<dyn SesionesRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        sesion_id: Uuid,
        pose_ids: &[Uuid],
    ) -> Result<(), DomainError> {
        self.repo.add_poses_to_sesion(sesion_id, pose_ids).await
    }
}

#[derive(Clone)]
pub struct RemovePoseFromSesionUseCase {
    repo: Arc<dyn SesionesRepository>,
}

impl RemovePoseFromSesionUseCase {
    pub fn new(repo: Arc<dyn SesionesRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, sesion_id: Uuid, pose_id: Uuid) -> Result<(), DomainError> {
        self.repo.remove_pose_from_sesion(sesion_id, pose_id).await
    }
}

#[derive(Clone)]
pub struct UpdateSesionCoverUseCase {
    repo: Arc<dyn SesionesRepository>,
}

impl UpdateSesionCoverUseCase {
    pub fn new(repo: Arc<dyn SesionesRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, sesion_id: Uuid, cover_url: &str) -> Result<Option<Sesion>, DomainError> {
        self.repo.update_cover(sesion_id, cover_url).await
    }
}

/// Añade las poses favoritas del usuario a una sesión existente y las quita de favoritos.
#[derive(Clone)]
pub struct AddFavoritesToSesionUseCase {
    sesiones_repo: Arc<dyn SesionesRepository>,
    favorites_repo: Arc<dyn FavoritesRepository>,
}

impl AddFavoritesToSesionUseCase {
    pub fn new(
        sesiones_repo: Arc<dyn SesionesRepository>,
        favorites_repo: Arc<dyn FavoritesRepository>,
    ) -> Self {
        Self {
            sesiones_repo,
            favorites_repo,
        }
    }

    pub async fn execute(
        &self,
        user_id: Uuid,
        sesion_id: Uuid,
    ) -> Result<(), DomainError> {
        let poses = self.favorites_repo.get_favorite_poses(user_id).await?;
        let pose_ids: Vec<Uuid> = poses.into_iter().map(|p| p.id).collect();
        if pose_ids.is_empty() {
            return Ok(());
        }
        self.sesiones_repo.add_poses_to_sesion(sesion_id, &pose_ids).await?;
        self.favorites_repo.remove_poses_from_favorites(user_id, &pose_ids).await
    }
}

/// Crea una sesión nueva con el nombre dado y mueve las poses favoritas del usuario a ella (luego las quita de favoritos).
#[derive(Clone)]
pub struct CreateSesionFromFavoritesUseCase {
    sesiones_repo: Arc<dyn SesionesRepository>,
    favorites_repo: Arc<dyn FavoritesRepository>,
}

impl CreateSesionFromFavoritesUseCase {
    pub fn new(
        sesiones_repo: Arc<dyn SesionesRepository>,
        favorites_repo: Arc<dyn FavoritesRepository>,
    ) -> Self {
        Self {
            sesiones_repo,
            favorites_repo,
        }
    }

    pub async fn execute(&self, user_id: Uuid, name: &str) -> Result<Sesion, DomainError> {
        let sesion = self.sesiones_repo.create(name).await?;
        let poses = self.favorites_repo.get_favorite_poses(user_id).await?;
        let pose_ids: Vec<Uuid> = poses.into_iter().map(|p| p.id).collect();
        if !pose_ids.is_empty() {
            self.sesiones_repo.add_poses_to_sesion(sesion.id, &pose_ids).await?;
            self.favorites_repo.remove_poses_from_favorites(user_id, &pose_ids).await?;
        }
        Ok(sesion)
    }
}

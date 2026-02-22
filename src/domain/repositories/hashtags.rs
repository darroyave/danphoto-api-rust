// Contrato del repositorio de hashtags (catálogo + relación con poses y posts)

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{Hashtag, Pose};

use super::error::DomainError;

#[async_trait]
pub trait HashtagsRepository: Send + Sync {
    async fn get_all(&self) -> Result<Vec<Hashtag>, DomainError>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Hashtag>, DomainError>;
    async fn create(&self, name: &str) -> Result<Hashtag, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
    /// Hashtags asociados a una pose (tabla hashtag_image).
    async fn get_hashtags_by_pose(&self, pose_id: Uuid) -> Result<Vec<Hashtag>, DomainError>;
    /// Añade hashtags a un post (tabla hashtag_pose). Idempotente por (post_id, hashtag_id).
    async fn add_hashtags_to_post(
        &self,
        post_id: Uuid,
        hashtag_ids: &[Uuid],
    ) -> Result<(), DomainError>;
    /// Añade un hashtag a una pose (hashtag_image). Idempotente.
    async fn add_hashtag_to_pose(&self, pose_id: Uuid, hashtag_id: Uuid) -> Result<(), DomainError>;
    /// Quita un hashtag de una pose.
    async fn remove_hashtag_from_pose(
        &self,
        pose_id: Uuid,
        hashtag_id: Uuid,
    ) -> Result<(), DomainError>;
    /// Quita todos los hashtags de una pose.
    async fn remove_all_hashtags_from_pose(&self, pose_id: Uuid) -> Result<(), DomainError>;
    /// Poses etiquetadas con un hashtag.
    async fn get_poses_by_hashtag(&self, hashtag_id: Uuid) -> Result<Vec<Pose>, DomainError>;
    /// Poses etiquetadas con un hashtag (paginado).
    async fn get_poses_by_hashtag_paginated(
        &self,
        hashtag_id: Uuid,
        page: u32,
        limit: u32,
    ) -> Result<Vec<Pose>, DomainError>;
}

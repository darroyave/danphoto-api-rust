// Casos de uso de Places (Kotlin domain/cases/places)

use crate::domain::{DomainError, Place, PlacesRepository};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct GetPlacesUseCase {
    repo: Arc<dyn PlacesRepository>,
}

impl GetPlacesUseCase {
    pub fn new(repo: Arc<dyn PlacesRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self) -> Result<Vec<Place>, DomainError> {
        self.repo.get_all().await
    }
}

#[derive(Clone)]
pub struct GetPlaceByIdUseCase {
    repo: Arc<dyn PlacesRepository>,
}

impl GetPlaceByIdUseCase {
    pub fn new(repo: Arc<dyn PlacesRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid) -> Result<Option<Place>, DomainError> {
        self.repo.get_by_id(id).await
    }
}

#[derive(Clone)]
pub struct CreatePlaceUseCase {
    repo: Arc<dyn PlacesRepository>,
}

impl CreatePlaceUseCase {
    pub fn new(repo: Arc<dyn PlacesRepository>) -> Self {
        Self { repo }
    }

    /// Crea un lugar con id conocido (imagen guardada como {id}.{ext}).
    pub async fn execute_with_id(
        &self,
        id: Uuid,
        name: &str,
        description: &str,
        address: &str,
        location: &str,
        latitude: f64,
        longitude: f64,
        url: &str,
        instagram: Option<&str>,
        website: Option<&str>,
    ) -> Result<Place, DomainError> {
        self.repo
            .create_with_id(
                id,
                name,
                description,
                address,
                location,
                latitude,
                longitude,
                url,
                instagram,
                website,
            )
            .await
    }
}

#[derive(Clone)]
pub struct UpdatePlaceUseCase {
    repo: Arc<dyn PlacesRepository>,
}

impl UpdatePlaceUseCase {
    pub fn new(repo: Arc<dyn PlacesRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        id: Uuid,
        name: Option<&str>,
        description: Option<&str>,
        address: Option<&str>,
        location: Option<&str>,
        latitude: Option<f64>,
        longitude: Option<f64>,
        url: Option<&str>,
        instagram: Option<&str>,
        website: Option<&str>,
    ) -> Result<Option<Place>, DomainError> {
        self.repo
            .update(
                id,
                name,
                description,
                address,
                location,
                latitude,
                longitude,
                url,
                instagram,
                website,
            )
            .await
    }
}

#[derive(Clone)]
pub struct DeletePlaceUseCase {
    repo: Arc<dyn PlacesRepository>,
}

impl DeletePlaceUseCase {
    pub fn new(repo: Arc<dyn PlacesRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid) -> Result<(), DomainError> {
        self.repo.delete(id).await
    }
}

// Estado compartido de la API (repositorios + auth). Los handlers construyen use cases al vuelo.

use std::sync::Arc;

use crate::domain::{
    AuthRepository, EventosRepository, FavoritesRepository, HashtagsRepository,
    PlacesRepository, PortfolioRepository, PosesRepository, PostsRepository,
    SesionesRepository, ThemeOfTheDayRepository, UsuariosRepository,
};

#[derive(Clone)]
pub struct AppState {
    pub eventos_repo: Arc<dyn EventosRepository>,
    pub theme_of_the_day_repo: Arc<dyn ThemeOfTheDayRepository>,
    pub hashtags_repo: Arc<dyn HashtagsRepository>,
    pub poses_repo: Arc<dyn PosesRepository>,
    pub posts_repo: Arc<dyn PostsRepository>,
    pub portfolio_repo: Arc<dyn PortfolioRepository>,
    pub favorites_repo: Arc<dyn FavoritesRepository>,
    pub places_repo: Arc<dyn PlacesRepository>,
    pub sesiones_repo: Arc<dyn SesionesRepository>,
    pub usuarios_repo: Arc<dyn UsuariosRepository>,
    pub jwt_secret: String,
    pub auth_repository: Arc<dyn AuthRepository>,
    /// Carpeta donde se guardan las imágenes de theme-of-the-day (desde config).
    pub theme_of_the_day_images_dir: String,
    /// Carpeta donde se guardan las imágenes de poses (desde config).
    pub poses_images_dir: String,
    /// Carpeta donde se guardan las imágenes de posts (desde config).
    pub posts_images_dir: String,
    /// Carpeta donde se guardan las imágenes del portfolio (desde config).
    pub portfolio_images_dir: String,
    /// Carpeta donde se guardan las imágenes de eventos (desde config).
    pub eventos_images_dir: String,
    /// Carpeta donde se guardan las imágenes de places (desde config).
    pub places_images_dir: String,
    /// Carpeta donde se guardan los avatares de perfil (desde config).
    pub profile_avatars_dir: String,
}

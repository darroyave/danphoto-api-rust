// Contratos de repositorios (puertos) - la aplicación depende de estos traits

mod auth;
mod error;
mod eventos;
mod favorites;
mod hashtags;
mod places;
mod portfolio;
mod poses;
mod posts;
mod sesiones;
mod theme_of_the_day;
mod usuarios;

pub use auth::{AuthRepository, AuthUser};
pub use error::DomainError;
pub use eventos::EventosRepository;
pub use favorites::FavoritesRepository;
pub use hashtags::HashtagsRepository;
pub use places::PlacesRepository;
pub use portfolio::PortfolioRepository;
pub use poses::PosesRepository;
pub use posts::PostsRepository;
pub use sesiones::SesionesRepository;
pub use theme_of_the_day::ThemeOfTheDayRepository;
pub use usuarios::UsuariosRepository;

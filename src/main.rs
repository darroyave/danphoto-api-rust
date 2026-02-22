// DanPhoto API - REST con arquitectura limpia (Rust)
// Dominio -> Aplicación (casos de uso) -> Infraestructura (Postgres) -> API (Axum)

mod api;
mod application;
mod config;
mod domain;
mod infrastructure;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let config = config::Config::from_env();
    config.validate().map_err(|e| anyhow::anyhow!("{}", e))?;

    let pool = infrastructure::get_pool(&config).await?;

    // Repositorios: eventos, tema del día, hashtags
    let eventos_repo: Arc<dyn domain::EventosRepository> =
        Arc::new(infrastructure::EventosRepositoryImpl::new(pool.clone()));
    let theme_of_the_day_repo: Arc<dyn domain::ThemeOfTheDayRepository> =
        Arc::new(infrastructure::ThemeOfTheDayRepositoryImpl::new(pool.clone()));
    let hashtags_repo: Arc<dyn domain::HashtagsRepository> =
        Arc::new(infrastructure::HashtagsRepositoryImpl::new(pool.clone()));

    // Contenido: poses, posts, portfolio, lugares, sesiones
    let poses_repo: Arc<dyn domain::PosesRepository> =
        Arc::new(infrastructure::PosesRepositoryImpl::new(pool.clone()));
    let posts_repo: Arc<dyn domain::PostsRepository> =
        Arc::new(infrastructure::PostsRepositoryImpl::new(pool.clone()));
    let portfolio_repo: Arc<dyn domain::PortfolioRepository> =
        Arc::new(infrastructure::PortfolioRepositoryImpl::new(pool.clone()));
    let places_repo: Arc<dyn domain::PlacesRepository> =
        Arc::new(infrastructure::PlacesRepositoryImpl::new(pool.clone()));
    let sesiones_repo: Arc<dyn domain::SesionesRepository> =
        Arc::new(infrastructure::SesionesRepositoryImpl::new(pool.clone()));

    // Usuario: favoritos, perfil, auth
    let favorites_repo: Arc<dyn domain::FavoritesRepository> =
        Arc::new(infrastructure::FavoritesRepositoryImpl::new(pool.clone()));
    let usuarios_repo: Arc<dyn domain::UsuariosRepository> =
        Arc::new(infrastructure::UsuariosRepositoryImpl::new(pool.clone()));
    let auth_repo: Arc<dyn domain::AuthRepository> =
        Arc::new(infrastructure::AuthRepositoryImpl::new(pool));

    std::fs::create_dir_all(&config.theme_of_the_day_images_dir).ok();
    std::fs::create_dir_all(&config.poses_images_dir).ok();
    std::fs::create_dir_all(&config.posts_images_dir).ok();
    std::fs::create_dir_all(&config.portfolio_images_dir).ok();

    let state = api::AppState {
        eventos_repo,
        theme_of_the_day_repo,
        hashtags_repo,
        poses_repo,
        posts_repo,
        portfolio_repo,
        favorites_repo,
        places_repo,
        sesiones_repo,
        usuarios_repo,
        jwt_secret: config.jwt_secret.clone(),
        auth_repository: auth_repo,
        theme_of_the_day_images_dir: config.theme_of_the_day_images_dir.clone(),
        poses_images_dir: config.poses_images_dir.clone(),
        posts_images_dir: config.posts_images_dir.clone(),
        portfolio_images_dir: config.portfolio_images_dir.clone(),
    };

    let app: Router = api::create_router(state, &config).layer(TraceLayer::new_for_http());

    let bind = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&bind).await?;
    println!("DanPhoto API listening on http://{}", bind);
    // SocketAddr necesario para rate limiting por IP (tower-governor).
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;
    Ok(())
}

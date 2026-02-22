use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::time::Duration;

/// Crea el pool de PostgreSQL usando la configuración proporcionada.
pub async fn get_pool(config: &crate::config::Config) -> Result<PgPool, sqlx::Error> {
    let mut opts = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .acquire_timeout(Duration::from_secs(config.acquire_timeout_secs));

    if let Some(secs) = config.database_idle_timeout_secs {
        opts = opts.idle_timeout(Some(Duration::from_secs(secs)));
    }
    if let Some(secs) = config.database_max_lifetime_secs {
        opts = opts.max_lifetime(Some(Duration::from_secs(secs)));
    }

    opts.connect(&config.database_url).await
}

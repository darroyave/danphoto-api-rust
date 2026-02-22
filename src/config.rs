// Configuración centralizada: variables de entorno con valores por defecto y validación al arranque.

/// Valor por defecto de JWT_SECRET; si se usa en producción, el arranque falla.
pub const JWT_SECRET_DEFAULT: &str = "cambiar-en-produccion";

/// Configuración de la aplicación (lectura desde env en el arranque).
#[derive(Clone, Debug)]
pub struct Config {
    /// URL de conexión a PostgreSQL.
    pub database_url: String,
    /// Secreto para firmar/verificar JWT. En producción no debe ser el valor por defecto.
    pub jwt_secret: String,
    /// Puerto HTTP (ej. 3000).
    pub port: u16,
    /// Máximo de conexiones en el pool de Postgres.
    pub max_connections: u32,
    /// Timeout en segundos al obtener una conexión del pool.
    pub acquire_timeout_secs: u64,
    /// Máximo tiempo en segundos que una conexión puede estar idle antes de cerrarse (None = no límite).
    pub database_idle_timeout_secs: Option<u64>,
    /// Vida máxima en segundos de una conexión en el pool (None = no límite; recomendable en producción).
    pub database_max_lifetime_secs: Option<u64>,
    /// Límite de intentos de login por minuto por IP (0 = desactivado).
    pub rate_limit_login_per_minute: u32,
    /// Orígenes CORS permitidos (vacío = permitir cualquier origen, adecuado para desarrollo).
    /// En producción conviene definir `CORS_ALLOWED_ORIGINS` con orígenes separados por coma (ej. `https://app.ejemplo.com,https://admin.ejemplo.com`).
    pub cors_allowed_origins: Vec<String>,
    /// Carpeta donde se guardan las imágenes de theme-of-the-day (POST con imagen base64).
    pub theme_of_the_day_images_dir: String,
}

impl Config {
    /// Carga la configuración desde variables de entorno (tras `dotenvy::dotenv()`).
    /// Usa valores por defecto cuando la variable no está definida.
    pub fn from_env() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://localhost/danphoto".to_string()),
            jwt_secret: std::env::var("JWT_SECRET").unwrap_or_else(|_| JWT_SECRET_DEFAULT.to_string()),
            port: std::env::var("PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3000),
            max_connections: std::env::var("DATABASE_MAX_CONNECTIONS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10),
            acquire_timeout_secs: std::env::var("DATABASE_ACQUIRE_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(5),
            database_idle_timeout_secs: std::env::var("DATABASE_IDLE_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok()),
            database_max_lifetime_secs: std::env::var("DATABASE_MAX_LIFETIME_SECS")
                .ok()
                .and_then(|s| s.parse().ok()),
            rate_limit_login_per_minute: std::env::var("RATE_LIMIT_LOGIN_PER_MINUTE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(5),
            cors_allowed_origins: std::env::var("CORS_ALLOWED_ORIGINS")
                .ok()
                .map(|s| {
                    s.split(',')
                        .map(|o| o.trim().to_string())
                        .filter(|o| !o.is_empty())
                        .collect()
                })
                .unwrap_or_default(),
            theme_of_the_day_images_dir: std::env::var("THEME_OF_THE_DAY_IMAGES_DIR")
                .unwrap_or_else(|_| "./uploads/theme-of-the-day".to_string()),
        }
    }

    /// Valida la configuración. En modo producción exige un JWT_SECRET distinto del valor por defecto.
    /// Devuelve `Ok(())` o un mensaje de error para mostrar al usuario.
    pub fn validate(&self) -> Result<(), String> {
        let production = std::env::var("RUN_MODE")
            .map(|v| v.eq_ignore_ascii_case("production"))
            .unwrap_or(false);

        if production {
            if self.jwt_secret.is_empty() {
                return Err("RUN_MODE=production exige JWT_SECRET no vacío".to_string());
            }
            if self.jwt_secret == JWT_SECRET_DEFAULT {
                return Err(
                    "RUN_MODE=production exige un JWT_SECRET seguro; no uses el valor por defecto. \
                     Define la variable de entorno JWT_SECRET."
                        .to_string(),
                );
            }
        }

        if self.database_url.is_empty() {
            return Err("DATABASE_URL no puede estar vacío".to_string());
        }

        Ok(())
    }
}

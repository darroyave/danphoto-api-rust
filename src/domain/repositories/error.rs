// Errores de dominio (reutilizados por todos los repositorios)

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("validation: {0}")]
    Validation(String),
    #[error("repository: {0}")]
    Repository(#[from] anyhow::Error),
}

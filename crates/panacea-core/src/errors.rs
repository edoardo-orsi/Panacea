/// Errors that can be returned by a [`crate::traits::StoreAdapter`].
#[derive(Debug, thiserror::Error)]
pub enum AdapterError {
    #[error("product not found: {0}")]
    NotFound(String),

    #[error("rate limited by store")]
    RateLimited,

    #[error("parse error: {0}")]
    ParseError(String),

    #[error("network error: {0}")]
    NetworkError(String),
}

/// Top-level application error type used across services.
#[derive(Debug, thiserror::Error)]
pub enum PanaceaError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("nats error: {0}")]
    Nats(String),

    #[error("adapter error: {0}")]
    Adapter(#[from] AdapterError),

    #[error("internal error: {0}")]
    Internal(String),
}

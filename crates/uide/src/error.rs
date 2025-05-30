use thiserror::Error;

/// UIDE error types
#[derive(Error, Debug)]
pub enum UideError {
    #[error("Storage error: {0}")]
    Storage(#[from] rocksdb::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] bincode::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Record not found: {id}")]
    RecordNotFound { id: String },

    #[error("Invalid query: {reason}")]
    InvalidQuery { reason: String },

    #[error("Index error: {reason}")]
    IndexError { reason: String },

    #[error("Type conversion error: {reason}")]
    TypeConversion { reason: String },

    #[error("Configuration error: {reason}")]
    Configuration { reason: String },

    #[error("Concurrent access error: {reason}")]
    ConcurrentAccess { reason: String },

    #[error("Internal error: {reason}")]
    Internal { reason: String },
}

impl UideError {
    pub fn record_not_found(id: impl ToString) -> Self {
        Self::RecordNotFound { id: id.to_string() }
    }

    pub fn invalid_query(reason: impl ToString) -> Self {
        Self::InvalidQuery { reason: reason.to_string() }
    }

    pub fn index_error(reason: impl ToString) -> Self {
        Self::IndexError { reason: reason.to_string() }
    }

    pub fn type_conversion(reason: impl ToString) -> Self {
        Self::TypeConversion { reason: reason.to_string() }
    }

    pub fn configuration(reason: impl ToString) -> Self {
        Self::Configuration { reason: reason.to_string() }
    }

    pub fn concurrent_access(reason: impl ToString) -> Self {
        Self::ConcurrentAccess { reason: reason.to_string() }
    }

    pub fn internal(reason: impl ToString) -> Self {
        Self::Internal { reason: reason.to_string() }
    }
}

/// Result type alias for UIDE operations
pub type Result<T> = std::result::Result<T, UideError>; 
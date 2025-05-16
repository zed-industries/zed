use std::fmt;

/// Error types for the vector store operations
#[derive(Debug)]
pub enum VectorStoreError {
    /// Database-related errors
    Database(String),
    /// Serialization errors
    Serialization(String),
    /// Dimension mismatch errors
    DimensionMismatch { expected: usize, got: usize },
    /// I/O errors
    Io(std::io::Error),
    /// Entry not found
    NotFound(String),
    /// Invalid input
    InvalidInput(String),
}

impl fmt::Display for VectorStoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Database(msg) => write!(f, "Database error: {}", msg),
            Self::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            Self::DimensionMismatch { expected, got } => write!(
                f,
                "Dimension mismatch: expected {}, got {}",
                expected, got
            ),
            Self::Io(err) => write!(f, "I/O error: {}", err),
            Self::NotFound(id) => write!(f, "Entry not found: {}", id),
            Self::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl std::error::Error for VectorStoreError {}

impl From<std::io::Error> for VectorStoreError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<heed::Error> for VectorStoreError {
    fn from(err: heed::Error) -> Self {
        Self::Database(err.to_string())
    }
}

impl From<serde_json::Error> for VectorStoreError {
    fn from(err: serde_json::Error) -> Self {
        Self::Serialization(err.to_string())
    }
} 
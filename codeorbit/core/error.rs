//! Error types and utilities for the CodeOrbit extension.

use thiserror::Error;
use std::fmt;

/// A type alias for `Result<T, Error>`.
pub type Result<T> = std::result::Result<T, Error>;

/// The error type for the CodeOrbit extension.
#[derive(Error, Debug)]
pub enum Error {
    /// An error that occurs during agent operations.
    #[error("Agent error: {0}")]
    AgentError(String),
    
    /// An error that occurs when an agent is already registered.
    #[error("Agent already registered: {0}")]
    AgentAlreadyRegistered(String),
    
    /// An error that occurs when an agent is not found.
    #[error("Agent not found: {0}")]
    AgentNotFound(String),
    
    /// An error that occurs during serialization.
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    /// An error that occurs during deserialization.
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
    
    /// An error that occurs when a lock is poisoned.
    #[error("Failed to acquire lock")]
    LockError,
    
    /// An error that occurs when a configuration is invalid.
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    /// An error that wraps other error types.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerializationError(err.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Other(anyhow::Error::new(err))
    }
}

/// A helper trait for adding context to errors.
pub trait Context<T, E> {
    /// Adds context to an error.
    fn context<C: fmt::Display + Send + Sync + 'static>(self, context: C) -> Result<T>;
}

impl<T, E: Into<Error>> Context<T, E> for std::result::Result<T, E> {
    fn context<C: fmt::Display + Send + Sync + 'static>(self, context: C) -> Result<T> {
        self.map_err(|e| {
            let e: Error = e.into();
            Error::Other(anyhow::Error::new(e).context(context))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    
    #[test]
    fn test_error_conversion() {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let error: Error = io_error.into();
        
        match error {
            Error::Other(_) => { /* expected */ },
            _ => panic!("Expected an Other error"),
        }
    }
    
    #[test]
    fn test_error_display() {
        let error = Error::AgentError("test error".to_string());
        assert_eq!(error.to_string(), "Agent error: test error");
        
        let error = Error::AgentAlreadyRegistered("test_agent".to_string());
        assert_eq!(error.to_string(), "Agent already registered: test_agent");
    }
}

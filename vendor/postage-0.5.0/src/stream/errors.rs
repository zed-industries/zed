use thiserror::Error;

/// An error type returned by `Stream::try_recv`, when the stream has no messages, or is closed.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum TryRecvError {
    /// The stream may produce an item at a later time
    #[error("TryRecvError::Pending")]
    Pending,
    /// The stream is closed, and will never produce an item
    #[error("TryRecvError::Closed")]
    Closed,
}

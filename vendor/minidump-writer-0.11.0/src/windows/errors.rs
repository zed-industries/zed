#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Scroll(#[from] scroll::Error),
    #[error("Failed to open thread")]
    ThreadOpen(#[source] std::io::Error),
    #[error("Failed to suspend thread")]
    ThreadSuspend(#[source] std::io::Error),
    #[error("Failed to get thread context")]
    ThreadContext(#[source] std::io::Error),
}

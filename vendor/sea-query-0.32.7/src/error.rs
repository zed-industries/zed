//! Error types used in sea-query.

/// Result type for sea-query
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// Column and value vector having different length
    ColValNumMismatch { col_len: usize, val_len: usize },
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ColValNumMismatch { col_len, val_len } => write!(
                f,
                "Columns and values length mismatch: {col_len} != {val_len}"
            ),
        }
    }
}

use thiserror::Error;

/// Represents an error when encoding/decoding raw byte buffers and frames
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum CodecError {
    #[error("{0}")]
    Command(&'static str),
    #[error("{0}")]
    Greeting(&'static str),
    #[error("{0}")]
    Mechanism(&'static str),
    #[error("{0}")]
    Decode(&'static str),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Other(&'static str),
}

pub(crate) type CodecResult<T> = Result<T, CodecError>;

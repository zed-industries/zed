pub type Result<T> = std::result::Result<T, RuntimeError>;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum RuntimeError {
    #[error("{0}")]
    DecodeError(#[from] data_encoding::DecodeError),
    #[error("Failed to get {0} directory")]
    DirNotFound(&'static str),
    #[error("Empty argv in kernelspec {kernel_name}")]
    EmptyArgv { kernel_name: String },
    #[error("Failed to execute `{command}` command")]
    CommandFailed {
        command: &'static str,
        #[source]
        source: std::io::Error,
    },
    #[error("Jupyter command failed with status: {0:?}")]
    JupyterCommandFailed(std::process::ExitStatus),
    #[error("Insufficient message parts {0}")]
    InsufficientMessageParts(usize),
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("Missing delimiter")]
    MissingDelimiter,
    #[error("Missing hmac")]
    MissingHmac,
    #[error("Error deserializing content for msg_type `{msg_type}`: {source}")]
    ParseError {
        msg_type: String,
        #[source]
        source: serde_json::Error,
    },
    #[cfg(feature = "ring")]
    #[error("{0}")]
    VerifyError(ring::error::Unspecified),
    #[cfg(feature = "aws-lc-rs")]
    #[error("{0}")]
    VerifyError(aws_lc_rs::error::Unspecified),
    #[error("{0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("{0}")]
    ZmqError(#[from] zeromq::ZmqError),
    #[error("{0}")]
    ZmqMessageError(String),
    #[error("Kernel '{name}' not found. Available kernels: {available:?}")]
    KernelNotFound {
        name: String,
        available: Vec<String>,
    },
    #[error("Failed to extract kernel id from connection file {path}")]
    KernelIdMissing { path: String },
    #[error("Kernel shutdown failed: {details}")]
    KernelShutdownFailed { details: String },
}

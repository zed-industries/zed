use thiserror::Error;

/// Represents an error when parsing an [`crate::Endpoint`]
#[derive(Error, Debug)]
pub enum EndpointError {
    #[error("Failed to parse IP address or port")]
    ParseIpAddr(#[from] std::net::AddrParseError),
    #[error("Unknown transport type {0}")]
    UnknownTransport(String),
    #[error("Invalid Syntax: {0}")]
    Syntax(&'static str),
}

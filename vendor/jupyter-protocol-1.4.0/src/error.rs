#[derive(Debug, thiserror::Error)]
pub enum JupyterError {
    #[error("Error deserializing content for msg_type `{msg_type}`: {source}")]
    ParseError {
        msg_type: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{0}")]
    SerdeError(#[from] serde_json::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("graph contains an edge with a missing endpoint: {edge_id}")]
    MissingEndpoint { edge_id: String },
}

pub type Result<T> = std::result::Result<T, Error>;

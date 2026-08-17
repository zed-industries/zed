//! This module defines the `Error` enum and the `Result` type alias used
//! throughout the crate.

#[derive(Debug, derive_more::From)]
pub enum Error {
    IO(std::io::Error),
    Yaml(serde_yaml::Error),
    GitHubWorkflowMismatch,
    JobIdAlreadyExists(String),
    UTF8(std::string::FromUtf8Error),
    OutdatedWorkflow,
    MissingWorkflowFile(std::path::PathBuf),
}

pub type Result<T> = std::result::Result<T, Error>;

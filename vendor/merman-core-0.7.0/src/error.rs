use crate::detect::DetectTypeError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    DetectType(#[from] DetectTypeError),

    #[error("Unsupported diagram type: {diagram_type}")]
    UnsupportedDiagram { diagram_type: String },

    #[error("Diagram parse error ({diagram_type}): {message}")]
    DiagramParse {
        diagram_type: String,
        message: String,
    },

    #[error(
        "Malformed YAML front-matter. If you were trying to use a YAML front-matter, please ensure that you've correctly opened and closed the YAML front-matter with un-indented `---` blocks"
    )]
    MalformedFrontMatter,

    #[error("Invalid directive JSON: {message}")]
    InvalidDirectiveJson { message: String },

    #[error("Invalid YAML front-matter: {message}")]
    InvalidFrontMatterYaml { message: String },
}

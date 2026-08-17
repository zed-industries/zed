use core::fmt;
use std::{num::ParseIntError, str::Utf8Error};

use fluent_uri::{resolve::ResolveError, ParseError, Uri};

/// Errors that can occur during reference resolution and resource handling.
#[derive(Debug)]
pub enum Error {
    /// A resource is not present in a registry and retrieving it failed.
    Unretrievable {
        uri: String,
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    /// A JSON Pointer leads to a part of a document that does not exist.
    PointerToNowhere { pointer: String },
    /// JSON Pointer contains invalid percent-encoded data.
    InvalidPercentEncoding { pointer: String, source: Utf8Error },
    /// Failed to parse array index in JSON Pointer.
    InvalidArrayIndex {
        pointer: String,
        index: String,
        source: ParseIntError,
    },
    /// An anchor does not exist within a particular resource.
    NoSuchAnchor { anchor: String },
    /// An anchor which could never exist in a resource was dereferenced.
    InvalidAnchor { anchor: String },
    /// An error occurred while parsing or manipulating a URI.
    InvalidUri(UriError),
    /// An unknown JSON Schema specification was encountered.
    UnknownSpecification { specification: String },
    /// A circular reference was detected in a meta-schema chain.
    CircularMetaschema { uri: String },
}

impl Error {
    pub(crate) fn pointer_to_nowhere(pointer: impl Into<String>) -> Error {
        Error::PointerToNowhere {
            pointer: pointer.into(),
        }
    }
    pub(crate) fn invalid_percent_encoding(pointer: impl Into<String>, source: Utf8Error) -> Error {
        Error::InvalidPercentEncoding {
            pointer: pointer.into(),
            source,
        }
    }
    pub(crate) fn invalid_array_index(
        pointer: impl Into<String>,
        index: impl Into<String>,
        source: ParseIntError,
    ) -> Error {
        Error::InvalidArrayIndex {
            pointer: pointer.into(),
            index: index.into(),
            source,
        }
    }
    pub(crate) fn invalid_anchor(anchor: impl Into<String>) -> Error {
        Error::InvalidAnchor {
            anchor: anchor.into(),
        }
    }
    pub(crate) fn no_such_anchor(anchor: impl Into<String>) -> Error {
        Error::NoSuchAnchor {
            anchor: anchor.into(),
        }
    }

    pub fn unknown_specification(specification: impl Into<String>) -> Error {
        Error::UnknownSpecification {
            specification: specification.into(),
        }
    }

    pub fn circular_metaschema(uri: impl Into<String>) -> Error {
        Error::CircularMetaschema { uri: uri.into() }
    }

    pub fn unretrievable(
        uri: impl Into<String>,
        source: Box<dyn std::error::Error + Send + Sync>,
    ) -> Error {
        Error::Unretrievable {
            uri: uri.into(),
            source,
        }
    }

    pub(crate) fn uri_parsing_error(uri: impl Into<String>, error: ParseError) -> Error {
        Error::InvalidUri(UriError::Parse {
            uri: uri.into(),
            is_reference: false,
            error,
        })
    }

    pub(crate) fn uri_reference_parsing_error(uri: impl Into<String>, error: ParseError) -> Error {
        Error::InvalidUri(UriError::Parse {
            uri: uri.into(),
            is_reference: true,
            error,
        })
    }

    pub(crate) fn uri_resolving_error(
        uri: impl Into<String>,
        base: Uri<&str>,
        error: ResolveError,
    ) -> Error {
        Error::InvalidUri(UriError::Resolve {
            uri: uri.into(),
            base: base.to_owned(),
            error,
        })
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Unretrievable { uri, source } => {
                f.write_fmt(format_args!("Resource '{uri}' is not present in a registry and retrieving it failed: {source}"))
            },
            Error::PointerToNowhere { pointer } => {
                f.write_fmt(format_args!("Pointer '{pointer}' does not exist"))
            }
            Error::InvalidPercentEncoding { pointer, .. } => {
                f.write_fmt(format_args!("Invalid percent encoding in pointer '{pointer}': the decoded bytes do not represent valid UTF-8"))
            }
            Error::InvalidArrayIndex { pointer, index, .. } => {
                f.write_fmt(format_args!("Failed to parse array index '{index}' in pointer '{pointer}'"))
            }
            Error::NoSuchAnchor { anchor } => {
                f.write_fmt(format_args!("Anchor '{anchor}' does not exist"))
            }
            Error::InvalidAnchor { anchor } => {
                f.write_fmt(format_args!("Anchor '{anchor}' is invalid"))
            }
            Error::InvalidUri(error) => error.fmt(f),
            Error::UnknownSpecification { specification } => {
                write!(f, "Unknown meta-schema: '{specification}'. Custom meta-schemas must be registered in the registry before use")
            }
            Error::CircularMetaschema { uri } => {
                write!(f, "Circular meta-schema reference detected at '{uri}'")
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Unretrievable { source, .. } => Some(&**source),
            Error::InvalidUri(error) => Some(error),
            Error::InvalidPercentEncoding { source, .. } => Some(source),
            Error::InvalidArrayIndex { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// Errors that can occur during URI handling.
#[derive(Debug)]
pub enum UriError {
    Parse {
        uri: String,
        is_reference: bool,
        error: ParseError,
    },
    Resolve {
        uri: String,
        base: Uri<String>,
        error: ResolveError,
    },
}

impl fmt::Display for UriError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UriError::Parse {
                uri,
                is_reference,
                error,
            } => {
                if *is_reference {
                    f.write_fmt(format_args!("Invalid URI reference '{uri}': {error}"))
                } else {
                    f.write_fmt(format_args!("Invalid URI '{uri}': {error}"))
                }
            }
            UriError::Resolve { uri, base, error } => f.write_fmt(format_args!(
                "Failed to resolve '{uri}' against '{base}': {error}"
            )),
        }
    }
}

impl std::error::Error for UriError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            UriError::Parse { error, .. } => Some(error),
            UriError::Resolve { error, .. } => Some(error),
        }
    }
}

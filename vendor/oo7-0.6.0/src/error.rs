use std::fmt;

/// Alias for [`std::result::Result`] with the error type [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// The error type for oo7.
#[derive(Debug)]
pub enum Error {
    /// File backend error.
    File(crate::file::Error),
    /// Secret Service error.
    DBus(crate::dbus::Error),
}

impl From<crate::file::Error> for Error {
    fn from(e: crate::file::Error) -> Self {
        Self::File(e)
    }
}

impl From<crate::dbus::Error> for Error {
    fn from(e: crate::dbus::Error) -> Self {
        Self::DBus(e)
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::File(e) => write!(f, "File backend error {e}"),
            Self::DBus(e) => write!(f, "DBus error {e}"),
        }
    }
}

/// Errors that can occur when working with schemas.
#[derive(Debug)]
#[cfg(feature = "schema")]
pub enum SchemaError {
    /// A required field is missing from the attributes.
    MissingField(&'static str),

    /// The schema name doesn't match the expected schema.
    SchemaMismatch {
        /// The expected schema name.
        expected: String,
        /// The actual schema name found.
        found: String,
    },

    /// A field value could not be parsed into the expected type.
    InvalidValue {
        /// The field name that has an invalid value.
        field: &'static str,
        /// The invalid value.
        value: String,
    },
}

#[cfg(feature = "schema")]
impl std::error::Error for SchemaError {}

#[cfg(feature = "schema")]
impl std::fmt::Display for SchemaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingField(field) => write!(f, "Missing required field: {field}"),
            Self::SchemaMismatch { expected, found } => {
                write!(f, "Schema mismatch: expected {expected}, found {found}")
            }
            Self::InvalidValue { field, value } => {
                write!(f, "Invalid field value for {field}: {value}")
            }
        }
    }
}

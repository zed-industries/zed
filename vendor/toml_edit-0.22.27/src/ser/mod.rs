//! Serializing Rust structures into TOML.
//!
//! This module contains all the Serde support for serializing Rust structures into TOML.

mod array;
mod key;
mod map;
mod pretty;
mod value;

use crate::visit_mut::VisitMut as _;
#[allow(clippy::wildcard_imports)]
use array::*;
#[allow(clippy::wildcard_imports)]
use map::*;

pub use value::ValueSerializer;

/// Serialize the given data structure as a TOML byte vector.
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, if `T` contains a map with non-string keys, or if `T` attempts to
/// serialize an unsupported datatype such as an enum, tuple, or tuple struct.
#[cfg(feature = "display")]
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>, Error>
where
    T: serde::ser::Serialize + ?Sized,
{
    to_string(value).map(|e| e.into_bytes())
}

/// Serialize the given data structure as a String of TOML.
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, if `T` contains a map with non-string keys, or if `T` attempts to
/// serialize an unsupported datatype such as an enum, tuple, or tuple struct.
///
/// # Examples
///
/// ```
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Config {
///     database: Database,
/// }
///
/// #[derive(Serialize)]
/// struct Database {
///     ip: String,
///     port: Vec<u16>,
///     connection_max: u32,
///     enabled: bool,
/// }
///
/// let config = Config {
///     database: Database {
///         ip: "192.168.1.1".to_string(),
///         port: vec![8001, 8002, 8003],
///         connection_max: 5000,
///         enabled: false,
///     },
/// };
///
/// let toml = toml_edit::ser::to_string(&config).unwrap();
/// println!("{}", toml)
/// ```
#[cfg(feature = "display")]
pub fn to_string<T>(value: &T) -> Result<String, Error>
where
    T: serde::ser::Serialize + ?Sized,
{
    to_document(value).map(|e| e.to_string())
}

/// Serialize the given data structure as a "pretty" String of TOML.
///
/// This is identical to `to_string` except the output string has a more
/// "pretty" output. See `ValueSerializer::pretty` for more details.
#[cfg(feature = "display")]
pub fn to_string_pretty<T>(value: &T) -> Result<String, Error>
where
    T: serde::ser::Serialize + ?Sized,
{
    let mut document = to_document(value)?;
    pretty::Pretty::new().visit_document_mut(&mut document);
    Ok(document.to_string())
}

/// Serialize the given data structure into a TOML document.
///
/// This would allow custom formatting to be applied, mixing with format preserving edits, etc.
pub fn to_document<T>(value: &T) -> Result<crate::DocumentMut, Error>
where
    T: serde::ser::Serialize + ?Sized,
{
    let value = value.serialize(ValueSerializer::new())?;
    let item = crate::Item::Value(value);
    let root = item
        .into_table()
        .map_err(|_| Error::UnsupportedType(None))?;
    Ok(root.into())
}

/// Errors that can occur when deserializing a type.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// Type could not be serialized to TOML
    UnsupportedType(Option<&'static str>),
    /// Value was out of range for the given type
    OutOfRange(Option<&'static str>),
    /// `None` could not be serialized to TOML
    UnsupportedNone,
    /// Key was not convertible to `String` for serializing to TOML
    KeyNotString,
    /// A serialized date was invalid
    DateInvalid,
    /// Other serialization error
    Custom(String),
}

impl Error {
    pub(crate) fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error::Custom(msg.to_string())
    }

    pub(crate) fn unsupported_type(t: Option<&'static str>) -> Self {
        Error::UnsupportedType(t)
    }

    fn out_of_range(t: Option<&'static str>) -> Self {
        Error::OutOfRange(t)
    }

    pub(crate) fn unsupported_none() -> Self {
        Error::UnsupportedNone
    }

    pub(crate) fn key_not_string() -> Self {
        Error::KeyNotString
    }

    fn date_invalid() -> Self {
        Error::DateInvalid
    }
}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::custom(msg)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedType(Some(t)) => write!(formatter, "unsupported {t} type"),
            Self::UnsupportedType(None) => write!(formatter, "unsupported rust type"),
            Self::OutOfRange(Some(t)) => write!(formatter, "out-of-range value for {t} type"),
            Self::OutOfRange(None) => write!(formatter, "out-of-range value"),
            Self::UnsupportedNone => "unsupported None value".fmt(formatter),
            Self::KeyNotString => "map key was not a string".fmt(formatter),
            Self::DateInvalid => "a serialized date was invalid".fmt(formatter),
            Self::Custom(s) => s.fmt(formatter),
        }
    }
}

impl From<crate::TomlError> for Error {
    fn from(e: crate::TomlError) -> Error {
        Self::custom(e)
    }
}

impl From<Error> for crate::TomlError {
    fn from(e: Error) -> crate::TomlError {
        Self::custom(e.to_string(), None)
    }
}

impl std::error::Error for Error {}

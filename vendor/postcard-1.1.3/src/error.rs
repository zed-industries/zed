use core::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

/// This is the error type used by Postcard
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "use-defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum Error {
    /// This is a feature that postcard will never implement
    WontImplement,
    /// This is a feature that postcard intends to support, but does not yet
    NotYetImplemented,
    /// The serialize buffer is full
    SerializeBufferFull,
    /// The length of a sequence must be known
    SerializeSeqLengthUnknown,
    /// Hit the end of buffer, expected more data
    DeserializeUnexpectedEnd,
    /// Found a varint that didn't terminate. Is the usize too big for this platform?
    DeserializeBadVarint,
    /// Found a bool that wasn't 0 or 1
    DeserializeBadBool,
    /// Found an invalid unicode char
    DeserializeBadChar,
    /// Tried to parse invalid utf-8
    DeserializeBadUtf8,
    /// Found an Option discriminant that wasn't 0 or 1
    DeserializeBadOption,
    /// Found an enum discriminant that was > `u32::MAX`
    DeserializeBadEnum,
    /// The original data was not well encoded
    DeserializeBadEncoding,
    /// Bad CRC while deserializing
    DeserializeBadCrc,
    /// Serde Serialization Error
    SerdeSerCustom,
    /// Serde Deserialization Error
    SerdeDeCustom,
    /// Error while processing `collect_str` during serialization
    CollectStrError,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        use Error::*;
        write!(
            f,
            "{}",
            match self {
                WontImplement => "This is a feature that PostCard will never implement",
                NotYetImplemented => {
                    "This is a feature that Postcard intends to support, but does not yet"
                }
                SerializeBufferFull => "The serialize buffer is full",
                SerializeSeqLengthUnknown => "The length of a sequence must be known",
                DeserializeUnexpectedEnd => "Hit the end of buffer, expected more data",
                DeserializeBadVarint => {
                    "Found a varint that didn't terminate. Is the usize too big for this platform?"
                }
                DeserializeBadBool => "Found a bool that wasn't 0 or 1",
                DeserializeBadChar => "Found an invalid unicode char",
                DeserializeBadUtf8 => "Tried to parse invalid utf-8",
                DeserializeBadOption => "Found an Option discriminant that wasn't 0 or 1",
                DeserializeBadEnum => "Found an enum discriminant that was > u32::max_value()",
                DeserializeBadEncoding => "The original data was not well encoded",
                DeserializeBadCrc => "Bad CRC while deserializing",
                SerdeSerCustom => "Serde Serialization Error",
                SerdeDeCustom => "Serde Deserialization Error",
                CollectStrError => "Error while processing `collect_str` during serialization",
            }
        )
    }
}

/// This is the Result type used by Postcard.
pub type Result<T> = ::core::result::Result<T, Error>;

impl serde::ser::Error for Error {
    fn custom<T>(_msg: T) -> Self
    where
        T: Display,
    {
        Error::SerdeSerCustom
    }
}

impl serde::de::Error for Error {
    fn custom<T>(_msg: T) -> Self
    where
        T: Display,
    {
        Error::SerdeDeCustom
    }
}

impl serde::ser::StdError for Error {}

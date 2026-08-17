//! Use the well-known [ISO 8601 format] when serializing and deserializing an [`OffsetDateTime`].
//!
//! Use this module in combination with serde's [`#[with]`][with] attribute.
//!
//! [ISO 8601 format]: https://www.iso.org/iso-8601-date-and-time-format.html
//! [with]: https://serde.rs/field-attrs.html#with

#[cfg(feature = "parsing")]
use core::marker::PhantomData;

#[cfg(feature = "parsing")]
use serde_core::Deserializer;
#[cfg(feature = "formatting")]
use serde_core::ser::Error as _;
#[cfg(feature = "formatting")]
use serde_core::{Serialize, Serializer};

#[cfg(feature = "parsing")]
use super::Visitor;
use crate::OffsetDateTime;
use crate::format_description::well_known::Iso8601;
use crate::format_description::well_known::iso8601::{Config, EncodedConfig};

/// The configuration of ISO 8601 used for serde implementations.
pub(crate) const SERDE_CONFIG: EncodedConfig =
    Config::DEFAULT.set_year_is_six_digits(true).encode();

/// Serialize an [`OffsetDateTime`] using the well-known ISO 8601 format.
#[cfg(feature = "formatting")]
#[inline]
pub fn serialize<S>(datetime: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    datetime
        .format(&Iso8601::<SERDE_CONFIG>)
        .map_err(S::Error::custom)?
        .serialize(serializer)
}

/// Deserialize an [`OffsetDateTime`] from its ISO 8601 representation.
#[cfg(feature = "parsing")]
#[inline]
pub fn deserialize<'a, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'a>,
{
    deserializer.deserialize_str(Visitor::<Iso8601<SERDE_CONFIG>>(PhantomData))
}

/// Use the well-known ISO 8601 format when serializing and deserializing an
/// [`Option<OffsetDateTime>`].
///
/// Use this module in combination with serde's [`#[with]`][with] attribute.
///
/// Note: Due to [serde-rs/serde#2878], you will need to apply `#[serde(default)]` if you want a
/// missing field to deserialize as `None`.
///
/// [ISO 8601 format]: https://www.iso.org/iso-8601-date-and-time-format.html
/// [with]: https://serde.rs/field-attrs.html#with
/// [serde-rs/serde#2878]: https://github.com/serde-rs/serde/issues/2878
pub mod option {
    use super::*;

    /// Serialize an [`Option<OffsetDateTime>`] using the well-known ISO 8601 format.
    #[cfg(feature = "formatting")]
    #[inline]
    pub fn serialize<S>(option: &Option<OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        option
            .map(|odt| odt.format(&Iso8601::<SERDE_CONFIG>))
            .transpose()
            .map_err(S::Error::custom)?
            .serialize(serializer)
    }

    /// Deserialize an [`Option<OffsetDateTime>`] from its ISO 8601 representation.
    #[cfg(feature = "parsing")]
    #[inline]
    pub fn deserialize<'a, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
    where
        D: Deserializer<'a>,
    {
        deserializer.deserialize_option(Visitor::<Option<Iso8601<SERDE_CONFIG>>>(PhantomData))
    }
}

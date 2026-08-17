//! Use the well-known [RFC3339 format] when serializing and deserializing an [`OffsetDateTime`].
//!
//! Use this module in combination with serde's [`#[with]`][with] attribute.
//!
//! [RFC3339 format]: https://tools.ietf.org/html/rfc3339#section-5.6
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
use crate::format_description::well_known::Rfc3339;

/// Serialize an [`OffsetDateTime`] using the well-known RFC3339 format.
#[cfg(feature = "formatting")]
#[inline]
pub fn serialize<S>(datetime: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    datetime
        .format(&Rfc3339)
        .map_err(S::Error::custom)?
        .serialize(serializer)
}

/// Deserialize an [`OffsetDateTime`] from its RFC3339 representation.
#[cfg(feature = "parsing")]
#[inline]
pub fn deserialize<'a, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'a>,
{
    deserializer.deserialize_str(Visitor::<Rfc3339>(PhantomData))
}

/// Use the well-known [RFC3339 format] when serializing and deserializing an
/// [`Option<OffsetDateTime>`].
///
/// Use this module in combination with serde's [`#[with]`][with] attribute.
///
/// Note: Due to [serde-rs/serde#2878], you will need to apply `#[serde(default)]` if you want a
/// missing field to deserialize as `None`.
///
/// [RFC3339 format]: https://tools.ietf.org/html/rfc3339#section-5.6
/// [with]: https://serde.rs/field-attrs.html#with
/// [serde-rs/serde#2878]: https://github.com/serde-rs/serde/issues/2878
pub mod option {
    use super::*;

    /// Serialize an [`Option<OffsetDateTime>`] using the well-known RFC3339 format.
    #[cfg(feature = "formatting")]
    #[inline]
    pub fn serialize<S>(option: &Option<OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        option
            .map(|odt| odt.format(&Rfc3339))
            .transpose()
            .map_err(S::Error::custom)?
            .serialize(serializer)
    }

    /// Deserialize an [`Option<OffsetDateTime>`] from its RFC3339 representation.
    #[cfg(feature = "parsing")]
    #[inline]
    pub fn deserialize<'a, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
    where
        D: Deserializer<'a>,
    {
        deserializer.deserialize_option(Visitor::<Option<Rfc3339>>(PhantomData))
    }
}

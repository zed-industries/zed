//! Treat an [`OffsetDateTime`] as a [Unix timestamp] with milliseconds for
//! the purposes of serde.
//!
//! Use this module in combination with serde's [`#[with]`][with] attribute.
//!
//! When deserializing, the offset is assumed to be UTC.
//!
//! [Unix timestamp]: https://en.wikipedia.org/wiki/Unix_time
//! [with]: https://serde.rs/field-attrs.html#with

use serde_core::{Deserialize, Deserializer, Serialize, Serializer};

use crate::OffsetDateTime;
use crate::error::ComponentRange;

/// Serialize an `OffsetDateTime` as its Unix timestamp with milliseconds
#[inline]
pub fn serialize<S>(datetime: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let timestamp = datetime.unix_timestamp_nanos() / 1_000_000;
    timestamp.serialize(serializer)
}

/// Deserialize an `OffsetDateTime` from its Unix timestamp with milliseconds
#[inline]
pub fn deserialize<'a, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'a>,
{
    let value: i128 = <_>::deserialize(deserializer)?;
    OffsetDateTime::from_unix_timestamp_nanos(value * 1_000_000)
        .map_err(ComponentRange::into_de_error)
}

/// Treat an `Option<OffsetDateTime>` as a [Unix timestamp] with milliseconds
/// for the purposes of serde.
///
/// Use this module in combination with serde's [`#[with]`][with] attribute.
///
/// Note: Due to [serde-rs/serde#2878], you will need to apply `#[serde(default)]` if you want a
/// missing field to deserialize as `None`.
///
/// When deserializing, the offset is assumed to be UTC.
///
/// [Unix timestamp]: https://en.wikipedia.org/wiki/Unix_time
/// [with]: https://serde.rs/field-attrs.html#with
/// [serde-rs/serde#2878]: https://github.com/serde-rs/serde/issues/2878
pub mod option {
    use super::*;

    /// Serialize an `Option<OffsetDateTime>` as its Unix timestamp with milliseconds
    #[inline]
    pub fn serialize<S>(option: &Option<OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        option
            .map(|timestamp| timestamp.unix_timestamp_nanos() / 1_000_000)
            .serialize(serializer)
    }

    /// Deserialize an `Option<OffsetDateTime>` from its Unix timestamp with milliseconds
    #[inline]
    pub fn deserialize<'a, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
    where
        D: Deserializer<'a>,
    {
        Option::deserialize(deserializer)?
            .map(|value: i128| OffsetDateTime::from_unix_timestamp_nanos(value * 1_000_000))
            .transpose()
            .map_err(ComponentRange::into_de_error)
    }
}

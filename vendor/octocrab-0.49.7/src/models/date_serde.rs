//! Serialization and Deserialization of timestamps in Github API
//!
//! GitHub API can give (from past experience) either:
//! - a seconds timestamp relative to Epoch, or
//! - a string containing the timestamp in [RFC 3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format.
//!
//! This module handles transparently both formats to deserialize to [`DateTime<Utc>`](chrono::DateTime). It mostly
//! redo things existing in [chrono::serde], because it is otherwise impossible to combine existing `serde_with` modules.

use core::fmt;

use chrono::{DateTime, LocalResult, TimeZone, Utc};
use serde::{de, Deserialize};

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: de::Deserializer<'de>,
{
    deserializer.deserialize_any(GithubTimestampVisitor)
}

/// Helper struct to tell serde the deserializer to use when working with Option<DateTime<Utc>>
#[derive(Debug, Deserialize)]
struct WrappedGithubTimestamp(#[serde(deserialize_with = "deserialize")] DateTime<Utc>);

pub(crate) fn deserialize_opt<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: de::Deserializer<'de>,
{
    Option::<WrappedGithubTimestamp>::deserialize(deserializer)
        .map(|opt_wrapped| opt_wrapped.map(|wrapped| wrapped.0))
}

struct GithubTimestampVisitor;

impl de::Visitor<'_> for GithubTimestampVisitor {
    type Value = DateTime<Utc>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .write_str("a RFC3339 date and time _string_ or a unix timestamp _integer_ in seconds")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        serde_from(Utc.timestamp_opt(v, 0), &v)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        serde_from(Utc.timestamp_opt(v as i64, 0), &v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        v.parse().map_err(E::custom)
    }
}

// Convert from chrono local result to something usable by serde
//
// Ref: https://github.com/chronotope/chrono/blob/bc410bb054932518822eb393147aad939862c7a5/src/naive/datetime/serde.rs#L1051
pub(crate) fn serde_from<T, E, V>(me: LocalResult<T>, ts: &V) -> Result<T, E>
where
    E: de::Error,
    V: fmt::Display,
    T: fmt::Display,
{
    match me {
        LocalResult::None => Err(E::custom(format!("value is not a legal timestamp: {ts}"))),
        LocalResult::Ambiguous(min, max) => Err(E::custom(format!(
            "value is an ambiguous timestamp: {ts}. It could be {min} or {max}"
        ))),
        LocalResult::Single(val) => Ok(val),
    }
}

use serde::{Deserialize, Serialize};

/// `KnownOrUnknown` is a type that represents either a known value ([`Known`](KnownOrUnknown::Known))
/// or an unknown value ([`Unknown`](KnownOrUnknown::Unknown)).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KnownOrUnknown<K, U> {
    /// A known value.
    Known(K),
    /// An unknown value.
    Unknown(U),
}

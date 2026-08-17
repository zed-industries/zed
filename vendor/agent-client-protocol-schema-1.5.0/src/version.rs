use derive_more::{Display, From};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Protocol version identifier.
///
/// This version is only bumped for breaking changes.
/// Non-breaking changes should be introduced via capabilities.
#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    JsonSchema,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    From,
    Display,
)]
pub struct ProtocolVersion(u16);

impl ProtocolVersion {
    /// Version `0` of the protocol.
    ///
    /// This was a pre-release version that shouldn't be used in production.
    /// It should likely be treated as unsupported.
    pub const V0: Self = Self(0);
    /// Version `1` of the protocol.
    ///
    /// <https://agentclientprotocol.com/protocol/overview>
    pub const V1: Self = Self(1);
    /// Version `2` of the protocol.
    ///
    /// This is an unstable draft used for protocol iteration. It is only
    /// available when the `unstable_protocol_v2` feature is enabled and must
    /// be selected explicitly.
    #[cfg(feature = "unstable_protocol_v2")]
    pub const V2: Self = Self(2);
    /// The latest stable supported version of the protocol.
    ///
    /// Currently this is version `1`.
    ///
    /// This shorthand is intentionally unavailable when the
    /// `unstable_protocol_v2` feature is enabled, so code that opts into the
    /// v2 draft must choose `V1` or `V2` explicitly.
    #[cfg(not(feature = "unstable_protocol_v2"))]
    pub const LATEST: Self = Self::V1;

    /// Returns the numeric protocol version.
    #[must_use]
    pub const fn as_u16(self) -> u16 {
        self.0
    }

    #[cfg(test)]
    #[must_use]
    const fn new(version: u16) -> Self {
        Self(version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_u64() {
        let json = "1";
        let version: ProtocolVersion = serde_json::from_str(json).unwrap();
        assert_eq!(version, ProtocolVersion::new(1));
    }

    #[test]
    fn test_deserialize_string_errors() {
        let json = "\"1.0.0\"";
        let result: Result<ProtocolVersion, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_large_number() {
        let json = "100000";
        let result: Result<ProtocolVersion, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_zero() {
        let json = "0";
        let version: ProtocolVersion = serde_json::from_str(json).unwrap();
        assert_eq!(version, ProtocolVersion::new(0));
    }

    #[test]
    fn test_deserialize_max_u16() {
        let json = "65535";
        let version: ProtocolVersion = serde_json::from_str(json).unwrap();
        assert_eq!(version, ProtocolVersion::new(65535));
    }

    #[test]
    fn test_as_u16() {
        assert_eq!(ProtocolVersion::V0.as_u16(), 0);
        assert_eq!(ProtocolVersion::V1.as_u16(), 1);

        #[cfg(not(feature = "unstable_protocol_v2"))]
        assert_eq!(ProtocolVersion::LATEST.as_u16(), 1);

        #[cfg(feature = "unstable_protocol_v2")]
        assert_eq!(ProtocolVersion::V2.as_u16(), 2);

        assert_eq!(ProtocolVersion::new(65535).as_u16(), 65535);
    }
}

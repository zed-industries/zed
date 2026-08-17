use serde::{Deserialize, Serialize};

#[derive(Debug, zvariant::Type, PartialEq, Eq, Copy, Clone)]
#[zvariant(signature = "s")]
/// Algorithm used to start a new session.
///
/// The communication between the Secret Service and the application can either
/// be encrypted or the items can be sent in plain text.
pub enum Algorithm {
    /// Plain text, per <https://specifications.freedesktop.org/secret-service-spec/latest/ch07s02.html>.
    Plain,
    /// Encrypted, per <https://specifications.freedesktop.org/secret-service-spec/latest/ch07s03.html>.
    Encrypted,
}

const PLAIN_ALGORITHM: &str = "plain";
const ENCRYPTED_ALGORITHM: &str = "dh-ietf1024-sha256-aes128-cbc-pkcs7";

impl Serialize for Algorithm {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Plain => str::serialize(PLAIN_ALGORITHM, serializer),
            Self::Encrypted => str::serialize(ENCRYPTED_ALGORITHM, serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Algorithm {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            PLAIN_ALGORITHM => Ok(Self::Plain),
            ENCRYPTED_ALGORITHM => Ok(Self::Encrypted),
            e => Err(serde::de::Error::custom(format!("Invalid algorithm {e}"))),
        }
    }
}

#[cfg(test)]
mod tests {
    use zvariant::{Endian, serialized::Context, to_bytes};

    use super::*;

    #[test]
    fn serialization() {
        let ctxt = Context::new_dbus(Endian::Little, 0);

        // Test serializing Plain
        let encoded = to_bytes(ctxt, &Algorithm::Plain).unwrap();
        let value: String = encoded.deserialize().unwrap().0;
        assert_eq!(value, "plain");

        // Test serializing Encrypted
        let encoded = to_bytes(ctxt, &Algorithm::Encrypted).unwrap();
        let value: String = encoded.deserialize().unwrap().0;
        assert_eq!(value, "dh-ietf1024-sha256-aes128-cbc-pkcs7");

        // Test deserializing plain
        let encoded = to_bytes(ctxt, &PLAIN_ALGORITHM).unwrap();
        let algo: Algorithm = encoded.deserialize().unwrap().0;
        assert_eq!(algo, Algorithm::Plain);

        // Test deserializing encrypted
        let encoded = to_bytes(ctxt, &ENCRYPTED_ALGORITHM).unwrap();
        let algo: Algorithm = encoded.deserialize().unwrap().0;
        assert_eq!(algo, Algorithm::Encrypted);

        // Test deserializing invalid algorithm
        let encoded = to_bytes(ctxt, &"invalid-algorithm").unwrap();
        let result: Result<(Algorithm, _), _> = encoded.deserialize();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid algorithm")
        );

        // Test roundtrip for Plain
        let original = Algorithm::Plain;
        let encoded = to_bytes(ctxt, &original).unwrap();
        let decoded: Algorithm = encoded.deserialize().unwrap().0;
        assert_eq!(original, decoded);

        // Test roundtrip for Encrypted
        let original = Algorithm::Encrypted;
        let encoded = to_bytes(ctxt, &original).unwrap();
        let decoded: Algorithm = encoded.deserialize().unwrap().0;
        assert_eq!(original, decoded);
    }
}

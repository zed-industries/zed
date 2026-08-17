use std::str::FromStr;

use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};

#[derive(Default, PartialEq, Eq, Copy, Clone, Debug, zvariant::Type)]
#[zvariant(signature = "s")]
pub enum ContentType {
    Text,
    #[default]
    Blob,
}

impl Serialize for ContentType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_str().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ContentType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl FromStr for ContentType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "text/plain" => Ok(Self::Text),
            "application/octet-stream" => Ok(Self::Blob),
            e => Err(format!("Invalid content type: {e}")),
        }
    }
}

impl ContentType {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text/plain",
            Self::Blob => "application/octet-stream",
        }
    }
}

/// A wrapper around a combination of (secret, content-type).
#[derive(Clone, PartialEq, Eq, Zeroize, ZeroizeOnDrop)]
pub enum Secret {
    /// Corresponds to [`ContentType::Text`]
    Text(String),
    /// Corresponds to [`ContentType::Blob`]
    Blob(Vec<u8>),
}

impl std::fmt::Debug for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(_) => write!(f, "Secret::Text([REDACTED])"),
            Self::Blob(_) => write!(f, "Secret::Blob([REDACTED])"),
        }
    }
}

impl Secret {
    /// Generate a random secret, used when creating a session collection.
    pub fn random() -> Result<Self, getrandom::Error> {
        let mut secret = [0; 64];
        // Equivalent of `ring::rand::SecureRandom`
        getrandom::fill(&mut secret)?;

        Ok(Self::blob(secret))
    }

    /// Create a text secret, stored with `text/plain` content type.
    pub fn text(value: impl AsRef<str>) -> Self {
        Self::Text(value.as_ref().to_owned())
    }

    /// Create a blob secret, stored with `application/octet-stream` content
    /// type.
    pub fn blob(value: impl AsRef<[u8]>) -> Self {
        Self::Blob(value.as_ref().to_owned())
    }

    pub const fn content_type(&self) -> ContentType {
        match self {
            Self::Text(_) => ContentType::Text,
            Self::Blob(_) => ContentType::Blob,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Text(text) => text.as_bytes(),
            Self::Blob(bytes) => bytes.as_ref(),
        }
    }

    pub fn with_content_type(content_type: ContentType, secret: impl AsRef<[u8]>) -> Self {
        match content_type {
            ContentType::Text => match String::from_utf8(secret.as_ref().to_owned()) {
                Ok(text) => Secret::text(text),
                Err(_e) => {
                    #[cfg(feature = "tracing")]
                    tracing::warn!(
                        "Failed to decode secret as UTF-8: {}, falling back to blob",
                        _e
                    );

                    Secret::blob(secret)
                }
            },
            _ => Secret::blob(secret),
        }
    }
}

impl From<&[u8]> for Secret {
    fn from(value: &[u8]) -> Self {
        Self::blob(value)
    }
}

impl From<Zeroizing<Vec<u8>>> for Secret {
    fn from(value: Zeroizing<Vec<u8>>) -> Self {
        Self::blob(value)
    }
}

impl From<Vec<u8>> for Secret {
    fn from(value: Vec<u8>) -> Self {
        Self::blob(value)
    }
}

impl From<&Vec<u8>> for Secret {
    fn from(value: &Vec<u8>) -> Self {
        Self::blob(value)
    }
}

impl<const N: usize> From<&[u8; N]> for Secret {
    fn from(value: &[u8; N]) -> Self {
        Self::blob(value)
    }
}

impl From<String> for Secret {
    fn from(value: String) -> Self {
        Self::text(value)
    }
}

impl From<&str> for Secret {
    fn from(value: &str) -> Self {
        Self::text(value)
    }
}

impl std::ops::Deref for Secret {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_bytes()
    }
}

impl AsRef<[u8]> for Secret {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

#[cfg(test)]
mod tests {
    use zvariant::{Endian, serialized::Context, to_bytes};

    use super::*;

    #[test]
    fn secret_debug_is_redacted() {
        let text_secret = Secret::text("password");
        let blob_secret = Secret::blob([1, 2, 3]);

        assert_eq!(format!("{:?}", text_secret), "Secret::Text([REDACTED])");
        assert_eq!(format!("{:?}", blob_secret), "Secret::Blob([REDACTED])");
    }

    #[test]
    fn content_type_serialization() {
        let ctxt = Context::new_dbus(Endian::Little, 0);

        // Test Text serialization
        let encoded = to_bytes(ctxt, &ContentType::Text).unwrap();
        let value: String = encoded.deserialize().unwrap().0;
        assert_eq!(value, "text/plain");

        // Test Blob serialization
        let encoded = to_bytes(ctxt, &ContentType::Blob).unwrap();
        let value: String = encoded.deserialize().unwrap().0;
        assert_eq!(value, "application/octet-stream");

        // Test Text deserialization
        let encoded = to_bytes(ctxt, &"text/plain").unwrap();
        let content_type: ContentType = encoded.deserialize().unwrap().0;
        assert_eq!(content_type, ContentType::Text);

        // Test Blob deserialization
        let encoded = to_bytes(ctxt, &"application/octet-stream").unwrap();
        let content_type: ContentType = encoded.deserialize().unwrap().0;
        assert_eq!(content_type, ContentType::Blob);

        // Test invalid content type deserialization
        let encoded = to_bytes(ctxt, &"invalid/type").unwrap();
        let result: Result<(ContentType, _), _> = encoded.deserialize();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid content type")
        );
    }

    #[test]
    fn content_type_from_str() {
        assert_eq!(
            ContentType::from_str("text/plain").unwrap(),
            ContentType::Text
        );
        assert_eq!(
            ContentType::from_str("application/octet-stream").unwrap(),
            ContentType::Blob
        );

        // Test error case
        let result = ContentType::from_str("invalid");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid content type"));
    }

    #[test]
    fn invalid_utf8() {
        // Test with invalid UTF-8 bytes
        let invalid_utf8 = vec![0xFF, 0xFE, 0xFD];

        // Should fall back to blob when UTF-8 decoding fails
        let secret = Secret::with_content_type(ContentType::Text, &invalid_utf8);
        assert_eq!(secret.content_type(), ContentType::Blob);
        assert_eq!(&*secret, &[0xFF, 0xFE, 0xFD]);

        // Test with valid UTF-8
        let valid_utf8 = "Hello, World!";
        let secret = Secret::with_content_type(ContentType::Text, valid_utf8.as_bytes());
        assert_eq!(secret.content_type(), ContentType::Text);
        assert_eq!(&*secret, valid_utf8.as_bytes());

        // Test with blob content type
        let data = vec![1, 2, 3, 4];
        let secret = Secret::with_content_type(ContentType::Blob, &data);
        assert_eq!(secret.content_type(), ContentType::Blob);
        assert_eq!(&*secret, &[1, 2, 3, 4]);
    }

    #[test]
    fn random() {
        let secret1 = Secret::random().unwrap();
        let secret2 = Secret::random().unwrap();

        // Random secrets should be blobs
        assert_eq!(secret1.content_type(), ContentType::Blob);
        assert_eq!(secret2.content_type(), ContentType::Blob);

        // Should be 64 bytes
        assert_eq!(secret1.as_bytes().len(), 64);
        assert_eq!(secret2.as_bytes().len(), 64);

        // Should be different
        assert_ne!(secret1.as_bytes(), secret2.as_bytes());
    }
}

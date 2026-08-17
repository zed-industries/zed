use serde::{Deserialize, Serialize};
#[cfg(feature = "native_crypto")]
use subtle::ConstantTimeEq;
use zbus::zvariant::Type;

// There is no constructor to avoid performing sanity checks, e.g. length.
/// A message authentication code. It provides constant-time comparison when
/// compared against another mac or against a slice of bytes.
#[derive(Deserialize, Serialize, Type, Clone)]
pub struct Mac(#[serde(with = "serde_bytes")] Vec<u8>);

impl std::fmt::Debug for Mac {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mac([REDACTED])")
    }
}

impl Mac {
    pub(crate) const fn new(inner: Vec<u8>) -> Self {
        Self(inner)
    }

    /// Constant-time comparison against a slice of bytes.
    fn verify_slice(&self, other: &[u8]) -> bool {
        #[cfg(feature = "native_crypto")]
        {
            self.0.ct_eq(other).into()
        }
        #[cfg(feature = "openssl_crypto")]
        {
            openssl::memcmp::eq(&self.0, other)
        }
    }

    // This is made private to prevent non-constant-time comparisons.
    pub(crate) fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl PartialEq for Mac {
    fn eq(&self, other: &Self) -> bool {
        self.verify_slice(&other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mac_debug_is_redacted() {
        let mac = Mac::new(vec![1, 2, 3, 4]);
        assert_eq!(format!("{:?}", mac), "Mac([REDACTED])");
    }
}

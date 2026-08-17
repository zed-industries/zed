//! SPKI fingerprint support.

use der::Writer;
use sha2::{Digest, Sha256};

/// Size of a SHA-256 SPKI fingerprint in bytes.
pub(crate) const SIZE: usize = 32;

/// Raw bytes of a SPKI fingerprint i.e. SHA-256 digest of
/// `SubjectPublicKeyInfo`'s DER encoding.
///
/// See [RFC7469 § 2.1.1] for more information.
///
/// [RFC7469 § 2.1.1]: https://datatracker.ietf.org/doc/html/rfc7469#section-2.1.1
pub type FingerprintBytes = [u8; SIZE];

/// Writer newtype which accepts DER being serialized on-the-fly and computes a
/// hash of the contents.
#[derive(Clone, Default)]
pub(crate) struct Builder {
    /// In-progress digest being computed from streaming DER.
    digest: Sha256,
}

impl Builder {
    /// Create a new fingerprint builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Finish computing a fingerprint, returning the computed digest.
    pub fn finish(self) -> FingerprintBytes {
        self.digest.finalize().into()
    }
}

impl Writer for Builder {
    fn write(&mut self, der_bytes: &[u8]) -> der::Result<()> {
        self.digest.update(der_bytes);
        Ok(())
    }
}

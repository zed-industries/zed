//! Digest Transform support

use core_foundation::base::{CFIndex, TCFType};
use core_foundation::data::CFData;
use core_foundation::error::CFError;
use core_foundation::string::CFString;
use core_foundation_sys::base::CFTypeRef;
use core_foundation_sys::data::CFDataRef;
use core_foundation_sys::string::CFStringRef;
use security_framework_sys::digest_transform::*;
use security_framework_sys::transform::kSecTransformInputAttributeName;
use std::ptr;

use crate::os::macos::transform::SecTransform;

#[derive(Debug, Copy, Clone)]
/// A type of digest.
pub struct DigestType(CFStringRef);

#[allow(missing_docs)]
impl DigestType {
    #[inline(always)]
    #[must_use]
    pub fn hmac_md5() -> Self {
        unsafe { Self(kSecDigestHMACMD5) }
    }

    #[inline(always)]
    #[must_use]
    pub fn hmac_sha1() -> Self {
        unsafe { Self(kSecDigestHMACSHA1) }
    }

    #[inline(always)]
    #[must_use]
    pub fn hmac_sha2() -> Self {
        unsafe { Self(kSecDigestHMACSHA2) }
    }

    #[inline(always)]
    #[must_use]
    pub fn md2() -> Self {
        unsafe { Self(kSecDigestMD2) }
    }

    #[inline(always)]
    #[must_use]
    pub fn md4() -> Self {
        unsafe { Self(kSecDigestMD4) }
    }

    #[inline(always)]
    #[must_use]
    pub fn md5() -> Self {
        unsafe { Self(kSecDigestMD5) }
    }

    #[inline(always)]
    #[must_use]
    pub fn sha1() -> Self {
        unsafe { Self(kSecDigestSHA1) }
    }

    #[inline(always)]
    #[must_use]
    pub fn sha2() -> Self {
        unsafe { Self(kSecDigestSHA2) }
    }

    #[inline(always)]
    fn to_type(self) -> CFTypeRef {
        self.0 as CFTypeRef
    }
}

/// A builder for digest transform operations.
pub struct Builder {
    digest_type: Option<DigestType>,
    digest_length: Option<CFIndex>,
    hmac_key: Option<CFData>,
}

impl Default for Builder {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl Builder {
    /// Returns a new builder with default settings.
    #[inline(always)]
    #[must_use]
    pub fn new() -> Self {
        Self {
            digest_type: None,
            digest_length: None,
            hmac_key: None,
        }
    }

    /// Sets the type of digest to perform.
    ///
    /// If not set, an appropriate digest will be selected for you.
    #[inline]
    pub fn type_(&mut self, digest_type: DigestType) -> &mut Self {
        self.digest_type = Some(digest_type);
        self
    }

    /// Sets the output length of the digest.
    ///
    /// If not set, an appropriate length will be selected for you. Some digest
    /// types only support specific output lengths.
    #[inline]
    pub fn length(&mut self, digest_length: CFIndex) -> &mut Self {
        self.digest_length = Some(digest_length);
        self
    }

    /// Sets the key used for HMAC digests.
    ///
    /// Only applies to `HmacMd5`, `HmacSha1`, and `HmacSha2` digests.
    #[inline]
    pub fn hmac_key(&mut self, hmac_key: CFData) -> &mut Self {
        self.hmac_key = Some(hmac_key);
        self
    }

    /// Computes the digest of the data.
    pub fn execute(&self, data: &CFData) -> Result<CFData, CFError> {
        unsafe {
            let digest_type = match self.digest_type {
                Some(ref digest_type) => digest_type.to_type(),
                None => ptr::null(),
            };

            let digest_length = self.digest_length.unwrap_or(0);

            let mut error = ptr::null_mut();
            let transform = SecDigestTransformCreate(digest_type, digest_length, &mut error);
            if transform.is_null() {
                return Err(CFError::wrap_under_create_rule(error));
            }
            let mut transform = SecTransform::wrap_under_create_rule(transform);

            if let Some(ref hmac_key) = self.hmac_key {
                let key = CFString::wrap_under_get_rule(kSecDigestHMACKeyAttribute);
                transform.set_attribute(&key, hmac_key)?;
            }

            let key = CFString::wrap_under_get_rule(kSecTransformInputAttributeName);
            transform.set_attribute(&key, data)?;

            let result = transform.execute()?;
            Ok(CFData::wrap_under_get_rule(
                result.as_CFTypeRef() as CFDataRef
            ))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn md5() {
        let data = CFData::from_buffer("The quick brown fox jumps over the lazy dog".as_bytes());
        let hash = Builder::new()
            .type_(DigestType::md5())
            .execute(&data)
            .unwrap();
        assert_eq!(
            hex::encode(hash.bytes()),
            "9e107d9d372bb6826bd81d3542a419d6"
        );
    }

    #[test]
    fn hmac_sha1() {
        let data = CFData::from_buffer("The quick brown fox jumps over the lazy dog".as_bytes());
        let key = CFData::from_buffer(b"key");
        let hash = Builder::new()
            .type_(DigestType::hmac_sha1())
            .hmac_key(key)
            .execute(&data)
            .unwrap();
        assert_eq!(
            hex::encode(hash.bytes()),
            "de7c9b85b8b78aa6bc8a7a36f70a90701c9db4d9"
        );
    }
}

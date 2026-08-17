use crate::{key, sign};

/// ActiveCertifiedKey wraps CertifiedKey and tracks OSCP and SCT state
/// in a single handshake.
pub(super) struct ActiveCertifiedKey<'a> {
    key: &'a sign::CertifiedKey,
    ocsp: Option<&'a [u8]>,
    sct_list: Option<&'a [u8]>,
}

impl<'a> ActiveCertifiedKey<'a> {
    pub(super) fn from_certified_key(key: &sign::CertifiedKey) -> ActiveCertifiedKey {
        ActiveCertifiedKey {
            key,
            ocsp: key.ocsp.as_deref(),
            sct_list: key.sct_list.as_deref(),
        }
    }

    /// Get the certificate chain
    #[inline]
    pub(super) fn get_cert(&self) -> &[key::Certificate] {
        &self.key.cert
    }

    /// Get the signing key
    #[inline]
    pub(super) fn get_key(&self) -> &dyn sign::SigningKey {
        &*self.key.key
    }

    #[inline]
    pub(super) fn get_ocsp(&self) -> Option<&[u8]> {
        self.ocsp
    }

    #[inline]
    pub(super) fn get_sct_list(&self) -> Option<&[u8]> {
        self.sct_list
    }
}

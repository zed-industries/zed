//! Bindings to Certificate Trust Lists (CTL) in winapi.

#![allow(dead_code)]

use std::io;
use std::mem;
use std::ptr;

use windows_sys::Win32::Security::Cryptography;

use crate::cert_context::CertContext;
use crate::Inner;

/// Wrapped `PCCTL_CONTEXT` which represents a certificate trust list to
/// Windows.
pub struct CtlContext(*const Cryptography::CTL_CONTEXT);

unsafe impl Send for CtlContext {}
unsafe impl Sync for CtlContext {}

impl Drop for CtlContext {
    fn drop(&mut self) {
        unsafe {
            Cryptography::CertFreeCTLContext(self.0);
        }
    }
}

impl Inner<*const Cryptography::CTL_CONTEXT> for CtlContext {
    unsafe fn from_inner(t: *const Cryptography::CTL_CONTEXT) -> CtlContext {
        CtlContext(t)
    }

    fn as_inner(&self) -> *const Cryptography::CTL_CONTEXT {
        self.0
    }

    fn get_mut(&mut self) -> &mut *const Cryptography::CTL_CONTEXT {
        &mut self.0
    }
}

impl CtlContext {
    /// Returns a builder reader to create an encoded `CtlContext`.
    pub fn builder() -> Builder {
        Builder {
            certificates: vec![],
            usages: vec![],
        }
    }
}

/// Used to build an encoded `CtlContext` which can be added to a `Memory` store
/// to get back the actual `CtlContext`.
pub struct Builder {
    certificates: Vec<CertContext>,
    usages: Vec<Vec<u8>>,
}

impl Builder {
    /// Adds a certificate to be passed to `CryptMsgEncodeAndSignCTL` later on.
    pub fn certificate(&mut self, cert: CertContext) -> &mut Builder {
        self.certificates.push(cert);
        self
    }

    /// Adds a usage string to be passed in the `SubjectUsage` field to
    /// `CryptMsgEncodeAndSignCTL` later on.
    pub fn usage(&mut self, usage: &str) -> &mut Builder {
        let mut usage = usage.as_bytes().to_owned();
        usage.push(0);
        self.usages.push(usage);
        self
    }

    /// Calls `CryptMsgEncodeAndSignCTL` to encode this list of certificates
    /// into a CTL.
    ///
    /// This can later be passed to `Memory::add_encoded_ctl`.
    pub fn encode_and_sign(&self) -> io::Result<Vec<u8>> {
        unsafe {
            let encoding = Cryptography::X509_ASN_ENCODING | Cryptography::PKCS_7_ASN_ENCODING;

            let mut usages = self
                .usages
                .iter()
                .map(|u| u.as_ptr() as _)
                .collect::<Vec<_>>();
            let mut entry_data = vec![];
            let mut entries = vec![];
            for certificate in &self.certificates {
                let data = cert_entry(certificate)?;
                entries.push(*(data.as_ptr() as *const Cryptography::CTL_ENTRY));
                entry_data.push(data);
            }

            let mut ctl_info: Cryptography::CTL_INFO = mem::zeroed();
            ctl_info.dwVersion = Cryptography::CTL_V1;
            ctl_info.SubjectUsage.cUsageIdentifier = usages.len() as u32;
            ctl_info.SubjectUsage.rgpszUsageIdentifier = usages.as_mut_ptr();
            ctl_info.SubjectAlgorithm.pszObjId = Cryptography::szOID_OIWSEC_sha1 as _;
            ctl_info.cCTLEntry = entries.len() as u32;
            ctl_info.rgCTLEntry = entries.as_mut_ptr();

            let mut sign_info: Cryptography::CMSG_SIGNED_ENCODE_INFO = mem::zeroed();
            sign_info.cbSize = mem::size_of_val(&sign_info) as u32;
            let mut encoded_certs = self
                .certificates
                .iter()
                .map(|c| Cryptography::CRYPT_INTEGER_BLOB {
                    cbData: (*c.as_inner()).cbCertEncoded,
                    pbData: (*c.as_inner()).pbCertEncoded,
                })
                .collect::<Vec<_>>();
            sign_info.rgCertEncoded = encoded_certs.as_mut_ptr();
            sign_info.cCertEncoded = encoded_certs.len() as u32;

            let flags = Cryptography::CMSG_ENCODE_SORTED_CTL_FLAG
                | Cryptography::CMSG_ENCODE_HASHED_SUBJECT_IDENTIFIER_FLAG;

            let mut size = 0;

            let res = Cryptography::CryptMsgEncodeAndSignCTL(
                encoding,
                &ctl_info,
                &sign_info,
                flags,
                ptr::null_mut(),
                &mut size,
            );
            if res == 0 {
                return Err(io::Error::last_os_error());
            }

            let mut encoded = vec![0; size as usize];

            let res = Cryptography::CryptMsgEncodeAndSignCTL(
                encoding,
                &ctl_info,
                &sign_info,
                flags,
                encoded.as_mut_ptr() as *mut u8,
                &mut size,
            );
            if res == 0 {
                return Err(io::Error::last_os_error());
            }

            Ok(encoded)
        }
    }
}

fn cert_entry(cert: &CertContext) -> io::Result<Vec<u8>> {
    unsafe {
        let mut size: u32 = 0;

        let res = Cryptography::CertCreateCTLEntryFromCertificateContextProperties(
            cert.as_inner(),
            0,
            ptr::null(),
            Cryptography::CTL_ENTRY_FROM_PROP_CHAIN_FLAG,
            ptr::null_mut(),
            ptr::null_mut(),
            &mut size,
        );
        if res == 0 {
            return Err(io::Error::last_os_error());
        }

        let mut entry = vec![0u8; size as usize];
        let res = Cryptography::CertCreateCTLEntryFromCertificateContextProperties(
            cert.as_inner(),
            0,
            ptr::null(),
            Cryptography::CTL_ENTRY_FROM_PROP_CHAIN_FLAG,
            ptr::null_mut(),
            entry.as_mut_ptr() as *mut Cryptography::CTL_ENTRY,
            &mut size,
        );
        if res == 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(entry)
        }
    }
}

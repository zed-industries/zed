//! `Verifier` implementation for Windows targets.
//!
//! The design of the rustls-native-certs crate for Windows doesn't work
//! completely enough. In general it is hard to emulate enough of what
//! Windows does to be compatible with all users' configurations, especially
//! when corporate MitM proxies or custom CAs or complex trust policies are
//! used. Instead, delegate to Windows's own certificate validation engine
//! directly.
//!
//! This implementation was modeled on:
//! * Chromium's [cert_verify_proc_win.cc] and [x509_util_win.cc]
//! * Golang's [root_windows.go]
//! * [Microsoft's Documentation] and [Microsoft's Example]
//!
//! [cert_verify_proc_win.cc]: <https://chromium.googlesource.com/chromium/src/net/+/refs/heads/main/cert/cert_verify_proc_win.cc>
//! [x509_util_win.cc]: <https://chromium.googlesource.com/chromium/src/net/+/refs/heads/main/cert/x509_util_win.cc>
//! [root_windows.go]: <https://github.com/golang/go/blob/master/src/crypto/x509/root_windows.go>
//! [Microsoft's Documentation]: <https://docs.microsoft.com/en-us/windows/win32/api/wincrypt/nf-wincrypt-certgetcertificatechain>
//! [Microsoft's Example]: <https://docs.microsoft.com/en-us/windows/win32/seccrypto/example-c-program-creating-a-certificate-chain>

use super::{log_server_cert, ALLOWED_EKUS};
use once_cell::sync::OnceCell;
use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerifier};
use rustls::crypto::{verify_tls12_signature, verify_tls13_signature, CryptoProvider};
use rustls::pki_types;
use rustls::{
    CertificateError, DigitallySignedStruct, Error as TlsError, Error::InvalidCertificate,
    SignatureScheme,
};
use std::{
    convert::TryInto,
    mem::{self, MaybeUninit},
    os::raw::c_void,
    ptr::{self, NonNull},
    sync::Arc,
};
use windows_sys::Win32::{
    Foundation::{
        BOOL, CERT_E_CN_NO_MATCH, CERT_E_EXPIRED, CERT_E_INVALID_NAME, CERT_E_UNTRUSTEDROOT,
        CERT_E_WRONG_USAGE, CRYPT_E_REVOKED, FILETIME, TRUE,
    },
    Security::Cryptography::{
        CertAddEncodedCertificateToStore, CertCloseStore, CertCreateCertificateChainEngine,
        CertFreeCertificateChain, CertFreeCertificateChainEngine, CertFreeCertificateContext,
        CertGetCertificateChain, CertOpenStore, CertSetCertificateContextProperty,
        CertVerifyCertificateChainPolicy, HTTPSPolicyCallbackData, AUTHTYPE_SERVER,
        CERT_CHAIN_CACHE_END_CERT, CERT_CHAIN_CONTEXT, CERT_CHAIN_ENGINE_CONFIG,
        CERT_CHAIN_POLICY_IGNORE_ALL_REV_UNKNOWN_FLAGS, CERT_CHAIN_POLICY_PARA,
        CERT_CHAIN_POLICY_SSL, CERT_CHAIN_POLICY_STATUS,
        CERT_CHAIN_REVOCATION_ACCUMULATIVE_TIMEOUT, CERT_CHAIN_REVOCATION_CHECK_END_CERT,
        CERT_CONTEXT, CERT_OCSP_RESPONSE_PROP_ID, CERT_SET_PROPERTY_IGNORE_PERSIST_ERROR_FLAG,
        CERT_STORE_ADD_ALWAYS, CERT_STORE_DEFER_CLOSE_UNTIL_LAST_FREE_FLAG, CERT_STORE_PROV_MEMORY,
        CERT_STRONG_SIGN_PARA, CERT_TRUST_IS_PARTIAL_CHAIN, CERT_USAGE_MATCH, CRYPT_INTEGER_BLOB,
        CTL_USAGE, USAGE_MATCH_TYPE_AND, X509_ASN_ENCODING,
    },
};

// The `windows-sys` definition for `CERT_CHAIN_PARA` does not take old OS versions
// into account so we define it ourselves for better (hypothetical) OS backwards compat.
// In the future a compile-time size assertion can be added against the upstream type to help stay in sync.
#[allow(non_camel_case_types, non_snake_case)]
#[repr(C)]
struct CERT_CHAIN_PARA {
    pub cbSize: u32,
    pub RequestedUsage: CERT_USAGE_MATCH,
    pub RequestedIssuancePolicy: CERT_USAGE_MATCH,
    pub dwUrlRetrievalTimeout: u32,
    pub fCheckRevocationFreshnessTime: BOOL,
    pub dwRevocationFreshnessTime: u32,
    pub pftCacheResync: *mut FILETIME,
    // XXX: `pStrongSignPara` and `dwStrongSignFlags` might or might not be defined on the current system. It started
    // being available in Windows 8. See https://docs.microsoft.com/en-us/windows/win32/api/wincrypt/ns-wincrypt-cert_chain_para
    #[cfg(not(target_vendor = "win7"))]
    pub pStrongSignPara: *const CERT_STRONG_SIGN_PARA,
    #[cfg(not(target_vendor = "win7"))]
    pub dwStrongSignFlags: u32,
}

use crate::verification::invalid_certificate;

// SAFETY: see method implementation
unsafe impl ZeroedWithSize for CERT_CHAIN_PARA {
    fn zeroed_with_size() -> Self {
        // SAFETY: `CERT_CHAIN_PARA` only contains pointers and integers, which are safe to zero.
        // Additionally, MSDN states you *MUST* zero all unused fields.
        let mut new: Self = unsafe { mem::zeroed() };
        new.cbSize = Self::SIZE;
        new
    }
}

// SAFETY: see method implementation
unsafe impl ZeroedWithSize for HTTPSPolicyCallbackData {
    fn zeroed_with_size() -> Self {
        // SAFETY: zeroed is needed here since it contains a union.
        let mut new: Self = unsafe { mem::zeroed() };
        new.Anonymous.cbSize = Self::SIZE;
        new
    }
}

// SAFETY: see method implementation
unsafe impl ZeroedWithSize for CERT_CHAIN_POLICY_PARA {
    fn zeroed_with_size() -> Self {
        // SAFETY: This structure only contains integers and pointers.
        let mut new: Self = unsafe { mem::zeroed() };
        new.cbSize = Self::SIZE;
        new
    }
}

// SAFETY: see method implementation
unsafe impl ZeroedWithSize for CERT_CHAIN_ENGINE_CONFIG {
    fn zeroed_with_size() -> Self {
        // SAFETY: This structure only contains integers and pointers.
        let mut new: Self = unsafe { mem::zeroed() };
        new.cbSize = Self::SIZE;
        new
    }
}

struct CertChain {
    inner: NonNull<CERT_CHAIN_CONTEXT>,
}

impl CertChain {
    fn verify_chain_policy(
        &self,
        mut server_null_terminated: Vec<u16>,
    ) -> Result<CERT_CHAIN_POLICY_STATUS, TlsError> {
        let mut extra_params = HTTPSPolicyCallbackData::zeroed_with_size();
        extra_params.dwAuthType = AUTHTYPE_SERVER;
        // `server_null_terminated` outlives `extra_params`.
        extra_params.pwszServerName = server_null_terminated.as_mut_ptr();

        let mut params = CERT_CHAIN_POLICY_PARA::zeroed_with_size();
        // Ignore any errors when trying to obtain OCSP revocation information.
        // This is also done in OpenSSL, Secure Transport from Apple, etc.
        params.dwFlags = CERT_CHAIN_POLICY_IGNORE_ALL_REV_UNKNOWN_FLAGS;
        // `extra_params` outlives `params`.
        params.pvExtraPolicyPara = NonNull::from(&mut extra_params).cast::<c_void>().as_ptr();

        let mut status: MaybeUninit<CERT_CHAIN_POLICY_STATUS> = MaybeUninit::uninit();

        // SAFETY: The certificate chain is non-null, `params` is valid for reads, and its valid to write to `status`.
        let res = unsafe {
            CertVerifyCertificateChainPolicy(
                CERT_CHAIN_POLICY_SSL,
                self.inner.as_ptr(),
                &params,
                status.as_mut_ptr(),
            )
        };

        // This should rarely, if ever, be false since it would imply no TLS verification
        // is currently possible on the system: https://docs.microsoft.com/en-us/windows/win32/api/wincrypt/nf-wincrypt-certverifycertificatechainpolicy#return-value
        if res != TRUE {
            return Err(TlsError::General(String::from(
                "TLS certificate verification was unavailable on the system!",
            )));
        }

        // SAFETY: The verification call was checked to have succeeded, so the status
        // is written correctly and initialized.
        let status = unsafe { status.assume_init() };
        Ok(status)
    }
}

impl Drop for CertChain {
    fn drop(&mut self) {
        // SAFETY: The pointer is guaranteed to be non-null.
        unsafe { CertFreeCertificateChain(self.inner.as_ptr()) }
    }
}

/// A representation of a certificate.
///
/// The `CertificateStore` must be opened with the correct flags to ensure the
/// certificate may outlive it; see the `CertificateStore` documentation.
struct Certificate {
    inner: NonNull<CERT_CONTEXT>,
}

impl Certificate {
    /// Sets the specified property of this certificate context.
    ///
    /// ### Safety
    /// `prop_data` must be a valid pointer for the property type.
    unsafe fn set_property(
        &mut self,
        prop_id: u32,
        prop_data: *const c_void,
    ) -> Result<(), TlsError> {
        // SAFETY: `cert` points to a valid certificate context and the OCSP data is valid to read.
        call_with_last_error(|| {
            (CertSetCertificateContextProperty(
                self.inner.as_ptr(),
                prop_id,
                CERT_SET_PROPERTY_IGNORE_PERSIST_ERROR_FLAG,
                prop_data,
            ) == TRUE)
                .then_some(())
        })
    }
}

impl Drop for Certificate {
    fn drop(&mut self) {
        // SAFETY: The certificate context is non-null and points to a valid location.
        unsafe { CertFreeCertificateContext(self.inner.as_ptr()) };
    }
}

#[derive(Debug)]
struct CertEngine {
    inner: NonNull<c_void>, // HCERTENGINECONTEXT
}

impl CertEngine {
    fn new_with_extra_roots(
        roots: impl IntoIterator<Item = pki_types::CertificateDer<'static>>,
    ) -> Result<Self, TlsError> {
        let mut exclusive_store = CertificateStore::new()?;
        for root in roots {
            exclusive_store.add_cert(&root)?;
        }

        let mut config = CERT_CHAIN_ENGINE_CONFIG::zeroed_with_size();
        config.hExclusiveRoot = exclusive_store.inner.as_ptr();

        let mut engine = 0;
        // SAFETY: `engine` is valid to be written to and the config is valid to be read.
        let res = unsafe { CertCreateCertificateChainEngine(&config, &mut engine) };

        #[allow(clippy::as_conversions)]
        let engine = call_with_last_error(|| match NonNull::new(engine as *mut c_void) {
            Some(c) if res == TRUE => Some(c),
            _ => None,
        })?;
        Ok(Self { inner: engine })
    }

    #[cfg(any(test, feature = "ffi-testing", feature = "dbg"))]
    fn new_with_fake_root(root: &[u8]) -> Result<Self, TlsError> {
        use windows_sys::Win32::Security::Cryptography::{
            CERT_CHAIN_CACHE_ONLY_URL_RETRIEVAL, CERT_CHAIN_ENABLE_CACHE_AUTO_UPDATE,
        };

        let mut root_store = CertificateStore::new()?;
        root_store.add_cert(root)?;

        let mut config = CERT_CHAIN_ENGINE_CONFIG::zeroed_with_size();
        // We use these flags for the following reasons:
        //
        // - CERT_CHAIN_CACHE_ONLY_URL_RETRIEVAL is used in an attempt to stop Windows from using the internet to
        // fetch anything during the tests, regardless of what test data is used.
        //
        // - CERT_CHAIN_ENABLE_CACHE_AUTO_UPDATE is used as a minor performance optimization to allow Windows to reuse
        // data inside of a test and avoid any extra parsing, etc, it might need to do pulling directly from the store each time.
        //
        // Ref: https://docs.microsoft.com/en-us/windows/win32/api/wincrypt/ns-wincrypt-cert_chain_engine_config
        config.dwFlags = CERT_CHAIN_CACHE_ONLY_URL_RETRIEVAL | CERT_CHAIN_ENABLE_CACHE_AUTO_UPDATE;
        config.hExclusiveRoot = root_store.inner.as_ptr();

        let mut engine = 0;
        // SAFETY: `engine` is valid to be written to and the config is valid to be read.
        let res = unsafe { CertCreateCertificateChainEngine(&config, &mut engine) };

        #[allow(clippy::as_conversions)]
        let engine = call_with_last_error(|| match NonNull::new(engine as *mut c_void) {
            Some(c) if res == TRUE => Some(c),
            _ => None,
        })?;

        Ok(Self { inner: engine })
    }

    #[allow(clippy::as_conversions)]
    fn as_ptr(&self) -> isize {
        self.inner.as_ptr() as isize
    }
}

impl Drop for CertEngine {
    fn drop(&mut self) {
        // SAFETY: The engine pointer is guaranteed to be non-null.
        unsafe { CertFreeCertificateChainEngine(self.as_ptr()) };
    }
}

// SAFETY: We know no other threads is mutating the `CertEngine`, because it would require `unsafe`.
// Across the FFI, `CertGetCertificateChain` don't mutate it either.
unsafe impl Sync for CertEngine {}
// SAFETY: All methods of `CertEngine`, including `Drop`, are safe to be called from other
// threads, because all contained resources are owned by Windows and we only maintain reference counted handles to them.
unsafe impl Send for CertEngine {}

/// An in-memory Windows certificate store.
///
/// # Safety
///
/// `CertificateStore` creates `Certificate` objects that may outlive the
/// `CertificateStore`. This is only safe to do if the certificate store is
/// constructed with `CERT_STORE_DEFER_CLOSE_UNTIL_LAST_FREE_FLAG`.
struct CertificateStore {
    inner: NonNull<c_void>, // HCERTSTORE
    // In production code, this is always `None`.
    //
    // During tests, we set this to `Some` as the tests use a
    // custom verification engine that only uses specific roots.
    engine: Option<CertEngine>, // HCERTENGINECONTEXT
}

impl Drop for CertificateStore {
    fn drop(&mut self) {
        // SAFETY: See the `CertificateStore` documentation.
        unsafe { CertCloseStore(self.inner.as_ptr(), 0) };
    }
}

impl CertificateStore {
    /// Creates a new, in-memory certificate store.
    fn new() -> Result<Self, TlsError> {
        let store = call_with_last_error(|| {
            // SAFETY: Called with valid constants and result is checked to be non-null.
            // The `CERT_STORE_DEFER_CLOSE_UNTIL_LAST_FREE_FLAG` flag is critical;
            // see the `CertificateStore` documentation for more info.
            NonNull::new(unsafe {
                CertOpenStore(
                    CERT_STORE_PROV_MEMORY,
                    0, // Set to zero since this uses `PROV_MEMORY`.
                    0, // This field shouldn't be used.
                    CERT_STORE_DEFER_CLOSE_UNTIL_LAST_FREE_FLAG,
                    ptr::null(),
                )
            })
        })?;

        // Use the system's default root store and rules.
        Ok(Self {
            inner: store,
            engine: None,
        })
    }

    #[cfg(any(test, feature = "ffi-testing", feature = "dbg"))]
    fn new_with_fake_root(root: &[u8]) -> Result<Self, TlsError> {
        let mut inner = Self::new()?;

        let mut root_store = CertificateStore::new()?;
        root_store.add_cert(root)?;

        let engine = CertEngine::new_with_fake_root(root)?;
        inner.engine = Some(engine);

        Ok(inner)
    }

    /// Adds the provided certificate to the store.
    ///
    /// The certificate must be encoded as ASN.1 DER.
    ///
    /// Errors if the certificate was malformed and couldn't be added.
    fn add_cert(&mut self, cert: &[u8]) -> Result<Certificate, TlsError> {
        let mut cert_context: *mut CERT_CONTEXT = ptr::null_mut();

        // SAFETY: `inner` is a valid certificate store, and `cert` is a valid a byte array valid
        // for reads, the correct length is being provided, and `cert_context` is valid to write to.
        let res = unsafe {
            CertAddEncodedCertificateToStore(
                self.inner.as_ptr(),
                X509_ASN_ENCODING,
                cert.as_ptr(),
                cert.len()
                    .try_into()
                    .map_err(|_| InvalidCertificate(CertificateError::BadEncoding))?,
                CERT_STORE_ADD_ALWAYS,
                &mut cert_context,
            )
        };

        // SAFETY: Constructing a `Certificate` is only safe if the store was
        // created with the right flags; see the `CertificateStore` docs.
        match (res, NonNull::new(cert_context)) {
            (TRUE, Some(cert)) => Ok(Certificate { inner: cert }),
            _ => Err(InvalidCertificate(CertificateError::BadEncoding)),
        }
    }

    fn new_chain_in(
        &self,
        certificate: &Certificate,
        now: pki_types::UnixTime,
        engine: Option<&CertEngine>,
    ) -> Result<CertChain, TlsError> {
        let mut cert_chain = ptr::null_mut();

        let mut parameters = CERT_CHAIN_PARA::zeroed_with_size();

        #[allow(clippy::as_conversions)]
        // https://docs.microsoft.com/en-us/windows/win32/api/wincrypt/ns-wincrypt-cert_usage_match
        let usage = CERT_USAGE_MATCH {
            dwType: USAGE_MATCH_TYPE_AND,
            Usage: CTL_USAGE {
                cUsageIdentifier: ALLOWED_EKUS.len() as u32,
                rgpszUsageIdentifier: ALLOWED_EKUS.as_ptr() as *mut windows_sys::core::PSTR,
            },
        };
        parameters.RequestedUsage = usage;

        #[allow(clippy::as_conversions)]
        let time = {
            /// Seconds between Jan 1st, 1601 and Jan 1, 1970.
            const UNIX_ADJUSTMENT: std::time::Duration =
                std::time::Duration::from_secs(11_644_473_600);

            let since_unix_epoch = now.as_secs();

            // Convert the duration from the UNIX epoch to the Window one, and then convert
            // the result into a `FILETIME` structure.

            let since_windows_epoch = since_unix_epoch + UNIX_ADJUSTMENT.as_secs();
            let intervals = (since_windows_epoch * 1_000_000_000) / 100;

            FILETIME {
                dwLowDateTime: (intervals & u32::MAX as u64) as u32,
                dwHighDateTime: (intervals >> 32) as u32,
            }
        };

        // `CERT_CHAIN_REVOCATION_CHECK_END_CERT` only checks revocation for end cert. See the crate's revocation documentation
        // for more details.
        // `CERT_CHAIN_REVOCATION_ACCUMULATIVE_TIMEOUT` accumulates network retrievals timeouts
        // to limit network time and improve performance.
        // `CERT_CHAIN_CACHE_END_CERT` speeds up the common case of multiple connections to same server.
        const FLAGS: u32 = CERT_CHAIN_REVOCATION_CHECK_END_CERT
            | CERT_CHAIN_REVOCATION_ACCUMULATIVE_TIMEOUT
            | CERT_CHAIN_CACHE_END_CERT;

        // Lowering URL retrieval timeout from default 15s to 10s to account for higher internet speeds
        parameters.dwUrlRetrievalTimeout = 10 * 1000; // milliseconds

        // SAFETY: `cert` points to a valid certificate context, parameters is valid for reads, `cert_chain` is valid
        // for writes, and the certificate store is valid and initialized.
        let res = unsafe {
            // XXX: Due to the redefinition of `CERT_CHAIN_PARA`, we need to do pointer casts
            // in order to pass our expanded structure into `CertGetCertificateChain`.
            // This is safe because the OS uses `cbSize` to know if the extra parameters
            // are present or not. As we set `cbSize` correctly, the fields can be read from correctly.
            let parameters = NonNull::from(&parameters).cast().as_ptr();

            CertGetCertificateChain(
                engine.map(CertEngine::as_ptr).unwrap_or(0),
                certificate.inner.as_ptr(),
                &time,
                self.inner.as_ptr(),
                parameters,
                FLAGS,
                ptr::null_mut(),
                &mut cert_chain,
            )
        };

        // XXX: Windows will internally map the chain's `TrustStatus.dwErrorStatus` to a `dwError` when
        // a chain policy is verified, so we only check for errors there.
        call_with_last_error(|| match NonNull::new(cert_chain) {
            Some(c) if res == TRUE => Some(CertChain { inner: c }),
            _ => None,
        })
    }
}

fn call_with_last_error<T, F: FnMut() -> Option<T>>(mut call: F) -> Result<T, TlsError> {
    if let Some(res) = call() {
        Ok(res)
    } else {
        Err(TlsError::General(
            std::io::Error::last_os_error().to_string(),
        ))
    }
}

/// A TLS certificate verifier that utilizes the Windows certificate facilities.
#[derive(Debug)]
pub struct Verifier {
    /// Testing only: The root CA certificate to trust.
    #[cfg(any(test, feature = "ffi-testing", feature = "dbg"))]
    test_only_root_ca_override: Option<Vec<u8>>,
    pub(super) crypto_provider: OnceCell<Arc<CryptoProvider>>,
    /// Extra trust anchors to add to the verifier above and beyond those provided by
    /// the system-provided trust stores.
    extra_roots: Option<CertEngine>,
}

impl Verifier {
    /// Creates a new instance of a TLS certificate verifier that utilizes the
    /// Windows certificate facilities.
    ///
    /// A [`CryptoProvider`] must be set with
    /// [`set_provider`][Verifier::set_provider]/[`with_provider`][Verifier::with_provider] or
    /// [`CryptoProvider::install_default`] before the verifier can be used.
    pub fn new() -> Self {
        Self {
            #[cfg(any(test, feature = "ffi-testing", feature = "dbg"))]
            test_only_root_ca_override: None,
            crypto_provider: OnceCell::new(),
            extra_roots: None,
        }
    }

    /// Creates a new instance of a TLS certificate verifier that utilizes the
    /// Windows certificate facilities and augmented by the provided extra root certificates.
    ///
    /// A [`CryptoProvider`] must be set with
    /// [`set_provider`][Verifier::set_provider]/[`with_provider`][Verifier::with_provider] or
    /// [`CryptoProvider::install_default`] before the verifier can be used.
    pub fn new_with_extra_roots(
        roots: impl IntoIterator<Item = pki_types::CertificateDer<'static>>,
    ) -> Result<Self, TlsError> {
        let cert_engine = CertEngine::new_with_extra_roots(roots)?;
        Ok(Self {
            #[cfg(any(test, feature = "ffi-testing", feature = "dbg"))]
            test_only_root_ca_override: None,
            crypto_provider: OnceCell::new(),
            extra_roots: Some(cert_engine),
        })
    }

    /// Creates a test-only TLS certificate verifier which trusts our fake root CA cert.
    #[cfg(any(test, feature = "ffi-testing", feature = "dbg"))]
    pub(crate) fn new_with_fake_root(root: &[u8]) -> Self {
        Self {
            test_only_root_ca_override: Some(root.into()),
            crypto_provider: OnceCell::new(),
            extra_roots: None,
        }
    }

    /// Verifies a certificate and its chain for the specified `server`.
    ///
    /// Return `Ok(())` if the certificate was valid.
    fn verify_certificate(
        &self,
        primary_cert: &[u8],
        intermediate_certs: &[&[u8]],
        server: &[u8],
        ocsp_data: Option<&[u8]>,
        now: pki_types::UnixTime,
    ) -> Result<(), TlsError> {
        #[cfg(any(test, feature = "ffi-testing", feature = "dbg"))]
        let mut store = match self.test_only_root_ca_override.as_ref() {
            Some(test_only_root_ca_override) => {
                CertificateStore::new_with_fake_root(test_only_root_ca_override)?
            }
            None => CertificateStore::new()?,
        };

        #[cfg(not(any(test, feature = "ffi-testing", feature = "dbg")))]
        let mut store = CertificateStore::new()?;

        let mut primary_cert = store.add_cert(primary_cert)?;

        for cert in intermediate_certs.iter().copied() {
            store.add_cert(cert)?;
        }

        if let Some(ocsp_data) = ocsp_data {
            #[allow(clippy::as_conversions)]
            let data = CRYPT_INTEGER_BLOB {
                cbData: ocsp_data.len().try_into().map_err(|_| {
                    invalid_certificate("Malformed OCSP response stapled to server certificate")
                })?,
                pbData: ocsp_data.as_ptr() as *mut u8,
            };

            // SAFETY: `data` is a valid pointer and matches the property ID.
            unsafe {
                primary_cert.set_property(
                    CERT_OCSP_RESPONSE_PROP_ID,
                    NonNull::from(&data).cast::<c_void>().as_ptr(),
                )?;
            }
        }

        // Encode UTF-16, null-terminated
        let server: Vec<u16> = server
            .iter()
            .map(|c| u16::from(*c))
            .chain(Some(0))
            .collect();

        let mut cert_chain = store.new_chain_in(&primary_cert, now, store.engine.as_ref())?;

        // We only use `TrustStatus` here because it hasn't had verification performed on it.
        // SAFETY: The pointer is guaranteed to be non-null.
        let is_partial_chain = unsafe { *cert_chain.inner.as_ptr() }
            .TrustStatus
            .dwErrorStatus
            & CERT_TRUST_IS_PARTIAL_CHAIN
            != 0;

        // If we have extra roots and building the chain gave us an error, we try to build a
        // new one with the extra roots.
        if is_partial_chain && self.extra_roots.is_some() {
            let mut store = CertificateStore::new()?;

            for cert in intermediate_certs.iter().copied() {
                store.add_cert(cert)?;
            }

            cert_chain = store.new_chain_in(&primary_cert, now, self.extra_roots.as_ref())?;
        }

        let status = cert_chain.verify_chain_policy(server)?;

        if status.dwError == 0 {
            return Ok(());
        }

        // Only map the errors we have tests for.
        #[allow(clippy::as_conversions)]
        let win_error = status.dwError as i32;
        Err(match win_error {
            CERT_E_CN_NO_MATCH | CERT_E_INVALID_NAME => {
                InvalidCertificate(CertificateError::NotValidForName)
            }
            CRYPT_E_REVOKED => InvalidCertificate(CertificateError::Revoked),
            CERT_E_EXPIRED => InvalidCertificate(CertificateError::Expired),
            CERT_E_UNTRUSTEDROOT => InvalidCertificate(CertificateError::UnknownIssuer),
            CERT_E_WRONG_USAGE => InvalidCertificate(CertificateError::InvalidPurpose),
            error_num => {
                let err = std::io::Error::from_raw_os_error(error_num);
                // The included error message has both the description and raw OS error code.
                invalid_certificate(err.to_string())
            }
        })
    }
}

impl ServerCertVerifier for Verifier {
    fn verify_server_cert(
        &self,
        end_entity: &pki_types::CertificateDer<'_>,
        intermediates: &[pki_types::CertificateDer<'_>],
        server_name: &pki_types::ServerName,
        ocsp_response: &[u8],
        now: pki_types::UnixTime,
    ) -> Result<rustls::client::danger::ServerCertVerified, TlsError> {
        log_server_cert(end_entity);

        let name = server_name.to_str();

        let intermediate_certs: Vec<&[u8]> = intermediates.iter().map(|c| c.as_ref()).collect();

        let ocsp_data = if !ocsp_response.is_empty() {
            Some(ocsp_response)
        } else {
            None
        };

        match self.verify_certificate(
            end_entity.as_ref(),
            &intermediate_certs,
            name.as_bytes(),
            ocsp_data,
            now,
        ) {
            Ok(()) => Ok(rustls::client::danger::ServerCertVerified::assertion()),
            Err(e) => {
                // SAFETY:
                // Errors are our own custom errors, WinAPI errors, or static strings.
                log::error!("failed to verify TLS certificate: {}", e);
                Err(e)
            }
        }
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &pki_types::CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, TlsError> {
        verify_tls12_signature(
            message,
            cert,
            dss,
            &self.get_provider().signature_verification_algorithms,
        )
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &pki_types::CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, TlsError> {
        verify_tls13_signature(
            message,
            cert,
            dss,
            &self.get_provider().signature_verification_algorithms,
        )
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.get_provider()
            .signature_verification_algorithms
            .supported_schemes()
    }
}

impl Default for Verifier {
    fn default() -> Self {
        Self::new()
    }
}

/// A trait to represent an object that can be safely created with all zero values
/// and have a size assigned to it.
///
/// # Safety
///
/// This has the same safety requirements as [std::mem::zeroed].
unsafe trait ZeroedWithSize: Sized {
    const SIZE: u32 = {
        let size = core::mem::size_of::<Self>();

        // NB: `TryInto` isn't stable in const yet.
        #[allow(clippy::as_conversions)]
        if size <= u32::MAX as usize {
            size as u32
        } else {
            panic!("structure was larger then DWORD")
        }
    };

    /// Returns a zeroed structure with its structure size (`cbSize`) field set to the correct value.
    fn zeroed_with_size() -> Self;
}

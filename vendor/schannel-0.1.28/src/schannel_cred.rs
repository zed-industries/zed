//! Schannel credentials.
use std::ptr;
use std::sync::Arc;
use std::{io, mem};

use windows_sys::Win32::Foundation;
use windows_sys::Win32::Security::Authentication::Identity;
use windows_sys::Win32::Security::{Credentials, Cryptography};

use crate::cert_context::CertContext;
use crate::Inner;

/// The communication direction that an `SchannelCred` will support.
#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    /// Server-side, inbound connections.
    Inbound,
    /// Client-side, outbound connections.
    Outbound,
}

/// Algorithms supported by Schannel.
// https://msdn.microsoft.com/en-us/library/windows/desktop/aa375549(v=vs.85).aspx
#[derive(Debug, Copy, Clone)]
#[repr(u32)]
#[non_exhaustive]
pub enum Algorithm {
    /// Advanced Encryption Standard (AES).
    Aes = Cryptography::ALG_CLASS_DATA_ENCRYPT
        | Cryptography::ALG_TYPE_BLOCK
        | Cryptography::ALG_SID_AES,
    /// 128 bit AES.
    Aes128 = Cryptography::ALG_CLASS_DATA_ENCRYPT
        | Cryptography::ALG_TYPE_BLOCK
        | Cryptography::ALG_SID_AES_128,
    /// 192 bit AES.
    Aes192 = Cryptography::ALG_CLASS_DATA_ENCRYPT
        | Cryptography::ALG_TYPE_BLOCK
        | Cryptography::ALG_SID_AES_192,
    /// 256 bit AES.
    Aes256 = Cryptography::ALG_CLASS_DATA_ENCRYPT
        | Cryptography::ALG_TYPE_BLOCK
        | Cryptography::ALG_SID_AES_256,
    /// Temporary algorithm identifier for handles of Diffie-Hellman–agreed keys.
    AgreedkeyAny = Cryptography::ALG_CLASS_KEY_EXCHANGE
        | Cryptography::ALG_TYPE_DH
        | Cryptography::ALG_SID_AGREED_KEY_ANY,
    /// An algorithm to create a 40-bit DES key that has parity bits and zeroed key bits to make
    /// its key length 64 bits.
    CylinkMek = Cryptography::ALG_CLASS_DATA_ENCRYPT
        | Cryptography::ALG_TYPE_BLOCK
        | Cryptography::ALG_SID_CYLINK_MEK,
    /// DES encryption algorithm.
    Des = Cryptography::ALG_CLASS_DATA_ENCRYPT
        | Cryptography::ALG_TYPE_BLOCK
        | Cryptography::ALG_SID_DES,
    /// DESX encryption algorithm.
    Desx = Cryptography::ALG_CLASS_DATA_ENCRYPT
        | Cryptography::ALG_TYPE_BLOCK
        | Cryptography::ALG_SID_DESX,
    /// Diffie-Hellman ephemeral key exchange algorithm.
    DhEphem = Cryptography::ALG_CLASS_KEY_EXCHANGE
        | Cryptography::ALG_TYPE_DH
        | Cryptography::ALG_SID_DH_EPHEM,
    /// Diffie-Hellman store and forward key exchange algorithm.
    DhSf = Cryptography::ALG_CLASS_KEY_EXCHANGE
        | Cryptography::ALG_TYPE_DH
        | Cryptography::ALG_SID_DH_SANDF,
    /// DSA public key signature algorithm.
    DssSign = Cryptography::ALG_CLASS_SIGNATURE
        | Cryptography::ALG_TYPE_DSS
        | Cryptography::ALG_SID_DSS_ANY,
    /// Elliptic curve Diffie-Hellman key exchange algorithm.
    Ecdh = Cryptography::ALG_CLASS_KEY_EXCHANGE
        | Cryptography::ALG_TYPE_DH
        | Cryptography::ALG_SID_ECDH,
    /// Ephemeral elliptic curve Diffie-Hellman key exchange algorithm.
    EcdhEphem = Cryptography::ALG_CLASS_KEY_EXCHANGE
        | Cryptography::ALG_TYPE_ECDH
        | Cryptography::ALG_SID_ECDH_EPHEM,
    /// Elliptic curve digital signature algorithm.
    Ecdsa = Cryptography::ALG_CLASS_SIGNATURE
        | Cryptography::ALG_TYPE_DSS
        | Cryptography::ALG_SID_ECDSA,
    /// One way function hashing algorithm.
    HashReplaceOwf = Cryptography::ALG_CLASS_HASH
        | Cryptography::ALG_TYPE_ANY
        | Cryptography::ALG_SID_HASH_REPLACE_OWF,
    /// Hughes MD5 hashing algorithm.
    HughesMd5 = Cryptography::ALG_CLASS_KEY_EXCHANGE
        | Cryptography::ALG_TYPE_ANY
        | Cryptography::ALG_SID_MD5,
    /// HMAC keyed hash algorithm.
    Hmac = Cryptography::ALG_CLASS_HASH | Cryptography::ALG_TYPE_ANY | Cryptography::ALG_SID_HMAC,
    /// MAC keyed hash algorithm.
    Mac = Cryptography::ALG_CLASS_HASH | Cryptography::ALG_TYPE_ANY | Cryptography::ALG_SID_MAC,
    /// MD2 hashing algorithm.
    Md2 = Cryptography::ALG_CLASS_HASH | Cryptography::ALG_TYPE_ANY | Cryptography::ALG_SID_MD2,
    /// MD4 hashing algorithm.
    Md4 = Cryptography::ALG_CLASS_HASH | Cryptography::ALG_TYPE_ANY | Cryptography::ALG_SID_MD4,
    /// MD5 hashing algorithm.
    Md5 = Cryptography::ALG_CLASS_HASH | Cryptography::ALG_TYPE_ANY | Cryptography::ALG_SID_MD5,
    /// No signature algorithm..
    NoSign =
        Cryptography::ALG_CLASS_SIGNATURE | Cryptography::ALG_TYPE_ANY | Cryptography::ALG_SID_ANY,
    /// RC2 block encryption algorithm.
    Rc2 = Cryptography::ALG_CLASS_DATA_ENCRYPT
        | Cryptography::ALG_TYPE_BLOCK
        | Cryptography::ALG_SID_RC2,
    /// RC4 stream encryption algorithm.
    Rc4 = Cryptography::ALG_CLASS_DATA_ENCRYPT
        | Cryptography::ALG_TYPE_STREAM
        | Cryptography::ALG_SID_RC4,
    /// RC5 block encryption algorithm.
    Rc5 = Cryptography::ALG_CLASS_DATA_ENCRYPT
        | Cryptography::ALG_TYPE_BLOCK
        | Cryptography::ALG_SID_RC5,
    /// RSA public key exchange algorithm.
    RsaKeyx = Cryptography::ALG_CLASS_KEY_EXCHANGE
        | Cryptography::ALG_TYPE_RSA
        | Cryptography::ALG_SID_RSA_ANY,
    /// RSA public key signature algorithm.
    RsaSign = Cryptography::ALG_CLASS_SIGNATURE
        | Cryptography::ALG_TYPE_RSA
        | Cryptography::ALG_SID_RSA_ANY,
    /// SHA hashing algorithm.
    Sha1 = Cryptography::ALG_CLASS_HASH | Cryptography::ALG_TYPE_ANY | Cryptography::ALG_SID_SHA1,
    /// 256 bit SHA hashing algorithm.
    Sha256 =
        Cryptography::ALG_CLASS_HASH | Cryptography::ALG_TYPE_ANY | Cryptography::ALG_SID_SHA_256,
    /// 384 bit SHA hashing algorithm.
    Sha384 =
        Cryptography::ALG_CLASS_HASH | Cryptography::ALG_TYPE_ANY | Cryptography::ALG_SID_SHA_384,
    /// 512 bit SHA hashing algorithm.
    Sha512 =
        Cryptography::ALG_CLASS_HASH | Cryptography::ALG_TYPE_ANY | Cryptography::ALG_SID_SHA_512,
    /// Triple DES encryption algorithm.
    TripleDes = Cryptography::ALG_CLASS_DATA_ENCRYPT
        | Cryptography::ALG_TYPE_BLOCK
        | Cryptography::ALG_SID_3DES,
    /// Two-key triple DES encryption with effective key length equal to 112 bits.
    TripleDes112 = Cryptography::ALG_CLASS_DATA_ENCRYPT
        | Cryptography::ALG_TYPE_BLOCK
        | Cryptography::ALG_SID_3DES_112,
}

/// Protocols supported by Schannel.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum Protocol {
    /// Secure Sockets Layer 3.0
    Ssl3,
    /// Transport Layer Security 1.0
    Tls10,
    /// Transport Layer Security 1.1
    Tls11,
    /// Transport Layer Security 1.2
    Tls12,
    /// Transport Layer Security 1.3
    Tls13,
}

impl Protocol {
    fn dword(self, direction: Direction) -> u32 {
        match (self, direction) {
            (Protocol::Ssl3, Direction::Inbound) => Identity::SP_PROT_SSL3_SERVER,
            (Protocol::Tls10, Direction::Inbound) => Identity::SP_PROT_TLS1_0_SERVER,
            (Protocol::Tls11, Direction::Inbound) => Identity::SP_PROT_TLS1_1_SERVER,
            (Protocol::Tls12, Direction::Inbound) => Identity::SP_PROT_TLS1_2_SERVER,
            (Protocol::Tls13, Direction::Inbound) => Identity::SP_PROT_TLS1_3_SERVER,
            (Protocol::Ssl3, Direction::Outbound) => Identity::SP_PROT_SSL3_CLIENT,
            (Protocol::Tls10, Direction::Outbound) => Identity::SP_PROT_TLS1_0_CLIENT,
            (Protocol::Tls11, Direction::Outbound) => Identity::SP_PROT_TLS1_1_CLIENT,
            (Protocol::Tls12, Direction::Outbound) => Identity::SP_PROT_TLS1_2_CLIENT,
            (Protocol::Tls13, Direction::Outbound) => Identity::SP_PROT_TLS1_3_CLIENT,
        }
    }
}

fn verify_min_os_build(major: u32, build: u32) -> Option<()> {
    use windows_sys::Win32::System::SystemInformation::OSVERSIONINFOW;

    let handle = std::ptr::NonNull::new(unsafe {
        windows_sys::Win32::System::LibraryLoader::GetModuleHandleW(windows_sys::w!("ntdll.dll"))
    })?;
    let rtl_get_ver = unsafe {
        windows_sys::Win32::System::LibraryLoader::GetProcAddress(handle.as_ptr(), windows_sys::s!("RtlGetVersion"))
    }?;

    type RtlGetVersionFunc = unsafe extern "system" fn(*mut OSVERSIONINFOW) -> i32;
    let proc: RtlGetVersionFunc = unsafe { mem::transmute(rtl_get_ver) };

    let mut info: OSVERSIONINFOW = unsafe { mem::zeroed() };
    info.dwOSVersionInfoSize = mem::size_of::<OSVERSIONINFOW>() as u32;

    unsafe { proc(&mut info) };

    if info.dwMajorVersion > major || (info.dwMajorVersion == major && info.dwBuildNumber >= build) {
        Some(())
    } else {
        None
    }
}

/// A builder type for `SchannelCred`s.
#[derive(Default, Debug)]
pub struct Builder {
    supported_algorithms: Option<Vec<Algorithm>>,
    enabled_protocols: Option<Vec<Protocol>>,
    certs: Vec<CertContext>,
}

impl Builder {
    /// Returns a new `Builder`.
    pub fn new() -> Builder {
        Builder::default()
    }

    /// Sets the algorithms supported for credentials created from this builder.
    pub fn supported_algorithms(&mut self, supported_algorithms: &[Algorithm]) -> &mut Builder {
        self.supported_algorithms = Some(supported_algorithms.to_owned());
        self
    }

    /// Sets the protocols enabled for credentials created from this builder.
    pub fn enabled_protocols(&mut self, enabled_protocols: &[Protocol]) -> &mut Builder {
        self.enabled_protocols = Some(enabled_protocols.to_owned());
        self
    }

    /// Add a certificate to get passed down when the credentials are acquired.
    ///
    /// Certificates passed here may specify a certificate that contains a
    /// private key to be used in authenticating the application. Typically,
    /// this is called once for each key exchange method supported by
    /// servers.
    ///
    /// Clients often do not call this function and either depend on Schannel to
    /// find an appropriate certificate or create a certificate later if needed.
    pub fn cert(&mut self, cx: CertContext) -> &mut Builder {
        self.certs.push(cx);
        self
    }

    /// Creates a new `SchannelCred`.
    pub fn acquire(&self, direction: Direction) -> io::Result<SchannelCred> {
        let mut enabled_protocols: u32 = 0;
        if let Some(ref enable_list) = self.enabled_protocols {
            enabled_protocols = enable_list
                .iter()
                .map(|p| p.dword(direction))
                .fold(0, |acc, p| acc | p);
        }

        unsafe {
            let mut cred_data: Identity::SCHANNEL_CRED = mem::zeroed();
            cred_data.dwVersion = Identity::SCHANNEL_CRED_VERSION;
            cred_data.dwFlags = Identity::SCH_USE_STRONG_CRYPTO | Identity::SCH_CRED_NO_DEFAULT_CREDS;
            cred_data.grbitEnabledProtocols = enabled_protocols;
            let mut certs = self.certs.iter().map(|c| c.as_inner()).collect::<Vec<_>>();
            cred_data.cCreds = certs.len() as u32;
            cred_data.paCred = certs.as_mut_ptr() as _;
            
            let mut tls_param: Identity::TLS_PARAMETERS = mem::zeroed();
            let mut cred_data2: Identity::SCH_CREDENTIALS = mem::zeroed();

            let mut pauthdata: *const core::ffi::c_void = ptr::null();
            if let Some(ref supported_algorithms) = self.supported_algorithms {
                cred_data.cSupportedAlgs = supported_algorithms.len() as u32;
                cred_data.palgSupportedAlgs = supported_algorithms.as_ptr() as *mut _;
            } else if verify_min_os_build(10, 17763).is_some() {
                // If no algorithms specified and should be supported, use new SCH_CREDENTIALS interface which supports TLS1.3.
                // Although we check for win10 build 17763 above, I have only seen this work on win 11.
                if enabled_protocols != 0 {
                    tls_param.grbitDisabledProtocols = !enabled_protocols;
                }
                // TODO: support something to select tls13-ciphers
                cred_data2.dwVersion = Identity::SCH_CREDENTIALS_VERSION;
                cred_data2.dwFlags = Identity::SCH_USE_STRONG_CRYPTO | Identity::SCH_CRED_NO_DEFAULT_CREDS;
                cred_data2.cCreds = certs.len() as u32;
                cred_data2.paCred = certs.as_mut_ptr() as _;
                cred_data2.cTlsParameters = 1;
                cred_data2.pTlsParameters = &mut tls_param;
                pauthdata = &mut cred_data2 as *const _ as *const _;
            }
            
            if pauthdata.is_null() {
                pauthdata = &mut cred_data as *const _ as *const _;
            }
            let direction = match direction {
                Direction::Inbound => Identity::SECPKG_CRED_INBOUND,
                Direction::Outbound => Identity::SECPKG_CRED_OUTBOUND,
            };
            let mut handle: Credentials::SecHandle = mem::zeroed();

            match Identity::AcquireCredentialsHandleA(
                ptr::null(),
                Identity::UNISP_NAME_A,
                direction,
                ptr::null_mut(),
                pauthdata,
                None,
                ptr::null_mut(),
                &mut handle,
                ptr::null_mut(),
            ) {
                Foundation::SEC_E_OK => Ok(SchannelCred::from_inner(handle)),
                err => Err(io::Error::from_raw_os_error(err)),
            }
        }
    }
}

/// An SChannel credential.
#[derive(Clone)]
pub struct SchannelCred(Arc<RawCredHandle>);

struct RawCredHandle(Credentials::SecHandle);

impl Drop for RawCredHandle {
    fn drop(&mut self) {
        unsafe {
            Identity::FreeCredentialsHandle(&self.0);
        }
    }
}

impl SchannelCred {
    /// Returns a builder.
    pub fn builder() -> Builder {
        Builder::new()
    }

    unsafe fn from_inner(inner: Credentials::SecHandle) -> SchannelCred {
        SchannelCred(Arc::new(RawCredHandle(inner)))
    }

    pub(crate) fn as_inner(&self) -> Credentials::SecHandle {
        self.0.as_ref().0
    }
}

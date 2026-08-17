//! Bindings to Windows `PCCERT_CONTEXT` APIs.

use std::ffi::{c_void, CStr, OsString};
use std::io;
use std::mem;
use std::os::windows::prelude::*;
use std::ptr;
use std::slice;

use windows_sys::Win32::Foundation;
use windows_sys::Win32::Security::Cryptography;

use crate::cert_store::CertStore;
use crate::crypt_prov::{CryptProv, ProviderType};
use crate::ncrypt_key::NcryptKey;
use crate::Inner;

/// A supported hashing algorithm
#[derive(Copy, Clone)]
pub struct HashAlgorithm(u32, usize);

#[allow(missing_docs)]
impl HashAlgorithm {
    pub fn md5() -> HashAlgorithm {
        HashAlgorithm(
            Cryptography::ALG_CLASS_HASH | Cryptography::ALG_TYPE_ANY | Cryptography::ALG_SID_MD5,
            16,
        )
    }

    pub fn sha1() -> HashAlgorithm {
        HashAlgorithm(
            Cryptography::ALG_CLASS_HASH | Cryptography::ALG_TYPE_ANY | Cryptography::ALG_SID_SHA1,
            20,
        )
    }

    pub fn sha256() -> HashAlgorithm {
        HashAlgorithm(
            Cryptography::ALG_CLASS_HASH
                | Cryptography::ALG_TYPE_ANY
                | Cryptography::ALG_SID_SHA_256,
            32,
        )
    }

    pub fn sha384() -> HashAlgorithm {
        HashAlgorithm(
            Cryptography::ALG_CLASS_HASH
                | Cryptography::ALG_TYPE_ANY
                | Cryptography::ALG_SID_SHA_384,
            48,
        )
    }

    pub fn sha512() -> HashAlgorithm {
        HashAlgorithm(
            Cryptography::ALG_CLASS_HASH
                | Cryptography::ALG_TYPE_ANY
                | Cryptography::ALG_SID_SHA_512,
            64,
        )
    }
}

/// Wrapper of a winapi certificate, or a `PCCERT_CONTEXT`.
#[derive(Debug)]
pub struct CertContext(*const Cryptography::CERT_CONTEXT);

unsafe impl Sync for CertContext {}
unsafe impl Send for CertContext {}

impl Drop for CertContext {
    fn drop(&mut self) {
        unsafe {
            Cryptography::CertFreeCertificateContext(self.0);
        }
    }
}

impl Clone for CertContext {
    fn clone(&self) -> CertContext {
        unsafe { CertContext(Cryptography::CertDuplicateCertificateContext(self.0)) }
    }
}

inner!(CertContext, *const Cryptography::CERT_CONTEXT);

impl CertContext {
    /// Decodes a DER-formatted X509 certificate.
    pub fn new(data: &[u8]) -> io::Result<CertContext> {
        let ret = unsafe {
            Cryptography::CertCreateCertificateContext(
                Cryptography::X509_ASN_ENCODING | Cryptography::PKCS_7_ASN_ENCODING,
                data.as_ptr(),
                data.len() as u32,
            )
        };
        if ret.is_null() {
            Err(io::Error::last_os_error())
        } else {
            Ok(CertContext(ret))
        }
    }

    /// Get certificate in binary DER form
    pub fn to_der(&self) -> &[u8] {
        self.get_encoded_bytes()
    }

    /// Certificate subject public key info
    pub fn subject_public_key_info_der(&self) -> io::Result<Vec<u8>> {
        unsafe {
            let mut len: u32 = 0;
            let ok = Cryptography::CryptEncodeObjectEx(
                Cryptography::X509_ASN_ENCODING,
                Cryptography::CERT_INFO_SUBJECT_PUBLIC_KEY_INFO_FLAG as *const u8,
                &(*(*self.0).pCertInfo).SubjectPublicKeyInfo
                    as *const Cryptography::CERT_PUBLIC_KEY_INFO as _,
                Cryptography::CRYPT_ENCODE_OBJECT_FLAGS::default(),
                ptr::null_mut(),
                ptr::null_mut(),
                &mut len,
            );
            if ok == 0 {
                return Err(io::Error::last_os_error());
            }
            if len > 0 {
                let mut buf = vec![0; len as usize];
                let ok = Cryptography::CryptEncodeObjectEx(
                    Cryptography::X509_ASN_ENCODING,
                    Cryptography::CERT_INFO_SUBJECT_PUBLIC_KEY_INFO_FLAG as *const u32 as *const _,
                    &(*(*self.0).pCertInfo).SubjectPublicKeyInfo
                        as *const Cryptography::CERT_PUBLIC_KEY_INFO as _,
                    Cryptography::CRYPT_ENCODE_OBJECT_FLAGS::default(),
                    ptr::null_mut(),
                    buf.as_mut_ptr() as _,
                    &mut len,
                );
                if ok == 0 {
                    return Err(io::Error::last_os_error());
                }
                return Ok(buf);
            }
        }
        Err(io::Error::last_os_error())
    }

    /// Decodes a PEM-formatted X509 certificate.
    pub fn from_pem(pem: &str) -> io::Result<CertContext> {
        unsafe {
            assert!(pem.len() <= u32::max_value() as usize);

            let mut len = 0;
            let ok = Cryptography::CryptStringToBinaryA(
                pem.as_ptr(),
                pem.len() as u32,
                Cryptography::CRYPT_STRING_BASE64HEADER,
                ptr::null_mut(),
                &mut len,
                ptr::null_mut(),
                ptr::null_mut(),
            );
            if ok == 0 {
                return Err(io::Error::last_os_error());
            }

            let mut buf = vec![0; len as usize];
            let ok = Cryptography::CryptStringToBinaryA(
                pem.as_ptr(),
                pem.len() as u32,
                Cryptography::CRYPT_STRING_BASE64HEADER,
                buf.as_mut_ptr(),
                &mut len,
                ptr::null_mut(),
                ptr::null_mut(),
            );
            if ok == 0 {
                return Err(io::Error::last_os_error());
            }

            CertContext::new(&buf)
        }
    }

    /// Get certificate as PEM-formatted X509 certificate.
    pub fn to_pem(&self) -> io::Result<String> {
        unsafe {
            let mut len = 0;
            let ok = Cryptography::CryptBinaryToStringA(
                (*self.0).pbCertEncoded,
                (*self.0).cbCertEncoded,
                Cryptography::CRYPT_STRING_BASE64HEADER,
                ptr::null_mut(),
                &mut len,
            );
            if ok == 0 {
                return Err(io::Error::last_os_error());
            }

            let mut buf = vec![0; len as usize];
            let ok = Cryptography::CryptBinaryToStringA(
                (*self.0).pbCertEncoded,
                (*self.0).cbCertEncoded,
                Cryptography::CRYPT_STRING_BASE64HEADER,
                buf.as_mut_ptr(),
                &mut len,
            );
            if ok == 0 {
                return Err(io::Error::last_os_error());
            }

            Ok(CStr::from_ptr(buf.as_ptr() as *const _)
                .to_string_lossy()
                .into_owned())
        }
    }

    /// Returns a hash of this certificate
    pub fn fingerprint(&self, alg: HashAlgorithm) -> io::Result<Vec<u8>> {
        unsafe {
            let mut buf = vec![0u8; alg.1];
            let mut len = buf.len() as u32;

            let ret = Cryptography::CryptHashCertificate(
                Cryptography::HCRYPTPROV_LEGACY::default(),
                alg.0,
                0,
                (*self.0).pbCertEncoded,
                (*self.0).cbCertEncoded,
                buf.as_mut_ptr(),
                &mut len,
            );

            if ret == 0 {
                return Err(io::Error::last_os_error());
            }
            Ok(buf)
        }
    }

    /// Returns the sha1 hash of this certificate
    ///
    /// The sha1 is returned as a 20-byte array representing the bits of the
    /// sha1 hash.
    #[deprecated(note = "please use fingerprint instead")]
    pub fn sha1(&self) -> io::Result<[u8; 20]> {
        let mut out = [0u8; 20];
        out.copy_from_slice(&self.fingerprint(HashAlgorithm::sha1())?);
        Ok(out)
    }

    /// Returns the `<SIGNATURE>/<HASH>` string representing the certificate
    /// signature.
    ///
    /// The `<SIGNATURE>` value identifies the CNG public key
    /// algorithm. The `<HASH>` value identifies the CNG hash algorithm.
    ///
    /// Common examples are:
    ///
    /// * `RSA/SHA1`
    /// * `RSA/SHA256`
    /// * `ECDSA/SHA256`
    pub fn sign_hash_algorithms(&self) -> io::Result<String> {
        self.get_string(Cryptography::CERT_SIGN_HASH_CNG_ALG_PROP_ID)
    }

    /// Returns the signature hash.
    pub fn signature_hash(&self) -> io::Result<Vec<u8>> {
        self.get_bytes(Cryptography::CERT_SIGNATURE_HASH_PROP_ID)
    }

    /// Returns the property displayed by the certificate UI. This property
    /// allows the user to describe the certificate's use.
    pub fn description(&self) -> io::Result<Vec<u8>> {
        self.get_bytes(Cryptography::CERT_DESCRIPTION_PROP_ID)
    }

    /// Returns a string that contains the display name for the certificate.
    pub fn friendly_name(&self) -> io::Result<String> {
        self.get_string(Cryptography::CERT_FRIENDLY_NAME_PROP_ID)
    }

    /// Configures the string that contains the display name for this
    /// certificate.
    pub fn set_friendly_name(&self, name: &str) -> io::Result<()> {
        self.set_string(Cryptography::CERT_FRIENDLY_NAME_PROP_ID, name)
    }

    /// Verifies the time validity of this certificate relative to the system's
    /// current time.
    pub fn is_time_valid(&self) -> io::Result<bool> {
        let ret =
            unsafe { Cryptography::CertVerifyTimeValidity(ptr::null_mut(), (*self.0).pCertInfo) };
        Ok(ret == 0)
    }

    /// Returns a builder used to acquire the private key corresponding to this certificate.
    pub fn private_key(&self) -> AcquirePrivateKeyOptions {
        AcquirePrivateKeyOptions {
            cert: self,
            flags: 0,
        }
    }

    /// Deletes this certificate from its certificate store.
    pub fn delete(self) -> io::Result<()> {
        unsafe {
            let ret = Cryptography::CertDeleteCertificateFromStore(self.0);
            mem::forget(self);
            if ret != 0 {
                Ok(())
            } else {
                Err(io::Error::last_os_error())
            }
        }
    }

    /// Returns a builder used to set the private key associated with this certificate.
    pub fn set_key_prov_info(&self) -> SetKeyProvInfo {
        SetKeyProvInfo {
            cert: self,
            container: None,
            provider: None,
            type_: 0,
            flags: 0,
            key_spec: 0,
        }
    }

    /// Returns the valid uses for this certificate
    pub fn valid_uses(&self) -> io::Result<ValidUses> {
        unsafe {
            let mut buf_len = 0;
            let ok =
                Cryptography::CertGetEnhancedKeyUsage(self.0, 0, ptr::null_mut(), &mut buf_len);

            if ok == 0 {
                return Err(io::Error::last_os_error());
            }

            let mut buf = vec![0u8; buf_len as usize];
            let cert_enhkey_usage = buf.as_mut_ptr() as *mut Cryptography::CTL_USAGE;

            let ok =
                Cryptography::CertGetEnhancedKeyUsage(self.0, 0, cert_enhkey_usage, &mut buf_len);
            if ok == 0 {
                return Err(io::Error::last_os_error());
            }

            let use_cnt = (*cert_enhkey_usage).cUsageIdentifier;
            if use_cnt == 0 {
                let last_error = io::Error::last_os_error();
                match last_error.raw_os_error() {
                    Some(Foundation::CRYPT_E_NOT_FOUND) => return Ok(ValidUses::All),
                    Some(Foundation::S_OK) => (),
                    _ => return Err(last_error),
                };
            }

            let mut oids: Vec<String> = Vec::with_capacity(use_cnt as usize);
            for i in 0..use_cnt {
                let oid_ptr = (*cert_enhkey_usage).rgpszUsageIdentifier;
                oids.push(
                    CStr::from_ptr(*(oid_ptr.offset(i as isize)) as *const _)
                        .to_string_lossy()
                        .into_owned(),
                );
            }
            Ok(ValidUses::Oids(oids))
        }
    }

    /// For a remote certificate, returns a certificate store containing any intermediate
    /// certificates provided by the remote sender.
    pub fn cert_store(&self) -> Option<CertStore> {
        unsafe {
            let chain = (*self.0).hCertStore;
            if chain.is_null() {
                None
            } else {
                Some(CertStore::from_inner(Cryptography::CertDuplicateStore(
                    chain,
                )))
            }
        }
    }

    fn get_encoded_bytes(&self) -> &[u8] {
        unsafe {
            let cert_ctx = *self.0;
            slice::from_raw_parts(cert_ctx.pbCertEncoded, cert_ctx.cbCertEncoded as usize)
        }
    }

    fn get_bytes(&self, prop: u32) -> io::Result<Vec<u8>> {
        unsafe {
            let mut len = 0;
            let ret = Cryptography::CertGetCertificateContextProperty(
                self.0,
                prop,
                ptr::null_mut(),
                &mut len,
            );
            if ret == 0 {
                return Err(io::Error::last_os_error());
            }

            let mut buf = vec![0u8; len as usize];
            let ret = Cryptography::CertGetCertificateContextProperty(
                self.0,
                prop,
                buf.as_mut_ptr() as *mut c_void,
                &mut len,
            );
            if ret == 0 {
                return Err(io::Error::last_os_error());
            }
            Ok(buf)
        }
    }

    fn get_string(&self, prop: u32) -> io::Result<String> {
        unsafe {
            let mut len = 0;
            let ret = Cryptography::CertGetCertificateContextProperty(
                self.0,
                prop,
                ptr::null_mut(),
                &mut len,
            );
            if ret == 0 {
                return Err(io::Error::last_os_error());
            }

            // Divide by 2 b/c `len` is the byte length, but we're allocating
            // u16 pairs which are 2 bytes each.
            let amt = (len / 2) as usize;
            let mut buf = vec![0u16; amt];
            let ret = Cryptography::CertGetCertificateContextProperty(
                self.0,
                prop,
                buf.as_mut_ptr() as *mut c_void,
                &mut len,
            );
            if ret == 0 {
                return Err(io::Error::last_os_error());
            }

            // Chop off the trailing nul byte
            Ok(OsString::from_wide(&buf[..amt - 1]).into_string().unwrap())
        }
    }

    fn set_string(&self, prop: u32, s: &str) -> io::Result<()> {
        unsafe {
            let data = s.encode_utf16().chain(Some(0)).collect::<Vec<_>>();
            let data = Cryptography::CRYPT_INTEGER_BLOB {
                cbData: (data.len() * 2) as u32,
                pbData: data.as_ptr() as *mut _,
            };
            let ret = Cryptography::CertSetCertificateContextProperty(
                self.0,
                prop,
                0,
                &data as *const _ as *const _,
            );
            if ret == 0 {
                Err(io::Error::last_os_error())
            } else {
                Ok(())
            }
        }
    }
}

impl PartialEq for CertContext {
    fn eq(&self, other: &CertContext) -> bool {
        self.get_encoded_bytes() == other.get_encoded_bytes()
    }
}

/// A builder type for certificate private key lookup.
pub struct AcquirePrivateKeyOptions<'a> {
    cert: &'a CertContext,
    flags: u32,
}

impl<'a> AcquirePrivateKeyOptions<'a> {
    /// If set, the certificate's public key will be compared with the private key to ensure a
    /// match.
    pub fn compare_key(&mut self, compare_key: bool) -> &mut AcquirePrivateKeyOptions<'a> {
        self.flag(Cryptography::CRYPT_ACQUIRE_COMPARE_KEY_FLAG, compare_key)
    }

    /// If set, the lookup will not display any user interface, even if that causes the lookup to
    /// fail.
    pub fn silent(&mut self, silent: bool) -> &mut AcquirePrivateKeyOptions<'a> {
        self.flag(Cryptography::CRYPT_ACQUIRE_SILENT_FLAG, silent)
    }

    fn flag(&mut self, flag: u32, set: bool) -> &mut AcquirePrivateKeyOptions<'a> {
        if set {
            self.flags |= flag;
        } else {
            self.flags &= !flag;
        }
        self
    }

    /// Acquires the private key handle.
    pub fn acquire(&self) -> io::Result<PrivateKey> {
        unsafe {
            let flags = self.flags | Cryptography::CRYPT_ACQUIRE_ALLOW_NCRYPT_KEY_FLAG;
            let mut handle = Cryptography::HCRYPTPROV_OR_NCRYPT_KEY_HANDLE::default();
            let mut spec = Cryptography::CERT_KEY_SPEC::default();
            let mut free = windows_sys::core::BOOL::default();
            let res = Cryptography::CryptAcquireCertificatePrivateKey(
                self.cert.0,
                flags,
                ptr::null_mut(),
                &mut handle,
                &mut spec,
                &mut free,
            );
            if res == 0 {
                return Err(io::Error::last_os_error());
            }
            assert_ne!(free, 0);
            if spec & Cryptography::CERT_NCRYPT_KEY_SPEC != 0 {
                Ok(PrivateKey::NcryptKey(NcryptKey::from_inner(handle)))
            } else {
                Ok(PrivateKey::CryptProv(CryptProv::from_inner(handle)))
            }
        }
    }
}

/// The private key associated with a certificate context.
pub enum PrivateKey {
    /// A CryptoAPI provider.
    CryptProv(CryptProv),
    /// A CNG provider.
    NcryptKey(NcryptKey),
}

/// A builder used to set the private key associated with a certificate.
pub struct SetKeyProvInfo<'a> {
    cert: &'a CertContext,
    container: Option<Vec<u16>>,
    provider: Option<Vec<u16>>,
    type_: u32,
    flags: u32,
    key_spec: u32,
}

impl<'a> SetKeyProvInfo<'a> {
    /// The name of the key container.
    ///
    /// If `type_` is not provided, this specifies the name of the key withing
    /// the CNG key storage provider.
    pub fn container(&mut self, container: &str) -> &mut SetKeyProvInfo<'a> {
        self.container = Some(container.encode_utf16().chain(Some(0)).collect());
        self
    }

    /// The name of the CSP.
    ///
    /// If `type_` is not provided, this contains the name of the CNG key
    /// storage provider.
    pub fn provider(&mut self, provider: &str) -> &mut SetKeyProvInfo<'a> {
        self.provider = Some(provider.encode_utf16().chain(Some(0)).collect());
        self
    }

    /// Sets the CSP type.
    ///
    /// If not provided, the key container is one of the CNG key storage
    /// providers.
    pub fn type_(&mut self, type_: ProviderType) -> &mut SetKeyProvInfo<'a> {
        self.type_ = type_.as_raw();
        self
    }

    /// If set, the handle to the key provider can be kept open for subsequent
    /// calls to cryptographic functions.
    pub fn keep_open(&mut self, keep_open: bool) -> &mut SetKeyProvInfo<'a> {
        self.flag(Cryptography::CERT_SET_KEY_PROV_HANDLE_PROP_ID, keep_open)
    }

    /// If set, the key container contains machine keys.
    pub fn machine_keyset(&mut self, machine_keyset: bool) -> &mut SetKeyProvInfo<'a> {
        self.flag(Cryptography::CRYPT_MACHINE_KEYSET, machine_keyset)
    }

    /// If set, the key container will attempt to open keys without any user
    /// interface prompts.
    pub fn silent(&mut self, silent: bool) -> &mut SetKeyProvInfo<'a> {
        self.flag(Cryptography::CRYPT_SILENT, silent)
    }

    fn flag(&mut self, flag: u32, on: bool) -> &mut SetKeyProvInfo<'a> {
        if on {
            self.flags |= flag;
        } else {
            self.flags &= !flag;
        }
        self
    }

    /// The specification of the private key to retrieve.
    pub fn key_spec(&mut self, key_spec: KeySpec) -> &mut SetKeyProvInfo<'a> {
        self.key_spec = key_spec.0;
        self
    }

    /// Sets the private key for this certificate.
    pub fn set(&mut self) -> io::Result<()> {
        unsafe {
            let container = self
                .container
                .as_ref()
                .map(|s| s.as_ptr())
                .unwrap_or(ptr::null());
            let provider = self
                .provider
                .as_ref()
                .map(|s| s.as_ptr())
                .unwrap_or(ptr::null());

            let info = Cryptography::CRYPT_KEY_PROV_INFO {
                pwszContainerName: container as *mut _,
                pwszProvName: provider as *mut _,
                dwProvType: self.type_,
                dwFlags: self.flags,
                cProvParam: 0,
                rgProvParam: ptr::null_mut(),
                dwKeySpec: self.key_spec,
            };

            let res = Cryptography::CertSetCertificateContextProperty(
                self.cert.0,
                Cryptography::CERT_KEY_PROV_INFO_PROP_ID,
                0,
                &info as *const _ as *const _,
            );
            if res != 0 {
                Ok(())
            } else {
                Err(io::Error::last_os_error())
            }
        }
    }
}

/// The specification of a private key.
#[derive(Copy, Clone)]
pub struct KeySpec(u32);

impl KeySpec {
    /// A key used to encrypt/decrypt session keys.
    pub fn key_exchange() -> KeySpec {
        KeySpec(Cryptography::AT_KEYEXCHANGE)
    }

    /// A key used to create and verify digital signatures.
    pub fn signature() -> KeySpec {
        KeySpec(Cryptography::AT_SIGNATURE)
    }
}

/// Valid uses of a Certificate - All, or specific OIDs
pub enum ValidUses {
    /// Certificate is valid for all uses
    All,

    /// Certificate is valid for uses specified. No entries means that the certificate
    /// has no valid uses.
    Oids(Vec<String>),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decode() {
        let der = include_bytes!("../test/cert.der");
        let pem = include_str!("../test/cert.pem");

        let der = CertContext::new(der).unwrap();
        let pem = CertContext::from_pem(pem).unwrap();
        assert_eq!(der, pem);
    }

    #[test]
    fn certcontext_to_der() {
        let der = include_bytes!("../test/cert.der");
        let cert = CertContext::new(der).unwrap();
        let der2 = CertContext::to_der(&cert);
        assert_eq!(der as &[u8], der2);
    }

    #[test]
    fn certcontext_to_pem() {
        let der = include_bytes!("../test/cert.der");
        let pem1 = include_str!("../test/cert.pem").replace('\r', "");

        let der = CertContext::new(der).unwrap();
        let pem2 = CertContext::to_pem(&der).unwrap().replace('\r', "");
        assert_eq!(pem1, pem2);
    }

    #[test]
    fn fingerprint() {
        let der = include_bytes!("../test/cert.der");
        let pem = include_str!("../test/cert.pem");

        let der = CertContext::new(der).unwrap();
        let pem = CertContext::from_pem(pem).unwrap();

        let hash = der.fingerprint(HashAlgorithm::sha1()).unwrap();
        assert_eq!(
            hash,
            vec![
                0x59, 0x17, 0x2D, 0x93, 0x13, 0xE8, 0x44, 0x59, 0xBC, 0xFF, 0x27, 0xF9, 0x67, 0xE7,
                0x9E, 0x6E, 0x92, 0x17, 0xE5, 0x84
            ]
        );
        assert_eq!(hash, pem.fingerprint(HashAlgorithm::sha1()).unwrap());

        let hash = der.fingerprint(HashAlgorithm::sha256()).unwrap();
        assert_eq!(
            hash,
            vec![
                0x47, 0x12, 0xB9, 0x39, 0xFB, 0xCB, 0x42, 0xA6, 0xB5, 0x10, 0x1B, 0x42, 0x13, 0x9A,
                0x25, 0xB1, 0x4F, 0x81, 0xB4, 0x18, 0xFA, 0xCA, 0xBD, 0x37, 0x87, 0x46, 0xF1, 0x2F,
                0x85, 0xCC, 0x65, 0x44
            ]
        );
        assert_eq!(hash, pem.fingerprint(HashAlgorithm::sha256()).unwrap());
    }
}

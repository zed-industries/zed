//! Public/private key processing.
//!
//! Asymmetric public key algorithms solve the problem of establishing and sharing
//! secret keys to securely send and receive messages.
//! This system uses a pair of keys: a public key, which can be freely
//! distributed, and a private key, which is kept to oneself. An entity may
//! encrypt information using a user's public key. The encrypted information can
//! only be deciphered using that user's private key.
//!
//! This module offers support for five popular algorithms:
//!
//! * RSA
//!
//! * DSA
//!
//! * Diffie-Hellman
//!
//! * Elliptic Curves
//!
//! * HMAC
//!
//! These algorithms rely on hard mathematical problems - namely integer factorization,
//! discrete logarithms, and elliptic curve relationships - that currently do not
//! yield efficient solutions. This property ensures the security of these
//! cryptographic algorithms.
//!
//! # Example
//!
//! Generate a 2048-bit RSA public/private key pair and print the public key.
//!
//! ```rust
//! use openssl::rsa::Rsa;
//! use openssl::pkey::PKey;
//! use std::str;
//!
//! let rsa = Rsa::generate(2048).unwrap();
//! let pkey = PKey::from_rsa(rsa).unwrap();
//!
//! let pub_key: Vec<u8> = pkey.public_key_to_pem().unwrap();
//! println!("{:?}", str::from_utf8(pub_key.as_slice()).unwrap());
//! ```
#![allow(clippy::missing_safety_doc)]
use crate::bio::{MemBio, MemBioSlice};
#[cfg(ossl110)]
use crate::cipher::CipherRef;
use crate::dh::Dh;
use crate::dsa::Dsa;
use crate::ec::EcKey;
use crate::error::ErrorStack;
#[cfg(ossl300)]
use crate::lib_ctx::LibCtxRef;
#[cfg(any(ossl110, boringssl, libressl370, awslc))]
use crate::pkey_ctx::PkeyCtx;
use crate::rsa::Rsa;
use crate::symm::Cipher;
use crate::util::{invoke_passwd_cb, CallbackState};
use crate::{cvt, cvt_p};
use foreign_types::{ForeignType, ForeignTypeRef};
use libc::{c_int, c_long};
use openssl_macros::corresponds;
use std::convert::{TryFrom, TryInto};
use std::ffi::{CStr, CString};
use std::fmt;
#[cfg(all(not(any(boringssl, awslc)), ossl110))]
use std::mem;
use std::ptr;

/// A tag type indicating that a key only has parameters.
pub enum Params {}

/// A tag type indicating that a key only has public components.
pub enum Public {}

/// A tag type indicating that a key has private components.
pub enum Private {}

/// An identifier of a kind of key.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Id(c_int);

impl Id {
    pub const RSA: Id = Id(ffi::EVP_PKEY_RSA);
    #[cfg(any(ossl111, libressl, boringssl, awslc))]
    pub const RSA_PSS: Id = Id(ffi::EVP_PKEY_RSA_PSS);
    #[cfg(not(boringssl))]
    pub const HMAC: Id = Id(ffi::EVP_PKEY_HMAC);
    #[cfg(not(any(boringssl, awslc)))]
    pub const CMAC: Id = Id(ffi::EVP_PKEY_CMAC);
    pub const DSA: Id = Id(ffi::EVP_PKEY_DSA);
    pub const DH: Id = Id(ffi::EVP_PKEY_DH);
    #[cfg(ossl110)]
    pub const DHX: Id = Id(ffi::EVP_PKEY_DHX);
    pub const EC: Id = Id(ffi::EVP_PKEY_EC);
    #[cfg(ossl111)]
    pub const SM2: Id = Id(ffi::EVP_PKEY_SM2);

    #[cfg(any(ossl110, boringssl, libressl360, awslc))]
    pub const HKDF: Id = Id(ffi::EVP_PKEY_HKDF);

    #[cfg(any(ossl111, boringssl, libressl370, awslc))]
    pub const ED25519: Id = Id(ffi::EVP_PKEY_ED25519);
    #[cfg(ossl111)]
    pub const ED448: Id = Id(ffi::EVP_PKEY_ED448);
    #[cfg(any(ossl111, boringssl, libressl370, awslc))]
    pub const X25519: Id = Id(ffi::EVP_PKEY_X25519);
    #[cfg(ossl111)]
    pub const X448: Id = Id(ffi::EVP_PKEY_X448);
    #[cfg(ossl111)]
    pub const POLY1305: Id = Id(ffi::EVP_PKEY_POLY1305);

    /// Creates a `Id` from an integer representation.
    pub const fn from_raw(value: c_int) -> Id {
        Id(value)
    }

    /// Returns the integer representation of the `Id`.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn as_raw(&self) -> c_int {
        self.0
    }
}

/// The name of a key algorithm, used with [`PKeyRef::is_a`].
///
/// In OpenSSL 3.0+, provider-supplied keys do not have a meaningful numeric
/// id (`EVP_PKEY_id` returns `-1`), so identifying them requires a name-based
/// check via `EVP_PKEY_is_a`. `KeyType` wraps the algorithm name as a static
/// C string and exposes constants for common algorithms.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct KeyType(&'static CStr);

impl KeyType {
    pub const RSA: KeyType = KeyType(c"RSA");
    pub const RSA_PSS: KeyType = KeyType(c"RSA-PSS");
    pub const DSA: KeyType = KeyType(c"DSA");
    pub const DH: KeyType = KeyType(c"DH");
    pub const EC: KeyType = KeyType(c"EC");
    pub const ED25519: KeyType = KeyType(c"ED25519");
    pub const ED448: KeyType = KeyType(c"ED448");
    pub const X25519: KeyType = KeyType(c"X25519");
    pub const X448: KeyType = KeyType(c"X448");
    pub const HMAC: KeyType = KeyType(c"HMAC");
    pub const CMAC: KeyType = KeyType(c"CMAC");
    pub const ML_DSA_44: KeyType = KeyType(c"ML-DSA-44");
    pub const ML_DSA_65: KeyType = KeyType(c"ML-DSA-65");
    pub const ML_DSA_87: KeyType = KeyType(c"ML-DSA-87");
    pub const ML_KEM_512: KeyType = KeyType(c"ML-KEM-512");
    pub const ML_KEM_768: KeyType = KeyType(c"ML-KEM-768");
    pub const ML_KEM_1024: KeyType = KeyType(c"ML-KEM-1024");

    /// Returns the algorithm name as a C string.
    #[cfg(ossl300)]
    pub(crate) fn as_cstr(&self) -> &'static CStr {
        self.0
    }
}

/// A trait indicating that a key has parameters.
pub unsafe trait HasParams {}

unsafe impl HasParams for Params {}

unsafe impl<T> HasParams for T where T: HasPublic {}

/// A trait indicating that a key has public components.
pub unsafe trait HasPublic {}

unsafe impl HasPublic for Public {}

unsafe impl<T> HasPublic for T where T: HasPrivate {}

/// A trait indicating that a key has private components.
pub unsafe trait HasPrivate {}

unsafe impl HasPrivate for Private {}

generic_foreign_type_and_impl_send_sync! {
    type CType = ffi::EVP_PKEY;
    fn drop = ffi::EVP_PKEY_free;

    /// A public or private key.
    pub struct PKey<T>;
    /// Reference to `PKey`.
    pub struct PKeyRef<T>;
}

impl<T> ToOwned for PKeyRef<T> {
    type Owned = PKey<T>;

    fn to_owned(&self) -> PKey<T> {
        unsafe {
            EVP_PKEY_up_ref(self.as_ptr());
            PKey::from_ptr(self.as_ptr())
        }
    }
}

impl<T> PKeyRef<T> {
    /// Returns a copy of the internal RSA key.
    #[corresponds(EVP_PKEY_get1_RSA)]
    pub fn rsa(&self) -> Result<Rsa<T>, ErrorStack> {
        unsafe {
            let rsa = cvt_p(ffi::EVP_PKEY_get1_RSA(self.as_ptr()))?;
            Ok(Rsa::from_ptr(rsa))
        }
    }

    /// Returns a copy of the internal DSA key.
    #[corresponds(EVP_PKEY_get1_DSA)]
    pub fn dsa(&self) -> Result<Dsa<T>, ErrorStack> {
        unsafe {
            let dsa = cvt_p(ffi::EVP_PKEY_get1_DSA(self.as_ptr()))?;
            Ok(Dsa::from_ptr(dsa))
        }
    }

    /// Returns a copy of the internal DH key.
    #[corresponds(EVP_PKEY_get1_DH)]
    pub fn dh(&self) -> Result<Dh<T>, ErrorStack> {
        unsafe {
            let dh = cvt_p(ffi::EVP_PKEY_get1_DH(self.as_ptr()))?;
            Ok(Dh::from_ptr(dh))
        }
    }

    /// Returns a copy of the internal elliptic curve key.
    #[corresponds(EVP_PKEY_get1_EC_KEY)]
    pub fn ec_key(&self) -> Result<EcKey<T>, ErrorStack> {
        unsafe {
            let ec_key = cvt_p(ffi::EVP_PKEY_get1_EC_KEY(self.as_ptr()))?;
            Ok(EcKey::from_ptr(ec_key))
        }
    }

    /// Returns the `Id` that represents the type of this key.
    ///
    /// In OpenSSL 3.0+, provider-supplied keys (such as ML-DSA or any key
    /// from a third-party provider) return `-1` from `EVP_PKEY_id`. Use
    /// [`PKeyRef::is_a`] for a name-based check that works for those keys.
    #[corresponds(EVP_PKEY_id)]
    pub fn id(&self) -> Id {
        unsafe { Id::from_raw(ffi::EVP_PKEY_id(self.as_ptr())) }
    }

    /// Returns `true` if this key is an instance of the algorithm named
    /// by `key_type`.
    ///
    /// This is the OpenSSL 3.0+ name-based equivalent of [`PKeyRef::id`].
    /// It is the only reliable way to identify provider-supplied keys,
    /// which return `-1` from `EVP_PKEY_id`.
    #[corresponds(EVP_PKEY_is_a)]
    #[cfg(ossl300)]
    pub fn is_a(&self, key_type: KeyType) -> bool {
        unsafe { ffi::EVP_PKEY_is_a(self.as_ptr(), key_type.as_cstr().as_ptr()) == 1 }
    }

    /// Returns the maximum size of a signature in bytes.
    #[corresponds(EVP_PKEY_size)]
    pub fn size(&self) -> usize {
        unsafe { ffi::EVP_PKEY_size(self.as_ptr()) as usize }
    }
}

impl<T> PKeyRef<T>
where
    T: HasPublic,
{
    to_pem! {
        /// Serializes the public key into a PEM-encoded SubjectPublicKeyInfo structure.
        ///
        /// The output will have a header of `-----BEGIN PUBLIC KEY-----`.
        #[corresponds(PEM_write_bio_PUBKEY)]
        public_key_to_pem,
        ffi::PEM_write_bio_PUBKEY
    }

    to_der! {
        /// Serializes the public key into a DER-encoded SubjectPublicKeyInfo structure.
        #[corresponds(i2d_PUBKEY)]
        public_key_to_der,
        ffi::i2d_PUBKEY
    }

    /// Returns the size of the key.
    ///
    /// This corresponds to the bit length of the modulus of an RSA key, and the bit length of the
    /// group order for an elliptic curve key, for example.
    #[corresponds(EVP_PKEY_bits)]
    pub fn bits(&self) -> u32 {
        unsafe { ffi::EVP_PKEY_bits(self.as_ptr()) as u32 }
    }

    ///Returns the number of security bits.
    ///
    ///Bits of security is defined in NIST SP800-57.
    #[corresponds(EVP_PKEY_security_bits)]
    #[cfg(any(ossl110, libressl360))]
    pub fn security_bits(&self) -> u32 {
        unsafe { ffi::EVP_PKEY_security_bits(self.as_ptr()) as u32 }
    }

    /// Compares the public component of this key with another.
    #[corresponds(EVP_PKEY_cmp)]
    pub fn public_eq<U>(&self, other: &PKeyRef<U>) -> bool
    where
        U: HasPublic,
    {
        let res = unsafe { ffi::EVP_PKEY_cmp(self.as_ptr(), other.as_ptr()) == 1 };
        // Clear the stack. OpenSSL will put an error on the stack when the
        // keys are different types in some situations.
        let _ = ErrorStack::get();
        res
    }

    /// Raw byte representation of a public key.
    ///
    /// This function only works for algorithms that support raw public keys.
    /// Currently this is: [`Id::X25519`], [`Id::ED25519`], [`Id::X448`] or [`Id::ED448`].
    #[corresponds(EVP_PKEY_get_raw_public_key)]
    #[cfg(any(ossl111, boringssl, libressl370, awslc))]
    pub fn raw_public_key(&self) -> Result<Vec<u8>, ErrorStack> {
        unsafe {
            let mut len = 0;
            cvt(ffi::EVP_PKEY_get_raw_public_key(
                self.as_ptr(),
                ptr::null_mut(),
                &mut len,
            ))?;
            let mut buf = vec![0u8; len];
            cvt(ffi::EVP_PKEY_get_raw_public_key(
                self.as_ptr(),
                buf.as_mut_ptr(),
                &mut len,
            ))?;
            buf.truncate(len);
            Ok(buf)
        }
    }
}

impl<T> PKeyRef<T>
where
    T: HasPrivate,
{
    private_key_to_pem! {
        /// Serializes the private key to a PEM-encoded PKCS#8 PrivateKeyInfo structure.
        ///
        /// The output will have a header of `-----BEGIN PRIVATE KEY-----`.
        #[corresponds(PEM_write_bio_PKCS8PrivateKey)]
        private_key_to_pem_pkcs8,
        /// Serializes the private key to a PEM-encoded PKCS#8 EncryptedPrivateKeyInfo structure.
        ///
        /// The output will have a header of `-----BEGIN ENCRYPTED PRIVATE KEY-----`.
        #[corresponds(PEM_write_bio_PKCS8PrivateKey)]
        private_key_to_pem_pkcs8_passphrase,
        ffi::PEM_write_bio_PKCS8PrivateKey
    }

    to_der! {
        /// Serializes the private key to a DER-encoded key type specific format.
        #[corresponds(i2d_PrivateKey)]
        private_key_to_der,
        ffi::i2d_PrivateKey
    }

    /// Raw byte representation of a private key.
    ///
    /// This function only works for algorithms that support raw private keys.
    /// Currently this is: [`Id::HMAC`], [`Id::X25519`], [`Id::ED25519`], [`Id::X448`] or [`Id::ED448`].
    #[corresponds(EVP_PKEY_get_raw_private_key)]
    #[cfg(any(ossl111, boringssl, libressl370, awslc))]
    pub fn raw_private_key(&self) -> Result<Vec<u8>, ErrorStack> {
        unsafe {
            let mut len = 0;
            cvt(ffi::EVP_PKEY_get_raw_private_key(
                self.as_ptr(),
                ptr::null_mut(),
                &mut len,
            ))?;
            let mut buf = vec![0u8; len];
            cvt(ffi::EVP_PKEY_get_raw_private_key(
                self.as_ptr(),
                buf.as_mut_ptr(),
                &mut len,
            ))?;
            buf.truncate(len);
            Ok(buf)
        }
    }

    /// Writes the algorithm-defined seed for an ML-DSA or ML-KEM private key
    /// into `buf`, returning the number of bytes written.
    ///
    /// `buf` must be at least 32 bytes long for ML-DSA and 64 bytes long for
    /// ML-KEM. The inverse of [`PKey::private_key_from_seed`].
    ///
    /// Errors when called on a key whose algorithm has no `"seed"`
    /// `OSSL_PARAM` (e.g. RSA, EC, Ed25519), when `buf` is too small for
    /// the algorithm's seed, or when the underlying key was imported from
    /// an encoded form that retains only the expanded private key with no
    /// seed.
    #[corresponds(EVP_PKEY_get_params)]
    #[cfg(ossl350)]
    pub fn seed_into(&self, buf: &mut [u8]) -> Result<usize, ErrorStack> {
        unsafe {
            let mut params = [
                ffi::OSSL_PARAM_construct_octet_string(
                    c"seed".as_ptr(),
                    buf.as_mut_ptr() as *mut libc::c_void,
                    buf.len(),
                ),
                ffi::OSSL_PARAM_construct_end(),
            ];
            cvt(ffi::EVP_PKEY_get_params(self.as_ptr(), params.as_mut_ptr()))?;
            // OpenSSL silently ignores OSSL_PARAMs the keymgmt doesn't
            // recognise and returns success. Detect that case via
            // OSSL_PARAM_modified before trusting return_size.
            if ffi::OSSL_PARAM_modified(&params[0]) == 0 {
                return Err(ErrorStack::get());
            }
            Ok(params[0].return_size)
        }
    }

    /// Serializes a private key into an unencrypted DER-formatted PKCS#8
    #[corresponds(i2d_PKCS8PrivateKey_bio)]
    pub fn private_key_to_pkcs8(&self) -> Result<Vec<u8>, ErrorStack> {
        unsafe {
            let bio = MemBio::new()?;
            cvt(ffi::i2d_PKCS8PrivateKey_bio(
                bio.as_ptr(),
                self.as_ptr(),
                ptr::null(),
                ptr::null_mut(),
                0,
                None,
                ptr::null_mut(),
            ))?;

            Ok(bio.get_buf().to_owned())
        }
    }

    /// Serializes a private key into a DER-formatted PKCS#8, using the supplied password to
    /// encrypt the key.
    #[corresponds(i2d_PKCS8PrivateKey_bio)]
    pub fn private_key_to_pkcs8_passphrase(
        &self,
        cipher: Cipher,
        passphrase: &[u8],
    ) -> Result<Vec<u8>, ErrorStack> {
        unsafe {
            let bio = MemBio::new()?;
            cvt(ffi::i2d_PKCS8PrivateKey_bio(
                bio.as_ptr(),
                self.as_ptr(),
                cipher.as_ptr(),
                passphrase.as_ptr() as *const _ as *mut _,
                passphrase.len().try_into().unwrap(),
                None,
                ptr::null_mut(),
            ))?;

            Ok(bio.get_buf().to_owned())
        }
    }
}

impl<T> fmt::Debug for PKey<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let alg = match self.id() {
            Id::RSA => "RSA",
            #[cfg(any(ossl111, libressl, boringssl, awslc))]
            Id::RSA_PSS => "RSA-PSS",
            #[cfg(not(boringssl))]
            Id::HMAC => "HMAC",
            #[cfg(not(any(boringssl, awslc)))]
            Id::CMAC => "CMAC",
            Id::DSA => "DSA",
            Id::DH => "DH",
            #[cfg(ossl110)]
            Id::DHX => "DHX",
            Id::EC => "EC",
            #[cfg(ossl111)]
            Id::SM2 => "SM2",
            #[cfg(any(ossl110, boringssl, libressl360, awslc))]
            Id::HKDF => "HKDF",
            #[cfg(any(ossl111, boringssl, libressl370, awslc))]
            Id::ED25519 => "Ed25519",
            #[cfg(ossl111)]
            Id::ED448 => "Ed448",
            #[cfg(any(ossl111, boringssl, libressl370, awslc))]
            Id::X25519 => "X25519",
            #[cfg(ossl111)]
            Id::X448 => "X448",
            #[cfg(ossl111)]
            Id::POLY1305 => "POLY1305",
            _ => "unknown",
        };
        fmt.debug_struct("PKey").field("algorithm", &alg).finish()
        // TODO: Print details for each specific type of key
    }
}

impl<T> Clone for PKey<T> {
    fn clone(&self) -> PKey<T> {
        PKeyRef::to_owned(self)
    }
}

impl<T> PKey<T> {
    /// Creates a new `PKey` containing an RSA key.
    #[corresponds(EVP_PKEY_set1_RSA)]
    pub fn from_rsa(rsa: Rsa<T>) -> Result<PKey<T>, ErrorStack> {
        // TODO: Next time we make backwards incompatible changes, this could
        // become an `&RsaRef<T>`. Same for all the other `from_*` methods.
        unsafe {
            let evp = cvt_p(ffi::EVP_PKEY_new())?;
            let pkey = PKey::from_ptr(evp);
            cvt(ffi::EVP_PKEY_set1_RSA(pkey.0, rsa.as_ptr()))?;
            Ok(pkey)
        }
    }

    /// Creates a new `PKey` containing a DSA key.
    #[corresponds(EVP_PKEY_set1_DSA)]
    pub fn from_dsa(dsa: Dsa<T>) -> Result<PKey<T>, ErrorStack> {
        unsafe {
            let evp = cvt_p(ffi::EVP_PKEY_new())?;
            let pkey = PKey::from_ptr(evp);
            cvt(ffi::EVP_PKEY_set1_DSA(pkey.0, dsa.as_ptr()))?;
            Ok(pkey)
        }
    }

    /// Creates a new `PKey` containing a Diffie-Hellman key.
    #[corresponds(EVP_PKEY_set1_DH)]
    #[cfg(not(boringssl))]
    pub fn from_dh(dh: Dh<T>) -> Result<PKey<T>, ErrorStack> {
        unsafe {
            let evp = cvt_p(ffi::EVP_PKEY_new())?;
            let pkey = PKey::from_ptr(evp);
            cvt(ffi::EVP_PKEY_set1_DH(pkey.0, dh.as_ptr()))?;
            Ok(pkey)
        }
    }

    /// Creates a new `PKey` containing a Diffie-Hellman key with type DHX.
    #[cfg(all(not(any(boringssl, awslc)), ossl110))]
    pub fn from_dhx(dh: Dh<T>) -> Result<PKey<T>, ErrorStack> {
        unsafe {
            let evp = cvt_p(ffi::EVP_PKEY_new())?;
            let pkey = PKey::from_ptr(evp);
            cvt(ffi::EVP_PKEY_assign(
                pkey.0,
                ffi::EVP_PKEY_DHX,
                dh.as_ptr().cast(),
            ))?;
            mem::forget(dh);
            Ok(pkey)
        }
    }

    /// Creates a new `PKey` containing an elliptic curve key.
    #[corresponds(EVP_PKEY_set1_EC_KEY)]
    pub fn from_ec_key(ec_key: EcKey<T>) -> Result<PKey<T>, ErrorStack> {
        unsafe {
            let evp = cvt_p(ffi::EVP_PKEY_new())?;
            let pkey = PKey::from_ptr(evp);
            cvt(ffi::EVP_PKEY_set1_EC_KEY(pkey.0, ec_key.as_ptr()))?;
            Ok(pkey)
        }
    }
}

impl PKey<Private> {
    /// Creates a new `PKey` containing an HMAC key.
    ///
    /// # Note
    ///
    /// To compute HMAC values, use the `sign` module.
    #[corresponds(EVP_PKEY_new_mac_key)]
    #[cfg(not(boringssl))]
    pub fn hmac(key: &[u8]) -> Result<PKey<Private>, ErrorStack> {
        #[cfg(awslc)]
        let key_len = key.len();
        #[cfg(not(awslc))]
        let key_len = key.len() as c_int;
        unsafe {
            assert!(key.len() <= c_int::MAX as usize);
            let key = cvt_p(ffi::EVP_PKEY_new_mac_key(
                ffi::EVP_PKEY_HMAC,
                ptr::null_mut(),
                key.as_ptr() as *const _,
                key_len,
            ))?;
            Ok(PKey::from_ptr(key))
        }
    }

    /// Creates a new `PKey` containing a CMAC key.
    ///
    /// Requires OpenSSL 1.1.0 or newer.
    ///
    /// # Note
    ///
    /// To compute CMAC values, use the `sign` module.
    #[cfg(all(not(any(boringssl, awslc)), ossl110))]
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn cmac(cipher: &Cipher, key: &[u8]) -> Result<PKey<Private>, ErrorStack> {
        let mut ctx = PkeyCtx::new_id(Id::CMAC)?;
        ctx.keygen_init()?;
        ctx.set_keygen_cipher(unsafe { CipherRef::from_ptr(cipher.as_ptr() as *mut _) })?;
        ctx.set_keygen_mac_key(key)?;
        ctx.keygen()
    }

    #[cfg(any(ossl111, boringssl, libressl370, awslc))]
    fn generate_eddsa(id: Id) -> Result<PKey<Private>, ErrorStack> {
        let mut ctx = PkeyCtx::new_id(id)?;
        ctx.keygen_init()?;
        ctx.keygen()
    }

    /// Generates a new private X25519 key.
    ///
    /// To import a private key from raw bytes see [`PKey::private_key_from_raw_bytes`].
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use openssl::pkey::{PKey, Id};
    /// use openssl::derive::Deriver;
    ///
    /// let public = // ...
    /// # &PKey::generate_x25519()?.raw_public_key()?;
    /// let public_key = PKey::public_key_from_raw_bytes(public, Id::X25519)?;
    ///
    /// let key = PKey::generate_x25519()?;
    /// let mut deriver = Deriver::new(&key)?;
    /// deriver.set_peer(&public_key)?;
    ///
    /// let secret = deriver.derive_to_vec()?;
    /// assert_eq!(secret.len(), 32);
    /// # Ok(()) }
    /// ```
    #[cfg(any(ossl111, boringssl, libressl370, awslc))]
    pub fn generate_x25519() -> Result<PKey<Private>, ErrorStack> {
        PKey::generate_eddsa(Id::X25519)
    }

    /// Generates a new private X448 key.
    ///
    /// To import a private key from raw bytes see [`PKey::private_key_from_raw_bytes`].
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use openssl::pkey::{PKey, Id};
    /// use openssl::derive::Deriver;
    ///
    /// let public = // ...
    /// # &PKey::generate_x448()?.raw_public_key()?;
    /// let public_key = PKey::public_key_from_raw_bytes(public, Id::X448)?;
    ///
    /// let key = PKey::generate_x448()?;
    /// let mut deriver = Deriver::new(&key)?;
    /// deriver.set_peer(&public_key)?;
    ///
    /// let secret = deriver.derive_to_vec()?;
    /// assert_eq!(secret.len(), 56);
    /// # Ok(()) }
    /// ```
    #[cfg(ossl111)]
    pub fn generate_x448() -> Result<PKey<Private>, ErrorStack> {
        PKey::generate_eddsa(Id::X448)
    }

    /// Generates a new private Ed25519 key.
    ///
    /// To import a private key from raw bytes see [`PKey::private_key_from_raw_bytes`].
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use openssl::pkey::{PKey, Id};
    /// use openssl::sign::Signer;
    ///
    /// let key = PKey::generate_ed25519()?;
    /// let public_key = key.raw_public_key()?;
    ///
    /// let mut signer = Signer::new_without_digest(&key)?;
    /// let digest = // ...
    /// # &vec![0; 32];
    /// let signature = signer.sign_oneshot_to_vec(digest)?;
    /// assert_eq!(signature.len(), 64);
    /// # Ok(()) }
    /// ```
    #[cfg(any(ossl111, boringssl, libressl370, awslc))]
    pub fn generate_ed25519() -> Result<PKey<Private>, ErrorStack> {
        PKey::generate_eddsa(Id::ED25519)
    }

    /// Generates a new private Ed448 key.
    ///
    /// To import a private key from raw bytes see [`PKey::private_key_from_raw_bytes`].
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use openssl::pkey::{PKey, Id};
    /// use openssl::sign::Signer;
    ///
    /// let key = PKey::generate_ed448()?;
    /// let public_key = key.raw_public_key()?;
    ///
    /// let mut signer = Signer::new_without_digest(&key)?;
    /// let digest = // ...
    /// # &vec![0; 32];
    /// let signature = signer.sign_oneshot_to_vec(digest)?;
    /// assert_eq!(signature.len(), 114);
    /// # Ok(()) }
    /// ```
    #[cfg(ossl111)]
    pub fn generate_ed448() -> Result<PKey<Private>, ErrorStack> {
        PKey::generate_eddsa(Id::ED448)
    }

    /// Generates a new EC key using the provided curve.
    ///
    /// Requires OpenSSL 3.0.0 or newer.
    #[corresponds(EVP_EC_gen)]
    #[cfg(ossl300)]
    pub fn ec_gen(curve: &str) -> Result<PKey<Private>, ErrorStack> {
        ffi::init();

        let curve = CString::new(curve).unwrap();
        unsafe {
            let ptr = cvt_p(ffi::EVP_EC_gen(curve.as_ptr()))?;
            Ok(PKey::from_ptr(ptr))
        }
    }

    private_key_from_pem! {
        /// Deserializes a private key from a PEM-encoded key type specific format.
        #[corresponds(PEM_read_bio_PrivateKey)]
        private_key_from_pem,

        /// Deserializes a private key from a PEM-encoded encrypted key type specific format.
        #[corresponds(PEM_read_bio_PrivateKey)]
        private_key_from_pem_passphrase,

        /// Deserializes a private key from a PEM-encoded encrypted key type specific format.
        ///
        /// The callback should fill the password into the provided buffer and return its length.
        #[corresponds(PEM_read_bio_PrivateKey)]
        private_key_from_pem_callback,
        PKey<Private>,
        ffi::PEM_read_bio_PrivateKey
    }

    from_der! {
        /// Decodes a DER-encoded private key.
        ///
        /// This function will attempt to automatically detect the underlying key format, and
        /// supports the unencrypted PKCS#8 PrivateKeyInfo structures as well as key type specific
        /// formats.
        #[corresponds(d2i_AutoPrivateKey)]
        private_key_from_der,
        PKey<Private>,
        ffi::d2i_AutoPrivateKey
    }

    /// Deserializes a DER-formatted PKCS#8 unencrypted private key.
    ///
    /// This method is mainly for interoperability reasons. Encrypted keyfiles should be preferred.
    pub fn private_key_from_pkcs8(der: &[u8]) -> Result<PKey<Private>, ErrorStack> {
        unsafe {
            ffi::init();
            let len = der.len().min(c_long::MAX as usize) as c_long;
            let p8inf = cvt_p(ffi::d2i_PKCS8_PRIV_KEY_INFO(
                ptr::null_mut(),
                &mut der.as_ptr(),
                len,
            ))?;
            let res = cvt_p(ffi::EVP_PKCS82PKEY(p8inf)).map(|p| PKey::from_ptr(p));
            ffi::PKCS8_PRIV_KEY_INFO_free(p8inf);
            res
        }
    }

    /// Deserializes a DER-formatted PKCS#8 private key, using a callback to retrieve the password
    /// if the key is encrypted.
    ///
    /// The callback should copy the password into the provided buffer and return the number of
    /// bytes written.
    #[corresponds(d2i_PKCS8PrivateKey_bio)]
    pub fn private_key_from_pkcs8_callback<F>(
        der: &[u8],
        callback: F,
    ) -> Result<PKey<Private>, ErrorStack>
    where
        F: FnOnce(&mut [u8]) -> Result<usize, ErrorStack>,
    {
        unsafe {
            ffi::init();
            let mut cb = CallbackState::new(callback);
            let bio = MemBioSlice::new(der)?;
            cvt_p(ffi::d2i_PKCS8PrivateKey_bio(
                bio.as_ptr(),
                ptr::null_mut(),
                Some(invoke_passwd_cb::<F>),
                &mut cb as *mut _ as *mut _,
            ))
            .map(|p| PKey::from_ptr(p))
        }
    }

    /// Deserializes a DER-formatted PKCS#8 private key, using the supplied password if the key is
    /// encrypted.
    ///
    /// # Panics
    ///
    /// Panics if `passphrase` contains an embedded null.
    #[corresponds(d2i_PKCS8PrivateKey_bio)]
    pub fn private_key_from_pkcs8_passphrase(
        der: &[u8],
        passphrase: &[u8],
    ) -> Result<PKey<Private>, ErrorStack> {
        unsafe {
            ffi::init();
            let bio = MemBioSlice::new(der)?;
            let passphrase = CString::new(passphrase).unwrap();
            cvt_p(ffi::d2i_PKCS8PrivateKey_bio(
                bio.as_ptr(),
                ptr::null_mut(),
                None,
                passphrase.as_ptr() as *const _ as *mut _,
            ))
            .map(|p| PKey::from_ptr(p))
        }
    }

    /// Creates a private key from its raw byte representation
    ///
    /// Algorithm types that support raw private keys are HMAC, X25519, ED25519, X448 or ED448
    #[corresponds(EVP_PKEY_new_raw_private_key)]
    #[cfg(any(ossl111, boringssl, libressl370, awslc))]
    pub fn private_key_from_raw_bytes(
        bytes: &[u8],
        key_type: Id,
    ) -> Result<PKey<Private>, ErrorStack> {
        unsafe {
            ffi::init();
            cvt_p(ffi::EVP_PKEY_new_raw_private_key(
                key_type.as_raw(),
                ptr::null_mut(),
                bytes.as_ptr(),
                bytes.len(),
            ))
            .map(|p| PKey::from_ptr(p))
        }
    }

    /// Creates a private key from its raw byte representation, identifying
    /// the algorithm by name and optionally taking a library context and
    /// property query string.
    ///
    /// This is required for algorithms that are only available via OpenSSL
    /// 3.0+ providers and have no associated `Id`, such as ML-DSA.
    #[corresponds(EVP_PKEY_new_raw_private_key_ex)]
    #[cfg(ossl300)]
    pub fn private_key_from_raw_bytes_ex(
        ctx: Option<&LibCtxRef>,
        key_type: KeyType,
        properties: Option<&CStr>,
        bytes: &[u8],
    ) -> Result<PKey<Private>, ErrorStack> {
        unsafe {
            ffi::init();
            cvt_p(ffi::EVP_PKEY_new_raw_private_key_ex(
                ctx.map_or(ptr::null_mut(), ForeignTypeRef::as_ptr),
                key_type.as_cstr().as_ptr(),
                properties.map_or(ptr::null(), |s| s.as_ptr()),
                bytes.as_ptr(),
                bytes.len(),
            ))
            .map(|p| PKey::from_ptr(p))
        }
    }

    /// Constructs a private keypair from an algorithm-defined `seed`.
    ///
    /// Only ML-DSA (32-byte seed) and ML-KEM (64-byte seed) are supported at
    /// this time, and require OpenSSL 3.5 or newer at runtime. The seed
    /// length is algorithm-defined and a wrong-sized seed is rejected by the
    /// provider.
    ///
    /// Internally this calls `EVP_PKEY_CTX_new_from_name` followed by
    /// `EVP_PKEY_fromdata` with a single `seed` octet-string `OSSL_PARAM` and
    /// `EVP_PKEY_KEYPAIR` selection.
    #[corresponds(EVP_PKEY_fromdata)]
    #[cfg(ossl350)]
    pub fn private_key_from_seed(
        ctx: Option<&LibCtxRef>,
        key_type: KeyType,
        properties: Option<&CStr>,
        seed: &[u8],
    ) -> Result<PKey<Private>, ErrorStack> {
        let mut builder = crate::ossl_param::OsslParamBuilder::new()?;
        builder.add_octet_string(c"seed", seed)?;
        let params = builder.to_param()?;
        unsafe {
            ffi::init();
            let pkey_ctx = cvt_p(ffi::EVP_PKEY_CTX_new_from_name(
                ctx.map_or(ptr::null_mut(), ForeignTypeRef::as_ptr),
                key_type.as_cstr().as_ptr(),
                properties.map_or(ptr::null(), |s| s.as_ptr()),
            ))?;
            // Take ownership immediately so the context is freed on early return.
            let pkey_ctx = PkeyCtx::<Private>::from_ptr(pkey_ctx);
            cvt(ffi::EVP_PKEY_fromdata_init(pkey_ctx.as_ptr()))?;
            let mut pkey: *mut ffi::EVP_PKEY = ptr::null_mut();
            cvt(ffi::EVP_PKEY_fromdata(
                pkey_ctx.as_ptr(),
                &mut pkey,
                ffi::EVP_PKEY_KEYPAIR,
                params.as_ptr(),
            ))?;
            Ok(PKey::from_ptr(pkey))
        }
    }
}

impl PKey<Public> {
    private_key_from_pem! {
        /// Decodes a PEM-encoded SubjectPublicKeyInfo structure.
        ///
        /// The input should have a header of `-----BEGIN PUBLIC KEY-----`.
        #[corresponds(PEM_read_bio_PUBKEY)]
        public_key_from_pem,

        /// Decodes a PEM-encoded SubjectPublicKeyInfo structure.
        #[corresponds(PEM_read_bio_PUBKEY)]
        public_key_from_pem_passphrase,

        /// Decodes a PEM-encoded SubjectPublicKeyInfo structure.
        ///
        /// The callback should fill the password into the provided buffer and return its length.
        #[corresponds(PEM_read_bio_PrivateKey)]
        public_key_from_pem_callback,
        PKey<Public>,
        ffi::PEM_read_bio_PUBKEY
    }

    from_der! {
        /// Decodes a DER-encoded SubjectPublicKeyInfo structure.
        #[corresponds(d2i_PUBKEY)]
        public_key_from_der,
        PKey<Public>,
        ffi::d2i_PUBKEY
    }

    /// Creates a public key from its raw byte representation
    ///
    /// Algorithm types that support raw public keys are X25519, ED25519, X448 or ED448
    #[corresponds(EVP_PKEY_new_raw_public_key)]
    #[cfg(any(ossl111, boringssl, libressl370, awslc))]
    pub fn public_key_from_raw_bytes(
        bytes: &[u8],
        key_type: Id,
    ) -> Result<PKey<Public>, ErrorStack> {
        unsafe {
            ffi::init();
            cvt_p(ffi::EVP_PKEY_new_raw_public_key(
                key_type.as_raw(),
                ptr::null_mut(),
                bytes.as_ptr(),
                bytes.len(),
            ))
            .map(|p| PKey::from_ptr(p))
        }
    }

    /// Creates a public key from its raw byte representation, identifying
    /// the algorithm by name and optionally taking a library context and
    /// property query string.
    ///
    /// This is required for algorithms that are only available via OpenSSL
    /// 3.0+ providers and have no associated `Id`, such as ML-DSA.
    #[corresponds(EVP_PKEY_new_raw_public_key_ex)]
    #[cfg(ossl300)]
    pub fn public_key_from_raw_bytes_ex(
        ctx: Option<&LibCtxRef>,
        key_type: KeyType,
        properties: Option<&CStr>,
        bytes: &[u8],
    ) -> Result<PKey<Public>, ErrorStack> {
        unsafe {
            ffi::init();
            cvt_p(ffi::EVP_PKEY_new_raw_public_key_ex(
                ctx.map_or(ptr::null_mut(), ForeignTypeRef::as_ptr),
                key_type.as_cstr().as_ptr(),
                properties.map_or(ptr::null(), |s| s.as_ptr()),
                bytes.as_ptr(),
                bytes.len(),
            ))
            .map(|p| PKey::from_ptr(p))
        }
    }
}

use ffi::EVP_PKEY_up_ref;

impl<T> TryFrom<EcKey<T>> for PKey<T> {
    type Error = ErrorStack;

    fn try_from(ec_key: EcKey<T>) -> Result<PKey<T>, ErrorStack> {
        PKey::from_ec_key(ec_key)
    }
}

impl<T> TryFrom<PKey<T>> for EcKey<T> {
    type Error = ErrorStack;

    fn try_from(pkey: PKey<T>) -> Result<EcKey<T>, ErrorStack> {
        pkey.ec_key()
    }
}

impl<T> TryFrom<Rsa<T>> for PKey<T> {
    type Error = ErrorStack;

    fn try_from(rsa: Rsa<T>) -> Result<PKey<T>, ErrorStack> {
        PKey::from_rsa(rsa)
    }
}

impl<T> TryFrom<PKey<T>> for Rsa<T> {
    type Error = ErrorStack;

    fn try_from(pkey: PKey<T>) -> Result<Rsa<T>, ErrorStack> {
        pkey.rsa()
    }
}

impl<T> TryFrom<Dsa<T>> for PKey<T> {
    type Error = ErrorStack;

    fn try_from(dsa: Dsa<T>) -> Result<PKey<T>, ErrorStack> {
        PKey::from_dsa(dsa)
    }
}

impl<T> TryFrom<PKey<T>> for Dsa<T> {
    type Error = ErrorStack;

    fn try_from(pkey: PKey<T>) -> Result<Dsa<T>, ErrorStack> {
        pkey.dsa()
    }
}

#[cfg(not(boringssl))]
impl<T> TryFrom<Dh<T>> for PKey<T> {
    type Error = ErrorStack;

    fn try_from(dh: Dh<T>) -> Result<PKey<T>, ErrorStack> {
        PKey::from_dh(dh)
    }
}

impl<T> TryFrom<PKey<T>> for Dh<T> {
    type Error = ErrorStack;

    fn try_from(pkey: PKey<T>) -> Result<Dh<T>, ErrorStack> {
        pkey.dh()
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    #[cfg(not(boringssl))]
    use crate::dh::Dh;
    use crate::dsa::Dsa;
    use crate::ec::EcKey;
    use crate::error::Error;
    use crate::nid::Nid;
    use crate::rsa::Rsa;
    use crate::symm::Cipher;

    use super::*;

    #[cfg(any(ossl111, awslc))]
    use crate::rand::rand_bytes;

    #[test]
    fn test_to_password() {
        let rsa = Rsa::generate(2048).unwrap();
        let pkey = PKey::from_rsa(rsa).unwrap();
        let pem = pkey
            .private_key_to_pem_pkcs8_passphrase(Cipher::aes_128_cbc(), b"foobar")
            .unwrap();
        PKey::private_key_from_pem_passphrase(&pem, b"foobar").unwrap();
        assert!(PKey::private_key_from_pem_passphrase(&pem, b"fizzbuzz").is_err());
    }

    #[test]
    fn test_unencrypted_pkcs8() {
        let key = include_bytes!("../test/pkcs8-nocrypt.der");
        let pkey = PKey::private_key_from_pkcs8(key).unwrap();
        let serialized = pkey.private_key_to_pkcs8().unwrap();
        let pkey2 = PKey::private_key_from_pkcs8(&serialized).unwrap();

        assert_eq!(
            pkey2.private_key_to_der().unwrap(),
            pkey.private_key_to_der().unwrap()
        );
    }

    #[test]
    fn test_encrypted_pkcs8_passphrase() {
        let key = include_bytes!("../test/pkcs8.der");
        PKey::private_key_from_pkcs8_passphrase(key, b"mypass").unwrap();

        let rsa = Rsa::generate(2048).unwrap();
        let pkey = PKey::from_rsa(rsa).unwrap();
        let der = pkey
            .private_key_to_pkcs8_passphrase(Cipher::aes_128_cbc(), b"mypass")
            .unwrap();
        let pkey2 = PKey::private_key_from_pkcs8_passphrase(&der, b"mypass").unwrap();
        assert_eq!(
            pkey.private_key_to_der().unwrap(),
            pkey2.private_key_to_der().unwrap()
        );
    }

    #[test]
    fn test_encrypted_pkcs8_callback() {
        let mut password_queried = false;
        let key = include_bytes!("../test/pkcs8.der");
        PKey::private_key_from_pkcs8_callback(key, |password| {
            password_queried = true;
            password[..6].copy_from_slice(b"mypass");
            Ok(6)
        })
        .unwrap();
        assert!(password_queried);
    }

    #[test]
    fn test_private_key_from_pem() {
        let key = include_bytes!("../test/key.pem");
        PKey::private_key_from_pem(key).unwrap();
    }

    #[test]
    fn test_public_key_from_pem() {
        let key = include_bytes!("../test/key.pem.pub");
        PKey::public_key_from_pem(key).unwrap();
    }

    #[test]
    fn test_public_key_from_der() {
        let key = include_bytes!("../test/key.der.pub");
        PKey::public_key_from_der(key).unwrap();
    }

    #[test]
    fn test_private_key_from_der() {
        let key = include_bytes!("../test/key.der");
        PKey::private_key_from_der(key).unwrap();
    }

    #[test]
    fn test_pem() {
        let key = include_bytes!("../test/key.pem");
        let key = PKey::private_key_from_pem(key).unwrap();

        let priv_key = key.private_key_to_pem_pkcs8().unwrap();
        let pub_key = key.public_key_to_pem().unwrap();

        // As a super-simple verification, just check that the buffers contain
        // the `PRIVATE KEY` or `PUBLIC KEY` strings.
        assert!(priv_key.windows(11).any(|s| s == b"PRIVATE KEY"));
        assert!(pub_key.windows(10).any(|s| s == b"PUBLIC KEY"));
    }

    #[test]
    fn test_rsa_accessor() {
        let rsa = Rsa::generate(2048).unwrap();
        let pkey = PKey::from_rsa(rsa).unwrap();
        pkey.rsa().unwrap();
        assert_eq!(pkey.id(), Id::RSA);
        assert!(pkey.dsa().is_err());
    }

    #[test]
    fn test_dsa_accessor() {
        let dsa = Dsa::generate(2048).unwrap();
        let pkey = PKey::from_dsa(dsa).unwrap();
        pkey.dsa().unwrap();
        assert_eq!(pkey.id(), Id::DSA);
        assert!(pkey.rsa().is_err());
    }

    #[test]
    #[cfg(not(boringssl))]
    fn test_dh_accessor() {
        let dh = include_bytes!("../test/dhparams.pem");
        let dh = Dh::params_from_pem(dh).unwrap();
        let pkey = PKey::from_dh(dh).unwrap();
        pkey.dh().unwrap();
        assert_eq!(pkey.id(), Id::DH);
        assert!(pkey.rsa().is_err());
    }

    #[test]
    fn test_ec_key_accessor() {
        let ec_key = EcKey::from_curve_name(Nid::X9_62_PRIME256V1).unwrap();
        let pkey = PKey::from_ec_key(ec_key).unwrap();
        pkey.ec_key().unwrap();
        assert_eq!(pkey.id(), Id::EC);
        assert!(pkey.rsa().is_err());
    }

    #[test]
    fn test_rsa_conversion() {
        let rsa = Rsa::generate(2048).unwrap();
        let pkey: PKey<Private> = rsa.clone().try_into().unwrap();
        let rsa_: Rsa<Private> = pkey.try_into().unwrap();
        // Eq is missing
        assert_eq!(rsa.p(), rsa_.p());
        assert_eq!(rsa.q(), rsa_.q());
    }

    #[test]
    fn test_dsa_conversion() {
        let dsa = Dsa::generate(2048).unwrap();
        let pkey: PKey<Private> = dsa.clone().try_into().unwrap();
        let dsa_: Dsa<Private> = pkey.try_into().unwrap();
        // Eq is missing
        assert_eq!(dsa.priv_key(), dsa_.priv_key());
    }

    #[test]
    fn test_ec_key_conversion() {
        let group = crate::ec::EcGroup::from_curve_name(crate::nid::Nid::X9_62_PRIME256V1).unwrap();
        let ec_key = EcKey::generate(&group).unwrap();
        let pkey: PKey<Private> = ec_key.clone().try_into().unwrap();
        let ec_key_: EcKey<Private> = pkey.try_into().unwrap();
        // Eq is missing
        assert_eq!(ec_key.private_key(), ec_key_.private_key());
    }

    #[test]
    #[cfg(any(ossl110, libressl360))]
    fn test_security_bits() {
        let group = crate::ec::EcGroup::from_curve_name(crate::nid::Nid::SECP521R1).unwrap();
        let ec_key = EcKey::generate(&group).unwrap();
        let pkey: PKey<Private> = ec_key.try_into().unwrap();

        assert_eq!(pkey.security_bits(), 256);
    }

    #[test]
    #[cfg(not(boringssl))]
    fn test_dh_conversion() {
        let dh_params = include_bytes!("../test/dhparams.pem");
        let dh_params = Dh::params_from_pem(dh_params).unwrap();
        let dh = dh_params.generate_key().unwrap();

        // Clone is missing for Dh, save the parameters
        let p = dh.prime_p().to_owned().unwrap();
        let q = dh.prime_q().map(|q| q.to_owned().unwrap());
        let g = dh.generator().to_owned().unwrap();

        let pkey: PKey<Private> = dh.try_into().unwrap();
        let dh_: Dh<Private> = pkey.try_into().unwrap();

        // Eq is missing
        assert_eq!(&p, dh_.prime_p());
        assert_eq!(q, dh_.prime_q().map(|q| q.to_owned().unwrap()));
        assert_eq!(&g, dh_.generator());
    }

    #[cfg(any(ossl111, boringssl, libressl370, awslc))]
    fn test_raw_public_key(gen: fn() -> Result<PKey<Private>, ErrorStack>, key_type: Id) {
        // Generate a new key
        let key = gen().unwrap();

        // Get the raw bytes, and create a new key from the raw bytes
        let raw = key.raw_public_key().unwrap();
        let from_raw = PKey::public_key_from_raw_bytes(&raw, key_type).unwrap();

        // Compare the der encoding of the original and raw / restored public key
        assert_eq!(
            key.public_key_to_der().unwrap(),
            from_raw.public_key_to_der().unwrap()
        );
    }

    #[cfg(any(ossl111, boringssl, libressl370, awslc))]
    fn test_raw_private_key(gen: fn() -> Result<PKey<Private>, ErrorStack>, key_type: Id) {
        // Generate a new key
        let key = gen().unwrap();

        // Get the raw bytes, and create a new key from the raw bytes
        let raw = key.raw_private_key().unwrap();
        let from_raw = PKey::private_key_from_raw_bytes(&raw, key_type).unwrap();

        // Compare the der encoding of the original and raw / restored public key
        assert_eq!(
            key.private_key_to_pkcs8().unwrap(),
            from_raw.private_key_to_pkcs8().unwrap()
        );
    }

    #[cfg(any(ossl111, boringssl, libressl370, awslc))]
    #[test]
    fn test_raw_public_key_bytes() {
        test_raw_public_key(PKey::generate_x25519, Id::X25519);
        test_raw_public_key(PKey::generate_ed25519, Id::ED25519);
        #[cfg(not(any(boringssl, libressl370, awslc)))]
        test_raw_public_key(PKey::generate_x448, Id::X448);
        #[cfg(not(any(boringssl, libressl370, awslc)))]
        test_raw_public_key(PKey::generate_ed448, Id::ED448);
    }

    #[cfg(any(ossl111, boringssl, libressl370, awslc))]
    #[test]
    fn test_raw_private_key_bytes() {
        test_raw_private_key(PKey::generate_x25519, Id::X25519);
        test_raw_private_key(PKey::generate_ed25519, Id::ED25519);
        #[cfg(not(any(boringssl, libressl370, awslc)))]
        test_raw_private_key(PKey::generate_x448, Id::X448);
        #[cfg(not(any(boringssl, libressl370, awslc)))]
        test_raw_private_key(PKey::generate_ed448, Id::ED448);
    }

    #[cfg(any(ossl111, awslc))]
    #[test]
    fn test_raw_hmac() {
        let mut test_bytes = vec![0u8; 32];
        rand_bytes(&mut test_bytes).unwrap();

        let hmac_key = PKey::hmac(&test_bytes).unwrap();
        assert!(hmac_key.raw_public_key().is_err());

        let key_bytes = hmac_key.raw_private_key().unwrap();
        assert_eq!(key_bytes, test_bytes);
    }

    #[cfg(any(ossl111, awslc))]
    #[test]
    fn test_raw_key_fail() {
        // Getting a raw byte representation will not work with Nist curves
        let group = crate::ec::EcGroup::from_curve_name(Nid::SECP256K1).unwrap();
        let ec_key = EcKey::generate(&group).unwrap();
        let pkey = PKey::from_ec_key(ec_key).unwrap();
        assert!(pkey.raw_private_key().is_err());
        assert!(pkey.raw_public_key().is_err());
    }

    #[cfg(ossl300)]
    #[test]
    fn test_is_a() {
        let rsa = Rsa::generate(2048).unwrap();
        let pkey = PKey::from_rsa(rsa).unwrap();
        assert!(pkey.is_a(KeyType::RSA));
        assert!(!pkey.is_a(KeyType::EC));
        assert!(!pkey.is_a(KeyType::ML_DSA_65));

        let ed = PKey::generate_ed25519().unwrap();
        assert!(ed.is_a(KeyType::ED25519));
        assert!(!ed.is_a(KeyType::X25519));
    }

    #[cfg(ossl300)]
    #[test]
    fn test_raw_public_key_from_bytes_ex() {
        let key = PKey::generate_ed25519().unwrap();
        let raw = key.raw_public_key().unwrap();
        let from_raw =
            PKey::public_key_from_raw_bytes_ex(None, KeyType::ED25519, None, &raw).unwrap();
        assert_eq!(
            key.public_key_to_der().unwrap(),
            from_raw.public_key_to_der().unwrap()
        );
        assert!(from_raw.is_a(KeyType::ED25519));

        // Wrong key length should fail.
        assert!(PKey::public_key_from_raw_bytes_ex(None, KeyType::ED25519, None, &[]).is_err());
    }

    #[cfg(ossl300)]
    #[test]
    fn test_raw_private_key_from_bytes_ex() {
        let key = PKey::generate_ed25519().unwrap();
        let raw = key.raw_private_key().unwrap();
        let from_raw =
            PKey::private_key_from_raw_bytes_ex(None, KeyType::ED25519, None, &raw).unwrap();
        assert_eq!(
            key.private_key_to_pkcs8().unwrap(),
            from_raw.private_key_to_pkcs8().unwrap()
        );
    }

    #[cfg(ossl300)]
    #[test]
    fn test_ec_gen() {
        let key = PKey::ec_gen("prime256v1").unwrap();
        assert!(key.ec_key().is_ok());
    }

    #[cfg(ossl350)]
    #[test]
    fn test_private_key_from_seed_mldsa() {
        // ML-DSA-65 KAT vector 0 from
        // https://github.com/post-quantum-cryptography/KAT/blob/main/MLDSA/kat_MLDSA_65_det_pure.rsp
        // (xi is the 32-byte ML-DSA key generation seed per FIPS 204).
        let xi = hex::decode("f696484048ec21f96cf50a56d0759c448f3779752f0383d37449690694cf7a68")
            .unwrap();
        let key = PKey::private_key_from_seed(None, KeyType::ML_DSA_65, None, &xi).unwrap();
        assert!(key.is_a(KeyType::ML_DSA_65));
        let expected_pk = "e50d03fff3b3a70961abbb92a390008dec1283f603f50cdbaaa3d00bd659bc767c3f24ec864ceb07b865aa148647698df8e63f244c4de08affc0210f1560f64822961972463e403bbe97ce7a539fc013527558ad824202a90b1e9a045d89a51c3a31d0330f2099d0f5e0b9e8de8d1e340c91d6a0f61cb8a6548e2614a1b6a2ad80f4e567f0f134700b1563ccaab71f28e7bf509858d85218166dd9a0e1dfad4bee180b4cdaf6e37623558f64fd124d3d7543aade0b28fb8f193159cea7dfb172174b6c25375c9c1903636bfaa41791b1f2f16158020806a1d95979f678a46a209a8780345d2d092c52b576b5e263e870570cc1084058676fbddb2c93bc87fd81a90f7081c04fb299415f761966614aedeea40386f0dbe97512956c3f16c3e210a364de926e37374637d95d0420de7f2f72365392a6d4392018762cd6aa4d6ec629f6d0605ab86862a34c3f1fb55695ae35e736404044aad617d192e8ff07a16f5c6291c2edb0bf1d601a6b08f1c9b444e31570113124cd20eeb299d30a4546243a9f20ed36fa963edab2f494cd92f766633b97237ccc3485387f4344839f4656fbf1eb7f4f24712f432f3b74df325747405bc9ee39f42f87653322f1d23c92c981953fc107570053ce46b6741410a99cdb1888d33943e191c0a395085b9d14a3fbdc58a3ea16706c937ea44aebc9764df142010eab022c40b28e63da853ae03843bfe02eed35331571ec89895c1ea2256cb7591e63c7a5870455663ef9804b84d524470a08cda9bbbfd07ba6537473163cf030849c5f31679c610d56d5e31c0e73f23098d3a19dd39afe507e25d053e7d5b0d9b18c53b153c2d5b162558939a6e24e7ba02d1d736b6a4c93a4f3bc50d4ab16ef350b411e6f4a734be03242fd67ee47eb4ec3d453d1a9254c4e02f68a366702ef2875932b72125ee81da1c10a336b4a4990a5e36f0b59b00e3471c56314d6e92bcb7bacd6219fd99c1ca50c3342ce62cd98be9458a17cf243c60c09b106e86fab345a997f7b46d4ac1c790c37dbdb93d29c532a5cb097a30f92d47c460ec8345b17ba5db77c1a6533a9448a353663f187517a399583b2f98cb0a8dc3f64d049716a5c8aee6ede0bb6958fc70f2fce706f20622d35e9ff2a1c30dd5e71bebe4a33fd74ab768cd34a2d9ec59845a8f38b5dd6c0008b678876e493c9afc2396a16721142803f1f38c579036858a25a1a1abfae94c7dc1ecb26c1d3b4d96209be238360fc8554e33f5fba2b92abf207b677d433b58275366b836be7081d7b50f9d29652c836ffb11596317cb3aaaf4ed41441298fb386fbd9237227bb7529bf5eeb7711bc6936cd4fba98b8404dd1e8650a3a1bf29869835797b9537db1afc0f4339ad3b296401100520dd43d2cd453534f1df776c0aa184f2e5cb658fee5b54bb44d9ee13b3486c37b1fea4284327ce15400ecd93a0c01852d045c3c7af348d4786845984fde0d086c115d4fcbcfee73688ef61601ce3560d6db6f0a6be4dc05640c575a2d24a6a5b5d697ecc3a6844bf7405f68c5450b1d67b5dfcfcc8f878d787f7f57d3875fdf345f1730f9e7493e9a4acacb7b8832b0141a1bdb082a95d8be8f5280035f42f05f9ecf663fa5d03b056c43bc39ba1a6f7375961c4e94830c51e276cc4bc826518f84f51e8ea6f59a3d12ad9d5ef2ca6db70155cabd655713641a885551ab65a358d2e7baf68a39567ba2278d9493562aa4e903ad6f304d2752064d8dbc8a2bf53e24d2f77e47da1d0519212148daf2cb99453b44c7337db46390a6d67d0bbdfa980bfca35d68df1e904168f64fd6b22710eac8bab8757a9e3bb43c5f907949cddecf0321d728a2bbb74e6cc4959c1516c9b2981150f054ec05bd3844e99a7788d5d018c2dc4642969059601a6928963500f085c84cda6454dfc4be63ba82182104499d778e0e998e1cf9086d7990ed03704753f10cb4df6076341f1d556aae9a15ade459e74817fa1d9cb8d0a816afc5947c81368bda9c3a587565b3c39199bf3e24254c601c43002c37b83e43116f25ebfb206d081d81c34618e53aba8ef65af1c5dff402839d71c0319cb7696922088cb9ab3f2eee8ea79228ae12dc9aa1db9acd1309d7171b47a7043fe73cfb4e6b11a3da910f5e5e734c26b41a93452848e735d1679963a413d69a0d275ba10693fb922d8f8c32310dee718125b29d1366201399eb253ba5a1fed099f9df91e3c59c16dfe8f7074045760527327e1e5852537ae96962553b69d85da962a5a6789d19fe585e257012132a7c91feaf4c58a4fa7c126fe68406f34ebf1f371adb4b30514b18dd6e7e659df07776238e48cb7fabd08b5f6a9fc05a7ffbf019a2632c257bf79636994c807fa2f513f60940800e290c2e684d9162858bca138a8634e23b1bb4b49f77af7eb717a79b2f293f814849a8d7e0aae2a734259395c4bf6a3a8deb37a0638121a9dcf83dfecd0c6c58a8eb05c4706e395a869c3ce01d42e31466fba05a45e4181dddb177fa20fef50a770d9da14cffb55ac3e829bd932eff759eeaeebd37d3ecb38f2046528affc969b008d2f9fad5acb4682f119011cbb4ffb11dae5d91dfe9ec7ba5142086e5c09eb398e3685413a394b385a5e377c4996848d862ed7f70b3bb75cff88cf89db9146ea82b5611569a8bb67dc95ab4135c2a427f12ba1c9b50cf86d1a238ba0c99c3d82dbc90dd0f7b281494df1a25848ecadfe915a95a43247bc5a55e1e2d90ed05f70be8b2e5fc9d5b";
        assert_eq!(hex::encode(key.raw_public_key().unwrap()), expected_pk);

        // Wrong seed length is rejected by the provider.
        assert!(PKey::private_key_from_seed(None, KeyType::ML_DSA_65, None, &[]).is_err());
    }

    #[cfg(ossl350)]
    #[test]
    fn test_private_key_from_seed_mlkem() {
        // ML-KEM-768 KAT vector 0 from
        // https://github.com/post-quantum-cryptography/KAT/blob/main/MLKEM/kat_MLKEM_768.rsp
        // (per FIPS 203 the OpenSSL "seed" param is d || z, 64 bytes total).
        let d = hex::decode("6dbbc4375136df3b07f7c70e639e223e177e7fd53b161b3f4d57791794f12624")
            .unwrap();
        let z = hex::decode("f696484048ec21f96cf50a56d0759c448f3779752f0383d37449690694cf7a68")
            .unwrap();
        let mut seed = d;
        seed.extend_from_slice(&z);
        let key = PKey::private_key_from_seed(None, KeyType::ML_KEM_768, None, &seed).unwrap();
        assert!(key.is_a(KeyType::ML_KEM_768));
        let expected_pk = "01f60af1dc8e6360ae78b59d4a5042eb9145a269046d6236b8304f305c2d9dcb189fe5a62df89b2f5a7bce3bbc753c1e78f730a99869f809aba856b676b707b26601d1d909bab32451494eb7d0a2153a6350b79789a9b115f83ea12037256562f06a1d5aba378da77039d3bdecaca8e6a22a49050a76300a0267cdb38b7ac77903c50ca53b99283cac6b95fba651b11a4d1a692e4072965060587669f253b1bb182e661446168ac60221894660020e9bb5f5b7124a0303e2543ea3ea6ce97a2482b255ca346fb27a847b33b93f3ab2d33064c6e6632d1a23f1144e907b246b479f4a5c928929a1e24150f5241258a5b67766a66f6a33846495907828ebe44ecc5b73124071ba479073910410a16d5d5696b48b194752979795772a91c348f502b37aa650983ebb89bf3c081ff273544129c9137a6e1834c8f2e7ce14c7870c53c05b9b94ecd38e6645911b0912336863ec168831f811881075cf38a59de4b5c738aa6ef03d779b295588cfb62491cc7b3e08b48473354f9ac8061c152a9e205997499b970b69bce66fe42bca2924ccdf0103d0a4c39193c2df25118d72b17aab26b0c60d4cd2c306ca4696c185de05035f4a09cf970aecc8cc93436f83b1aeaf452c41929a2eabc151938f74c93b858546df2264eeeab602e04a85c522f8fb1a5214afd8d4cae57a47b6f381a23126bd9917173128af917f1d483691c450d1151cfe9a1492d473ed862e27da92500c86a20019e9f975e4f54ad319ba2c5630c4014219d7ba235456fe530140193d662445e6a941d1e238567ba8d4d95ab1c7447d690821876d017270cfb169f2d792f03c800720697b410ab41c66f2b24585125655eb10aa1087ffcb7750cb887ad4467377500a6a7d3a82976b415a54469577b4138d919b03f4c9a4d3390bdcb6f1717a5fa4ab25a34f4ba5039bb22c7f3c234ea4427347aa7251464e631904d7cac4784f78b49d5f4a104a301809a779f6466131f9c62bb67147f4cd4973a6aa1c29ae6a8647b6268be089fe048ce990cd638743d285c889a707f581b63af41731f0246b054bc4b47aab01b6842a2709d02e8158ab90f48b69d136082b34cb0673b74aa3f54508ed029fb8f5045ee0639e150ee3b3c85f68a310ec0441980100b42abf2bad10d4a9e0c7b2bc5bbcaf73cbcdc49dc2c949111936779b178974a0392947745a47189bc3fa8a679c80af964a9f9b1b56577274a2a669d2da6704aa496af407fa1aa964cc3dc3140f5f959a7ea974bdb1b83e48a99c0a3e2d75b0669b5c1278962540609166266da18886fc237af30cefd569dbe399e6652e45f06a5dfc9a758a4987088ff8e38a3cf36b9d988f0e070b68d0b88f7bcc41306080d889780c7e238895ccaa4f3577225cca4c8a9330ce613e717798c9670924b271ac402b51538b8b5967ac490dcab5300e6c54d6a3632f3b973e4186ee1a7e2e85649185b26370c387235c4df28a9937a49d4078bf883f4e6346cb3251d9e13f1bda087b285afaa80e262641c5527b0a184b8bc84a62e577314658e2029d850064f7a7b81f253e7cc124a9c5b039dc9b179a80c2f6aee6ea0815172537331a57b505baa76ff5b4c1f0da754b6194f4b39a9b18730d3cdab925d691ed77a8db9927ea233ac2a12744fdc27e5d221b9369adb325d8";
        assert_eq!(hex::encode(key.raw_public_key().unwrap()), expected_pk);

        // Wrong seed length is rejected by the provider.
        assert!(PKey::private_key_from_seed(None, KeyType::ML_KEM_768, None, &[]).is_err());
    }

    #[cfg(ossl350)]
    #[test]
    fn test_private_key_from_seed_invalid_algorithm() {
        let seed = [0u8; 64];
        assert!(
            PKey::private_key_from_seed(None, KeyType::RSA, None, &seed).is_err(),
            "Unexpectedly accepted a seed-only fromdata import",
        );
    }

    #[cfg(ossl350)]
    #[test]
    fn test_seed_into_mldsa_roundtrip() {
        let xi = hex::decode("f696484048ec21f96cf50a56d0759c448f3779752f0383d37449690694cf7a68")
            .unwrap();
        let key = PKey::private_key_from_seed(None, KeyType::ML_DSA_65, None, &xi).unwrap();

        // Exact-sized buffer succeeds.
        let mut exact = [0u8; 32];
        let n = key.seed_into(&mut exact).unwrap();
        assert_eq!(n, 32);
        assert_eq!(&exact[..], &xi[..]);

        // Buffer too small is rejected by OpenSSL.
        let mut small = [0u8; 16];
        assert!(key.seed_into(&mut small).is_err());

        // Buffer larger than required succeeds; the trailing bytes are
        // left untouched.
        let mut large = [0xaau8; 64];
        let n = key.seed_into(&mut large).unwrap();
        assert_eq!(n, 32);
        assert_eq!(&large[..32], &xi[..]);
        assert!(large[32..].iter().all(|&b| b == 0xaa));
    }

    #[cfg(ossl350)]
    #[test]
    fn test_seed_into_mlkem_roundtrip() {
        let d = hex::decode("6dbbc4375136df3b07f7c70e639e223e177e7fd53b161b3f4d57791794f12624")
            .unwrap();
        let z = hex::decode("f696484048ec21f96cf50a56d0759c448f3779752f0383d37449690694cf7a68")
            .unwrap();
        let mut seed = d;
        seed.extend_from_slice(&z);
        let key = PKey::private_key_from_seed(None, KeyType::ML_KEM_768, None, &seed).unwrap();
        let mut buf = [0u8; 64];
        let n = key.seed_into(&mut buf).unwrap();
        assert_eq!(n, 64);
        assert_eq!(&buf[..], &seed[..]);
    }

    /// `seed_into()` must error on key types that don't have a "seed" OSSL_PARAM.
    #[cfg(ossl350)]
    #[test]
    fn test_seed_into_rejects_non_pq_algorithms() {
        let mut buf = [0u8; 64];
        let rsa = PKey::from_rsa(Rsa::generate(2048).unwrap()).unwrap();
        assert!(rsa.seed_into(&mut buf).is_err());

        let ed = PKey::generate_ed25519().unwrap();
        assert!(ed.seed_into(&mut buf).is_err());
    }

    #[test]
    fn test_public_eq() {
        let rsa = Rsa::generate(2048).unwrap();
        let pkey1 = PKey::from_rsa(rsa).unwrap();

        let group = crate::ec::EcGroup::from_curve_name(Nid::X9_62_PRIME256V1).unwrap();
        let ec_key = EcKey::generate(&group).unwrap();
        let pkey2 = PKey::from_ec_key(ec_key).unwrap();

        assert!(!pkey1.public_eq(&pkey2));
        assert!(Error::get().is_none());
    }
}

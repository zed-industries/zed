//! The asymmetric encryption context.
//!
//! # Examples
//!
//! Encrypt data with RSA
//!
//! ```
//! use openssl::rsa::Rsa;
//! use openssl::pkey::PKey;
//! use openssl::pkey_ctx::PkeyCtx;
//!
//! let key = Rsa::generate(4096).unwrap();
//! let key = PKey::from_rsa(key).unwrap();
//!
//! let mut ctx = PkeyCtx::new(&key).unwrap();
//! ctx.encrypt_init().unwrap();
//!
//! let data = b"Some Crypto Text";
//! let mut ciphertext = vec![];
//! ctx.encrypt_to_vec(data, &mut ciphertext).unwrap();
//! ```

#![cfg_attr(
    not(any(boringssl, awslc)),
    doc = r#"\
Generate a CMAC key

```
use openssl::pkey_ctx::PkeyCtx;
use openssl::pkey::Id;
use openssl::cipher::Cipher;

let mut ctx = PkeyCtx::new_id(Id::CMAC).unwrap();
ctx.keygen_init().unwrap();
ctx.set_keygen_cipher(Cipher::aes_128_cbc()).unwrap();
ctx.set_keygen_mac_key(b"0123456789abcdef").unwrap();
let cmac_key = ctx.keygen().unwrap();
```"#
)]

//!
//! Sign and verify data with RSA
//!
//! ```
//! use openssl::pkey_ctx::PkeyCtx;
//! use openssl::pkey::PKey;
//! use openssl::rsa::Rsa;
//!
//! // Generate a random RSA key.
//! let key = Rsa::generate(4096).unwrap();
//! let key = PKey::from_rsa(key).unwrap();
//!
//! let text = b"Some Crypto Text";
//!
//! // Create the signature.
//! let mut ctx = PkeyCtx::new(&key).unwrap();
//! ctx.sign_init().unwrap();
//! let mut signature = vec![];
//! ctx.sign_to_vec(text, &mut signature).unwrap();
//!
//! // Verify the signature.
//! let mut ctx = PkeyCtx::new(&key).unwrap();
//! ctx.verify_init().unwrap();
//! let valid = ctx.verify(text, &signature).unwrap();
//! assert!(valid);
//! ```
use crate::bn::BigNumRef;
#[cfg(not(any(boringssl, awslc)))]
use crate::cipher::CipherRef;
use crate::error::ErrorStack;
use crate::md::MdRef;
use crate::nid::Nid;
use crate::pkey::{HasPrivate, HasPublic, Id, PKey, PKeyRef, Params, Private};
use crate::rsa::Padding;
use crate::sign::RsaPssSaltlen;
use crate::{cvt, cvt_p};
use cfg_if::cfg_if;
use foreign_types::{ForeignType, ForeignTypeRef};
#[cfg(not(any(boringssl, awslc)))]
use libc::c_int;
#[cfg(ossl320)]
use libc::c_uint;
use openssl_macros::corresponds;
use std::convert::TryFrom;
use std::ptr;

/// HKDF modes of operation.
#[cfg(any(ossl111, libressl360))]
pub struct HkdfMode(c_int);

#[cfg(any(ossl111, libressl360))]
impl HkdfMode {
    /// This is the default mode. Calling [`derive`][PkeyCtxRef::derive] on a [`PkeyCtxRef`] set up
    /// for HKDF will perform an extract followed by an expand operation in one go. The derived key
    /// returned will be the result after the expand operation. The intermediate fixed-length
    /// pseudorandom key K is not returned.
    pub const EXTRACT_THEN_EXPAND: Self = HkdfMode(ffi::EVP_PKEY_HKDEF_MODE_EXTRACT_AND_EXPAND);

    /// In this mode calling [`derive`][PkeyCtxRef::derive] will just perform the extract operation.
    /// The value returned will be the intermediate fixed-length pseudorandom key K.
    ///
    /// The digest, key and salt values must be set before a key is derived or an error occurs.
    pub const EXTRACT_ONLY: Self = HkdfMode(ffi::EVP_PKEY_HKDEF_MODE_EXTRACT_ONLY);

    /// In this mode calling [`derive`][PkeyCtxRef::derive] will just perform the expand operation.
    /// The input key should be set to the intermediate fixed-length pseudorandom key K returned
    /// from a previous extract operation.
    ///
    /// The digest, key and info values must be set before a key is derived or an error occurs.
    pub const EXPAND_ONLY: Self = HkdfMode(ffi::EVP_PKEY_HKDEF_MODE_EXPAND_ONLY);
}

/// Nonce type for ECDSA and DSA.
#[cfg(ossl320)]
#[derive(Debug, PartialEq)]
pub struct NonceType(c_uint);

#[cfg(ossl320)]
impl NonceType {
    /// This is the default mode. It uses a random value for the nonce k as defined in FIPS 186-4 Section 6.3
    /// “Secret Number Generation”.
    pub const RANDOM_K: Self = NonceType(0);

    /// Uses a deterministic value for the nonce k as defined in RFC #6979 (See Section 3.2 “Generation of k”).
    pub const DETERMINISTIC_K: Self = NonceType(1);
}

generic_foreign_type_and_impl_send_sync! {
    type CType = ffi::EVP_PKEY_CTX;
    fn drop = ffi::EVP_PKEY_CTX_free;

    /// A context object which can perform asymmetric cryptography operations.
    pub struct PkeyCtx<T>;
    /// A reference to a [`PkeyCtx`].
    pub struct PkeyCtxRef<T>;
}

impl<T> PkeyCtx<T> {
    /// Creates a new pkey context using the provided key.
    #[corresponds(EVP_PKEY_CTX_new)]
    #[inline]
    pub fn new(pkey: &PKeyRef<T>) -> Result<Self, ErrorStack> {
        unsafe {
            let ptr = cvt_p(ffi::EVP_PKEY_CTX_new(pkey.as_ptr(), ptr::null_mut()))?;
            Ok(PkeyCtx::from_ptr(ptr))
        }
    }
}

impl PkeyCtx<()> {
    /// Creates a new pkey context for the specified algorithm ID.
    #[corresponds(EVP_PKEY_CTX_new_id)]
    #[inline]
    pub fn new_id(id: Id) -> Result<Self, ErrorStack> {
        unsafe {
            let ptr = cvt_p(ffi::EVP_PKEY_CTX_new_id(id.as_raw(), ptr::null_mut()))?;
            Ok(PkeyCtx::from_ptr(ptr))
        }
    }
}

impl<T> PkeyCtxRef<T>
where
    T: HasPublic,
{
    /// Prepares the context for encryption using the public key.
    #[corresponds(EVP_PKEY_encrypt_init)]
    #[inline]
    pub fn encrypt_init(&mut self) -> Result<(), ErrorStack> {
        unsafe {
            cvt(ffi::EVP_PKEY_encrypt_init(self.as_ptr()))?;
        }

        Ok(())
    }

    /// Prepares the context for signature verification using the public key.
    #[corresponds(EVP_PKEY_verify_init)]
    #[inline]
    pub fn verify_init(&mut self) -> Result<(), ErrorStack> {
        unsafe {
            cvt(ffi::EVP_PKEY_verify_init(self.as_ptr()))?;
        }

        Ok(())
    }

    /// Prepares the context for signature recovery using the public key.
    #[corresponds(EVP_PKEY_verify_recover_init)]
    #[inline]
    pub fn verify_recover_init(&mut self) -> Result<(), ErrorStack> {
        unsafe {
            cvt(ffi::EVP_PKEY_verify_recover_init(self.as_ptr()))?;
        }

        Ok(())
    }

    /// Encrypts data using the public key.
    ///
    /// If `to` is set to `None`, an upper bound on the number of bytes required for the output buffer will be
    /// returned.
    #[corresponds(EVP_PKEY_encrypt)]
    #[inline]
    pub fn encrypt(&mut self, from: &[u8], to: Option<&mut [u8]>) -> Result<usize, ErrorStack> {
        let mut written = to.as_ref().map_or(0, |b| b.len());
        unsafe {
            cvt(ffi::EVP_PKEY_encrypt(
                self.as_ptr(),
                to.map_or(ptr::null_mut(), |b| b.as_mut_ptr()),
                &mut written,
                from.as_ptr(),
                from.len(),
            ))?;
        }

        Ok(written)
    }

    /// Like [`Self::encrypt`] but appends ciphertext to a [`Vec`].
    pub fn encrypt_to_vec(&mut self, from: &[u8], out: &mut Vec<u8>) -> Result<usize, ErrorStack> {
        let base = out.len();
        let len = self.encrypt(from, None)?;
        out.resize(base + len, 0);
        let len = self.encrypt(from, Some(&mut out[base..]))?;
        out.truncate(base + len);
        Ok(len)
    }

    /// Verifies the signature of data using the public key.
    ///
    /// Returns `Ok(true)` if the signature is valid, `Ok(false)` if the signature is invalid, and `Err` if an error
    /// occurred.
    ///
    /// # Note
    ///
    /// This verifies the signature of the *raw* data. It is more common to compute and verify the signature of the
    /// cryptographic hash of an arbitrary amount of data. The [`MdCtx`](crate::md_ctx::MdCtx) type can be used to do
    /// that.
    #[corresponds(EVP_PKEY_verify)]
    #[inline]
    pub fn verify(&mut self, data: &[u8], sig: &[u8]) -> Result<bool, ErrorStack> {
        unsafe {
            let r = ffi::EVP_PKEY_verify(
                self.as_ptr(),
                sig.as_ptr(),
                sig.len(),
                data.as_ptr(),
                data.len(),
            );
            // `EVP_PKEY_verify` is not terribly consistent about how it,
            // reports errors. It does not clearly distinguish between 0 and
            // -1, and may put errors on the stack in both cases. If there's
            // errors on the stack, we return `Err()`, else we return
            // `Ok(false)`.
            if r <= 0 {
                let errors = ErrorStack::get();
                if !errors.errors().is_empty() {
                    return Err(errors);
                }
            }

            Ok(r == 1)
        }
    }

    /// Recovers the original data signed by the private key. You almost
    /// always want `verify` instead.
    ///
    /// Returns the number of bytes written to `to`, or the number of bytes
    /// that would be written, if `to` is `None.
    #[corresponds(EVP_PKEY_verify_recover)]
    #[inline]
    pub fn verify_recover(
        &mut self,
        sig: &[u8],
        to: Option<&mut [u8]>,
    ) -> Result<usize, ErrorStack> {
        let mut written = to.as_ref().map_or(0, |b| b.len());
        unsafe {
            cvt(ffi::EVP_PKEY_verify_recover(
                self.as_ptr(),
                to.map_or(ptr::null_mut(), |b| b.as_mut_ptr()),
                &mut written,
                sig.as_ptr(),
                sig.len(),
            ))?;
        }

        Ok(written)
    }
}

impl<T> PkeyCtxRef<T>
where
    T: HasPrivate,
{
    /// Prepares the context for decryption using the private key.
    #[corresponds(EVP_PKEY_decrypt_init)]
    #[inline]
    pub fn decrypt_init(&mut self) -> Result<(), ErrorStack> {
        unsafe {
            cvt(ffi::EVP_PKEY_decrypt_init(self.as_ptr()))?;
        }

        Ok(())
    }

    /// Prepares the context for signing using the private key.
    #[corresponds(EVP_PKEY_sign_init)]
    #[inline]
    pub fn sign_init(&mut self) -> Result<(), ErrorStack> {
        unsafe {
            cvt(ffi::EVP_PKEY_sign_init(self.as_ptr()))?;
        }

        Ok(())
    }

    /// Sets the peer key used for secret derivation.
    #[corresponds(EVP_PKEY_derive_set_peer)]
    pub fn derive_set_peer<U>(&mut self, key: &PKeyRef<U>) -> Result<(), ErrorStack>
    where
        U: HasPublic,
    {
        unsafe {
            cvt(ffi::EVP_PKEY_derive_set_peer(self.as_ptr(), key.as_ptr()))?;
        }

        Ok(())
    }

    /// Decrypts data using the private key.
    ///
    /// If `to` is set to `None`, an upper bound on the number of bytes required for the output buffer will be
    /// returned.
    #[corresponds(EVP_PKEY_decrypt)]
    #[inline]
    pub fn decrypt(&mut self, from: &[u8], to: Option<&mut [u8]>) -> Result<usize, ErrorStack> {
        let mut written = to.as_ref().map_or(0, |b| b.len());
        unsafe {
            cvt(ffi::EVP_PKEY_decrypt(
                self.as_ptr(),
                to.map_or(ptr::null_mut(), |b| b.as_mut_ptr()),
                &mut written,
                from.as_ptr(),
                from.len(),
            ))?;
        }

        Ok(written)
    }

    /// Like [`Self::decrypt`] but appends plaintext to a [`Vec`].
    pub fn decrypt_to_vec(&mut self, from: &[u8], out: &mut Vec<u8>) -> Result<usize, ErrorStack> {
        let base = out.len();
        let len = self.decrypt(from, None)?;
        out.resize(base + len, 0);
        let len = self.decrypt(from, Some(&mut out[base..]))?;
        out.truncate(base + len);
        Ok(len)
    }

    /// Signs the contents of `data`.
    ///
    /// If `sig` is set to `None`, an upper bound on the number of bytes required for the output buffer will be
    /// returned.
    ///
    /// # Note
    ///
    /// This computes the signature of the *raw* bytes of `data`. It is more common to sign the cryptographic hash of
    /// an arbitrary amount of data. The [`MdCtx`](crate::md_ctx::MdCtx) type can be used to do that.
    #[corresponds(EVP_PKEY_sign)]
    #[inline]
    pub fn sign(&mut self, data: &[u8], sig: Option<&mut [u8]>) -> Result<usize, ErrorStack> {
        let mut written = sig.as_ref().map_or(0, |b| b.len());
        unsafe {
            cvt(ffi::EVP_PKEY_sign(
                self.as_ptr(),
                sig.map_or(ptr::null_mut(), |b| b.as_mut_ptr()),
                &mut written,
                data.as_ptr(),
                data.len(),
            ))?;
        }

        Ok(written)
    }

    /// Like [`Self::sign`] but appends the signature to a [`Vec`].
    pub fn sign_to_vec(&mut self, data: &[u8], sig: &mut Vec<u8>) -> Result<usize, ErrorStack> {
        let base = sig.len();
        let len = self.sign(data, None)?;
        sig.resize(base + len, 0);
        let len = self.sign(data, Some(&mut sig[base..]))?;
        sig.truncate(base + len);
        Ok(len)
    }
}

impl<T> PkeyCtxRef<T> {
    /// Prepares the context for shared secret derivation.
    #[corresponds(EVP_PKEY_derive_init)]
    #[inline]
    pub fn derive_init(&mut self) -> Result<(), ErrorStack> {
        unsafe {
            cvt(ffi::EVP_PKEY_derive_init(self.as_ptr()))?;
        }

        Ok(())
    }

    /// Prepares the context for key generation.
    #[corresponds(EVP_PKEY_keygen_init)]
    #[inline]
    pub fn keygen_init(&mut self) -> Result<(), ErrorStack> {
        unsafe {
            cvt(ffi::EVP_PKEY_keygen_init(self.as_ptr()))?;
        }

        Ok(())
    }

    /// Prepares the context for key parameter generation.
    #[corresponds(EVP_PKEY_paramgen_init)]
    #[inline]
    pub fn paramgen_init(&mut self) -> Result<(), ErrorStack> {
        unsafe {
            cvt(ffi::EVP_PKEY_paramgen_init(self.as_ptr()))?;
        }

        Ok(())
    }

    /// Sets which algorithm was used to compute the digest used in a
    /// signature. With RSA signatures this causes the signature to be wrapped
    /// in a `DigestInfo` structure. This is almost always what you want with
    /// RSA signatures.
    #[corresponds(EVP_PKEY_CTX_set_signature_md)]
    #[inline]
    pub fn set_signature_md(&self, md: &MdRef) -> Result<(), ErrorStack> {
        unsafe {
            cvt(ffi::EVP_PKEY_CTX_set_signature_md(
                self.as_ptr(),
                md.as_ptr(),
            ))?;
        }
        Ok(())
    }

    /// Sets the DH paramgen prime length.
    ///
    /// This is only useful for DH keys.
    #[corresponds(EVP_PKEY_CTX_set_dh_paramgen_prime_len)]
    #[cfg(not(boringssl))]
    #[inline]
    pub fn set_dh_paramgen_prime_len(&mut self, bits: u32) -> Result<(), ErrorStack> {
        unsafe {
            cvt(ffi::EVP_PKEY_CTX_set_dh_paramgen_prime_len(
                self.as_ptr(),
                bits as i32,
            ))?;
        }

        Ok(())
    }

    /// Sets the DH paramgen generator.
    ///
    /// This is only useful for DH keys.
    #[corresponds(EVP_PKEY_CTX_set_dh_paramgen_generator)]
    #[cfg(not(boringssl))]
    #[inline]
    pub fn set_dh_paramgen_generator(&mut self, bits: u32) -> Result<(), ErrorStack> {
        unsafe {
            cvt(ffi::EVP_PKEY_CTX_set_dh_paramgen_generator(
                self.as_ptr(),
                bits as i32,
            ))?;
        }

        Ok(())
    }

    /// Sets the DSA paramgen bits.
    ///
    /// This is only useful for DSA keys.
    #[corresponds(EVP_PKEY_CTX_set_dsa_paramgen_bits)]
    #[inline]
    pub fn set_dsa_paramgen_bits(&mut self, bits: u32) -> Result<(), ErrorStack> {
        unsafe {
            cvt(ffi::EVP_PKEY_CTX_set_dsa_paramgen_bits(
                self.as_ptr(),
                bits as i32,
            ))?;
        }

        Ok(())
    }

    /// Sets the EC paramgen curve NID.
    ///
    /// This is only useful for EC keys.
    #[corresponds(EVP_PKEY_CTX_set_ec_paramgen_curve_nid)]
    #[inline]
    pub fn set_ec_paramgen_curve_nid(&mut self, nid: Nid) -> Result<(), ErrorStack> {
        unsafe {
            cvt(ffi::EVP_PKEY_CTX_set_ec_paramgen_curve_nid(
                self.as_ptr(),
                nid.as_raw(),
            ))?;
        }

        Ok(())
    }

    /// Returns the RSA padding mode in use.
    ///
    /// This is only useful for RSA keys.
    #[corresponds(EVP_PKEY_CTX_get_rsa_padding)]
    #[inline]
    pub fn rsa_padding(&self) -> Result<Padding, ErrorStack> {
        let mut pad = 0;
        unsafe {
            cvt(ffi::EVP_PKEY_CTX_get_rsa_padding(self.as_ptr(), &mut pad))?;
        }

        Ok(Padding::from_raw(pad))
    }

    /// Sets the RSA padding mode.
    ///
    /// This is only useful for RSA keys.
    #[corresponds(EVP_PKEY_CTX_set_rsa_padding)]
    #[inline]
    pub fn set_rsa_padding(&mut self, padding: Padding) -> Result<(), ErrorStack> {
        unsafe {
            cvt(ffi::EVP_PKEY_CTX_set_rsa_padding(
                self.as_ptr(),
                padding.as_raw(),
            ))?;
        }

        Ok(())
    }

    /// Sets the RSA keygen bits.
    ///
    /// This is only useful for RSA keys.
    #[corresponds(EVP_PKEY_CTX_set_rsa_keygen_bits)]
    #[inline]
    pub fn set_rsa_keygen_bits(&mut self, bits: u32) -> Result<(), ErrorStack> {
        unsafe {
            cvt(ffi::EVP_PKEY_CTX_set_rsa_keygen_bits(
                self.as_ptr(),
                bits as i32,
            ))?;
        }

        Ok(())
    }

    /// Sets the RSA keygen public exponent.
    ///
    /// This is only useful for RSA keys.
    #[corresponds(EVP_PKEY_CTX_set1_rsa_keygen_pubexp)]
    #[inline]
    pub fn set_rsa_keygen_pubexp(&mut self, pubexp: &BigNumRef) -> Result<(), ErrorStack> {
        unsafe {
            cfg_if! {
                if #[cfg(ossl300)] {
                    cvt(ffi::EVP_PKEY_CTX_set1_rsa_keygen_pubexp(
                        self.as_ptr(),
                        pubexp.as_ptr(),
                    ))?;
                } else {
                    cvt(ffi::EVP_PKEY_CTX_set_rsa_keygen_pubexp(
                        self.as_ptr(),
                        // Dupe the BN because the EVP_PKEY_CTX takes ownership of it and will free it.
                        cvt_p(ffi::BN_dup(pubexp.as_ptr()))?,
                    ))?;
                }
            }
        }

        Ok(())
    }

    /// Sets the RSA PSS salt length.
    ///
    /// This is only useful for RSA keys.
    #[corresponds(EVP_PKEY_CTX_set_rsa_pss_saltlen)]
    #[inline]
    pub fn set_rsa_pss_saltlen(&mut self, len: RsaPssSaltlen) -> Result<(), ErrorStack> {
        unsafe {
            cvt(ffi::EVP_PKEY_CTX_set_rsa_pss_saltlen(
                self.as_ptr(),
                len.as_raw(),
            ))
            .map(|_| ())
        }
    }

    /// Sets the RSA MGF1 algorithm.
    ///
    /// This is only useful for RSA keys.
    #[corresponds(EVP_PKEY_CTX_set_rsa_mgf1_md)]
    #[inline]
    pub fn set_rsa_mgf1_md(&mut self, md: &MdRef) -> Result<(), ErrorStack> {
        unsafe {
            cvt(ffi::EVP_PKEY_CTX_set_rsa_mgf1_md(
                self.as_ptr(),
                md.as_ptr(),
            ))?;
        }

        Ok(())
    }

    /// Sets the RSA OAEP algorithm.
    ///
    /// This is only useful for RSA keys.
    #[corresponds(EVP_PKEY_CTX_set_rsa_oaep_md)]
    #[inline]
    pub fn set_rsa_oaep_md(&mut self, md: &MdRef) -> Result<(), ErrorStack> {
        unsafe {
            cvt(ffi::EVP_PKEY_CTX_set_rsa_oaep_md(
                self.as_ptr(),
                md.as_ptr() as *mut _,
            ))?;
        }

        Ok(())
    }

    /// Sets the RSA OAEP label.
    ///
    /// This is only useful for RSA keys.
    #[corresponds(EVP_PKEY_CTX_set0_rsa_oaep_label)]
    pub fn set_rsa_oaep_label(&mut self, label: &[u8]) -> Result<(), ErrorStack> {
        use crate::LenType;
        let len = LenType::try_from(label.len()).unwrap();

        unsafe {
            let p = cvt_p(ffi::OPENSSL_malloc(label.len() as _))?;
            ptr::copy_nonoverlapping(label.as_ptr(), p as *mut _, label.len());

            let r = cvt(ffi::EVP_PKEY_CTX_set0_rsa_oaep_label(
                self.as_ptr(),
                p as *mut _,
                len,
            ));
            if r.is_err() {
                ffi::OPENSSL_free(p);
            }
            r?;
        }

        Ok(())
    }

    /// Sets the cipher used during key generation.
    #[cfg(not(any(boringssl, awslc)))]
    #[corresponds(EVP_PKEY_CTX_ctrl)]
    #[inline]
    pub fn set_keygen_cipher(&mut self, cipher: &CipherRef) -> Result<(), ErrorStack> {
        unsafe {
            cvt(ffi::EVP_PKEY_CTX_ctrl(
                self.as_ptr(),
                -1,
                ffi::EVP_PKEY_OP_KEYGEN,
                ffi::EVP_PKEY_CTRL_CIPHER,
                0,
                cipher.as_ptr() as *mut _,
            ))?;
        }

        Ok(())
    }

    /// Sets the key MAC key used during key generation.
    #[cfg(not(any(boringssl, awslc)))]
    #[corresponds(EVP_PKEY_CTX_ctrl)]
    #[inline]
    pub fn set_keygen_mac_key(&mut self, key: &[u8]) -> Result<(), ErrorStack> {
        let len = c_int::try_from(key.len()).unwrap();

        unsafe {
            cvt(ffi::EVP_PKEY_CTX_ctrl(
                self.as_ptr(),
                -1,
                ffi::EVP_PKEY_OP_KEYGEN,
                ffi::EVP_PKEY_CTRL_SET_MAC_KEY,
                len,
                key.as_ptr() as *mut _,
            ))?;
        }

        Ok(())
    }

    /// Sets the digest used for HKDF derivation.
    ///
    /// Requires OpenSSL 1.1.0 or newer.
    #[corresponds(EVP_PKEY_CTX_set_hkdf_md)]
    #[cfg(any(ossl110, boringssl, libressl360, awslc))]
    #[inline]
    pub fn set_hkdf_md(&mut self, digest: &MdRef) -> Result<(), ErrorStack> {
        unsafe {
            cvt(ffi::EVP_PKEY_CTX_set_hkdf_md(
                self.as_ptr(),
                digest.as_ptr(),
            ))?;
        }

        Ok(())
    }

    /// Sets the HKDF mode of operation.
    ///
    /// Defaults to [`HkdfMode::EXTRACT_THEN_EXPAND`].
    ///
    /// WARNING: Although this API calls it a "mode", HKDF-Extract and HKDF-Expand are distinct
    /// operations with distinct inputs and distinct kinds of keys. Callers should not pass input
    /// secrets for one operation into the other.
    ///
    /// Requires OpenSSL 1.1.1 or newer.
    #[corresponds(EVP_PKEY_CTX_set_hkdf_mode)]
    #[cfg(any(ossl111, libressl360))]
    #[inline]
    pub fn set_hkdf_mode(&mut self, mode: HkdfMode) -> Result<(), ErrorStack> {
        unsafe {
            cvt(ffi::EVP_PKEY_CTX_set_hkdf_mode(self.as_ptr(), mode.0))?;
        }

        Ok(())
    }

    /// Sets the input material for HKDF generation as the "key".
    ///
    /// Which input is the key depends on the "mode" (see [`set_hkdf_mode`][Self::set_hkdf_mode]).
    /// If [`HkdfMode::EXTRACT_THEN_EXPAND`] or [`HkdfMode::EXTRACT_ONLY`], this function specifies
    /// the input keying material (IKM) for HKDF-Extract. If [`HkdfMode::EXPAND_ONLY`], it instead
    /// specifies the pseudorandom key (PRK) for HKDF-Expand.
    ///
    /// Requires OpenSSL 1.1.0 or newer.
    #[corresponds(EVP_PKEY_CTX_set1_hkdf_key)]
    #[cfg(any(ossl110, boringssl, libressl360, awslc))]
    #[inline]
    pub fn set_hkdf_key(&mut self, key: &[u8]) -> Result<(), ErrorStack> {
        #[cfg(not(any(boringssl, awslc)))]
        let len = c_int::try_from(key.len()).unwrap();
        #[cfg(any(boringssl, awslc))]
        let len = key.len();

        unsafe {
            cvt(ffi::EVP_PKEY_CTX_set1_hkdf_key(
                self.as_ptr(),
                key.as_ptr(),
                len,
            ))?;
        }

        Ok(())
    }

    /// Sets the salt value for HKDF generation.
    ///
    /// If performing HKDF-Expand only, this parameter is ignored.
    ///
    /// Requires OpenSSL 1.1.0 or newer.
    #[corresponds(EVP_PKEY_CTX_set1_hkdf_salt)]
    #[cfg(any(ossl110, boringssl, libressl360, awslc))]
    #[inline]
    pub fn set_hkdf_salt(&mut self, salt: &[u8]) -> Result<(), ErrorStack> {
        #[cfg(not(any(boringssl, awslc)))]
        let len = c_int::try_from(salt.len()).unwrap();
        #[cfg(any(boringssl, awslc))]
        let len = salt.len();

        unsafe {
            cvt(ffi::EVP_PKEY_CTX_set1_hkdf_salt(
                self.as_ptr(),
                salt.as_ptr(),
                len,
            ))?;
        }

        Ok(())
    }

    /// Appends info bytes for HKDF generation.
    ///
    /// If performing HKDF-Extract only, this parameter is ignored.
    ///
    /// Requires OpenSSL 1.1.0 or newer.
    #[corresponds(EVP_PKEY_CTX_add1_hkdf_info)]
    #[cfg(any(ossl110, boringssl, libressl360, awslc))]
    #[inline]
    pub fn add_hkdf_info(&mut self, info: &[u8]) -> Result<(), ErrorStack> {
        #[cfg(not(any(boringssl, awslc)))]
        let len = c_int::try_from(info.len()).unwrap();
        #[cfg(any(boringssl, awslc))]
        let len = info.len();

        unsafe {
            cvt(ffi::EVP_PKEY_CTX_add1_hkdf_info(
                self.as_ptr(),
                info.as_ptr(),
                len,
            ))?;
        }

        Ok(())
    }

    /// Derives a shared secret between two keys.
    ///
    /// If `buf` is set to `None`, an upper bound on the number of bytes required for the buffer will be returned.
    #[corresponds(EVP_PKEY_derive)]
    #[allow(unused_mut)]
    pub fn derive(&mut self, mut buf: Option<&mut [u8]>) -> Result<usize, ErrorStack> {
        // On OpenSSL 1.1.x some pmeths ignore *keylen and unconditionally
        // write the full natural output size (X25519, X448, HKDF-extract),
        // which can overflow a shorter caller-provided buffer. Others honor
        // *keylen by truncating the output (notably the default ECDH
        // EVP_PKEY_EC pmeth, where the OpenSSL source explicitly documents
        // that *keylen below the natural size "is not an error, the result
        // is truncated").
        //
        // We can't distinguish those two groups from the probe alone, so
        // when the probe reports a natural size larger than the caller's
        // buffer, derive into a temporary buffer of the probed size and
        // copy the leading bytes out. This prevents the OOB write for the
        // ignore-*keylen group and produces the same bytes for the
        // honor-*keylen group (ECDH_compute_key copies leading bytes of
        // the shared secret either way).
        //
        // Some pmeths (HKDF extract-and-expand and expand-only on 1.1.x)
        // don't support the NULL-out probe and fail it with an empty error
        // stack; those honor *keylen during derivation, so clear the
        // errors and proceed with the direct path. usize::MAX is a
        // sentinel some pmeths use when *keylen is caller-chosen.
        //
        // 3.0+ providers check the buffer size themselves, so this whole
        // dance is cfg-gated to 1.1.x and LibreSSL.
        #[cfg(any(all(ossl110, not(ossl300)), libressl))]
        {
            if let Some(b) = buf.as_deref_mut() {
                let mut required = 0;
                let probe_ok = unsafe {
                    ffi::EVP_PKEY_derive(self.as_ptr(), ptr::null_mut(), &mut required) == 1
                };
                if !probe_ok {
                    let _ = ErrorStack::get();
                } else if required != usize::MAX && b.len() < required {
                    let mut temp = vec![0u8; required];
                    let mut len = required;
                    unsafe {
                        cvt(ffi::EVP_PKEY_derive(
                            self.as_ptr(),
                            temp.as_mut_ptr(),
                            &mut len,
                        ))?;
                    }
                    let copy_len = b.len().min(len);
                    b[..copy_len].copy_from_slice(&temp[..copy_len]);
                    return Ok(copy_len);
                }
            }
        }
        let mut len = buf.as_ref().map_or(0, |b| b.len());
        unsafe {
            cvt(ffi::EVP_PKEY_derive(
                self.as_ptr(),
                buf.map_or(ptr::null_mut(), |b| b.as_mut_ptr()),
                &mut len,
            ))?;
        }

        Ok(len)
    }

    /// Like [`Self::derive`] but appends the secret to a [`Vec`].
    pub fn derive_to_vec(&mut self, buf: &mut Vec<u8>) -> Result<usize, ErrorStack> {
        let base = buf.len();
        let len = self.derive(None)?;
        buf.resize(base + len, 0);
        let len = self.derive(Some(&mut buf[base..]))?;
        buf.truncate(base + len);
        Ok(len)
    }

    /// Generates a new public/private keypair.
    #[corresponds(EVP_PKEY_keygen)]
    #[inline]
    pub fn keygen(&mut self) -> Result<PKey<Private>, ErrorStack> {
        unsafe {
            let mut key = ptr::null_mut();
            cvt(ffi::EVP_PKEY_keygen(self.as_ptr(), &mut key))?;
            Ok(PKey::from_ptr(key))
        }
    }

    /// Generates a new set of key parameters.
    #[corresponds(EVP_PKEY_paramgen)]
    #[inline]
    pub fn paramgen(&mut self) -> Result<PKey<Params>, ErrorStack> {
        unsafe {
            let mut key = ptr::null_mut();
            cvt(ffi::EVP_PKEY_paramgen(self.as_ptr(), &mut key))?;
            Ok(PKey::from_ptr(key))
        }
    }

    /// Sets the nonce type for a private key context.
    ///
    /// The nonce for DSA and ECDSA can be either random (the default) or deterministic (as defined by RFC 6979).
    ///
    /// This is only useful for DSA and ECDSA.
    /// Requires OpenSSL 3.2.0 or newer.
    #[cfg(ossl320)]
    #[corresponds(EVP_PKEY_CTX_set_params)]
    pub fn set_nonce_type(&mut self, nonce_type: NonceType) -> Result<(), ErrorStack> {
        let nonce_field_name = c"nonce-type";
        let mut nonce_type = nonce_type.0;
        unsafe {
            let param_nonce =
                ffi::OSSL_PARAM_construct_uint(nonce_field_name.as_ptr(), &mut nonce_type);
            let param_end = ffi::OSSL_PARAM_construct_end();

            let params = [param_nonce, param_end];
            cvt(ffi::EVP_PKEY_CTX_set_params(self.as_ptr(), params.as_ptr()))?;
        }
        Ok(())
    }

    /// Gets the nonce type for a private key context.
    ///
    /// The nonce for DSA and ECDSA can be either random (the default) or deterministic (as defined by RFC 6979).
    ///
    /// This is only useful for DSA and ECDSA.
    /// Requires OpenSSL 3.2.0 or newer.
    #[cfg(ossl320)]
    #[corresponds(EVP_PKEY_CTX_get_params)]
    pub fn nonce_type(&mut self) -> Result<NonceType, ErrorStack> {
        let nonce_field_name = c"nonce-type";
        let mut nonce_type: c_uint = 0;
        unsafe {
            let param_nonce =
                ffi::OSSL_PARAM_construct_uint(nonce_field_name.as_ptr(), &mut nonce_type);
            let param_end = ffi::OSSL_PARAM_construct_end();

            let mut params = [param_nonce, param_end];
            cvt(ffi::EVP_PKEY_CTX_get_params(
                self.as_ptr(),
                params.as_mut_ptr(),
            ))?;
        }
        Ok(NonceType(nonce_type))
    }

    /// Sets the context string for an ML-DSA signing or verification
    /// operation, as defined in FIPS 204 §5.
    ///
    /// Requires OpenSSL 3.5.0 or newer.
    #[cfg(ossl350)]
    #[corresponds(EVP_PKEY_CTX_set_params)]
    pub fn set_context_string(&mut self, context: &[u8]) -> Result<(), ErrorStack> {
        let mut builder = crate::ossl_param::OsslParamBuilder::new()?;
        builder.add_octet_string(c"context-string", context)?;
        let params = builder.to_param()?;
        unsafe {
            cvt(ffi::EVP_PKEY_CTX_set_params(self.as_ptr(), params.as_ptr()))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::bn::BigNum;
    #[cfg(not(any(boringssl, awslc)))]
    use crate::cipher::Cipher;
    use crate::ec::{EcGroup, EcKey};
    use crate::hash::{hash, MessageDigest};
    use crate::md::Md;
    #[cfg(ossl350)]
    use crate::md_ctx::MdCtx;
    use crate::nid::Nid;
    #[cfg(ossl350)]
    use crate::pkey::KeyType;
    use crate::pkey::PKey;
    use crate::rsa::Rsa;
    use crate::sign::Verifier;
    #[cfg(not(boringssl))]
    use cfg_if::cfg_if;

    #[test]
    fn rsa() {
        let key = include_bytes!("../test/rsa.pem");
        let rsa = Rsa::private_key_from_pem(key).unwrap();
        let pkey = PKey::from_rsa(rsa).unwrap();

        let mut ctx = PkeyCtx::new(&pkey).unwrap();
        ctx.encrypt_init().unwrap();
        ctx.set_rsa_padding(Padding::PKCS1).unwrap();

        let pt = "hello world".as_bytes();
        let mut ct = vec![];
        ctx.encrypt_to_vec(pt, &mut ct).unwrap();

        ctx.decrypt_init().unwrap();
        ctx.set_rsa_padding(Padding::PKCS1).unwrap();

        let mut out = vec![];
        ctx.decrypt_to_vec(&ct, &mut out).unwrap();

        assert_eq!(pt, out);
    }

    #[test]
    fn rsa_oaep() {
        let key = include_bytes!("../test/rsa.pem");
        let rsa = Rsa::private_key_from_pem(key).unwrap();
        let pkey = PKey::from_rsa(rsa).unwrap();

        let mut ctx = PkeyCtx::new(&pkey).unwrap();
        ctx.encrypt_init().unwrap();
        ctx.set_rsa_padding(Padding::PKCS1_OAEP).unwrap();
        ctx.set_rsa_oaep_md(Md::sha256()).unwrap();
        ctx.set_rsa_mgf1_md(Md::sha256()).unwrap();

        let pt = "hello world".as_bytes();
        let mut ct = vec![];
        ctx.encrypt_to_vec(pt, &mut ct).unwrap();

        ctx.decrypt_init().unwrap();
        ctx.set_rsa_padding(Padding::PKCS1_OAEP).unwrap();
        ctx.set_rsa_oaep_md(Md::sha256()).unwrap();
        ctx.set_rsa_mgf1_md(Md::sha256()).unwrap();

        let mut out = vec![];
        ctx.decrypt_to_vec(&ct, &mut out).unwrap();

        assert_eq!(pt, out);
    }

    #[test]
    fn rsa_sign() {
        let key = include_bytes!("../test/rsa.pem");
        let rsa = Rsa::private_key_from_pem(key).unwrap();
        let pkey = PKey::from_rsa(rsa).unwrap();

        let mut ctx = PkeyCtx::new(&pkey).unwrap();
        ctx.sign_init().unwrap();
        ctx.set_rsa_padding(Padding::PKCS1).unwrap();
        ctx.set_signature_md(Md::sha384()).unwrap();

        let msg = b"hello world";
        let digest = hash(MessageDigest::sha384(), msg).unwrap();
        let mut signature = vec![];
        ctx.sign_to_vec(&digest, &mut signature).unwrap();

        let mut verifier = Verifier::new(MessageDigest::sha384(), &pkey).unwrap();
        verifier.update(msg).unwrap();
        assert!(matches!(verifier.verify(&signature), Ok(true)));
    }

    #[test]
    fn rsa_sign_pss() {
        let key = include_bytes!("../test/rsa.pem");
        let rsa = Rsa::private_key_from_pem(key).unwrap();
        let pkey = PKey::from_rsa(rsa).unwrap();

        let mut ctx = PkeyCtx::new(&pkey).unwrap();
        ctx.sign_init().unwrap();
        ctx.set_rsa_padding(Padding::PKCS1_PSS).unwrap();
        ctx.set_signature_md(Md::sha384()).unwrap();
        ctx.set_rsa_pss_saltlen(RsaPssSaltlen::custom(14)).unwrap();

        let msg = b"hello world";
        let digest = hash(MessageDigest::sha384(), msg).unwrap();
        let mut signature = vec![];
        ctx.sign_to_vec(&digest, &mut signature).unwrap();

        let mut verifier = Verifier::new(MessageDigest::sha384(), &pkey).unwrap();
        verifier.set_rsa_padding(Padding::PKCS1_PSS).unwrap();
        verifier
            .set_rsa_pss_saltlen(RsaPssSaltlen::custom(14))
            .unwrap();
        verifier.update(msg).unwrap();
        assert!(matches!(verifier.verify(&signature), Ok(true)));
    }

    #[test]
    fn derive() {
        let group = EcGroup::from_curve_name(Nid::X9_62_PRIME256V1).unwrap();
        let key1 = EcKey::generate(&group).unwrap();
        let key1 = PKey::from_ec_key(key1).unwrap();
        let key2 = EcKey::generate(&group).unwrap();
        let key2 = PKey::from_ec_key(key2).unwrap();

        let mut ctx = PkeyCtx::new(&key1).unwrap();
        ctx.derive_init().unwrap();
        ctx.derive_set_peer(&key2).unwrap();

        let mut buf = vec![];
        ctx.derive_to_vec(&mut buf).unwrap();
    }

    #[test]
    #[cfg(any(ossl111, libressl370))]
    fn derive_undersized_buffer() {
        // Without the temp-buffer fallback in this crate, X25519 on 1.1.x
        // would OOB into a 4-byte buffer because it ignores *keylen.
        // On 1.1.x / LibreSSL the fallback kicks in and we return the
        // truncated prefix. On 3.0+ the provider rejects undersized
        // buffers before any write happens, so the call errors out.
        let key1 = PKey::generate_x25519().unwrap();
        let key2 = PKey::generate_x25519().unwrap();

        let mut ctx = PkeyCtx::new(&key1).unwrap();
        ctx.derive_init().unwrap();
        ctx.derive_set_peer(&key2).unwrap();

        let mut buf = [0u8; 4];
        let result = ctx.derive(Some(&mut buf));
        #[cfg(any(all(ossl110, not(ossl300)), libressl))]
        assert_eq!(result.unwrap(), 4);
        #[cfg(all(ossl300, not(libressl)))]
        assert!(result.is_err());
    }

    #[test]
    #[cfg(not(any(boringssl, awslc)))]
    fn cmac_keygen() {
        let mut ctx = PkeyCtx::new_id(Id::CMAC).unwrap();
        ctx.keygen_init().unwrap();
        ctx.set_keygen_cipher(Cipher::aes_128_cbc()).unwrap();
        ctx.set_keygen_mac_key(&hex::decode("9294727a3638bb1c13f48ef8158bfc9d").unwrap())
            .unwrap();
        ctx.keygen().unwrap();
    }

    #[test]
    #[cfg(not(boringssl))]
    fn dh_paramgen() {
        let mut ctx = PkeyCtx::new_id(Id::DH).unwrap();
        ctx.paramgen_init().unwrap();
        ctx.set_dh_paramgen_prime_len(512).unwrap();
        ctx.set_dh_paramgen_generator(2).unwrap();
        let params = ctx.paramgen().unwrap();

        assert_eq!(params.size(), 64);
    }

    #[test]
    #[cfg(not(boringssl))]
    fn dsa_paramgen() {
        let mut ctx = PkeyCtx::new_id(Id::DSA).unwrap();
        ctx.paramgen_init().unwrap();
        ctx.set_dsa_paramgen_bits(2048).unwrap();
        let params = ctx.paramgen().unwrap();

        let size = {
            cfg_if! {
                if #[cfg(awslc)] {
                    72
                } else if #[cfg(libressl)] {
                    48
                } else {
                    64
                }
            }
        };
        assert_eq!(params.size(), size);
    }

    #[test]
    fn ec_keygen() {
        let mut ctx = PkeyCtx::new_id(Id::EC).unwrap();
        ctx.paramgen_init().unwrap();
        ctx.set_ec_paramgen_curve_nid(Nid::X9_62_PRIME256V1)
            .unwrap();
        let params = ctx.paramgen().unwrap();

        assert_eq!(params.size(), 72);
    }

    #[test]
    fn rsa_keygen() {
        let pubexp = BigNum::from_u32(65537).unwrap();
        let mut ctx = PkeyCtx::new_id(Id::RSA).unwrap();
        ctx.keygen_init().unwrap();
        ctx.set_rsa_keygen_pubexp(&pubexp).unwrap();
        ctx.set_rsa_keygen_bits(2048).unwrap();
        let key = ctx.keygen().unwrap();

        assert_eq!(key.bits(), 2048);
    }

    #[test]
    #[cfg(any(ossl110, boringssl, libressl360, awslc))]
    fn hkdf() {
        let mut ctx = PkeyCtx::new_id(Id::HKDF).unwrap();
        ctx.derive_init().unwrap();
        ctx.set_hkdf_md(Md::sha256()).unwrap();
        ctx.set_hkdf_key(&hex::decode("0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b").unwrap())
            .unwrap();
        ctx.set_hkdf_salt(&hex::decode("000102030405060708090a0b0c").unwrap())
            .unwrap();
        ctx.add_hkdf_info(&hex::decode("f0f1f2f3f4f5f6f7f8f9").unwrap())
            .unwrap();
        let mut out = [0; 42];
        ctx.derive(Some(&mut out)).unwrap();

        assert_eq!(
            &out[..],
            hex::decode("3cb25f25faacd57a90434f64d0362f2a2d2d0a90cf1a5a4c5db02d56ecc4c5bf34007208d5b887185865")
                .unwrap()
        );
    }

    #[test]
    #[cfg(any(ossl111, libressl360))]
    fn hkdf_expand() {
        let mut ctx = PkeyCtx::new_id(Id::HKDF).unwrap();
        ctx.derive_init().unwrap();
        ctx.set_hkdf_mode(HkdfMode::EXPAND_ONLY).unwrap();
        ctx.set_hkdf_md(Md::sha256()).unwrap();
        ctx.set_hkdf_key(
            &hex::decode("077709362c2e32df0ddc3f0dc47bba6390b6c73bb50f9c3122ec844ad7c2b3e5")
                .unwrap(),
        )
        .unwrap();
        ctx.add_hkdf_info(&hex::decode("f0f1f2f3f4f5f6f7f8f9").unwrap())
            .unwrap();
        let mut out = [0; 42];
        ctx.derive(Some(&mut out)).unwrap();

        assert_eq!(
            &out[..],
            hex::decode("3cb25f25faacd57a90434f64d0362f2a2d2d0a90cf1a5a4c5db02d56ecc4c5bf34007208d5b887185865")
                .unwrap()
        );
    }

    #[test]
    #[cfg(any(ossl111, libressl360))]
    fn hkdf_extract() {
        let mut ctx = PkeyCtx::new_id(Id::HKDF).unwrap();
        ctx.derive_init().unwrap();
        ctx.set_hkdf_mode(HkdfMode::EXTRACT_ONLY).unwrap();
        ctx.set_hkdf_md(Md::sha256()).unwrap();
        ctx.set_hkdf_key(&hex::decode("0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b").unwrap())
            .unwrap();
        ctx.set_hkdf_salt(&hex::decode("000102030405060708090a0b0c").unwrap())
            .unwrap();
        let mut out = vec![];
        ctx.derive_to_vec(&mut out).unwrap();

        assert_eq!(
            &out[..],
            hex::decode("077709362c2e32df0ddc3f0dc47bba6390b6c73bb50f9c3122ec844ad7c2b3e5")
                .unwrap()
        );
    }

    #[test]
    fn verify_fail() {
        let key1 = Rsa::generate(4096).unwrap();
        let key1 = PKey::from_rsa(key1).unwrap();

        let data = b"Some Crypto Text";

        let mut ctx = PkeyCtx::new(&key1).unwrap();
        ctx.sign_init().unwrap();
        let mut signature = vec![];
        ctx.sign_to_vec(data, &mut signature).unwrap();

        let bad_data = b"Some Crypto text";

        ctx.verify_init().unwrap();
        let valid = ctx.verify(bad_data, &signature);
        assert!(matches!(valid, Ok(false) | Err(_)));
        assert!(ErrorStack::get().errors().is_empty());
    }

    #[test]
    fn verify_fail_ec() {
        let key1 =
            EcKey::generate(&EcGroup::from_curve_name(Nid::X9_62_PRIME256V1).unwrap()).unwrap();
        let key1 = PKey::from_ec_key(key1).unwrap();

        let data = b"Some Crypto Text";
        let mut ctx = PkeyCtx::new(&key1).unwrap();
        ctx.verify_init().unwrap();
        assert!(matches!(ctx.verify(data, &[0; 64]), Ok(false) | Err(_)));
        assert!(ErrorStack::get().errors().is_empty());
    }

    #[test]
    fn test_verify_recover() {
        let key = Rsa::generate(2048).unwrap();
        let key = PKey::from_rsa(key).unwrap();

        let digest = [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ];

        let mut ctx = PkeyCtx::new(&key).unwrap();
        ctx.sign_init().unwrap();
        ctx.set_rsa_padding(Padding::PKCS1).unwrap();
        ctx.set_signature_md(Md::sha256()).unwrap();
        let mut signature = vec![];
        ctx.sign_to_vec(&digest, &mut signature).unwrap();

        // Attempt recovery of just the digest.
        let mut ctx = PkeyCtx::new(&key).unwrap();
        ctx.verify_recover_init().unwrap();
        ctx.set_rsa_padding(Padding::PKCS1).unwrap();
        ctx.set_signature_md(Md::sha256()).unwrap();
        let length = ctx.verify_recover(&signature, None).unwrap();
        let mut result_buf = vec![0; length];
        let length = ctx
            .verify_recover(&signature, Some(&mut result_buf))
            .unwrap();
        assert_eq!(length, digest.len());
        // result_buf contains the digest
        assert_eq!(result_buf[..length], digest);

        // Attempt recovery of the entire DigestInfo
        let mut ctx = PkeyCtx::new(&key).unwrap();
        ctx.verify_recover_init().unwrap();
        ctx.set_rsa_padding(Padding::PKCS1).unwrap();
        let length = ctx.verify_recover(&signature, None).unwrap();
        let mut result_buf = vec![0; length];
        let length = ctx
            .verify_recover(&signature, Some(&mut result_buf))
            .unwrap();
        // 32-bytes of SHA256 digest + the ASN.1 DigestInfo structure == 51 bytes
        assert_eq!(length, 51);
        // The digest is the end of the DigestInfo structure.
        assert_eq!(result_buf[length - digest.len()..length], digest);
    }

    #[test]
    #[cfg(ossl320)]
    fn set_nonce_type() {
        let key1 =
            EcKey::generate(&EcGroup::from_curve_name(Nid::X9_62_PRIME256V1).unwrap()).unwrap();
        let key1 = PKey::from_ec_key(key1).unwrap();

        let mut ctx = PkeyCtx::new(&key1).unwrap();
        ctx.sign_init().unwrap();
        ctx.set_nonce_type(NonceType::DETERMINISTIC_K).unwrap();
        let nonce_type = ctx.nonce_type().unwrap();
        assert_eq!(nonce_type, NonceType::DETERMINISTIC_K);
        assert!(ErrorStack::get().errors().is_empty());
    }

    // Test vector from
    // https://github.com/openssl/openssl/blob/openssl-3.2.0/test/recipes/30-test_evp_data/evppkey_ecdsa_rfc6979.txt
    #[test]
    #[cfg(ossl320)]
    fn ecdsa_deterministic_signature() {
        let private_key_pem = "-----BEGIN PRIVATE KEY-----
MEECAQAwEwYHKoZIzj0CAQYIKoZIzj0DAQcEJzAlAgEBBCDJr6nYRbp1FmtcIVdnsdaTTlDD2zbo
mxJ7imIrEg9nIQ==
-----END PRIVATE KEY-----";

        let key1 = EcKey::private_key_from_pem(private_key_pem.as_bytes()).unwrap();
        let key1 = PKey::from_ec_key(key1).unwrap();
        let input = "sample";
        let expected_output = hex::decode("3046022100EFD48B2AACB6A8FD1140DD9CD45E81D69D2C877B56AAF991C34D0EA84EAF3716022100F7CB1C942D657C41D436C7A1B6E29F65F3E900DBB9AFF4064DC4AB2F843ACDA8").unwrap();

        let hashed_input = hash(MessageDigest::sha256(), input.as_bytes()).unwrap();
        let mut ctx = PkeyCtx::new(&key1).unwrap();
        ctx.sign_init().unwrap();
        ctx.set_signature_md(Md::sha256()).unwrap();
        ctx.set_nonce_type(NonceType::DETERMINISTIC_K).unwrap();

        let mut output = vec![];
        ctx.sign_to_vec(&hashed_input, &mut output).unwrap();
        assert_eq!(output, expected_output);
        assert!(ErrorStack::get().errors().is_empty());
    }

    #[test]
    #[cfg(ossl350)]
    fn set_context_string_mldsa() {
        // ML-DSA-65 KAT vector 0 from
        // https://github.com/post-quantum-cryptography/KAT/blob/main/MLDSA/kat_MLDSA_65_det_pure.rsp
        let xi = hex::decode("f696484048ec21f96cf50a56d0759c448f3779752f0383d37449690694cf7a68")
            .unwrap();
        let msg = hex::decode("6dbbc4375136df3b07f7c70e639e223e").unwrap();
        let ctx = hex::decode("480c658c0cb3e040bde084345cef0df7").unwrap();
        let expected_sig = hex::decode("0e9e903efe9d51ef5356b9794e52cf55cbfde58596f7e81faafe2921928279aab3ec9ba9a072c2c68adc72f0c2946fd4f5adb4c722a2f0e55676f1c79cbdc1d8afd19cf86156b5f7f8d012cd8245fb03fe30c62ea63110a722782367983030afadd16391b345f00d9c6d9750ae4b7099d5fcb66b944a213cf4881fa0f806e293554e869205e3e8e9abe67d0749d8c72fb882d3f3d1512f3077845787050527bde82da3cfc8ae9d0ab7827281a14f2bf6131cb79d27f4c781d0e8e42da9fdecb5be1979c9ccde17f1cc5b94f2f8e0a93d3e085ec72e221ca7d6bdad712a2c649b7bfdd0ad02306dd49cbdc298d4b23e73db05aa6a8b1514396e3032fe512d8cf58c5b61a546e1de56d98ad98c6d6a81db9865566a7cad4d533cb1d7770f2807819bb436e89045f8ac363aa6af27b5b7488d2303b10ba6e80e61ec0b8ae5850c96edeaf97ed2ebe64f77df8708c31bc6f6d4598dfc7c7ddb12b16b85adac7d8f1c2d961983ca2db804e3062c8e94d37c97181abfa111529adf15ed04dc403e9758d545cee70e868c222ed724d0f9483d7d2c5c44ad28662382ec40b7949ce08a6acb69d05a90d52ce487ce8fb6b5e5b91a2187d021ece47700f2f8eb5d2eb8e86000f14b978a051009c54ab1f1b90daa1d0d6711249d766d91a5f8c8604568bfc2f52822c52d625f451c194a5cfa4151b4fda81873e5f444144344eb75410033e2ba3d77850cae4b34495943a3880e273bb8842295ad853d1c7055a290e5ca084ba993c18810a85392ba042b7efe1e002c923723a43cc7b2dedca6123beebc4968ce42bb10d19111b3f77f519c19e9fc8111616dfd53b01eee401af9530568ffe98164e2d2e220a66422efb00bc90b07e48804c67db44f79e0595ad4a31e9b760c01e9fc239fabf55837316d7a13fd069f86563936e7b61ccb7d0dee4bd93e6b129c7da92a4516b7fa263cb6bf5940228312973b442f305dcfc40f843be2da8d693450f9ecf339bc9419f543a2fa0d4dc7793aced11d8e50d68003c2bd0212a7af1a5cfd7e8e54f14d8c627b3cd9558fbab33bf6209e485e59a9c9feecc54caa2dfece838b97b313c169cf27404cfc6dc5174856300208e10244183b2fcd28d34d67228eb9611b8f739a112178dc143fda2b8e64417e0f4c18f875eff34ec80a0fe04b5016473385a3e9c7b863b49ca05ebc3a34724a99488eba3dd49608da335a2acda60a91f3b8fb04ca30de481c0e1f17af2f13227b183345250b523257e08b58e15eb7b22a82ce581e63310a35cffb159ec276ecf3fac1ccefc23389de6c29b7e4454987611b315fb7684c72376ba448161cd82bc6a924095097c897b414750df37d34bbd2844979fbbdf68110afc3e14cce7cf963c375c92dcf65e4d633093bb8094052f8e9e6c4f3f650e9082f7346867794474a89f74e543714a5ad03ba003ffc1c716d56fb6078798e68ad4072d7db5099d0eb2c7ffdf21bcb4308965c13e0e1bd36bfab7c4267fa0c02ac393e70883b734027b9b087dabcd2c9bb65d9f769eae934c2b0415b2a5f2d7933e259ba51b2bd82fc08aa496e5fd3c4c0668e432dda489b4b9626006a2a958c0758624544279336421e92c477b09b968137686b68c5fbee84629b2c6b5506ad42198eefe6dfed5273fd74387faec972120aeaf85bec1491293362ebb2bda24eb7b3632c0950cf16f793fb333ca0025f57ce51b141cdc34456bfce530b417b00b574cbd4d13be8ef48fd3326948f7adc5b1f0398fcf0281e040407b3f9b7f79e4f1e89fb5eca34c9b387b0db3a5e98e253b999cd4c1616fd7e3fd26aa896c2118fe1e92499ddf5ff1125377a59987e84e03181262f7ad843b61b6450d92372cbcac16ba5b9b8d8deee7781f841672d91f336ce220fbfff7e32d61c98f5249e67bfef35edc3d9c3630b4f956c44c4a3e93f851c7775b64d759f2632b5ee38f0f3fff016a1fd3999b50c4f1de82cec0f11c34e494d51ad34df7bd812899daec5dfbd1b8405baf261e1da7898ba1dc1ced8afa4c23d662859d8a68205b62b6f908e2411d3a125e71442f74fc08b02e65195a63f198e1cb6c408e6a1b157d2a4b85a792cac2a07ddaea79c5858ce0ebdcdeab199483e2367f9b4785344447dca9251eb3df906cf4934228c4c028495f00e130722e175243791680bf540ca60e4c17c910637f6db35c1670d5c6d7e8264fa97fa540c870c87d1aa441599ee6fe6c22eabf8fc49c824b3bfac6491a2349ea33c4cb6cb19c0facb7d0cdb62db2ba56a5c337f9fe7b4ab5936275f98fe487b6c4805be709f67456caebd3043737a5974b0ca932b19c435bed01010c4617d350d803ff22dbbb3442189d9f81a0e5b47edfa4ea0a832fb14d27b19be226b42910ed10ac2d404abd96262bde3c391b238fa9bcfd3734275dacfc411811e01b1178750c745fe9462740c3f91f130535a7feb6be25b34a3314666ebaef01f5ec8b656add9b87e59c5bae71153e91be21505f27246035ec6c2569d7f165723a169cda18aaadb1481d84976da019a149b02f5b1f9b8c090a8115ecb3be65423aa1461e35c804a411b1cb191a8734233ca7a42acaee8eb4f451aa3089b9542a3c65e92a3a74b14443d1a8d72846fa7846098ec3233d2553ed4ab542c8c425cbabb94bb2fdb5223acb999ba31a19a0ff9b646fc4297832d36722e0af0c03899c8ce17c79303b091512f0106b4285e6e8faf09325f0bc3d8d724575c9556931a4dc34c93d81a79f25199ed712a97309630b5635ae067edd4e17be9d4e194905f8d3c7c6ad93bd219beac7c389f62f8adec30a72bcbc15e8586b9e2b057140eb95fadd960389230edb3925dd9a7deb86f72502ffcebbe9a65a99e6cecf61823f9cd326f298b9430a0f476e57093895e26b0375c327b29c3e37f14474af815dd90cfbc8e61fc2f0b528b3db0e5a5407db99c3c5c2862e3b68084305e93c79f1b962a6a5d28c76969e1d5d1e82ad831ee1b6238208890db7006b395771e34c1a0d25ebf651273ba3d495417033163b46f6a176b79c446c92ebbed4bc84f0e1effc0b38d3e269cd39f392472f1634acd1bd178a8ea5a54e045c45e19d54bc286e9fb0e324c30e00593529aa28f7b66288144ba6db9ff9f368565944bf776e93e27741fd7ded89144bad8a3c1287498973d7cd8d883fe3012db6af76e9ee1dfc189e1e7f97f1e23b288c5776de731b286a80fa349b09db359a5cfafc83c46de3b43cfe9e96270e18749a3af460d1312c4eb058935f13eaa29e8836877a88734356a816b319178776310086551003227f0b191aa6058cf8209de79cf106d57c4b769f142054ccc16b0348bf620f1a28dca8ff02e11fdae7cd155383b59e4acdfc53795786b759b5a03abb1b25adccf711e30ce1b783dbf75421d6ca5e146c6deca0da3078575c936063131c14f57ec4eef5fa5047f3e3be6fe2d3940878f905ee5c7e46cfa43575b64b6a5f71eb715e2cf1aa514df138234317b401d57f1a61ff08a0eedaa6b4370e61003532fffabfbe368eb465230b7f25c80bf73ae239b9deb8ce54b64bfc06820c01507aa2d9a8a6a9f5ed2d8912b4e1b43192baee1e10df1c15c88f7fb4381cac124848b55549239e2f1acc7a4352c4aeb56307d670ea52217b272345effaba37c04dccf9f4d8a9c89e2d51ee3cfd8a53e369256800af40e230d2c554114e99db275e8f8dc34ad6f2ecca886e018bce9822041a9196d4efe9ec952ae2ff4fc8256084923075295a82034909942c87b64e5bb941eae7de6cba2540c594b3f1f4a24d6fc25d1182a88b9e6dc4616a3a76d03523e8c34c71684d8c5e5be134402df6153e9230f6e013ec6131e5af35a4b8c5aa227b2452cc8232bbc904d4b822eeddf96c89b16af80856afe9d5a66f914b81b2d52efc451e6f3d92cfba8070a9b3957e74db749c1b94dc5acd950b6a553909229fd916a1cfd56701f57a0971224070eb24d3718a665dda9b4ca9171ffbdfea55ef194f91ecc61d04cf5068cb3b7e481073b9396e8ec349e5257db91f0f00d9df9d3b465ca93c15c9efefb9aed057343eb169b206aa0db60db319801899a2b6527e1f97263444599b555dab700dffc302d6767385f982c78b4932ca12a16d575752b915b034dd69f3473c52097fc6078dd7ef6462b820bee65980510367235191d900033f13b7e2bd43cd423fbed47aa54377f67be5583c1c385c659449bf202032502d2db40101cdb5543ddfcea652c5b3c711d932c2c22ec67256b194e5bcab3ae054d02b22e7193c7c32aa3a958822cd5917117f53569ad42a5b30a64781355d22ff3bb70b76430b7e8e538df4c5411a09e6fecc4086d86aebfe497315056d11660165b47383302d6461a7f0f2d0c6e894c33c71c1c4eaa715d55ec369bbf7d86623a7b2b76bed3b1356ed80f8a1ee77229ab6cddd96eaa724fa740b480ae6fb1768c67adc5ce85dcf0f46026fdcdb4f346c60c2478f6da4b33fc06a0645fb5629b80e96d6f78aa4ae1c5cefefb24912de843f509e9609fc969be9a66c28d5608d6bcdc6bc0e6ca5752142e196679935fc745f1a84fed03d47276e29ffefa61b4232b737583c1c2e93752a7acbac2f747698b9b023a4000122a706c758fd300000000000000000000000000000000000000000000000000080f13161a1e").unwrap();

        let key = PKey::private_key_from_seed(None, KeyType::ML_DSA_65, None, &xi).unwrap();

        // Verify the KAT signature against the matching context string.
        let mut md_ctx = MdCtx::new().unwrap();
        let pkey_ctx = md_ctx.digest_verify_init(None, &key).unwrap();
        pkey_ctx.set_context_string(&ctx).unwrap();
        assert!(md_ctx.digest_verify(&msg, &expected_sig).unwrap());

        // Sign and verify roundtrip with the same context string.
        let mut md_ctx = MdCtx::new().unwrap();
        let pkey_ctx = md_ctx.digest_sign_init(None, &key).unwrap();
        pkey_ctx.set_context_string(&ctx).unwrap();
        let mut sig = vec![];
        md_ctx.digest_sign_to_vec(&msg, &mut sig).unwrap();

        let mut md_ctx = MdCtx::new().unwrap();
        let pkey_ctx = md_ctx.digest_verify_init(None, &key).unwrap();
        pkey_ctx.set_context_string(&ctx).unwrap();
        assert!(md_ctx.digest_verify(&msg, &sig).unwrap());

        assert!(ErrorStack::get().errors().is_empty());
    }
}

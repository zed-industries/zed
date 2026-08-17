// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

use crate::aws_lc::{
    EVP_DigestSign, EVP_DigestSignInit, EVP_DigestVerify, EVP_DigestVerifyInit, EVP_PKEY_CTX_new,
    EVP_PKEY_CTX_new_id, EVP_PKEY_bits, EVP_PKEY_cmp, EVP_PKEY_derive, EVP_PKEY_derive_init,
    EVP_PKEY_derive_set_peer, EVP_PKEY_get0_EC_KEY, EVP_PKEY_get0_RSA,
    EVP_PKEY_get_raw_private_key, EVP_PKEY_get_raw_public_key, EVP_PKEY_id, EVP_PKEY_keygen,
    EVP_PKEY_keygen_init, EVP_PKEY_new_raw_private_key, EVP_PKEY_new_raw_public_key, EVP_PKEY_sign,
    EVP_PKEY_sign_init, EVP_PKEY_size, EVP_PKEY_up_ref, EVP_PKEY_verify, EVP_PKEY_verify_init,
    EVP_marshal_private_key, EVP_marshal_private_key_v2, EVP_marshal_public_key,
    EVP_parse_private_key, EVP_parse_public_key, EC_KEY, EVP_PKEY, EVP_PKEY_CTX, EVP_PKEY_ED25519,
    RSA,
};
#[cfg(all(feature = "unstable", not(feature = "fips")))]
use crate::aws_lc::{
    EVP_PKEY_pqdsa_new_raw_private_key, EVP_PKEY_pqdsa_new_raw_public_key, EVP_PKEY_PQDSA,
    NID_MLDSA44, NID_MLDSA65, NID_MLDSA87,
};
use crate::cbb::LcCBB;
use crate::digest::digest_ctx::DigestContext;
use crate::digest::Digest;
use crate::error::{KeyRejected, Unspecified};
use crate::fips::indicator_check;
use crate::pkcs8::Version;
use crate::ptr::{ConstPointer, LcPtr};
use crate::{cbs, digest};
use core::ffi::c_int;
use std::ptr::{null, null_mut};

impl PartialEq<Self> for LcPtr<EVP_PKEY> {
    /// Only compares params and public key
    fn eq(&self, other: &Self) -> bool {
        // EVP_PKEY_cmp only compares params and public key
        1 == unsafe { EVP_PKEY_cmp(self.as_const_ptr(), other.as_const_ptr()) }
    }
}

#[allow(non_camel_case_types)]
pub(crate) trait EVP_PKEY_CTX_consumer: Fn(*mut EVP_PKEY_CTX) -> Result<(), ()> {}

impl<T> EVP_PKEY_CTX_consumer for T where T: Fn(*mut EVP_PKEY_CTX) -> Result<(), ()> {}

#[allow(non_upper_case_globals, clippy::type_complexity)]
pub(crate) const No_EVP_PKEY_CTX_consumer: Option<fn(*mut EVP_PKEY_CTX) -> Result<(), ()>> = None;

impl ConstPointer<'_, EVP_PKEY> {
    pub(crate) fn validate_as_ed25519(&self) -> Result<(), KeyRejected> {
        const ED25519_KEY_TYPE: c_int = EVP_PKEY_ED25519;
        const ED25519_MIN_BITS: c_int = 253;
        const ED25519_MAX_BITS: c_int = 256;

        let key_type = self.id();
        if key_type != ED25519_KEY_TYPE {
            return Err(KeyRejected::wrong_algorithm());
        }

        let bits: c_int = self.key_size_bits().try_into().unwrap();
        if bits < ED25519_MIN_BITS {
            return Err(KeyRejected::too_small());
        }

        if bits > ED25519_MAX_BITS {
            return Err(KeyRejected::too_large());
        }
        Ok(())
    }

    // EVP_PKEY_NONE = 0;
    // EVP_PKEY_RSA = 6;
    // EVP_PKEY_RSA_PSS = 912;
    // EVP_PKEY_DSA = 116;
    // EVP_PKEY_EC = 408;
    // EVP_PKEY_ED25519 = 949;
    // EVP_PKEY_X25519 = 948;
    // EVP_PKEY_KYBER512 = 970;
    // EVP_PKEY_HKDF = 969;
    // EVP_PKEY_DH = 28;
    // EVP_PKEY_RSA2 = 19;
    // EVP_PKEY_X448 = 961;
    // EVP_PKEY_ED448 = 960;
    pub(crate) fn id(&self) -> i32 {
        unsafe { EVP_PKEY_id(self.as_const_ptr()) }
    }

    pub(crate) fn key_size_bytes(&self) -> usize {
        self.key_size_bits() / 8
    }

    pub(crate) fn key_size_bits(&self) -> usize {
        unsafe { EVP_PKEY_bits(self.as_const_ptr()) }
            .try_into()
            .unwrap()
    }

    pub(crate) fn signature_size_bytes(&self) -> usize {
        unsafe { EVP_PKEY_size(self.as_const_ptr()) }
            .try_into()
            .unwrap()
    }

    #[allow(dead_code)]
    pub(crate) fn get_ec_key(&self) -> Result<ConstPointer<'_, EC_KEY>, KeyRejected> {
        self.project_const_lifetime(unsafe {
            |evp_pkey| EVP_PKEY_get0_EC_KEY(evp_pkey.as_const_ptr())
        })
        .map_err(|()| KeyRejected::wrong_algorithm())
    }

    pub(crate) fn get_rsa(&self) -> Result<ConstPointer<'_, RSA>, KeyRejected> {
        self.project_const_lifetime(unsafe {
            |evp_pkey| EVP_PKEY_get0_RSA(evp_pkey.as_const_ptr())
        })
        .map_err(|()| KeyRejected::wrong_algorithm())
    }

    pub(crate) fn marshal_rfc5280_public_key(&self) -> Result<Vec<u8>, Unspecified> {
        // Data shows that the SubjectPublicKeyInfo is roughly 356% to 375% increase in size compared to the RSA key
        // size in bytes for keys ranging from 2048-bit to 4096-bit. So size the initial capacity to be roughly
        // 500% as a conservative estimate to avoid needing to reallocate for any key in that range.
        let mut cbb = LcCBB::new(self.key_size_bytes() * 5);
        if 1 != unsafe { EVP_marshal_public_key(cbb.as_mut_ptr(), self.as_const_ptr()) } {
            return Err(Unspecified);
        }
        cbb.into_vec()
    }

    pub(crate) fn marshal_rfc5208_private_key(
        &self,
        version: Version,
    ) -> Result<Vec<u8>, Unspecified> {
        let key_size_bytes =
            TryInto::<usize>::try_into(unsafe { EVP_PKEY_bits(self.as_const_ptr()) })
                .expect("fit in usize")
                / 8;
        let mut cbb = LcCBB::new(key_size_bytes * 5);
        match version {
            Version::V1 => {
                if 1 != unsafe { EVP_marshal_private_key(cbb.as_mut_ptr(), self.as_const_ptr()) } {
                    return Err(Unspecified);
                }
            }
            Version::V2 => {
                if 1 != unsafe { EVP_marshal_private_key_v2(cbb.as_mut_ptr(), self.as_const_ptr()) }
                {
                    return Err(Unspecified);
                }
            }
        }
        cbb.into_vec()
    }

    pub(crate) fn marshal_raw_private_key(&self) -> Result<Vec<u8>, Unspecified> {
        let mut size = 0;
        if 1 != unsafe { EVP_PKEY_get_raw_private_key(self.as_const_ptr(), null_mut(), &mut size) }
        {
            return Err(Unspecified);
        }
        let mut buffer = vec![0u8; size];
        let buffer_size = self.marshal_raw_private_to_buffer(&mut buffer)?;
        debug_assert_eq!(buffer_size, size);
        Ok(buffer)
    }

    pub(crate) fn marshal_raw_private_to_buffer(
        &self,
        buffer: &mut [u8],
    ) -> Result<usize, Unspecified> {
        let mut key_len = buffer.len();
        if 1 == unsafe {
            EVP_PKEY_get_raw_private_key(self.as_const_ptr(), buffer.as_mut_ptr(), &mut key_len)
        } {
            Ok(key_len)
        } else {
            Err(Unspecified)
        }
    }

    #[allow(dead_code)]
    pub(crate) fn marshal_raw_public_key(&self) -> Result<Vec<u8>, Unspecified> {
        let mut size = 0;
        if 1 != unsafe { EVP_PKEY_get_raw_public_key(self.as_const_ptr(), null_mut(), &mut size) } {
            return Err(Unspecified);
        }
        let mut buffer = vec![0u8; size];
        let buffer_size = self.marshal_raw_public_to_buffer(&mut buffer)?;
        debug_assert_eq!(buffer_size, size);
        Ok(buffer)
    }

    pub(crate) fn marshal_raw_public_to_buffer(
        &self,
        buffer: &mut [u8],
    ) -> Result<usize, Unspecified> {
        let mut key_len = buffer.len();
        if 1 == unsafe {
            // `EVP_PKEY_get_raw_public_key` writes the total length
            // to `encapsulate_key_size` in the event that the buffer we provide is larger then
            // required.
            EVP_PKEY_get_raw_public_key(self.as_const_ptr(), buffer.as_mut_ptr(), &mut key_len)
        } {
            Ok(key_len)
        } else {
            Err(Unspecified)
        }
    }
}

impl LcPtr<EVP_PKEY> {
    #[inline]
    pub unsafe fn as_mut_unsafe_ptr(&self) -> *mut EVP_PKEY {
        self.as_const_ptr().cast_mut()
    }

    pub(crate) fn parse_rfc5280_public_key(
        bytes: &[u8],
        evp_pkey_type: c_int,
    ) -> Result<Self, KeyRejected> {
        let mut cbs = cbs::build_CBS(bytes);
        // Also checks the validity of the key
        let evp_pkey = LcPtr::new(unsafe { EVP_parse_public_key(&mut cbs) })
            .map_err(|()| KeyRejected::invalid_encoding())?;
        evp_pkey
            .as_const()
            .id()
            .eq(&evp_pkey_type)
            .then_some(evp_pkey)
            .ok_or(KeyRejected::wrong_algorithm())
    }

    pub(crate) fn parse_rfc5208_private_key(
        bytes: &[u8],
        evp_pkey_type: c_int,
    ) -> Result<Self, KeyRejected> {
        let mut cbs = cbs::build_CBS(bytes);
        // Also checks the validity of the key
        let evp_pkey = LcPtr::new(unsafe { EVP_parse_private_key(&mut cbs) })
            .map_err(|()| KeyRejected::invalid_encoding())?;
        evp_pkey
            .as_const()
            .id()
            .eq(&evp_pkey_type)
            .then_some(evp_pkey)
            .ok_or(KeyRejected::wrong_algorithm())
    }

    #[allow(non_snake_case)]
    pub(crate) fn create_EVP_PKEY_CTX(&self) -> Result<LcPtr<EVP_PKEY_CTX>, ()> {
        // The only modification made by EVP_PKEY_CTX_new to `priv_key` is to increment its
        // refcount. AWS-LC's refcount operations are thread-safe: lock-free `_Atomic` CAS on the
        // C11-atomic build, `InterlockedIncrement`-style atomics on Windows, and a mutex-protected
        // fallback otherwise. See:
        // https://github.com/aws/aws-lc/blob/main/crypto/refcount_c11.c
        // https://github.com/aws/aws-lc/blob/main/crypto/refcount_win.c
        // https://github.com/aws/aws-lc/blob/main/crypto/refcount_lock.c
        LcPtr::new(unsafe { EVP_PKEY_CTX_new(self.as_mut_unsafe_ptr(), null_mut()) })
    }

    pub(crate) fn parse_raw_private_key(
        bytes: &[u8],
        evp_pkey_type: c_int,
    ) -> Result<Self, KeyRejected> {
        #[cfg(all(feature = "unstable", not(feature = "fips")))]
        if evp_pkey_type == EVP_PKEY_PQDSA {
            return match bytes.len() {
                2560 => Self::new(unsafe {
                    EVP_PKEY_pqdsa_new_raw_private_key(NID_MLDSA44, bytes.as_ptr(), bytes.len())
                }),
                4032 => Self::new(unsafe {
                    EVP_PKEY_pqdsa_new_raw_private_key(NID_MLDSA65, bytes.as_ptr(), bytes.len())
                }),
                4896 => Self::new(unsafe {
                    EVP_PKEY_pqdsa_new_raw_private_key(NID_MLDSA87, bytes.as_ptr(), bytes.len())
                }),
                _ => Err(()),
            }
            .map_err(|()| KeyRejected::invalid_encoding());
        }

        Self::new(unsafe {
            EVP_PKEY_new_raw_private_key(evp_pkey_type, null_mut(), bytes.as_ptr(), bytes.len())
        })
        .map_err(|()| KeyRejected::unspecified())
    }

    pub(crate) fn parse_raw_public_key(
        bytes: &[u8],
        evp_pkey_type: c_int,
    ) -> Result<Self, KeyRejected> {
        #[cfg(all(feature = "unstable", not(feature = "fips")))]
        if evp_pkey_type == EVP_PKEY_PQDSA {
            return match bytes.len() {
                1312 => Self::new(unsafe {
                    EVP_PKEY_pqdsa_new_raw_public_key(NID_MLDSA44, bytes.as_ptr(), bytes.len())
                }),
                1952 => Self::new(unsafe {
                    EVP_PKEY_pqdsa_new_raw_public_key(NID_MLDSA65, bytes.as_ptr(), bytes.len())
                }),
                2592 => Self::new(unsafe {
                    EVP_PKEY_pqdsa_new_raw_public_key(NID_MLDSA87, bytes.as_ptr(), bytes.len())
                }),
                _ => Err(()),
            }
            .map_err(|()| KeyRejected::unspecified());
        }

        Self::new(unsafe {
            EVP_PKEY_new_raw_public_key(evp_pkey_type, null_mut(), bytes.as_ptr(), bytes.len())
        })
        .map_err(|()| KeyRejected::invalid_encoding())
    }

    pub(crate) fn sign<F>(
        &self,
        message: &[u8],
        digest: Option<&'static digest::Algorithm>,
        padding_fn: Option<F>,
    ) -> Result<Box<[u8]>, Unspecified>
    where
        F: EVP_PKEY_CTX_consumer,
    {
        let mut md_ctx = DigestContext::new_uninit();
        let evp_md = if let Some(alg) = digest {
            digest::match_digest_type(&alg.id).as_const_ptr()
        } else {
            null()
        };
        let mut pctx = null_mut::<EVP_PKEY_CTX>();
        if 1 != unsafe {
            // EVP_DigestSignInit does not mutate |pkey| for thread-safety purposes and may be
            // used concurrently with other non-mutating functions on |pkey|.
            // https://github.com/aws/aws-lc/blob/9b4b5a15a97618b5b826d742419ccd54c819fa42/include/openssl/evp.h#L297-L313
            EVP_DigestSignInit(
                md_ctx.as_mut_ptr(),
                &mut pctx,
                evp_md,
                null_mut(),
                self.as_mut_unsafe_ptr(),
            )
        } {
            return Err(Unspecified);
        }

        if let Some(pad_fn) = padding_fn {
            pad_fn(pctx)?;
        }

        // Determine the maximum length of the signature.
        let mut sig_len = 0;
        if 1 != unsafe {
            EVP_DigestSign(
                md_ctx.as_mut_ptr(),
                null_mut(),
                &mut sig_len,
                message.as_ptr(),
                message.len(),
            )
        } {
            return Err(Unspecified);
        }
        if sig_len == 0 {
            return Err(Unspecified);
        }

        let mut signature = vec![0u8; sig_len];
        if 1 != indicator_check!(unsafe {
            EVP_DigestSign(
                md_ctx.as_mut_ptr(),
                signature.as_mut_ptr(),
                &mut sig_len,
                message.as_ptr(),
                message.len(),
            )
        }) {
            return Err(Unspecified);
        }
        signature.truncate(sig_len);
        Ok(signature.into_boxed_slice())
    }

    pub(crate) fn sign_digest<F>(
        &self,
        digest: &Digest,
        padding_fn: Option<F>,
    ) -> Result<Box<[u8]>, Unspecified>
    where
        F: EVP_PKEY_CTX_consumer,
    {
        let mut pctx = self.create_EVP_PKEY_CTX()?;

        if 1 != unsafe { EVP_PKEY_sign_init(pctx.as_mut_ptr()) } {
            return Err(Unspecified);
        }

        if let Some(pad_fn) = padding_fn {
            pad_fn(pctx.as_mut_ptr())?;
        }

        let msg_digest = digest.as_ref();
        let mut sig_len = 0;
        if 1 != unsafe {
            EVP_PKEY_sign(
                pctx.as_mut_ptr(),
                null_mut(),
                &mut sig_len,
                msg_digest.as_ptr(),
                msg_digest.len(),
            )
        } {
            return Err(Unspecified);
        }

        let mut signature = vec![0u8; sig_len];
        if 1 != indicator_check!(unsafe {
            EVP_PKEY_sign(
                pctx.as_mut_ptr(),
                signature.as_mut_ptr(),
                &mut sig_len,
                msg_digest.as_ptr(),
                msg_digest.len(),
            )
        }) {
            return Err(Unspecified);
        }
        signature.truncate(sig_len);

        Ok(signature.into_boxed_slice())
    }

    pub(crate) fn verify<F>(
        &self,
        msg: &[u8],
        digest: Option<&'static digest::Algorithm>,
        padding_fn: Option<F>,
        signature: &[u8],
    ) -> Result<(), Unspecified>
    where
        F: EVP_PKEY_CTX_consumer,
    {
        let mut md_ctx = DigestContext::new_uninit();

        let evp_md = if let Some(alg) = digest {
            digest::match_digest_type(&alg.id).as_const_ptr()
        } else {
            null()
        };

        let mut pctx = null_mut::<EVP_PKEY_CTX>();

        if 1 != unsafe {
            EVP_DigestVerifyInit(
                md_ctx.as_mut_ptr(),
                &mut pctx,
                evp_md,
                null_mut(),
                self.as_mut_unsafe_ptr(),
            )
        } {
            return Err(Unspecified);
        }
        if let Some(pad_fn) = padding_fn {
            pad_fn(pctx)?;
        }

        if 1 != indicator_check!(unsafe {
            EVP_DigestVerify(
                md_ctx.as_mut_ptr(),
                signature.as_ptr(),
                signature.len(),
                msg.as_ptr(),
                msg.len(),
            )
        }) {
            return Err(Unspecified);
        }

        Ok(())
    }

    pub(crate) fn verify_digest_sig<F>(
        &self,
        digest: &Digest,
        padding_fn: Option<F>,
        signature: &[u8],
    ) -> Result<(), Unspecified>
    where
        F: EVP_PKEY_CTX_consumer,
    {
        let mut pctx = self.create_EVP_PKEY_CTX()?;

        if 1 != unsafe { EVP_PKEY_verify_init(pctx.as_mut_ptr()) } {
            return Err(Unspecified);
        }

        if let Some(pad_fn) = padding_fn {
            pad_fn(pctx.as_mut_ptr())?;
        }

        let msg_digest = digest.as_ref();

        if 1 == unsafe {
            indicator_check!(EVP_PKEY_verify(
                pctx.as_mut_ptr(),
                signature.as_ptr(),
                signature.len(),
                msg_digest.as_ptr(),
                msg_digest.len(),
            ))
        } {
            Ok(())
        } else {
            Err(Unspecified)
        }
    }

    pub(crate) fn agree(&self, peer_key: &mut Self) -> Result<Box<[u8]>, Unspecified> {
        let mut pctx = self.create_EVP_PKEY_CTX()?;

        if 1 != unsafe { EVP_PKEY_derive_init(pctx.as_mut_ptr()) } {
            return Err(Unspecified);
        }

        let mut secret_len = 0;
        if 1 != unsafe { EVP_PKEY_derive_set_peer(pctx.as_mut_ptr(), peer_key.as_mut_ptr()) } {
            return Err(Unspecified);
        }

        if 1 != unsafe { EVP_PKEY_derive(pctx.as_mut_ptr(), null_mut(), &mut secret_len) } {
            return Err(Unspecified);
        }

        let mut secret = vec![0u8; secret_len];
        if 1 != indicator_check!(unsafe {
            EVP_PKEY_derive(pctx.as_mut_ptr(), secret.as_mut_ptr(), &mut secret_len)
        }) {
            return Err(Unspecified);
        }
        secret.truncate(secret_len);

        Ok(secret.into_boxed_slice())
    }

    pub(crate) fn generate<F>(pkey_type: c_int, params_fn: Option<F>) -> Result<Self, Unspecified>
    where
        F: EVP_PKEY_CTX_consumer,
    {
        let mut pkey_ctx = LcPtr::new(unsafe { EVP_PKEY_CTX_new_id(pkey_type, null_mut()) })?;

        if 1 != unsafe { EVP_PKEY_keygen_init(pkey_ctx.as_mut_ptr()) } {
            return Err(Unspecified);
        }

        if let Some(pad_fn) = params_fn {
            pad_fn(pkey_ctx.as_mut_ptr())?;
        }

        let mut pkey = null_mut::<EVP_PKEY>();

        if 1 != indicator_check!(unsafe { EVP_PKEY_keygen(pkey_ctx.as_mut_ptr(), &mut pkey) }) {
            return Err(Unspecified);
        }

        Ok(LcPtr::new(pkey)?)
    }
}

impl Clone for LcPtr<EVP_PKEY> {
    fn clone(&self) -> Self {
        // EVP_PKEY_up_ref increments the refcount using AWS-LC's thread-safe refcount
        // implementation: lock-free `_Atomic` CAS on the C11-atomic build, `InterlockedIncrement`-
        // style atomics on Windows, and a mutex-protected fallback otherwise. See:
        // https://github.com/aws/aws-lc/blob/main/crypto/refcount_c11.c
        // https://github.com/aws/aws-lc/blob/main/crypto/refcount_win.c
        // https://github.com/aws/aws-lc/blob/main/crypto/refcount_lock.c
        assert_eq!(
            1,
            unsafe { EVP_PKEY_up_ref(self.as_mut_unsafe_ptr()) },
            "infallible AWS-LC function"
        );
        Self::new(unsafe { self.as_mut_unsafe_ptr() }).expect("non-null AWS-LC EVP_PKEY pointer")
    }
}

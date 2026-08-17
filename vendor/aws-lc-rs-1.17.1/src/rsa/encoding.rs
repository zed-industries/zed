// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

/// [RFC 8017](https://www.rfc-editor.org/rfc/rfc8017.html)
///
/// PKCS #1: RSA Cryptography Specifications Version 2.2
pub(in crate::rsa) mod rfc8017 {
    use crate::aws_lc::{
        EVP_PKEY_assign_RSA, EVP_PKEY_new, RSA_parse_private_key, RSA_public_key_from_bytes,
        RSA_public_key_to_bytes, EVP_PKEY,
    };
    use crate::cbs;
    use crate::error::{KeyRejected, Unspecified};
    use crate::ptr::{DetachableLcPtr, LcPtr};
    use std::ptr::null_mut;

    /// DER encode a RSA public key to `RSAPublicKey` structure.
    pub(in crate::rsa) fn encode_public_key_der(
        pubkey: &LcPtr<EVP_PKEY>,
    ) -> Result<Box<[u8]>, Unspecified> {
        let mut pubkey_bytes = null_mut::<u8>();
        let mut outlen: usize = 0;
        if 1 != unsafe {
            RSA_public_key_to_bytes(
                &mut pubkey_bytes,
                &mut outlen,
                pubkey.as_const().get_rsa()?.as_const_ptr(),
            )
        } {
            return Err(Unspecified);
        }
        let pubkey_bytes = LcPtr::new(pubkey_bytes)?;
        let pubkey_slice = unsafe { pubkey_bytes.as_slice(outlen) };
        let pubkey_vec = Vec::from(pubkey_slice);
        Ok(pubkey_vec.into_boxed_slice())
    }

    /// Decode a DER encoded `RSAPublicKey` structure.
    #[inline]
    pub(in crate::rsa) fn decode_public_key_der(
        public_key: &[u8],
    ) -> Result<LcPtr<EVP_PKEY>, KeyRejected> {
        let mut rsa = DetachableLcPtr::new(unsafe {
            RSA_public_key_from_bytes(public_key.as_ptr(), public_key.len())
        })?;

        let mut pkey = LcPtr::new(unsafe { EVP_PKEY_new() })?;

        if 1 != unsafe { EVP_PKEY_assign_RSA(pkey.as_mut_ptr(), rsa.as_mut_ptr()) } {
            return Err(KeyRejected::unspecified());
        }

        rsa.detach();

        Ok(pkey)
    }

    /// Decodes a DER encoded `RSAPrivateKey` structure.
    #[inline]
    pub(in crate::rsa) fn decode_private_key_der(
        private_key: &[u8],
    ) -> Result<LcPtr<EVP_PKEY>, KeyRejected> {
        let mut cbs = cbs::build_CBS(private_key);

        let mut rsa = DetachableLcPtr::new(unsafe { RSA_parse_private_key(&mut cbs) })?;

        let mut pkey = LcPtr::new(unsafe { EVP_PKEY_new() })?;

        if 1 != unsafe { EVP_PKEY_assign_RSA(pkey.as_mut_ptr(), rsa.as_mut_ptr()) } {
            return Err(KeyRejected::unspecified());
        }

        rsa.detach();

        Ok(pkey)
    }
}

/// [RFC 5280](https://www.rfc-editor.org/rfc/rfc5280.html)
///
/// Encodings that use the `SubjectPublicKeyInfo` structure.
pub(in crate::rsa) mod rfc5280 {
    use crate::aws_lc::{EVP_PKEY, EVP_PKEY_RSA, EVP_PKEY_RSA_PSS};
    use crate::buffer::Buffer;
    use crate::encoding::PublicKeyX509Der;
    use crate::error::{KeyRejected, Unspecified};
    use crate::ptr::LcPtr;

    pub(in crate::rsa) fn encode_public_key_der(
        key: &LcPtr<EVP_PKEY>,
    ) -> Result<PublicKeyX509Der<'static>, Unspecified> {
        let der = key.as_const().marshal_rfc5280_public_key()?;
        Ok(PublicKeyX509Der::from(Buffer::new(der)))
    }

    pub(in crate::rsa) fn decode_public_key_der(
        value: &[u8],
    ) -> Result<LcPtr<EVP_PKEY>, KeyRejected> {
        LcPtr::<EVP_PKEY>::parse_rfc5280_public_key(value, EVP_PKEY_RSA).or(
            // Does anyone encode with this OID?
            LcPtr::<EVP_PKEY>::parse_rfc5280_public_key(value, EVP_PKEY_RSA_PSS),
        )
    }
}

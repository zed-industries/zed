// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

pub(crate) mod key_pair;
pub(crate) mod signature;

use crate::aws_lc::{EVP_PKEY, EVP_PKEY_PQDSA, NID_MLDSA44, NID_MLDSA65, NID_MLDSA87};
use crate::error::{KeyRejected, Unspecified};
use crate::ptr::LcPtr;
use core::ffi::c_int;

#[derive(Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub(crate) enum AlgorithmID {
    ML_DSA_44,
    ML_DSA_65,
    ML_DSA_87,
}

impl AlgorithmID {
    #[allow(dead_code)]
    pub(crate) const fn from_nid(nid: c_int) -> Result<Self, Unspecified> {
        match nid {
            NID_MLDSA44 => Ok(Self::ML_DSA_44),
            NID_MLDSA65 => Ok(Self::ML_DSA_65),
            NID_MLDSA87 => Ok(Self::ML_DSA_87),
            _ => Err(Unspecified),
        }
    }

    pub(crate) const fn nid(&self) -> c_int {
        match self {
            Self::ML_DSA_44 => NID_MLDSA44,
            Self::ML_DSA_65 => NID_MLDSA65,
            Self::ML_DSA_87 => NID_MLDSA87,
        }
    }

    #[allow(dead_code)]
    pub(crate) const fn priv_key_size_bytes(&self) -> usize {
        match self {
            Self::ML_DSA_44 => 2560,
            Self::ML_DSA_65 => 4032,
            Self::ML_DSA_87 => 4896,
        }
    }

    pub(crate) const fn pub_key_size_bytes(&self) -> usize {
        match self {
            Self::ML_DSA_44 => 1312,
            Self::ML_DSA_65 => 1952,
            Self::ML_DSA_87 => 2592,
        }
    }

    pub(crate) const fn seed_size_bytes(&self) -> usize {
        // All ML-DSA variants use 32-byte seeds per FIPS 204
        match self {
            Self::ML_DSA_44 | Self::ML_DSA_65 | Self::ML_DSA_87 => 32,
        }
    }

    pub(crate) const fn signature_size_bytes(&self) -> usize {
        match self {
            Self::ML_DSA_44 => 2420,
            Self::ML_DSA_65 => 3309,
            Self::ML_DSA_87 => 4627,
        }
    }
}

pub(crate) fn validate_pqdsa_evp_key(
    evp_pkey: &LcPtr<EVP_PKEY>,
    id: &'static AlgorithmID,
) -> Result<(), KeyRejected> {
    if evp_pkey.as_const().key_size_bytes() == id.pub_key_size_bytes() {
        Ok(())
    } else {
        Err(KeyRejected::unspecified())
    }
}

pub(crate) fn parse_pqdsa_public_key(
    key_bytes: &[u8],
    id: &'static AlgorithmID,
) -> Result<LcPtr<EVP_PKEY>, KeyRejected> {
    LcPtr::<EVP_PKEY>::parse_rfc5280_public_key(key_bytes, EVP_PKEY_PQDSA)
        .or(LcPtr::<EVP_PKEY>::parse_raw_public_key(
            key_bytes,
            EVP_PKEY_PQDSA,
        ))
        .and_then(|key| validate_pqdsa_evp_key(&key, id).map(|()| key))
}

#[cfg(test)]
mod tests {
    use crate::aws_lc::{
        EVP_PKEY_cmp, EVP_PKEY, EVP_PKEY_PQDSA, NID_MLDSA44, NID_MLDSA65, NID_MLDSA87,
    };

    use crate::evp_pkey::*;

    use crate::pkcs8::Version;
    use crate::pqdsa::key_pair::evp_key_pqdsa_generate;
    use crate::pqdsa::AlgorithmID;
    use crate::ptr::LcPtr;

    #[test]
    fn test_keygen() {
        for nid in [NID_MLDSA44, NID_MLDSA65, NID_MLDSA87] {
            let key = evp_key_pqdsa_generate(nid).unwrap();
            println!("key size: {:?}", key.as_const().key_size_bytes());
            test_serialization_for(&key, &AlgorithmID::from_nid(nid).unwrap());
            test_signing_for(&key, &AlgorithmID::from_nid(nid).unwrap());
        }
    }

    fn test_serialization_for(evp_pkey: &LcPtr<EVP_PKEY>, id: &AlgorithmID) {
        let public_buffer = evp_pkey.as_const().marshal_rfc5280_public_key().unwrap();
        println!("public marshall: {public_buffer:?}");
        let key_public =
            LcPtr::<EVP_PKEY>::parse_rfc5280_public_key(&public_buffer, EVP_PKEY_PQDSA).unwrap();

        let private_buffer = evp_pkey
            .as_const()
            .marshal_rfc5208_private_key(Version::V1)
            .unwrap();
        println!("private marshall: {private_buffer:?}");
        let key_private =
            LcPtr::<EVP_PKEY>::parse_rfc5208_private_key(&private_buffer, EVP_PKEY_PQDSA).unwrap();

        let raw_public_buffer = key_public.as_const().marshal_raw_public_key().unwrap();
        assert_eq!(raw_public_buffer.len(), id.pub_key_size_bytes());
        println!("raw public size: {}", raw_public_buffer.len());
        let key_public2 =
            LcPtr::<EVP_PKEY>::parse_raw_public_key(&raw_public_buffer, EVP_PKEY_PQDSA).unwrap();

        assert_eq!(1, unsafe {
            EVP_PKEY_cmp(key_public.as_const_ptr(), key_public2.as_const_ptr())
        });

        let raw_private_buffer = key_private.as_const().marshal_raw_private_key().unwrap();
        assert_eq!(raw_private_buffer.len(), id.priv_key_size_bytes());
        println!("raw private size: {}", raw_private_buffer.len());
        let key_private2 =
            LcPtr::<EVP_PKEY>::parse_raw_private_key(&raw_private_buffer, EVP_PKEY_PQDSA).unwrap();
        assert_eq!(1, unsafe {
            EVP_PKEY_cmp(key_private.as_const_ptr(), key_private2.as_const_ptr())
        });
    }

    fn test_signing_for(evp_pkey: &LcPtr<EVP_PKEY>, id: &AlgorithmID) {
        let message = b"hello world";
        let signature = evp_pkey
            .sign(message, None, No_EVP_PKEY_CTX_consumer)
            .unwrap();
        println!("signature size: {}", signature.len());
        assert_eq!(signature.len(), evp_pkey.as_const().signature_size_bytes());
        assert_eq!(signature.len(), id.signature_size_bytes());
        evp_pkey
            .verify(message, None, No_EVP_PKEY_CTX_consumer, &signature)
            .unwrap();
        println!("verified: {signature:?}");
    }
}

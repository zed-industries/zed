// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

use crate::aws_lc::{EVP_PKEY, EVP_PKEY_EC};
use crate::ec::encoding::sec1::parse_sec1_public_point;
use crate::ec::validate_ec_evp_key;

use crate::error::KeyRejected;
use crate::ptr::LcPtr;

// [SEC 1](https://secg.org/sec1-v2.pdf)
//
// SEC 1: Elliptic Curve Cryptography, Version 2.0
pub(crate) mod sec1 {
    use crate::aws_lc::{
        point_conversion_form_t, BN_bn2cbb_padded, EC_GROUP_get_curve_name, EC_KEY_get0_group,
        EC_KEY_get0_private_key, EC_KEY_get0_public_key, EC_KEY_new, EC_KEY_set_group,
        EC_KEY_set_private_key, EC_KEY_set_public_key, EC_POINT_mul, EC_POINT_new,
        EC_POINT_oct2point, EC_POINT_point2cbb, EVP_PKEY_assign_EC_KEY, EVP_PKEY_get0_EC_KEY,
        EVP_PKEY_new, NID_X9_62_prime256v1, NID_secp256k1, NID_secp384r1, NID_secp521r1, BIGNUM,
        EC_GROUP, EC_POINT, EVP_PKEY,
    };
    use crate::cbb::LcCBB;
    use crate::ec::{
        compressed_public_key_size_bytes, ec_group_from_nid, uncompressed_public_key_size_bytes,
        validate_ec_evp_key, KeyRejected,
    };
    use crate::error::Unspecified;
    use crate::ptr::{ConstPointer, DetachableLcPtr, LcPtr};
    use std::ptr::{null, null_mut};

    pub(crate) fn parse_sec1_public_point(
        key_bytes: &[u8],
        expected_curve_nid: i32,
    ) -> Result<LcPtr<EVP_PKEY>, KeyRejected> {
        let ec_group = ec_group_from_nid(expected_curve_nid)?;
        let mut ec_point = LcPtr::new(unsafe { EC_POINT_new(ec_group.as_const_ptr()) })?;

        if 1 != unsafe {
            EC_POINT_oct2point(
                ec_group.as_const_ptr(),
                ec_point.as_mut_ptr(),
                key_bytes.as_ptr(),
                key_bytes.len(),
                null_mut(),
            )
        } {
            return Err(KeyRejected::invalid_encoding());
        }
        from_ec_public_point(&ec_group, &ec_point)
    }

    #[inline]
    fn from_ec_public_point(
        ec_group: &ConstPointer<EC_GROUP>,
        public_ec_point: &LcPtr<EC_POINT>,
    ) -> Result<LcPtr<EVP_PKEY>, KeyRejected> {
        let nid = unsafe { EC_GROUP_get_curve_name(ec_group.as_const_ptr()) };
        let mut ec_key = DetachableLcPtr::new(unsafe { EC_KEY_new() })?;
        if 1 != unsafe { EC_KEY_set_group(ec_key.as_mut_ptr(), ec_group.as_const_ptr()) } {
            return Err(KeyRejected::unexpected_error());
        }
        if 1 != unsafe {
            EC_KEY_set_public_key(ec_key.as_mut_ptr(), public_ec_point.as_const_ptr())
        } {
            return Err(KeyRejected::inconsistent_components());
        }

        let mut pkey = LcPtr::new(unsafe { EVP_PKEY_new() })?;

        if 1 != unsafe { EVP_PKEY_assign_EC_KEY(pkey.as_mut_ptr(), ec_key.as_mut_ptr()) } {
            return Err(KeyRejected::unexpected_error());
        }

        ec_key.detach();

        validate_ec_evp_key(&pkey.as_const(), nid)?;

        Ok(pkey)
    }

    pub(crate) fn parse_sec1_private_bn(
        priv_key: &[u8],
        nid: i32,
    ) -> Result<LcPtr<EVP_PKEY>, KeyRejected> {
        let ec_group = ec_group_from_nid(nid)?;
        let priv_key = LcPtr::<BIGNUM>::try_from(priv_key)?;

        let pkey = from_ec_private_bn(&ec_group, &priv_key.as_const())?;

        Ok(pkey)
    }

    fn from_ec_private_bn(
        ec_group: &ConstPointer<EC_GROUP>,
        private_big_num: &ConstPointer<BIGNUM>,
    ) -> Result<LcPtr<EVP_PKEY>, KeyRejected> {
        let mut ec_key = DetachableLcPtr::new(unsafe { EC_KEY_new() })?;
        if 1 != unsafe { EC_KEY_set_group(ec_key.as_mut_ptr(), ec_group.as_const_ptr()) } {
            return Err(KeyRejected::unexpected_error());
        }
        if 1 != unsafe {
            EC_KEY_set_private_key(ec_key.as_mut_ptr(), private_big_num.as_const_ptr())
        } {
            return Err(KeyRejected::invalid_encoding());
        }
        let mut pub_key = LcPtr::new(unsafe { EC_POINT_new(ec_group.as_const_ptr()) })?;
        if 1 != unsafe {
            EC_POINT_mul(
                ec_group.as_const_ptr(),
                pub_key.as_mut_ptr(),
                private_big_num.as_const_ptr(),
                null(),
                null(),
                null_mut(),
            )
        } {
            return Err(KeyRejected::unexpected_error());
        }
        if 1 != unsafe { EC_KEY_set_public_key(ec_key.as_mut_ptr(), pub_key.as_const_ptr()) } {
            return Err(KeyRejected::unexpected_error());
        }
        let expected_curve_nid = unsafe { EC_GROUP_get_curve_name(ec_group.as_const_ptr()) };

        let mut pkey = LcPtr::new(unsafe { EVP_PKEY_new() })?;

        if 1 != unsafe { EVP_PKEY_assign_EC_KEY(pkey.as_mut_ptr(), ec_key.as_mut_ptr()) } {
            return Err(KeyRejected::unexpected_error());
        }
        ec_key.detach();

        // Validate the EC_KEY before returning it.
        validate_ec_evp_key(&pkey.as_const(), expected_curve_nid)?;

        Ok(pkey)
    }
    pub(crate) fn marshal_sec1_public_point(
        evp_pkey: &LcPtr<EVP_PKEY>,
        compressed: bool,
    ) -> Result<Vec<u8>, Unspecified> {
        let pub_key_size = if compressed {
            compressed_public_key_size_bytes(evp_pkey.as_const().key_size_bits())
        } else {
            uncompressed_public_key_size_bytes(evp_pkey.as_const().key_size_bits())
        };
        let mut cbb = LcCBB::new(pub_key_size);
        marshal_sec1_public_point_into_cbb(&mut cbb, evp_pkey, compressed)?;
        cbb.into_vec()
    }

    pub(crate) fn marshal_sec1_public_point_into_buffer(
        buffer: &mut [u8],
        evp_pkey: &LcPtr<EVP_PKEY>,
        compressed: bool,
    ) -> Result<usize, Unspecified> {
        let mut cbb = LcCBB::new_from_slice(buffer);
        marshal_sec1_public_point_into_cbb(&mut cbb, evp_pkey, compressed)?;
        cbb.finish()
    }

    fn marshal_sec1_public_point_into_cbb(
        cbb: &mut LcCBB,
        evp_pkey: &LcPtr<EVP_PKEY>,
        compressed: bool,
    ) -> Result<(), Unspecified> {
        let ec_key = evp_pkey.project_const_lifetime(unsafe {
            |evp_pkey| EVP_PKEY_get0_EC_KEY(evp_pkey.as_const_ptr())
        })?;
        let ec_group = ec_key
            .project_const_lifetime(unsafe { |ec_key| EC_KEY_get0_group(ec_key.as_const_ptr()) })?;
        let ec_point = ec_key.project_const_lifetime(unsafe {
            |ec_key| EC_KEY_get0_public_key(ec_key.as_const_ptr())
        })?;

        let point_conversion_form = if compressed {
            point_conversion_form_t::POINT_CONVERSION_COMPRESSED
        } else {
            point_conversion_form_t::POINT_CONVERSION_UNCOMPRESSED
        };

        if 1 != unsafe {
            EC_POINT_point2cbb(
                cbb.as_mut_ptr(),
                ec_group.as_const_ptr(),
                ec_point.as_const_ptr(),
                point_conversion_form,
                null_mut(),
            )
        } {
            return Err(Unspecified);
        }
        Ok(())
    }

    pub(crate) fn marshal_sec1_private_key(
        evp_pkey: &LcPtr<EVP_PKEY>,
    ) -> Result<Vec<u8>, Unspecified> {
        let ec_key = evp_pkey.project_const_lifetime(unsafe {
            |evp_pkey| EVP_PKEY_get0_EC_KEY(evp_pkey.as_const_ptr())
        })?;
        let ec_group = ec_key
            .project_const_lifetime(unsafe { |ec_key| EC_KEY_get0_group(ec_key.as_const_ptr()) })?;
        let nid = unsafe { EC_GROUP_get_curve_name(ec_group.as_const_ptr()) };
        #[allow(non_upper_case_globals)]
        let key_size: usize = match nid {
            NID_X9_62_prime256v1 | NID_secp256k1 => Ok(32usize),
            NID_secp384r1 => Ok(48usize),
            NID_secp521r1 => Ok(66usize),
            _ => Err(Unspecified),
        }?;
        let private_bn = ec_key.project_const_lifetime(unsafe {
            |ec_key| EC_KEY_get0_private_key(ec_key.as_const_ptr())
        })?;

        let mut cbb = LcCBB::new(key_size);
        if 1 != unsafe { BN_bn2cbb_padded(cbb.as_mut_ptr(), key_size, private_bn.as_const_ptr()) } {
            return Err(Unspecified);
        }
        cbb.into_vec()
    }
}

pub(crate) mod rfc5915 {
    use crate::aws_lc::{
        EC_KEY_get_enc_flags, EC_KEY_marshal_private_key, EC_KEY_parse_private_key,
        EVP_PKEY_get0_EC_KEY, EVP_PKEY_new, EVP_PKEY_set1_EC_KEY, EVP_PKEY,
    };
    use crate::cbb::LcCBB;
    use crate::cbs::build_CBS;
    use crate::ec::ec_group_from_nid;
    use crate::error::{KeyRejected, Unspecified};
    use crate::ptr::LcPtr;

    pub(crate) fn parse_rfc5915_private_key(
        key_bytes: &[u8],
        expected_curve_nid: i32,
    ) -> Result<LcPtr<EVP_PKEY>, KeyRejected> {
        let ec_group = ec_group_from_nid(expected_curve_nid)?;
        let mut cbs = build_CBS(key_bytes);
        let mut ec_key =
            LcPtr::new(unsafe { EC_KEY_parse_private_key(&mut cbs, ec_group.as_const_ptr()) })?;
        let mut evp_pkey = LcPtr::new(unsafe { EVP_PKEY_new() })?;
        if 1 != unsafe { EVP_PKEY_set1_EC_KEY(evp_pkey.as_mut_ptr(), ec_key.as_mut_ptr()) } {
            return Err(KeyRejected::unexpected_error());
        }
        Ok(evp_pkey)
    }

    pub(crate) fn marshal_rfc5915_private_key(
        evp_pkey: &LcPtr<EVP_PKEY>,
    ) -> Result<Vec<u8>, Unspecified> {
        let ec_key = evp_pkey.project_const_lifetime(unsafe {
            |evp_pkey| EVP_PKEY_get0_EC_KEY(evp_pkey.as_const_ptr())
        })?;
        let mut cbb = LcCBB::new(evp_pkey.as_const().key_size_bytes());
        let enc_flags = unsafe { EC_KEY_get_enc_flags(ec_key.as_const_ptr()) };
        if 1 != unsafe {
            EC_KEY_marshal_private_key(cbb.as_mut_ptr(), ec_key.as_const_ptr(), enc_flags)
        } {
            return Err(Unspecified);
        }
        cbb.into_vec()
    }
}

pub(crate) fn parse_ec_public_key(
    key_bytes: &[u8],
    expected_curve_nid: i32,
) -> Result<LcPtr<EVP_PKEY>, KeyRejected> {
    LcPtr::<EVP_PKEY>::parse_rfc5280_public_key(key_bytes, EVP_PKEY_EC)
        .or(parse_sec1_public_point(key_bytes, expected_curve_nid))
        .and_then(|key| validate_ec_evp_key(&key.as_const(), expected_curve_nid).map(|()| key))
}

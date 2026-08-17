// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

use crate::aws_lc::{BN_bin2bn, BN_bn2bin, BN_new, BN_num_bytes, BN_set_u64, BIGNUM};
use crate::ptr::{ConstPointer, DetachableLcPtr, LcPtr};
use core::ptr::null_mut;

impl TryFrom<&[u8]> for LcPtr<BIGNUM> {
    type Error = ();

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        unsafe { LcPtr::new(BN_bin2bn(bytes.as_ptr(), bytes.len(), null_mut())) }
    }
}

impl TryFrom<&[u8]> for DetachableLcPtr<BIGNUM> {
    type Error = ();

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        unsafe { DetachableLcPtr::new(BN_bin2bn(bytes.as_ptr(), bytes.len(), null_mut())) }
    }
}

impl TryFrom<u64> for DetachableLcPtr<BIGNUM> {
    type Error = ();

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        unsafe {
            let mut bn = DetachableLcPtr::new(BN_new())?;
            if 1 != BN_set_u64(bn.as_mut_ptr(), value) {
                return Err(());
            }
            Ok(bn)
        }
    }
}

impl ConstPointer<'_, BIGNUM> {
    pub(crate) fn to_be_bytes(&self) -> Vec<u8> {
        unsafe {
            let bn_bytes = BN_num_bytes(self.as_const_ptr());
            let mut byte_vec = Vec::with_capacity(bn_bytes as usize);
            let out_bytes = BN_bn2bin(self.as_const_ptr(), byte_vec.as_mut_ptr());
            debug_assert_eq!(out_bytes, bn_bytes as usize);
            byte_vec.set_len(out_bytes);
            byte_vec
        }
    }
}

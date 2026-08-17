// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

use crate::aws_lc::{
    EVP_DigestInit_ex, EVP_MD_CTX_cleanup, EVP_MD_CTX_copy, EVP_MD_CTX_init, EVP_MD_CTX,
};
use crate::digest::{match_digest_type, Algorithm};
use crate::error::Unspecified;
use core::mem::MaybeUninit;
use core::ptr::null_mut;

pub(crate) struct DigestContext(EVP_MD_CTX);

impl DigestContext {
    pub fn new(algorithm: &'static Algorithm) -> Result<DigestContext, Unspecified> {
        let evp_md_type = match_digest_type(&algorithm.id);
        let mut dc = Self::new_uninit();
        unsafe {
            if 1 != EVP_DigestInit_ex(dc.as_mut_ptr(), evp_md_type.as_const_ptr(), null_mut()) {
                return Err(Unspecified);
            }
            Ok(dc)
        }
    }

    pub fn new_uninit() -> DigestContext {
        let mut dc = MaybeUninit::<EVP_MD_CTX>::uninit();
        unsafe {
            EVP_MD_CTX_init(dc.as_mut_ptr());
            Self(dc.assume_init())
        }
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut EVP_MD_CTX {
        &mut self.0
    }

    pub(crate) fn as_ptr(&self) -> *const EVP_MD_CTX {
        &self.0
    }
}

unsafe impl Send for DigestContext {}
unsafe impl Sync for DigestContext {}

impl Clone for DigestContext {
    fn clone(&self) -> Self {
        self.try_clone().expect("Unable to clone DigestContext")
    }
}

impl Drop for DigestContext {
    fn drop(&mut self) {
        unsafe {
            EVP_MD_CTX_cleanup(self.as_mut_ptr());
        }
    }
}

impl DigestContext {
    fn try_clone(&self) -> Result<Self, &'static str> {
        let mut dc = MaybeUninit::<EVP_MD_CTX>::uninit();
        unsafe {
            // The first parameter of `EVP_MD_CTX_copy` should not be initialized.
            // https://github.com/aws/aws-lc/blob/98ccf4a316401112943bed604562102ad52efac6/include/openssl/digest.h#L280
            if 1 != EVP_MD_CTX_copy(dc.as_mut_ptr(), self.as_ptr()) {
                return Err("EVP_MD_CTX_copy failed");
            }
            Ok(Self(dc.assume_init()))
        }
    }
}

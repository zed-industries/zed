// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

use crate::aws_lc::{CBB_cleanup, CBB_finish, CBB_init, CBB_init_fixed, CBB};
use crate::error::Unspecified;
use crate::ptr::LcPtr;
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::ptr::null_mut;

pub(crate) struct LcCBB<'a>(CBB, PhantomData<&'a CBB>);

impl LcCBB<'static> {
    pub(crate) fn new(initial_capacity: usize) -> LcCBB<'static> {
        let mut cbb = MaybeUninit::<CBB>::uninit();
        let cbb = unsafe {
            CBB_init(cbb.as_mut_ptr(), initial_capacity);
            cbb.assume_init()
        };
        Self(cbb, PhantomData)
    }

    pub(crate) fn into_vec(mut self) -> Result<Vec<u8>, Unspecified> {
        let mut out_data = null_mut::<u8>();
        let mut out_len: usize = 0;

        if 1 != unsafe { CBB_finish(self.as_mut_ptr(), &mut out_data, &mut out_len) } {
            return Err(Unspecified);
        }

        let out_data = LcPtr::new(out_data)?;
        let slice = unsafe { std::slice::from_raw_parts(out_data.as_const_ptr(), out_len) };
        // `to_vec()` copies the data into a new `Vec`
        Ok(slice.to_vec())
    }
}

impl<'a> LcCBB<'a> {
    pub(crate) fn new_from_slice(buffer: &'a mut [u8]) -> LcCBB<'a> {
        let mut cbb = MaybeUninit::<CBB>::uninit();
        let cbb = unsafe {
            CBB_init_fixed(cbb.as_mut_ptr(), buffer.as_mut_ptr(), buffer.len());
            cbb.assume_init()
        };
        Self(cbb, PhantomData)
    }

    pub(crate) fn finish(mut self) -> Result<usize, Unspecified> {
        let mut pkcs8_bytes_ptr = null_mut::<u8>();
        let mut out_len: usize = 0;
        if 1 != unsafe { CBB_finish(self.as_mut_ptr(), &mut pkcs8_bytes_ptr, &mut out_len) } {
            return Err(Unspecified);
        }
        Ok(out_len)
    }
}
impl LcCBB<'_> {
    pub(crate) fn as_mut_ptr(&mut self) -> *mut CBB {
        &mut self.0
    }
}

impl Drop for LcCBB<'_> {
    fn drop(&mut self) {
        unsafe {
            CBB_cleanup(&mut self.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LcCBB;
    use crate::aws_lc::CBB_add_asn1_bool;

    #[test]
    fn dynamic_vec() {
        let mut cbb = LcCBB::new(4);
        assert_eq!(1, unsafe { CBB_add_asn1_bool(cbb.as_mut_ptr(), 1) });
        let vec = cbb.into_vec().expect("be copied to buffer");
        assert_eq!(vec.as_slice(), &[1, 1, 255]);
    }

    #[test]
    fn dynamic_buffer_grows() {
        let mut cbb = LcCBB::new(1);
        assert_eq!(1, unsafe { CBB_add_asn1_bool(cbb.as_mut_ptr(), 1) });
        let vec = cbb.into_vec().expect("be copied to buffer");
        assert_eq!(vec.as_slice(), &[1, 1, 255]);
    }

    #[test]
    fn fixed_buffer() {
        let mut buffer = [0u8; 4];
        let mut cbb = LcCBB::new_from_slice(&mut buffer);
        assert_eq!(1, unsafe { CBB_add_asn1_bool(cbb.as_mut_ptr(), 1) });
        let out_len = cbb.finish().expect("cbb finishable");
        assert_eq!(&buffer[..out_len], &[1, 1, 255]);
    }

    #[test]
    fn fixed_buffer_no_growth() {
        let mut buffer = [0u8; 1];
        let mut cbb = LcCBB::new_from_slice(&mut buffer);
        assert_ne!(1, unsafe { CBB_add_asn1_bool(cbb.as_mut_ptr(), 1) });
    }
}

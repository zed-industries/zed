// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

use crate::aws_lc::{CBS_init, CBS};
use core::mem::MaybeUninit;

#[inline]
#[allow(non_snake_case)]
pub fn build_CBS(data: &[u8]) -> CBS {
    let mut cbs = MaybeUninit::<CBS>::uninit();
    unsafe { CBS_init(cbs.as_mut_ptr(), data.as_ptr(), data.len()) };
    unsafe { cbs.assume_init() }
}

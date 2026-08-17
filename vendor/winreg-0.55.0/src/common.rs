// Copyright 2023, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
#![macro_use]
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::slice;

macro_rules! werr {
    ($e:expr) => {
        Err(io::Error::from_raw_os_error($e as i32))
    };
}

pub(crate) fn to_utf16<P: AsRef<OsStr>>(s: P) -> Vec<u16> {
    s.as_ref().encode_wide().chain(Some(0)).collect()
}

pub(crate) fn v16_to_v8(v: &[u16]) -> Vec<u8> {
    unsafe { slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * 2).to_vec() }
}

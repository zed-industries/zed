// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::PathBuf;
use std::slice::from_raw_parts;

pub trait ToWide {
    fn to_wide(&self) -> Vec<u16>;
    fn to_wide_null(&self) -> Vec<u16>;
}
impl<T> ToWide for T where T: AsRef<OsStr> {
    #[inline]
    fn to_wide(&self) -> Vec<u16> {
        self.as_ref().encode_wide().collect()
    }
    #[inline]
    fn to_wide_null(&self) -> Vec<u16> {
        self.as_ref().encode_wide().chain(Some(0)).collect()
    }
}
pub trait FromWide where Self: Sized {
    fn from_wide(wide: &[u16]) -> Self;
    #[inline]
    fn from_wide_null(wide: &[u16]) -> Self {
        let len = wide.iter().take_while(|&&c| c != 0).count();
        Self::from_wide(&wide[..len])
    }
    #[inline]
    unsafe fn from_wide_ptr(wide: *const u16, len: usize) -> Self {
        assert!(!wide.is_null());
        Self::from_wide(from_raw_parts(wide, len))
    }
    #[inline]
    unsafe fn from_wide_ptr_null(wide: *const u16) -> Self {
        assert!(!wide.is_null());
        for i in 0.. {
            if *wide.offset(i) == 0 {
                return Self::from_wide_ptr(wide, i as usize)
            }
        }
        unreachable!()
    }
}
impl FromWide for OsString {
    #[inline]
    fn from_wide(wide: &[u16]) -> OsString {
        OsStringExt::from_wide(wide)
    }
}
impl FromWide for PathBuf {
    #[inline]
    fn from_wide(wide: &[u16]) -> PathBuf {
        <OsString as OsStringExt>::from_wide(wide).into()
    }
}

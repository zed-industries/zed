// Copyright 2015, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.

//! Traits for loading/saving Registry values
use super::enums::*;
use super::winapi::shared::winerror;
use super::RegValue;
use super::{to_utf16, v16_to_v8};
use std::ffi::{OsStr, OsString};
use std::io;
use std::os::windows::ffi::OsStringExt;
use std::slice;

/// A trait for types that can be loaded from registry values.
///
/// **NOTE:** Uses `from_utf16_lossy` when converting to `String`.
///
/// **NOTE:** When converting to `String` or `OsString`, trailing `NULL` characters are trimmed
/// and line separating `NULL` characters in `REG_MULTI_SZ` are replaced by `\n`
/// effectively representing the value as a multiline string.
/// When converting to `Vec<String>` or `Vec<OsString>` `NULL` is used as a strings separator.
pub trait FromRegValue: Sized {
    fn from_reg_value(val: &RegValue) -> io::Result<Self>;
}

impl FromRegValue for String {
    fn from_reg_value(val: &RegValue) -> io::Result<String> {
        match val.vtype {
            REG_SZ | REG_EXPAND_SZ | REG_MULTI_SZ => {
                let words = unsafe {
                    #[allow(clippy::cast_ptr_alignment)]
                    slice::from_raw_parts(val.bytes.as_ptr() as *const u16, val.bytes.len() / 2)
                };
                let mut s = String::from_utf16_lossy(words);
                while s.ends_with('\u{0}') {
                    s.pop();
                }
                if val.vtype == REG_MULTI_SZ {
                    return Ok(s.replace("\u{0}", "\n"));
                }
                Ok(s)
            }
            _ => werr!(winerror::ERROR_BAD_FILE_TYPE),
        }
    }
}

impl FromRegValue for Vec<String> {
    fn from_reg_value(val: &RegValue) -> io::Result<Vec<String>> {
        match val.vtype {
            REG_MULTI_SZ => {
                let words = unsafe {
                    slice::from_raw_parts(val.bytes.as_ptr() as *const u16, val.bytes.len() / 2)
                };
                let mut s = String::from_utf16_lossy(words);
                while s.ends_with('\u{0}') {
                    s.pop();
                }
                let v: Vec<String> = s.split('\u{0}').map(|x| x.to_owned()).collect();
                Ok(v)
            }
            _ => werr!(winerror::ERROR_BAD_FILE_TYPE),
        }
    }
}

impl FromRegValue for OsString {
    fn from_reg_value(val: &RegValue) -> io::Result<OsString> {
        match val.vtype {
            REG_SZ | REG_EXPAND_SZ | REG_MULTI_SZ => {
                let mut words = unsafe {
                    #[allow(clippy::cast_ptr_alignment)]
                    slice::from_raw_parts(val.bytes.as_ptr() as *const u16, val.bytes.len() / 2)
                };
                while let Some(0) = words.last() {
                    words = &words[0..words.len() - 1];
                }
                let s = OsString::from_wide(words);
                Ok(s)
            }
            _ => werr!(winerror::ERROR_BAD_FILE_TYPE),
        }
    }
}

impl FromRegValue for Vec<OsString> {
    fn from_reg_value(val: &RegValue) -> io::Result<Vec<OsString>> {
        match val.vtype {
            REG_MULTI_SZ => {
                let mut words = unsafe {
                    slice::from_raw_parts(val.bytes.as_ptr() as *const u16, val.bytes.len() / 2)
                };
                while let Some(0) = words.last() {
                    words = &words[0..words.len() - 1];
                }
                let v: Vec<OsString> = words
                    .split(|ch| *ch == 0u16)
                    .map(|x| OsString::from_wide(x))
                    .collect();
                Ok(v)
            }
            _ => werr!(winerror::ERROR_BAD_FILE_TYPE),
        }
    }
}

impl FromRegValue for u32 {
    fn from_reg_value(val: &RegValue) -> io::Result<u32> {
        match val.vtype {
            #[allow(clippy::cast_ptr_alignment)]
            REG_DWORD => Ok(unsafe { *(val.bytes.as_ptr() as *const u32) }),
            _ => werr!(winerror::ERROR_BAD_FILE_TYPE),
        }
    }
}

impl FromRegValue for u64 {
    fn from_reg_value(val: &RegValue) -> io::Result<u64> {
        match val.vtype {
            #[allow(clippy::cast_ptr_alignment)]
            REG_QWORD => Ok(unsafe { *(val.bytes.as_ptr() as *const u64) }),
            _ => werr!(winerror::ERROR_BAD_FILE_TYPE),
        }
    }
}

/// A trait for types that can be written into registry values.
///
/// **NOTE:** Adds trailing `NULL` character to `str`, `String`, `OsStr` and `OsString` values
pub trait ToRegValue {
    fn to_reg_value(&self) -> RegValue;
}

macro_rules! to_reg_value_sz {
    ($t:ty$(, $l:lifetime)*) => {
        impl<$($l,)*> ToRegValue for $t {
            fn to_reg_value(&self) -> RegValue {
                RegValue {
                    bytes: v16_to_v8(&to_utf16(self)),
                    vtype: REG_SZ,
                }
            }
        }
    }
}

to_reg_value_sz!(String);
to_reg_value_sz!(&'a str, 'a);
to_reg_value_sz!(OsString);
to_reg_value_sz!(&'a OsStr, 'a);

macro_rules! to_reg_value_multi_sz {
    ($t:ty$(, $l:lifetime)*) => {
        impl<$($l,)*> ToRegValue for Vec<$t> {
            fn to_reg_value(&self) -> RegValue {
                let mut os_strings = self
                    .into_iter()
                    .map(to_utf16)
                    .collect::<Vec<_>>()
                    .concat();
                os_strings.push(0);
                RegValue {
                    bytes: v16_to_v8(&os_strings),
                    vtype: REG_MULTI_SZ,
                }
            }
        }
    }
}

to_reg_value_multi_sz!(String);
to_reg_value_multi_sz!(&'a str, 'a);
to_reg_value_multi_sz!(OsString);
to_reg_value_multi_sz!(&'a OsStr, 'a);

impl ToRegValue for u32 {
    fn to_reg_value(&self) -> RegValue {
        let bytes: Vec<u8> =
            unsafe { slice::from_raw_parts((self as *const u32) as *const u8, 4).to_vec() };
        RegValue {
            bytes,
            vtype: REG_DWORD,
        }
    }
}

impl ToRegValue for u64 {
    fn to_reg_value(&self) -> RegValue {
        let bytes: Vec<u8> =
            unsafe { slice::from_raw_parts((self as *const u64) as *const u8, 8).to_vec() };
        RegValue {
            bytes,
            vtype: REG_QWORD,
        }
    }
}

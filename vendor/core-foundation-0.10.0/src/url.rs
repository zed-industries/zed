// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A URL type for Core Foundation.

pub use core_foundation_sys::url::*;

use crate::base::{CFIndex, TCFType};
use crate::string::CFString;

use core_foundation_sys::base::{kCFAllocatorDefault, Boolean};
use std::fmt;
use std::path::{Path, PathBuf};
use std::ptr;

use libc::{c_char, strlen, PATH_MAX};

#[cfg(unix)]
use std::ffi::OsStr;
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;

declare_TCFType!(CFURL, CFURLRef);
impl_TCFType!(CFURL, CFURLRef, CFURLGetTypeID);

impl fmt::Debug for CFURL {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let string: CFString = TCFType::wrap_under_get_rule(CFURLGetString(self.0));
            write!(f, "{}", string)
        }
    }
}

impl CFURL {
    pub fn from_path<P: AsRef<Path>>(path: P, isDirectory: bool) -> Option<CFURL> {
        let path_bytes;
        #[cfg(unix)]
        {
            path_bytes = path.as_ref().as_os_str().as_bytes()
        }
        #[cfg(not(unix))]
        {
            // XXX: Getting non-valid UTF8 paths into CoreFoundation on Windows is going to be unpleasant
            // CFURLGetWideFileSystemRepresentation might help
            path_bytes = match path.as_ref().to_str() {
                Some(path) => path,
                None => return None,
            }
        }

        unsafe {
            let url_ref = CFURLCreateFromFileSystemRepresentation(
                ptr::null_mut(),
                path_bytes.as_ptr(),
                path_bytes.len() as CFIndex,
                isDirectory as u8,
            );
            if url_ref.is_null() {
                return None;
            }
            Some(TCFType::wrap_under_create_rule(url_ref))
        }
    }

    pub fn from_file_system_path(
        filePath: CFString,
        pathStyle: CFURLPathStyle,
        isDirectory: bool,
    ) -> CFURL {
        unsafe {
            let url_ref = CFURLCreateWithFileSystemPath(
                kCFAllocatorDefault,
                filePath.as_concrete_TypeRef(),
                pathStyle,
                isDirectory as u8,
            );
            TCFType::wrap_under_create_rule(url_ref)
        }
    }

    #[cfg(unix)]
    pub fn to_path(&self) -> Option<PathBuf> {
        // implementing this on Windows is more complicated because of the different OsStr representation
        unsafe {
            let mut buf = [0u8; PATH_MAX as usize];
            let result = CFURLGetFileSystemRepresentation(
                self.0,
                true as Boolean,
                buf.as_mut_ptr(),
                buf.len() as CFIndex,
            );
            if result == false as Boolean {
                return None;
            }
            let len = strlen(buf.as_ptr() as *const c_char);
            let path = OsStr::from_bytes(&buf[0..len]);
            Some(PathBuf::from(path))
        }
    }

    pub fn get_string(&self) -> CFString {
        unsafe { TCFType::wrap_under_get_rule(CFURLGetString(self.0)) }
    }

    pub fn get_file_system_path(&self, pathStyle: CFURLPathStyle) -> CFString {
        unsafe {
            TCFType::wrap_under_create_rule(CFURLCopyFileSystemPath(
                self.as_concrete_TypeRef(),
                pathStyle,
            ))
        }
    }

    pub fn absolute(&self) -> CFURL {
        unsafe { TCFType::wrap_under_create_rule(CFURLCopyAbsoluteURL(self.as_concrete_TypeRef())) }
    }
}

#[test]
fn file_url_from_path() {
    let path = "/usr/local/foo/";
    let cfstr_path = CFString::from_static_string(path);
    let cfurl = CFURL::from_file_system_path(cfstr_path, kCFURLPOSIXPathStyle, true);
    assert_eq!(cfurl.get_string().to_string(), "file:///usr/local/foo/");
}

#[cfg(unix)]
#[test]
fn non_utf8() {
    use std::ffi::OsStr;
    let path = Path::new(OsStr::from_bytes(b"/\xC0/blame"));
    let cfurl = CFURL::from_path(path, false).unwrap();
    assert_eq!(cfurl.to_path().unwrap(), path);
    let len = unsafe { CFURLGetBytes(cfurl.as_concrete_TypeRef(), ptr::null_mut(), 0) };
    assert_eq!(len, 17);
}

#[test]
fn absolute_file_url() {
    use core_foundation_sys::url::CFURLCreateWithFileSystemPathRelativeToBase;
    use std::path::PathBuf;

    let path = "/usr/local/foo";
    let file = "bar";

    let cfstr_path = CFString::from_static_string(path);
    let cfstr_file = CFString::from_static_string(file);
    let cfurl_base = CFURL::from_file_system_path(cfstr_path, kCFURLPOSIXPathStyle, true);
    let cfurl_relative: CFURL = unsafe {
        let url_ref = CFURLCreateWithFileSystemPathRelativeToBase(
            kCFAllocatorDefault,
            cfstr_file.as_concrete_TypeRef(),
            kCFURLPOSIXPathStyle,
            false as u8,
            cfurl_base.as_concrete_TypeRef(),
        );
        TCFType::wrap_under_create_rule(url_ref)
    };

    let mut absolute_path = PathBuf::from(path);
    absolute_path.push(file);

    assert_eq!(
        cfurl_relative
            .get_file_system_path(kCFURLPOSIXPathStyle)
            .to_string(),
        file
    );
    assert_eq!(
        cfurl_relative
            .absolute()
            .get_file_system_path(kCFURLPOSIXPathStyle)
            .to_string(),
        absolute_path.to_str().unwrap()
    );
}

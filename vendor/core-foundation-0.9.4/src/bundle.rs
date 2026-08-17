// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Core Foundation Bundle Type

use core_foundation_sys::base::kCFAllocatorDefault;
pub use core_foundation_sys::bundle::*;
use core_foundation_sys::url::kCFURLPOSIXPathStyle;
use std::path::PathBuf;

use crate::base::{CFType, TCFType};
use crate::dictionary::CFDictionary;
use crate::string::CFString;
use crate::url::CFURL;
use std::os::raw::c_void;

declare_TCFType! {
    /// A Bundle type.
    CFBundle, CFBundleRef
}
impl_TCFType!(CFBundle, CFBundleRef, CFBundleGetTypeID);

impl CFBundle {
    pub fn new(bundleURL: CFURL) -> Option<CFBundle> {
        unsafe {
            let bundle_ref = CFBundleCreate(kCFAllocatorDefault, bundleURL.as_concrete_TypeRef());
            if bundle_ref.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(bundle_ref))
            }
        }
    }

    pub fn bundle_with_identifier(identifier: CFString) -> Option<CFBundle> {
        unsafe {
            let bundle_ref = CFBundleGetBundleWithIdentifier(identifier.as_concrete_TypeRef());
            if bundle_ref.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_get_rule(bundle_ref))
            }
        }
    }

    pub fn function_pointer_for_name(&self, function_name: CFString) -> *const c_void {
        unsafe {
            CFBundleGetFunctionPointerForName(
                self.as_concrete_TypeRef(),
                function_name.as_concrete_TypeRef(),
            )
        }
    }

    pub fn main_bundle() -> CFBundle {
        unsafe {
            let bundle_ref = CFBundleGetMainBundle();
            TCFType::wrap_under_get_rule(bundle_ref)
        }
    }

    pub fn info_dictionary(&self) -> CFDictionary<CFString, CFType> {
        unsafe {
            let info_dictionary = CFBundleGetInfoDictionary(self.0);
            TCFType::wrap_under_get_rule(info_dictionary)
        }
    }

    pub fn executable_url(&self) -> Option<CFURL> {
        unsafe {
            let exe_url = CFBundleCopyExecutableURL(self.0);
            if exe_url.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(exe_url))
            }
        }
    }

    /// Bundle's own location
    pub fn bundle_url(&self) -> Option<CFURL> {
        unsafe {
            let bundle_url = CFBundleCopyBundleURL(self.0);
            if bundle_url.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(bundle_url))
            }
        }
    }

    /// Bundle's own location
    pub fn path(&self) -> Option<PathBuf> {
        let url = self.bundle_url()?;
        Some(PathBuf::from(
            url.get_file_system_path(kCFURLPOSIXPathStyle).to_string(),
        ))
    }

    /// Bundle's resources location
    pub fn bundle_resources_url(&self) -> Option<CFURL> {
        unsafe {
            let bundle_url = CFBundleCopyResourcesDirectoryURL(self.0);
            if bundle_url.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(bundle_url))
            }
        }
    }

    /// Bundle's resources location
    pub fn resources_path(&self) -> Option<PathBuf> {
        let url = self.bundle_resources_url()?;
        Some(PathBuf::from(
            url.get_file_system_path(kCFURLPOSIXPathStyle).to_string(),
        ))
    }

    pub fn private_frameworks_url(&self) -> Option<CFURL> {
        unsafe {
            let fw_url = CFBundleCopyPrivateFrameworksURL(self.0);
            if fw_url.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(fw_url))
            }
        }
    }

    pub fn shared_support_url(&self) -> Option<CFURL> {
        unsafe {
            let fw_url = CFBundleCopySharedSupportURL(self.0);
            if fw_url.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(fw_url))
            }
        }
    }
}

#[test]
fn safari_executable_url() {
    use crate::string::CFString;
    use crate::url::{kCFURLPOSIXPathStyle, CFURL};

    let cfstr_path = CFString::from_static_string("/Applications/Safari.app");
    let cfurl_path = CFURL::from_file_system_path(cfstr_path, kCFURLPOSIXPathStyle, true);
    let cfurl_executable = CFBundle::new(cfurl_path)
        .expect("Safari not present")
        .executable_url();
    assert!(cfurl_executable.is_some());
    assert_eq!(
        cfurl_executable
            .unwrap()
            .absolute()
            .get_file_system_path(kCFURLPOSIXPathStyle)
            .to_string(),
        "/Applications/Safari.app/Contents/MacOS/Safari"
    );
}

#[test]
fn safari_private_frameworks_url() {
    use crate::string::CFString;
    use crate::url::{kCFURLPOSIXPathStyle, CFURL};

    let cfstr_path = CFString::from_static_string("/Applications/Safari.app");
    let cfurl_path = CFURL::from_file_system_path(cfstr_path, kCFURLPOSIXPathStyle, true);
    let cfurl_executable = CFBundle::new(cfurl_path)
        .expect("Safari not present")
        .private_frameworks_url();
    assert!(cfurl_executable.is_some());
    assert_eq!(
        cfurl_executable
            .unwrap()
            .absolute()
            .get_file_system_path(kCFURLPOSIXPathStyle)
            .to_string(),
        "/Applications/Safari.app/Contents/Frameworks"
    );
}

#[test]
fn non_existant_bundle() {
    use crate::string::CFString;
    use crate::url::{kCFURLPOSIXPathStyle, CFURL};

    let cfstr_path = CFString::from_static_string("/usr/local/foo");
    let cfurl_path = CFURL::from_file_system_path(cfstr_path, kCFURLPOSIXPathStyle, true);
    assert!(CFBundle::new(cfurl_path).is_none());
}

#[macro_use]
extern crate core_foundation;

use core_foundation::base::{CFComparisonResult, TCFType};
use std::os::raw::c_void;

// sys equivalent stuff that must be declared

#[repr(C)]
pub struct __CFFooBar(c_void);

pub type CFFooBarRef = *const __CFFooBar;

extern "C" {
    pub fn CFFooBarGetTypeID() -> core_foundation::base::CFTypeID;
    pub fn fake_compare(
        this: CFFooBarRef,
        other: CFFooBarRef,
        context: *mut c_void,
    ) -> CFComparisonResult;
}

// Try to use the macros outside of the crate

declare_TCFType!(CFFooBar, CFFooBarRef);
impl_TCFType!(CFFooBar, CFFooBarRef, CFFooBarGetTypeID);
impl_CFTypeDescription!(CFFooBar);
impl_CFComparison!(CFFooBar, fake_compare);

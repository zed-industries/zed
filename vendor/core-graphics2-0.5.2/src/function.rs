use std::ptr::null;

use core_foundation::{
    base::{CFTypeID, TCFType},
    impl_CFTypeDescription, impl_TCFType,
};
use libc::c_void;

use crate::base::CGFloat;

#[repr(C)]
pub struct __CGFunction(c_void);

pub type CGFunctionRef = *const __CGFunction;

pub type CGFunctionEvaluateCallback = extern "C" fn(*const c_void, *const CGFloat, *mut CGFloat);
pub type CGFunctionReleaseInfoCallback = extern "C" fn(*mut c_void);

#[repr(C)]
pub struct CGFunctionCallbacks {
    pub version: u32,
    pub evaluate: CGFunctionEvaluateCallback,
    pub releaseInfo: CGFunctionReleaseInfoCallback,
}

extern "C" {
    pub fn CGFunctionGetTypeID() -> CFTypeID;
    pub fn CGFunctionCreate(
        info: *mut c_void,
        domainDimension: usize,
        domain: *const CGFloat,
        rangeDimension: usize,
        range: *const CGFloat,
        callbacks: *const CGFunctionCallbacks,
    ) -> CGFunctionRef;
    pub fn CGFunctionRetain(function: CGFunctionRef) -> CGFunctionRef;
    pub fn CGFunctionRelease(function: CGFunctionRef);
}

pub struct CGFunction(CGFunctionRef);

impl Drop for CGFunction {
    fn drop(&mut self) {
        unsafe { CGFunctionRelease(self.0) }
    }
}

impl_TCFType!(CGFunction, CGFunctionRef, CGFunctionGetTypeID);
impl_CFTypeDescription!(CGFunction);

impl CGFunction {
    pub unsafe fn new(
        info: *mut c_void,
        domain: Option<&[CGFloat]>,
        range: Option<&[CGFloat]>,
        callbacks: Option<&CGFunctionCallbacks>,
    ) -> Option<Self> {
        let domain_dimension = domain.map_or(0, |d| d.len());
        let domain = domain.map_or(null(), |d| d.as_ptr());
        let range_dimension = range.map_or(0, |r| r.len());
        let range = range.map_or(null(), |r| r.as_ptr());
        let callbacks = callbacks.map_or(null(), |c| c as *const _);
        let function = CGFunctionCreate(info, domain_dimension, domain, range_dimension, range, callbacks);
        if function.is_null() {
            None
        } else {
            Some(TCFType::wrap_under_create_rule(function))
        }
    }
}

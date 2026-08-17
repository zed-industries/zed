use std::ptr::{null, null_mut};

use core_foundation::{
    base::{CFTypeID, TCFType},
    impl_CFTypeDescription, impl_TCFType,
};
use libc::c_void;

use crate::{
    color_space::{CGColorSpace, CGColorSpaceRef},
    function::{CGFunction, CGFunctionRef},
    geometry::CGPoint,
};

#[repr(C)]
pub struct __CGShading(c_void);

pub type CGShadingRef = *mut __CGShading;

extern "C" {
    pub fn CGShadingGetTypeID() -> CFTypeID;
    pub fn CGShadingCreateAxial(
        space: CGColorSpaceRef,
        start: CGPoint,
        end: CGPoint,
        function: CGFunctionRef,
        extendStart: bool,
        extendEnd: bool,
    ) -> CGShadingRef;
    pub fn CGShadingCreateRadial(
        space: CGColorSpaceRef,
        start: CGPoint,
        startRadius: f64,
        end: CGPoint,
        endRadius: f64,
        function: CGFunctionRef,
        extendStart: bool,
        extendEnd: bool,
    ) -> CGShadingRef;
    pub fn CGShadingRetain(shading: CGShadingRef) -> CGShadingRef;
    pub fn CGShadingRelease(shading: CGShadingRef);
}

pub struct CGShading(CGShadingRef);

impl Drop for CGShading {
    fn drop(&mut self) {
        unsafe { CGShadingRelease(self.0) }
    }
}

impl_TCFType!(CGShading, CGShadingRef, CGShadingGetTypeID);
impl_CFTypeDescription!(CGShading);

impl CGShading {
    pub fn new_axial(
        space: Option<&CGColorSpace>,
        start: CGPoint,
        end: CGPoint,
        function: Option<&CGFunction>,
        extend_start: bool,
        extend_end: bool,
    ) -> Option<Self> {
        unsafe {
            let shading = CGShadingCreateAxial(
                space.map_or(null_mut(), |s| s.as_concrete_TypeRef()),
                start,
                end,
                function.map_or(null(), |f| f.as_concrete_TypeRef()),
                extend_start,
                extend_end,
            );
            if shading.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(shading))
            }
        }
    }

    pub fn new_radial(
        space: Option<&CGColorSpace>,
        start: CGPoint,
        start_radius: f64,
        end: CGPoint,
        end_radius: f64,
        function: Option<&CGFunction>,
        extend_start: bool,
        extend_end: bool,
    ) -> Option<Self> {
        unsafe {
            let shading = CGShadingCreateRadial(
                space.map_or(null_mut(), |s| s.as_concrete_TypeRef()),
                start,
                start_radius,
                end,
                end_radius,
                function.map_or(null(), |f| f.as_concrete_TypeRef()),
                extend_start,
                extend_end,
            );
            if shading.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(shading))
            }
        }
    }
}

use std::ptr::{null, null_mut};

use bitflags::bitflags;
use core_foundation::{
    array::{CFArray, CFArrayRef},
    base::{CFTypeID, TCFType},
    impl_CFTypeDescription, impl_TCFType,
};
use libc::{c_void, size_t};

use crate::{
    base::CGFloat,
    color::CGColor,
    color_space::{CGColorSpace, CGColorSpaceRef},
};

#[repr(C)]
pub struct __CGGradient(c_void);

pub type CGGradientRef = *const __CGGradient;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct CGGradientDrawingOptions: u32 {
        #[doc(alias = "kCGGradientDrawsBeforeStartLocation")]
        const BeforeStartLocation = (1 << 0);
        #[doc(alias = "kCGGradientDrawsAfterEndLocation")]
        const AfterEndLocation = (1 << 1);
    }
}

extern "C" {
    pub fn CGGradientGetTypeID() -> CFTypeID;
    pub fn CGGradientCreateWithColorComponents(
        space: CGColorSpaceRef,
        components: *const CGFloat,
        locations: *const CGFloat,
        count: size_t,
    ) -> CGGradientRef;
    pub fn CGGradientCreateWithColors(space: CGColorSpaceRef, colors: CFArrayRef, locations: *const CGFloat) -> CGGradientRef;
    pub fn CGGradientRetain(gradient: CGGradientRef) -> CGGradientRef;
    pub fn CGGradientRelease(gradient: CGGradientRef);
}

pub struct CGGradient(CGGradientRef);

impl Drop for CGGradient {
    fn drop(&mut self) {
        unsafe { CGGradientRelease(self.0) }
    }
}

impl_TCFType!(CGGradient, CGGradientRef, CGGradientGetTypeID);
impl_CFTypeDescription!(CGGradient);

impl CGGradient {
    pub fn from_color_components(
        color_space: Option<&CGColorSpace>,
        components: Option<&[CGFloat]>,
        locations: Option<&[CGFloat]>,
    ) -> Option<CGGradient> {
        unsafe {
            let gradient = CGGradientCreateWithColorComponents(
                color_space.map_or(null_mut(), |cs| cs.as_concrete_TypeRef()),
                components.map_or(null(), |c| c.as_ptr()),
                locations.map_or(null(), |l| l.as_ptr()),
                locations.map_or(0, |l| l.len() as size_t),
            );
            if gradient.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(gradient))
            }
        }
    }

    pub fn from_colors(color_space: Option<&CGColorSpace>, colors: Option<&CFArray<CGColor>>, locations: Option<&[CGFloat]>) -> Option<CGGradient> {
        unsafe {
            let gradient = CGGradientCreateWithColors(
                color_space.map_or(null_mut(), |cs| cs.as_concrete_TypeRef()),
                colors.map_or(null(), |c| c.as_concrete_TypeRef()),
                locations.map_or(null(), |l| l.as_ptr()),
            );
            if gradient.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(gradient))
            }
        }
    }
}

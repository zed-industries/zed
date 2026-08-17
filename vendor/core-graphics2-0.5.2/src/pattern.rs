use core_foundation::{
    base::{CFTypeID, TCFType},
    impl_CFTypeDescription, impl_TCFType,
};
use libc::c_void;

use crate::{affine_transform::CGAffineTransform, base::CGFloat, context::CGContextRef, geometry::CGRect};

#[repr(C)]
pub struct __CGPattern(c_void);

pub type CGPatternRef = *const __CGPattern;

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGPatternTiling {
    #[doc(alias = "kCGPatternTilingNoDistortion")]
    NoDistortion,
    #[doc(alias = "kCGPatternTilingConstantSpacingMinimalDistortion")]
    ConstantSpacingMinimalDistortion,
    #[doc(alias = "kCGPatternTilingConstantSpacing")]
    ConstantSpacing,
}

pub type CGPatternDrawPatternCallback = extern "C" fn(*mut c_void, CGContextRef);
pub type CGPatternReleaseInfoCallback = extern "C" fn(*mut c_void);

pub struct CGPatternCallbacks {
    pub version: u32,
    pub drawPattern: CGPatternDrawPatternCallback,
    pub releaseInfo: CGPatternReleaseInfoCallback,
}

extern "C" {
    pub fn CGPatternGetTypeID() -> CFTypeID;
    pub fn CGPatternCreate(
        info: *mut c_void,
        bounds: CGRect,
        matrix: CGAffineTransform,
        xStep: CGFloat,
        yStep: CGFloat,
        tiling: CGPatternTiling,
        isColored: i32,
        callbacks: *const CGPatternCallbacks,
    ) -> CGPatternRef;
    pub fn CGPatternRetain(pattern: CGPatternRef) -> CGPatternRef;
    pub fn CGPatternRelease(pattern: CGPatternRef);
}

pub struct CGPattern(CGPatternRef);

impl Drop for CGPattern {
    fn drop(&mut self) {
        unsafe { CGPatternRelease(self.0) }
    }
}

impl_TCFType!(CGPattern, CGPatternRef, CGPatternGetTypeID);
impl_CFTypeDescription!(CGPattern);

impl CGPattern {
    pub unsafe fn new(
        info: *mut c_void,
        bounds: CGRect,
        matrix: CGAffineTransform,
        xStep: CGFloat,
        yStep: CGFloat,
        tiling: CGPatternTiling,
        is_colored: i32,
        callbacks: Option<&CGPatternCallbacks>,
    ) -> Self {
        unsafe {
            let pattern =
                CGPatternCreate(info, bounds, matrix, xStep, yStep, tiling, is_colored, callbacks.map_or(std::ptr::null(), |c| c as *const _));
            TCFType::wrap_under_create_rule(pattern)
        }
    }
}

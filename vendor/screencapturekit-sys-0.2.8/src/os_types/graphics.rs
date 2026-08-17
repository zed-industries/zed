#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::upper_case_acronyms)]

use objc::{runtime::Class, *};
use objc_foundation::INSObject;
use objc_id::Id;

use super::base::CGFloat;

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct CGColor {
    _unused: [u8; 0],
}
unsafe impl Message for CGColor {}
impl INSObject for CGColor {
    fn class() -> &'static Class {
        Class::get("CGColor")
            .expect("Missing CGColor class, check that the binary is linked with CoreGraphics")
    }
}

impl CGColor {
    pub fn rgb(red: CGFloat, green: CGFloat, blue: CGFloat, alpha: CGFloat) -> Id<Self> {
        unsafe {
            let ptr = CGColorCreateGenericRGB(red, green, blue, alpha);
            Id::from_ptr(ptr)
        }
    }
}

pub type CGColorRef = *mut CGColor;

extern "C" {
    pub fn CGColorCreateGenericRGB(
        red: CGFloat,
        green: CGFloat,
        blue: CGFloat,
        alpha: CGFloat,
    ) -> CGColorRef;
}

// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::sys::CGColorRef;
use crate::base::CGFloat;
use core_foundation::base::CFTypeID;
use core_foundation::base::TCFType;
use core_foundation::{declare_TCFType, impl_TCFType};

pub use super::sys::CGColorRef as SysCGColorRef;

declare_TCFType! {
    CGColor, CGColorRef
}
impl_TCFType!(CGColor, CGColorRef, CGColorGetTypeID);

impl CGColor {
    pub fn rgb(red: CGFloat, green: CGFloat, blue: CGFloat, alpha: CGFloat) -> Self {
        unsafe {
            let ptr = CGColorCreateGenericRGB(red, green, blue, alpha);
            CGColor::wrap_under_create_rule(ptr)
        }
    }
}

#[cfg_attr(feature = "link", link(name = "CoreGraphics", kind = "framework"))]
extern "C" {
    fn CGColorCreateGenericRGB(
        red: CGFloat,
        green: CGFloat,
        blue: CGFloat,
        alpha: CGFloat,
    ) -> crate::sys::CGColorRef;
    fn CGColorGetTypeID() -> CFTypeID;
}

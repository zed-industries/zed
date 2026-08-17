// Copyright 2016 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Evil private APIs.
//!
//! These are liable to change at any time. Use with caution!

use crate::geometry::CGRect;
use libc::{c_int, c_uint};
use std::ptr;

pub struct CGSRegion {
    region: ffi::CGSRegionRef,
}

impl Drop for CGSRegion {
    fn drop(&mut self) {
        unsafe { ffi::CGSRegionRelease(self.region) }
    }
}

impl CGSRegion {
    #[inline]
    pub fn from_rect(rect: &CGRect) -> CGSRegion {
        unsafe {
            let mut region = ptr::null_mut();
            assert!(ffi::CGSNewRegionWithRect(rect, &mut region) == 0);
            CGSRegion { region }
        }
    }

    #[inline]
    pub fn from_rects(rects: &[CGRect]) -> CGSRegion {
        unsafe {
            let mut region = ptr::null_mut();
            assert!(
                ffi::CGSNewRegionWithRectList(rects.as_ptr(), rects.len() as c_uint, &mut region)
                    == 0
            );
            CGSRegion { region }
        }
    }
}

/// This should always be memory-safe; the window server rejects any invalid surface IDs.
pub struct CGSSurface {
    context_id: c_uint,
    window_number: c_int,
    surface_id: c_uint,
}

impl CGSSurface {
    #[inline]
    pub fn from_ids(context_id: c_uint, window_number: c_int, surface_id: c_uint) -> CGSSurface {
        CGSSurface {
            context_id,
            window_number,
            surface_id,
        }
    }

    #[inline]
    pub fn id(&self) -> c_uint {
        self.surface_id
    }

    #[inline]
    pub fn set_shape(&self, region: &CGSRegion) {
        unsafe {
            assert!(
                ffi::CGSSetSurfaceShape(
                    self.context_id,
                    self.window_number,
                    self.surface_id,
                    region.region
                ) == 0
            )
        }
    }
}

mod ffi {
    use crate::geometry::CGRect;
    use libc::{c_int, c_uint};

    // This is an enum so that we can't easily make instances of this opaque type.
    pub enum CGSRegionObject {}

    pub type CGError = OSStatus;
    pub type CGSRegionRef = *mut CGSRegionObject;
    pub type OSStatus = i32;

    #[cfg_attr(feature = "link", link(name = "CoreGraphics", kind = "framework"))]
    extern "C" {
        pub fn CGSRegionRelease(region: CGSRegionRef);
        pub fn CGSNewRegionWithRect(rect: *const CGRect, outRegion: *mut CGSRegionRef) -> CGError;
        pub fn CGSNewRegionWithRectList(
            rects: *const CGRect,
            rectCount: c_uint,
            outRegion: *mut CGSRegionRef,
        ) -> CGError;

        pub fn CGSSetSurfaceShape(
            contextID: c_uint,
            windowNumber: c_int,
            surfaceID: c_uint,
            region: CGSRegionRef,
        ) -> CGError;
    }
}

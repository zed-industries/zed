// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::line::CTLine;
use core_foundation::array::{CFArray, CFArrayRef};
use core_foundation::base::{CFRange, CFTypeID, TCFType};
use core_foundation::{declare_TCFType, impl_CFTypeDescription, impl_TCFType};
use core_graphics::context::{CGContext, CGContextRef};
use core_graphics::geometry::CGPoint;
use core_graphics::path::{CGPath, SysCGPathRef};
use foreign_types::{ForeignType, ForeignTypeRef};
use std::os::raw::c_void;

#[repr(C)]
pub struct __CTFrame(c_void);

pub type CTFrameRef = *const __CTFrame;

declare_TCFType! {
    CTFrame, CTFrameRef
}
impl_TCFType!(CTFrame, CTFrameRef, CTFrameGetTypeID);
impl_CFTypeDescription!(CTFrame);

impl CTFrame {
    /// The `CGPath` used to create this `CTFrame`.
    pub fn get_path(&self) -> CGPath {
        unsafe { CGPath::from_ptr(CTFrameGetPath(self.as_concrete_TypeRef())).clone() }
    }

    /// Returns an owned copy of the underlying lines.
    ///
    /// Each line is retained, and will remain valid past the life of this `CTFrame`.
    pub fn get_lines(&self) -> Vec<CTLine> {
        unsafe {
            let array_ref = CTFrameGetLines(self.as_concrete_TypeRef());
            let array: CFArray<CTLine> = CFArray::wrap_under_get_rule(array_ref);
            array
                .iter()
                .map(|l| CTLine::wrap_under_get_rule(l.as_concrete_TypeRef()))
                .collect()
        }
    }

    /// Return the origin of each line in a given range.
    ///
    /// If no range is provided, returns the origin of each line in the frame.
    ///
    /// If the length of the range is 0, returns the origin of all lines from
    /// the range's start to the end.
    ///
    /// The origin is the position relative to the path used to create this `CTFFrame`;
    /// to get the path use [`get_path`].
    ///
    /// [`get_path`]: #method.get_path
    pub fn get_line_origins(&self, range: impl Into<Option<CFRange>>) -> Vec<CGPoint> {
        let range = range.into().unwrap_or_else(|| CFRange::init(0, 0));
        let len = match range.length {
            // range length of 0 means 'all remaining lines'
            0 => unsafe {
                let array_ref = CTFrameGetLines(self.as_concrete_TypeRef());
                let array: CFArray<CTLine> = CFArray::wrap_under_get_rule(array_ref);
                array.len() - range.location
            },
            n => n,
        };
        let len = len.max(0) as usize;
        let mut out = vec![CGPoint::new(0., 0.); len];
        unsafe {
            CTFrameGetLineOrigins(self.as_concrete_TypeRef(), range, out.as_mut_ptr());
        }
        out
    }

    pub fn draw(&self, context: &CGContextRef) {
        unsafe {
            CTFrameDraw(self.as_concrete_TypeRef(), context.as_ptr());
        }
    }
}

#[cfg_attr(feature = "link", link(name = "CoreText", kind = "framework"))]
extern "C" {
    fn CTFrameGetTypeID() -> CFTypeID;
    fn CTFrameGetLines(frame: CTFrameRef) -> CFArrayRef;
    fn CTFrameDraw(frame: CTFrameRef, context: *mut <CGContext as ForeignType>::CType);
    fn CTFrameGetLineOrigins(frame: CTFrameRef, range: CFRange, origins: *mut CGPoint);
    fn CTFrameGetPath(frame: CTFrameRef) -> SysCGPathRef;
}

// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::run::CTRun;
use core_foundation::array::{CFArray, CFArrayRef};
use core_foundation::attributed_string::CFAttributedStringRef;
use core_foundation::base::{CFIndex, CFRange, CFTypeID, TCFType};
use core_foundation::{declare_TCFType, impl_CFTypeDescription, impl_TCFType};
use core_graphics::base::CGFloat;
use core_graphics::context::CGContext;
use core_graphics::geometry::{CGPoint, CGRect};
use foreign_types::ForeignType;
use std::os::raw::c_void;

#[repr(C)]
pub struct __CTLine(c_void);

pub type CTLineRef = *const __CTLine;

declare_TCFType! {
    CTLine, CTLineRef
}
impl_TCFType!(CTLine, CTLineRef, CTLineGetTypeID);
impl_CFTypeDescription!(CTLine);

/// Metrics for a given line.
pub struct TypographicBounds {
    pub width: CGFloat,
    pub ascent: CGFloat,
    pub descent: CGFloat,
    pub leading: CGFloat,
}

impl CTLine {
    pub fn new_with_attributed_string(string: CFAttributedStringRef) -> Self {
        unsafe {
            let ptr = CTLineCreateWithAttributedString(string);
            CTLine::wrap_under_create_rule(ptr)
        }
    }

    pub fn glyph_runs(&self) -> CFArray<CTRun> {
        unsafe { TCFType::wrap_under_get_rule(CTLineGetGlyphRuns(self.0)) }
    }

    pub fn get_string_range(&self) -> CFRange {
        unsafe { CTLineGetStringRange(self.as_concrete_TypeRef()) }
    }

    pub fn draw(&self, context: &CGContext) {
        unsafe { CTLineDraw(self.as_concrete_TypeRef(), context.as_ptr()) }
    }

    pub fn get_image_bounds(&self, context: &CGContext) -> CGRect {
        unsafe { CTLineGetImageBounds(self.as_concrete_TypeRef(), context.as_ptr()) }
    }

    pub fn get_typographic_bounds(&self) -> TypographicBounds {
        let mut ascent = 0.0;
        let mut descent = 0.0;
        let mut leading = 0.0;
        unsafe {
            let width = CTLineGetTypographicBounds(
                self.as_concrete_TypeRef(),
                &mut ascent,
                &mut descent,
                &mut leading,
            );
            TypographicBounds {
                width,
                ascent,
                descent,
                leading,
            }
        }
    }

    pub fn get_string_index_for_position(&self, position: CGPoint) -> CFIndex {
        unsafe { CTLineGetStringIndexForPosition(self.as_concrete_TypeRef(), position) }
    }

    pub fn get_string_offset_for_string_index(&self, charIndex: CFIndex) -> CGFloat {
        unsafe {
            CTLineGetOffsetForStringIndex(self.as_concrete_TypeRef(), charIndex, std::ptr::null())
        }
    }
}

#[cfg_attr(feature = "link", link(name = "CoreText", kind = "framework"))]
extern "C" {
    fn CTLineGetTypeID() -> CFTypeID;
    fn CTLineGetGlyphRuns(line: CTLineRef) -> CFArrayRef;
    fn CTLineGetStringRange(line: CTLineRef) -> CFRange;

    // Creating Lines
    fn CTLineCreateWithAttributedString(string: CFAttributedStringRef) -> CTLineRef;

    // Drawing the Line
    fn CTLineDraw(line: CTLineRef, context: *const core_graphics::sys::CGContext);

    // Measuring Lines
    fn CTLineGetImageBounds(
        line: CTLineRef,
        context: *const core_graphics::sys::CGContext,
    ) -> CGRect;
    fn CTLineGetTypographicBounds(
        line: CTLineRef,
        ascent: *mut CGFloat,
        descent: *mut CGFloat,
        leading: *mut CGFloat,
    ) -> CGFloat;

    // Getting Line Positioning
    fn CTLineGetStringIndexForPosition(line: CTLineRef, position: CGPoint) -> CFIndex;
    fn CTLineGetOffsetForStringIndex(
        line: CTLineRef,
        charIndex: CFIndex,
        secondaryOffset: *const CGFloat,
    ) -> CGFloat;
}

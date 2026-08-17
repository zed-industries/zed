// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core_foundation::base::{CFIndex, CFRange, CFType, CFTypeID, TCFType};
use core_foundation::dictionary::{CFDictionary, CFDictionaryRef};
use core_foundation::string::CFString;
use core_foundation::{declare_TCFType, impl_CFTypeDescription, impl_TCFType};
use core_graphics::base::CGFloat;
use core_graphics::font::CGGlyph;
use core_graphics::geometry::CGPoint;
use std::borrow::Cow;
use std::os::raw::c_void;
use std::slice;

use crate::line::TypographicBounds;

#[repr(C)]
pub struct __CTRun(c_void);

pub type CTRunRef = *const __CTRun;

declare_TCFType! {
    CTRun, CTRunRef
}
impl_TCFType!(CTRun, CTRunRef, CTRunGetTypeID);
impl_CFTypeDescription!(CTRun);

impl CTRun {
    pub fn attributes(&self) -> Option<CFDictionary<CFString, CFType>> {
        unsafe {
            let attrs = CTRunGetAttributes(self.0);
            if attrs.is_null() {
                return None;
            }
            Some(TCFType::wrap_under_get_rule(attrs))
        }
    }
    pub fn glyph_count(&self) -> CFIndex {
        unsafe { CTRunGetGlyphCount(self.0) }
    }

    pub fn glyphs(&self) -> Cow<[CGGlyph]> {
        unsafe {
            // CTRunGetGlyphsPtr can return null under some not understood circumstances.
            // If it does the Apple documentation tells us to allocate our own buffer and call
            // CTRunGetGlyphs
            let count = CTRunGetGlyphCount(self.0);
            let glyphs_ptr = CTRunGetGlyphsPtr(self.0);
            if !glyphs_ptr.is_null() {
                Cow::from(slice::from_raw_parts(glyphs_ptr, count as usize))
            } else {
                let mut vec = Vec::with_capacity(count as usize);
                // "If the length of the range is set to 0, then the copy operation will continue
                // from the start index of the range to the end of the run"
                CTRunGetGlyphs(self.0, CFRange::init(0, 0), vec.as_mut_ptr());
                vec.set_len(count as usize);
                Cow::from(vec)
            }
        }
    }

    pub fn positions(&self) -> Cow<[CGPoint]> {
        unsafe {
            // CTRunGetPositionsPtr can return null under some not understood circumstances.
            // If it does the Apple documentation tells us to allocate our own buffer and call
            // CTRunGetPositions
            let count = CTRunGetGlyphCount(self.0);
            let positions_ptr = CTRunGetPositionsPtr(self.0);
            if !positions_ptr.is_null() {
                Cow::from(slice::from_raw_parts(positions_ptr, count as usize))
            } else {
                let mut vec = Vec::with_capacity(count as usize);
                // "If the length of the range is set to 0, then the copy operation will continue
                // from the start index of the range to the end of the run"
                CTRunGetPositions(self.0, CFRange::init(0, 0), vec.as_mut_ptr());
                vec.set_len(count as usize);
                Cow::from(vec)
            }
        }
    }

    pub fn get_typographic_bounds(&self) -> TypographicBounds {
        let mut ascent = 0.0;
        let mut descent = 0.0;
        let mut leading = 0.0;
        unsafe {
            // The portion of the run to calculate the typographic bounds for. By setting this to 0,
            // CoreText will measure the bounds from start to end, see https://developer.apple.com/documentation/coretext/1510569-ctrungettypographicbounds?language=objc.
            let range = CFRange {
                location: 0,
                length: 0,
            };

            let width = CTRunGetTypographicBounds(
                self.as_concrete_TypeRef(),
                range,
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

    pub fn string_indices(&self) -> Cow<[CFIndex]> {
        unsafe {
            // CTRunGetStringIndicesPtr can return null under some not understood circumstances.
            // If it does the Apple documentation tells us to allocate our own buffer and call
            // CTRunGetStringIndices
            let count = CTRunGetGlyphCount(self.0);
            let indices_ptr = CTRunGetStringIndicesPtr(self.0);
            if !indices_ptr.is_null() {
                Cow::from(slice::from_raw_parts(indices_ptr, count as usize))
            } else {
                let mut vec = Vec::with_capacity(count as usize);
                // "If the length of the range is set to 0, then the copy operation will continue
                // from the start index of the range to the end of the run"
                CTRunGetStringIndices(self.0, CFRange::init(0, 0), vec.as_mut_ptr());
                vec.set_len(count as usize);
                Cow::from(vec)
            }
        }
    }
}

#[test]
fn create_runs() {
    use crate::font;
    use crate::line::*;
    use crate::string_attributes::*;
    use core_foundation::attributed_string::CFMutableAttributedString;
    let mut string = CFMutableAttributedString::new();
    string.replace_str(&CFString::new("Food"), CFRange::init(0, 0));
    let len = string.char_len();
    unsafe {
        string.set_attribute(
            CFRange::init(0, len),
            kCTFontAttributeName,
            &font::new_from_name("Helvetica", 16.).unwrap(),
        );
    }
    let line = CTLine::new_with_attributed_string(string.as_concrete_TypeRef());
    let runs = line.glyph_runs();
    assert_eq!(runs.len(), 1);
    for run in runs.iter() {
        assert_eq!(run.glyph_count(), 4);
        let font = run
            .attributes()
            .unwrap()
            .get(CFString::new("NSFont"))
            .downcast::<font::CTFont>()
            .unwrap();
        assert_eq!(font.pt_size(), 16.);

        let positions = run.positions();
        assert_eq!(positions.len(), 4);
        assert!(positions[0].x < positions[1].x);

        let glyphs = run.glyphs();
        assert_eq!(glyphs.len(), 4);
        assert_ne!(glyphs[0], glyphs[1]);
        assert_eq!(glyphs[1], glyphs[2]);

        let indices = run.string_indices();
        assert_eq!(indices.as_ref(), &[0, 1, 2, 3]);
    }
}

#[cfg_attr(feature = "link", link(name = "CoreText", kind = "framework"))]
extern "C" {
    fn CTRunGetTypeID() -> CFTypeID;
    fn CTRunGetAttributes(run: CTRunRef) -> CFDictionaryRef;
    fn CTRunGetGlyphCount(run: CTRunRef) -> CFIndex;
    fn CTRunGetPositionsPtr(run: CTRunRef) -> *const CGPoint;
    fn CTRunGetPositions(run: CTRunRef, range: CFRange, buffer: *const CGPoint);
    fn CTRunGetStringIndicesPtr(run: CTRunRef) -> *const CFIndex;
    fn CTRunGetStringIndices(run: CTRunRef, range: CFRange, buffer: *const CFIndex);
    fn CTRunGetGlyphsPtr(run: CTRunRef) -> *const CGGlyph;
    fn CTRunGetGlyphs(run: CTRunRef, range: CFRange, buffer: *const CGGlyph);
    fn CTRunGetTypographicBounds(
        line: CTRunRef,
        range: CFRange,
        ascent: *mut CGFloat,
        descent: *mut CGFloat,
        leading: *mut CGFloat,
    ) -> CGFloat;
}

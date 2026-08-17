// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::data_provider::CGDataProvider;
use crate::geometry::CGRect;
use core_foundation::array::{CFArray, CFArrayRef};
use core_foundation::base::{CFRelease, CFRetain, CFType, CFTypeID, TCFType};
use core_foundation::data::{CFData, CFDataRef};
use core_foundation::dictionary::{CFDictionary, CFDictionaryRef};
use core_foundation::number::CFNumber;
use core_foundation::string::{CFString, CFStringRef};
use std::ptr::NonNull;

use foreign_types::{foreign_type, ForeignType};

use libc::{c_int, size_t};

pub use core_graphics_types::base::CGGlyph;

foreign_type! {
    #[doc(hidden)]
    pub unsafe type CGFont: Send + Sync {
        type CType = crate::sys::CGFont;
        fn drop = |p| CFRelease(p as *mut _);
        fn clone = |p| CFRetain(p as *const _) as *mut _;
    }
}

impl CGFont {
    pub fn type_id() -> CFTypeID {
        unsafe { CGFontGetTypeID() }
    }

    pub fn from_data_provider(provider: CGDataProvider) -> Result<CGFont, ()> {
        unsafe {
            let font_ref = CGFontCreateWithDataProvider(provider.as_ptr());
            match NonNull::new(font_ref) {
                Some(font_ref) => Ok(CGFont(font_ref)),
                None => Err(()),
            }
        }
    }

    pub fn from_name(name: &CFString) -> Result<CGFont, ()> {
        unsafe {
            let font_ref = CGFontCreateWithFontName(name.as_concrete_TypeRef());
            match NonNull::new(font_ref) {
                Some(font_ref) => Ok(CGFont(font_ref)),
                None => Err(()),
            }
        }
    }

    pub fn create_copy_from_variations(
        &self,
        vars: &CFDictionary<CFString, CFNumber>,
    ) -> Result<CGFont, ()> {
        unsafe {
            let font_ref =
                CGFontCreateCopyWithVariations(self.as_ptr(), vars.as_concrete_TypeRef());
            match NonNull::new(font_ref) {
                Some(font_ref) => Ok(CGFont(font_ref)),
                None => Err(()),
            }
        }
    }

    pub fn postscript_name(&self) -> CFString {
        unsafe {
            let string_ref = CGFontCopyPostScriptName(self.as_ptr());
            TCFType::wrap_under_create_rule(string_ref)
        }
    }

    pub fn get_glyph_b_boxes(&self, glyphs: &[CGGlyph], bboxes: &mut [CGRect]) -> bool {
        unsafe {
            assert!(bboxes.len() >= glyphs.len());
            CGFontGetGlyphBBoxes(
                self.as_ptr(),
                glyphs.as_ptr(),
                glyphs.len(),
                bboxes.as_mut_ptr(),
            )
        }
    }

    pub fn get_glyph_advances(&self, glyphs: &[CGGlyph], advances: &mut [c_int]) -> bool {
        unsafe {
            assert!(advances.len() >= glyphs.len());
            CGFontGetGlyphAdvances(
                self.as_ptr(),
                glyphs.as_ptr(),
                glyphs.len(),
                advances.as_mut_ptr(),
            )
        }
    }

    pub fn ascent(&self) -> c_int {
        unsafe { CGFontGetAscent(self.as_ptr()) }
    }

    pub fn descent(&self) -> c_int {
        unsafe { CGFontGetDescent(self.as_ptr()) }
    }

    pub fn leading(&self) -> c_int {
        unsafe { CGFontGetLeading(self.as_ptr()) }
    }

    pub fn cap_height(&self) -> c_int {
        unsafe { CGFontGetCapHeight(self.as_ptr()) }
    }

    pub fn x_height(&self) -> c_int {
        unsafe { CGFontGetXHeight(self.as_ptr()) }
    }

    pub fn get_units_per_em(&self) -> c_int {
        unsafe { CGFontGetUnitsPerEm(self.as_ptr()) }
    }

    pub fn copy_table_tags(&self) -> CFArray<u32> {
        unsafe { TCFType::wrap_under_create_rule(CGFontCopyTableTags(self.as_ptr())) }
    }

    pub fn copy_table_for_tag(&self, tag: u32) -> Option<CFData> {
        let data_ref = unsafe { CGFontCopyTableForTag(self.as_ptr(), tag) };
        if !data_ref.is_null() {
            Some(unsafe { TCFType::wrap_under_create_rule(data_ref) })
        } else {
            None
        }
    }

    pub fn copy_variations(&self) -> Option<CFDictionary<CFString, CFNumber>> {
        let variations = unsafe { CGFontCopyVariations(self.as_ptr()) };
        if !variations.is_null() {
            Some(unsafe { TCFType::wrap_under_create_rule(variations) })
        } else {
            None
        }
    }

    pub fn copy_variation_axes(&self) -> Option<CFArray<CFDictionary<CFString, CFType>>> {
        let axes = unsafe { CGFontCopyVariationAxes(self.as_ptr()) };
        if !axes.is_null() {
            Some(unsafe { TCFType::wrap_under_create_rule(axes) })
        } else {
            None
        }
    }
}

#[cfg_attr(feature = "link", link(name = "CoreGraphics", kind = "framework"))]
extern "C" {
    // TODO: basically nothing has bindings (even commented-out) besides what we use.
    fn CGFontCreateWithDataProvider(
        provider: crate::sys::CGDataProviderRef,
    ) -> crate::sys::CGFontRef;
    fn CGFontCreateWithFontName(name: CFStringRef) -> crate::sys::CGFontRef;
    fn CGFontCreateCopyWithVariations(
        font: crate::sys::CGFontRef,
        vars: CFDictionaryRef,
    ) -> crate::sys::CGFontRef;
    fn CGFontGetTypeID() -> CFTypeID;

    fn CGFontCopyPostScriptName(font: crate::sys::CGFontRef) -> CFStringRef;

    // These do the same thing as CFRetain/CFRelease, except
    // gracefully handle a NULL argument. We don't use them.
    //fn CGFontRetain(font: ::sys::CGFontRef);
    //fn CGFontRelease(font: ::sys::CGFontRef);

    fn CGFontGetGlyphBBoxes(
        font: crate::sys::CGFontRef,
        glyphs: *const CGGlyph,
        count: size_t,
        bboxes: *mut CGRect,
    ) -> bool;
    fn CGFontGetGlyphAdvances(
        font: crate::sys::CGFontRef,
        glyphs: *const CGGlyph,
        count: size_t,
        advances: *mut c_int,
    ) -> bool;

    fn CGFontGetAscent(font: crate::sys::CGFontRef) -> c_int;
    fn CGFontGetDescent(font: crate::sys::CGFontRef) -> c_int;
    fn CGFontGetLeading(font: crate::sys::CGFontRef) -> c_int;
    fn CGFontGetCapHeight(font: crate::sys::CGFontRef) -> c_int;
    fn CGFontGetXHeight(font: crate::sys::CGFontRef) -> c_int;
    fn CGFontGetUnitsPerEm(font: crate::sys::CGFontRef) -> c_int;

    fn CGFontCopyTableTags(font: crate::sys::CGFontRef) -> CFArrayRef;
    fn CGFontCopyTableForTag(font: crate::sys::CGFontRef, tag: u32) -> CFDataRef;
    fn CGFontCopyVariations(font: crate::sys::CGFontRef) -> CFDictionaryRef;
    fn CGFontCopyVariationAxes(font: crate::sys::CGFontRef) -> CFArrayRef;
}

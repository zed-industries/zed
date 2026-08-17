use std::ptr::null;

use core_foundation::{
    array::{CFArray, CFArrayRef},
    base::{CFType, CFTypeID, TCFType},
    data::{CFData, CFDataRef},
    dictionary::{CFDictionary, CFDictionaryRef},
    impl_CFTypeDescription, impl_TCFType,
    number::CFNumber,
    string::{CFString, CFStringRef},
};
use libc::{c_int, c_ushort, c_void, size_t};

use crate::{
    base::CGFloat,
    data_provider::{CGDataProvider, CGDataProviderRef},
    geometry::CGRect,
};

#[repr(C)]
pub struct __CGFont(c_void);

pub type CGFontRef = *mut __CGFont;

pub type CGFontIndex = c_ushort;

pub type CGGlyph = CGFontIndex;

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGFontPostScriptFormat {
    #[doc(alias = "kCGFontPostScriptFormatType1")]
    Type1  = 1,
    #[doc(alias = "kCGFontPostScriptFormatType3")]
    Type3  = 3,
    #[doc(alias = "kCGFontPostScriptFormatType42")]
    Type42 = 42,
}

pub const kCGFontIndexMax: CGFontIndex = u16::MAX - 1;
pub const kCGFontIndexInvalid: CGFontIndex = u16::MAX;
pub const kCGGlyphMax: CGFontIndex = kCGFontIndexMax;

extern "C" {
    pub fn CGFontGetTypeID() -> CFTypeID;
    pub fn CGFontCreateWithPlatformFont(platformFontReference: *mut c_void) -> CGFontRef;
    pub fn CGFontCreateWithDataProvider(provider: CGDataProviderRef) -> CGFontRef;
    pub fn CGFontCreateWithFontName(name: CFStringRef) -> CGFontRef;
    pub fn CGFontCreateCopyWithVariations(font: CGFontRef, variations: CFDictionaryRef) -> CGFontRef;
    pub fn CGFontRetain(font: CGFontRef) -> CGFontRef;
    pub fn CGFontRelease(font: CGFontRef);
    pub fn CGFontGetNumberOfGlyphs(font: CGFontRef) -> size_t;
    pub fn CGFontGetUnitsPerEm(font: CGFontRef) -> c_int;
    pub fn CGFontCopyPostScriptName(font: CGFontRef) -> CFStringRef;
    pub fn CGFontCopyFullName(font: CGFontRef) -> CFStringRef;
    pub fn CGFontGetAscent(font: CGFontRef) -> c_int;
    pub fn CGFontGetDescent(font: CGFontRef) -> c_int;
    pub fn CGFontGetLeading(font: CGFontRef) -> c_int;
    pub fn CGFontGetCapHeight(font: CGFontRef) -> c_int;
    pub fn CGFontGetXHeight(font: CGFontRef) -> c_int;
    pub fn CGFontGetFontBBox(font: CGFontRef) -> CGRect;
    pub fn CGFontGetItalicAngle(font: CGFontRef) -> CGFloat;
    pub fn CGFontGetStemV(font: CGFontRef) -> CGFloat;
    pub fn CGFontCopyVariationAxes(font: CGFontRef) -> CFArrayRef;
    pub fn CGFontCopyVariations(font: CGFontRef) -> CFDictionaryRef;
    pub fn CGFontGetGlyphAdvances(font: CGFontRef, glyphs: *const CGGlyph, count: size_t, advances: *mut c_int) -> bool;
    pub fn CGFontGetGlyphBBoxes(font: CGFontRef, glyphs: *const CGGlyph, count: size_t, bboxes: *mut CGRect) -> bool;
    pub fn CGFontGetGlyphWithGlyphName(font: CGFontRef, name: CFStringRef) -> CGGlyph;
    pub fn CGFontCopyGlyphNameForGlyph(font: CGFontRef, glyph: CGGlyph) -> CFStringRef;
    pub fn CGFontCanCreatePostScriptSubset(font: CGFontRef, format: CGFontPostScriptFormat) -> bool;
    pub fn CGFontCreatePostScriptSubset(
        font: CGFontRef,
        subsetName: CFStringRef,
        format: CGFontPostScriptFormat,
        glyphs: *const CGGlyph,
        count: size_t,
        encoding: *const CGGlyph,
    ) -> CFDataRef;
    pub fn CGFontCreatePostScriptEncoding(font: CGFontRef, encoding: *const CGGlyph) -> CFDataRef;
    pub fn CGFontCopyTableTags(font: CGFontRef) -> CFArrayRef;
    pub fn CGFontCopyTableForTag(font: CGFontRef, tag: u32) -> CFDataRef;

    pub static kCGFontVariationAxisName: CFStringRef;
    pub static kCGFontVariationAxisMinValue: CFStringRef;
    pub static kCGFontVariationAxisMaxValue: CFStringRef;
    pub static kCGFontVariationAxisDefaultValue: CFStringRef;
}

pub struct CGFont(CGFontRef);

impl Drop for CGFont {
    fn drop(&mut self) {
        unsafe { CGFontRelease(self.0) }
    }
}

impl_TCFType!(CGFont, CGFontRef, CGFontGetTypeID);
impl_CFTypeDescription!(CGFont);

impl CGFont {
    pub fn from_data_provider(provider: &CGDataProvider) -> Option<Self> {
        unsafe {
            let font = CGFontCreateWithDataProvider(provider.as_concrete_TypeRef());
            if font.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(font))
            }
        }
    }

    pub fn from_name(name: &CFString) -> Option<Self> {
        unsafe {
            let font = CGFontCreateWithFontName(name.as_concrete_TypeRef());
            if font.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(font))
            }
        }
    }

    pub fn new_copy_from_variations(&self, variations: Option<&CFDictionary<CFString, CFNumber>>) -> Option<CGFont> {
        unsafe {
            let font = CGFontCreateCopyWithVariations(self.as_concrete_TypeRef(), variations.as_ref().map_or(null(), |v| v.as_concrete_TypeRef()));
            if font.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(font))
            }
        }
    }

    pub fn number_of_glyphs(&self) -> usize {
        unsafe { CGFontGetNumberOfGlyphs(self.as_concrete_TypeRef()) }
    }

    pub fn units_per_em(&self) -> i32 {
        unsafe { CGFontGetUnitsPerEm(self.as_concrete_TypeRef()) as i32 }
    }

    pub fn copy_post_script_name(&self) -> CFString {
        unsafe {
            let name = CGFontCopyPostScriptName(self.as_concrete_TypeRef());
            TCFType::wrap_under_create_rule(name)
        }
    }

    pub fn copy_full_name(&self) -> CFString {
        unsafe {
            let name = CGFontCopyFullName(self.as_concrete_TypeRef());
            TCFType::wrap_under_create_rule(name)
        }
    }

    pub fn ascent(&self) -> i32 {
        unsafe { CGFontGetAscent(self.as_concrete_TypeRef()) as i32 }
    }

    pub fn descent(&self) -> i32 {
        unsafe { CGFontGetDescent(self.as_concrete_TypeRef()) as i32 }
    }

    pub fn leading(&self) -> i32 {
        unsafe { CGFontGetLeading(self.as_concrete_TypeRef()) as i32 }
    }

    pub fn cap_height(&self) -> i32 {
        unsafe { CGFontGetCapHeight(self.as_concrete_TypeRef()) as i32 }
    }

    pub fn x_height(&self) -> i32 {
        unsafe { CGFontGetXHeight(self.as_concrete_TypeRef()) as i32 }
    }

    pub fn font_b_box(&self) -> CGRect {
        unsafe { CGFontGetFontBBox(self.as_concrete_TypeRef()) }
    }

    pub fn italic_angle(&self) -> CGFloat {
        unsafe { CGFontGetItalicAngle(self.as_concrete_TypeRef()) }
    }

    pub fn stem_v(&self) -> CGFloat {
        unsafe { CGFontGetStemV(self.as_concrete_TypeRef()) }
    }

    pub fn copy_variation_axes(&self) -> Option<CFArray<CFDictionary<CFString, CFType>>> {
        unsafe {
            let axes = CGFontCopyVariationAxes(self.as_concrete_TypeRef());
            if axes.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(axes))
            }
        }
    }

    pub fn copy_variations(&self) -> Option<CFDictionary<CFString, CFNumber>> {
        let variations = unsafe { CGFontCopyVariations(self.as_concrete_TypeRef()) };
        if !variations.is_null() {
            Some(unsafe { TCFType::wrap_under_create_rule(variations) })
        } else {
            None
        }
    }

    pub fn glyph_advances(&self, glyphs: &[CGGlyph], advances: &mut [c_int]) -> bool {
        if glyphs.len() > advances.len() {
            return false;
        }

        unsafe { CGFontGetGlyphAdvances(self.as_concrete_TypeRef(), glyphs.as_ptr(), glyphs.len(), advances.as_mut_ptr()) }
    }

    pub fn glyph_b_boxes(&self, glyphs: &[CGGlyph], bboxes: &mut [CGRect]) -> bool {
        if glyphs.len() > bboxes.len() {
            return false;
        }

        unsafe { CGFontGetGlyphBBoxes(self.as_concrete_TypeRef(), glyphs.as_ptr(), glyphs.len(), bboxes.as_mut_ptr()) }
    }

    pub fn glyph_with_glyph_name(&self, name: &CFString) -> CGGlyph {
        unsafe { CGFontGetGlyphWithGlyphName(self.as_concrete_TypeRef(), name.as_concrete_TypeRef()) }
    }

    pub fn copy_glyph_name_for_glyph(&self, glyph: CGGlyph) -> CFString {
        unsafe {
            let name = CGFontCopyGlyphNameForGlyph(self.as_concrete_TypeRef(), glyph);
            TCFType::wrap_under_create_rule(name)
        }
    }

    pub fn copy_table_tags(&self) -> CFArray<u32> {
        unsafe { TCFType::wrap_under_create_rule(CGFontCopyTableTags(self.as_concrete_TypeRef())) }
    }

    pub fn copy_table_for_tag(&self, tag: u32) -> Option<CFData> {
        let data = unsafe { CGFontCopyTableForTag(self.as_concrete_TypeRef(), tag) };
        if data.is_null() {
            None
        } else {
            Some(unsafe { TCFType::wrap_under_create_rule(data) })
        }
    }
}

pub enum CGFontVariationAxis {
    Name,
    MinValue,
    MaxValue,
    DefaultValue,
}

impl From<CGFontVariationAxis> for CFStringRef {
    fn from(axis: CGFontVariationAxis) -> Self {
        unsafe {
            match axis {
                CGFontVariationAxis::Name => kCGFontVariationAxisName,
                CGFontVariationAxis::MinValue => kCGFontVariationAxisMinValue,
                CGFontVariationAxis::MaxValue => kCGFontVariationAxisMaxValue,
                CGFontVariationAxis::DefaultValue => kCGFontVariationAxisDefaultValue,
            }
        }
    }
}

impl From<CGFontVariationAxis> for CFString {
    fn from(axis: CGFontVariationAxis) -> Self {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(axis)) }
    }
}

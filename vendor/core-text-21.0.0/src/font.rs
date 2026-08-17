// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_upper_case_globals)]

use crate::font_descriptor;
use crate::font_descriptor::{CTFontDescriptor, CTFontDescriptorRef, CTFontOrientation};
use crate::font_descriptor::{
    CTFontSymbolicTraits, CTFontTraits, SymbolicTraitAccessors, TraitAccessors,
};
use crate::font_manager::create_font_descriptor;

use core_foundation::array::{CFArray, CFArrayRef};
use core_foundation::base::{CFIndex, CFOptionFlags, CFType, CFTypeID, CFTypeRef, TCFType};
use core_foundation::data::{CFData, CFDataRef};
use core_foundation::dictionary::{CFDictionary, CFDictionaryRef};
use core_foundation::number::CFNumber;
use core_foundation::string::{CFString, CFStringRef, UniChar};
use core_foundation::url::{CFURLRef, CFURL};
use core_foundation::{declare_TCFType, impl_CFTypeDescription, impl_TCFType};
use core_graphics::base::CGFloat;
use core_graphics::context::CGContext;
use core_graphics::font::{CGFont, CGGlyph};
use core_graphics::geometry::{CGAffineTransform, CGPoint, CGRect, CGSize};
use core_graphics::path::CGPath;

use foreign_types::ForeignType;
use libc::{self, size_t};
use std::os::raw::c_void;
use std::ptr;

type CGContextRef = *mut <CGContext as ForeignType>::CType;
type CGFontRef = *mut <CGFont as ForeignType>::CType;
type CGPathRef = *mut <CGPath as ForeignType>::CType;

pub type CTFontUIFontType = u32;
// kCTFontNoFontType: CTFontUIFontType = -1;
pub const kCTFontUserFontType: CTFontUIFontType = 0;
pub const kCTFontUserFixedPitchFontType: CTFontUIFontType = 1;
pub const kCTFontSystemFontType: CTFontUIFontType = 2;
pub const kCTFontEmphasizedSystemFontType: CTFontUIFontType = 3;
pub const kCTFontSmallSystemFontType: CTFontUIFontType = 4;
pub const kCTFontSmallEmphasizedSystemFontType: CTFontUIFontType = 5;
pub const kCTFontMiniSystemFontType: CTFontUIFontType = 6;
pub const kCTFontMiniEmphasizedSystemFontType: CTFontUIFontType = 7;
pub const kCTFontViewsFontType: CTFontUIFontType = 8;
pub const kCTFontApplicationFontType: CTFontUIFontType = 9;
pub const kCTFontLabelFontType: CTFontUIFontType = 10;
pub const kCTFontMenuTitleFontType: CTFontUIFontType = 11;
pub const kCTFontMenuItemFontType: CTFontUIFontType = 12;
pub const kCTFontMenuItemMarkFontType: CTFontUIFontType = 13;
pub const kCTFontMenuItemCmdKeyFontType: CTFontUIFontType = 14;
pub const kCTFontWindowTitleFontType: CTFontUIFontType = 15;
pub const kCTFontPushButtonFontType: CTFontUIFontType = 16;
pub const kCTFontUtilityWindowTitleFontType: CTFontUIFontType = 17;
pub const kCTFontAlertHeaderFontType: CTFontUIFontType = 18;
pub const kCTFontSystemDetailFontType: CTFontUIFontType = 19;
pub const kCTFontEmphasizedSystemDetailFontType: CTFontUIFontType = 20;
pub const kCTFontToolbarFontType: CTFontUIFontType = 21;
pub const kCTFontSmallToolbarFontType: CTFontUIFontType = 22;
pub const kCTFontMessageFontType: CTFontUIFontType = 23;
pub const kCTFontPaletteFontType: CTFontUIFontType = 24;
pub const kCTFontToolTipFontType: CTFontUIFontType = 25;
pub const kCTFontControlContentFontType: CTFontUIFontType = 26;

pub type CTFontTableTag = u32;
// TODO: create bindings for enum with 'chars' values

pub type CTFontTableOptions = u32;
pub const kCTFontTableOptionsNoOptions: CTFontTableOptions = 0;
pub const kCTFontTableOptionsExcludeSynthetic: CTFontTableOptions = 1 << 0;

pub type CTFontOptions = CFOptionFlags;
pub const kCTFontOptionsDefault: CTFontOptions = 0;
pub const kCTFontOptionsPreventAutoActivation: CTFontOptions = 1 << 0;
pub const kCTFontOptionsPreferSystemFont: CTFontOptions = 1 << 2;

pub enum CTFontNameSpecifier {
    Copyright,
    Family,
    SubFamily,
    Style,
    Unique,
    Full,
    Version,
    PostScript,
    Trademark,
    Manufacturer,
    Designer,
    Description,
    VendorURL,
    DesignerURL,
    License,
    LicenseURL,
    SampleText,
    PostScriptCID,
}

impl From<CTFontNameSpecifier> for CFStringRef {
    fn from(val: CTFontNameSpecifier) -> Self {
        unsafe {
            match val {
                CTFontNameSpecifier::Copyright => kCTFontCopyrightNameKey,
                CTFontNameSpecifier::Family => kCTFontFamilyNameKey,
                CTFontNameSpecifier::SubFamily => kCTFontSubFamilyNameKey,
                CTFontNameSpecifier::Style => kCTFontStyleNameKey,
                CTFontNameSpecifier::Unique => kCTFontUniqueNameKey,
                CTFontNameSpecifier::Full => kCTFontFullNameKey,
                CTFontNameSpecifier::Version => kCTFontVersionNameKey,
                CTFontNameSpecifier::PostScript => kCTFontPostScriptNameKey,
                CTFontNameSpecifier::Trademark => kCTFontTrademarkNameKey,
                CTFontNameSpecifier::Manufacturer => kCTFontManufacturerNameKey,
                CTFontNameSpecifier::Designer => kCTFontDesignerNameKey,
                CTFontNameSpecifier::Description => kCTFontDescriptionNameKey,
                CTFontNameSpecifier::VendorURL => kCTFontVendorURLNameKey,
                CTFontNameSpecifier::DesignerURL => kCTFontDesignerURLNameKey,
                CTFontNameSpecifier::License => kCTFontLicenseNameKey,
                CTFontNameSpecifier::LicenseURL => kCTFontLicenseURLNameKey,
                CTFontNameSpecifier::SampleText => kCTFontSampleTextNameKey,
                CTFontNameSpecifier::PostScriptCID => kCTFontPostScriptCIDNameKey,
            }
        }
    }
}

#[repr(C)]
pub struct __CTFont(c_void);

pub type CTFontRef = *const __CTFont;

declare_TCFType! {
    CTFont, CTFontRef
}
impl_TCFType!(CTFont, CTFontRef, CTFontGetTypeID);
impl_CFTypeDescription!(CTFont);

unsafe impl Send for CTFont {}
unsafe impl Sync for CTFont {}

pub fn new_from_CGFont(cgfont: &CGFont, pt_size: f64) -> CTFont {
    unsafe {
        let font_ref = CTFontCreateWithGraphicsFont(
            cgfont.as_ptr() as *mut _,
            pt_size as CGFloat,
            ptr::null(),
            ptr::null(),
        );
        CTFont::wrap_under_create_rule(font_ref)
    }
}

pub fn new_from_CGFont_with_variations(
    cgfont: &CGFont,
    pt_size: f64,
    variations: &CFDictionary<CFString, CFNumber>,
) -> CTFont {
    unsafe {
        let font_desc = font_descriptor::new_from_variations(variations);
        let font_ref = CTFontCreateWithGraphicsFont(
            cgfont.as_ptr() as *mut _,
            pt_size as CGFloat,
            ptr::null(),
            font_desc.as_concrete_TypeRef(),
        );
        CTFont::wrap_under_create_rule(font_ref)
    }
}

pub fn new_from_descriptor(desc: &CTFontDescriptor, pt_size: f64) -> CTFont {
    unsafe {
        let font_ref = CTFontCreateWithFontDescriptor(
            desc.as_concrete_TypeRef(),
            pt_size as CGFloat,
            ptr::null(),
        );
        CTFont::wrap_under_create_rule(font_ref)
    }
}

pub fn new_from_descriptor_and_options(
    desc: &CTFontDescriptor,
    pt_size: f64,
    options: CTFontOptions,
) -> CTFont {
    unsafe {
        let font_ref = CTFontCreateWithFontDescriptorAndOptions(
            desc.as_concrete_TypeRef(),
            pt_size as CGFloat,
            ptr::null(),
            options,
        );
        CTFont::wrap_under_create_rule(font_ref)
    }
}

pub fn new_from_buffer(buffer: &[u8]) -> Result<CTFont, ()> {
    let ct_font_descriptor = create_font_descriptor(buffer)?;
    Ok(new_from_descriptor(&ct_font_descriptor, 16.0))
}

pub fn new_from_name(name: &str, pt_size: f64) -> Result<CTFont, ()> {
    unsafe {
        let name: CFString = name.parse().unwrap();
        let font_ref =
            CTFontCreateWithName(name.as_concrete_TypeRef(), pt_size as CGFloat, ptr::null());
        if font_ref.is_null() {
            Err(())
        } else {
            Ok(CTFont::wrap_under_create_rule(font_ref))
        }
    }
}

pub fn new_from_name_and_options(
    name: &str,
    pt_size: f64,
    options: CTFontOptions,
) -> Result<CTFont, ()> {
    unsafe {
        let name: CFString = name.parse().unwrap();
        let font_ref = CTFontCreateWithNameAndOptions(
            name.as_concrete_TypeRef(),
            pt_size as CGFloat,
            ptr::null(),
            options,
        );
        if font_ref.is_null() {
            Err(())
        } else {
            Ok(CTFont::wrap_under_create_rule(font_ref))
        }
    }
}

pub fn new_ui_font_for_language(
    ui_type: CTFontUIFontType,
    size: f64,
    language: Option<CFString>,
) -> CTFont {
    unsafe {
        let font_ref = CTFontCreateUIFontForLanguage(
            ui_type,
            size,
            language
                .as_ref()
                .map(|x| x.as_concrete_TypeRef())
                .unwrap_or(std::ptr::null()),
        );
        if font_ref.is_null() {
            // CTFontCreateUIFontForLanguage can fail, but is unlikely to do so during
            // normal usage (if you pass a bad ui_type it will). To make things more
            // convenient, just panic if it fails.
            panic!();
        } else {
            CTFont::wrap_under_create_rule(font_ref)
        }
    }
}

impl CTFont {
    // Properties
    pub fn symbolic_traits(&self) -> CTFontSymbolicTraits {
        unsafe { CTFontGetSymbolicTraits(self.0) }
    }
}

impl CTFont {
    // Creation methods
    pub fn copy_to_CGFont(&self) -> CGFont {
        unsafe {
            let cgfont_ref = CTFontCopyGraphicsFont(self.0, ptr::null_mut());
            CGFont::from_ptr(cgfont_ref as *mut _)
        }
    }

    pub fn copy_descriptor(&self) -> CTFontDescriptor {
        unsafe {
            let desc = CTFontCopyFontDescriptor(self.0);
            CTFontDescriptor::wrap_under_create_rule(desc)
        }
    }

    pub fn clone_with_font_size(&self, size: f64) -> CTFont {
        unsafe {
            let font_ref =
                CTFontCreateCopyWithAttributes(self.0, size as CGFloat, ptr::null(), ptr::null());
            CTFont::wrap_under_create_rule(font_ref)
        }
    }

    pub fn clone_with_symbolic_traits(
        &self,
        trait_value: CTFontSymbolicTraits,
        trait_mask: CTFontSymbolicTraits,
    ) -> Option<CTFont> {
        unsafe {
            let font_ref = CTFontCreateCopyWithSymbolicTraits(
                self.0,
                0.0,
                ptr::null(),
                trait_value,
                trait_mask,
            );
            if font_ref.is_null() {
                None
            } else {
                Some(CTFont::wrap_under_create_rule(font_ref))
            }
        }
    }

    // Names
    pub fn get_string_by_name_key(&self, name_key: CTFontNameSpecifier) -> Option<String> {
        unsafe {
            let result = CTFontCopyName(self.as_concrete_TypeRef(), name_key.into());
            if result.is_null() {
                None
            } else {
                Some(CFString::wrap_under_create_rule(result).to_string())
            }
        }
    }

    pub fn family_name(&self) -> String {
        let value = self.get_string_by_name_key(CTFontNameSpecifier::Family);
        value.expect("Fonts should always have a family name.")
    }

    pub fn face_name(&self) -> String {
        let value = self.get_string_by_name_key(CTFontNameSpecifier::SubFamily);
        value.expect("Fonts should always have a face name.")
    }

    pub fn unique_name(&self) -> String {
        let value = self.get_string_by_name_key(CTFontNameSpecifier::Unique);
        value.expect("Fonts should always have a unique name.")
    }

    pub fn postscript_name(&self) -> String {
        let value = self.get_string_by_name_key(CTFontNameSpecifier::PostScript);
        value.expect("Fonts should always have a PostScript name.")
    }

    pub fn display_name(&self) -> String {
        let value = self.get_string_by_name_key(CTFontNameSpecifier::Full);
        value.expect("Fonts should always have a PostScript name.")
    }

    pub fn style_name(&self) -> String {
        let value = self.get_string_by_name_key(CTFontNameSpecifier::Style);
        value.expect("Fonts should always have a style name.")
    }

    pub fn all_traits(&self) -> CTFontTraits {
        unsafe { CTFontTraits::wrap_under_create_rule(CTFontCopyTraits(self.0)) }
    }

    // Font metrics
    pub fn ascent(&self) -> CGFloat {
        unsafe { CTFontGetAscent(self.0) }
    }

    pub fn descent(&self) -> CGFloat {
        unsafe { CTFontGetDescent(self.0) }
    }

    pub fn underline_thickness(&self) -> CGFloat {
        unsafe { CTFontGetUnderlineThickness(self.0) }
    }

    pub fn underline_position(&self) -> CGFloat {
        unsafe { CTFontGetUnderlinePosition(self.0) }
    }

    pub fn slant_angle(&self) -> CGFloat {
        unsafe { CTFontGetSlantAngle(self.0) }
    }

    pub fn cap_height(&self) -> CGFloat {
        unsafe { CTFontGetCapHeight(self.0) }
    }

    pub fn bounding_box(&self) -> CGRect {
        unsafe { CTFontGetBoundingBox(self.0) }
    }

    pub fn leading(&self) -> CGFloat {
        unsafe { CTFontGetLeading(self.0) }
    }

    pub fn units_per_em(&self) -> libc::c_uint {
        unsafe { CTFontGetUnitsPerEm(self.0) }
    }

    pub fn x_height(&self) -> CGFloat {
        unsafe { CTFontGetXHeight(self.0) }
    }

    pub fn pt_size(&self) -> CGFloat {
        unsafe { CTFontGetSize(self.0) }
    }

    pub fn get_glyph_with_name(&self, glyph_name: &str) -> CGGlyph {
        let glyph_name = CFString::new(glyph_name);
        unsafe { CTFontGetGlyphWithName(self.0, glyph_name.as_concrete_TypeRef()) }
    }

    pub unsafe fn get_glyphs_for_characters(
        &self,
        characters: *const UniChar,
        glyphs: *mut CGGlyph,
        count: CFIndex,
    ) -> bool {
        CTFontGetGlyphsForCharacters(self.0, characters, glyphs, count)
    }

    pub unsafe fn get_advances_for_glyphs(
        &self,
        orientation: CTFontOrientation,
        glyphs: *const CGGlyph,
        advances: *mut CGSize,
        count: CFIndex,
    ) -> f64 {
        CTFontGetAdvancesForGlyphs(self.0, orientation, glyphs, advances, count)
    }

    pub unsafe fn get_vertical_translations_for_glyphs(
        &self,
        orientation: CTFontOrientation,
        glyphs: *const CGGlyph,
        translations: *mut CGSize,
        count: CFIndex,
    ) {
        CTFontGetVerticalTranslationsForGlyphs(self.0, orientation, glyphs, translations, count)
    }

    pub fn get_font_table(&self, tag: u32) -> Option<CFData> {
        unsafe {
            let result = CTFontCopyTable(
                self.0,
                tag as CTFontTableTag,
                kCTFontTableOptionsExcludeSynthetic,
            );
            if result.is_null() {
                None
            } else {
                Some(CFData::wrap_under_create_rule(result))
            }
        }
    }

    pub fn get_available_font_tables(&self) -> Option<CFArray<CTFontTableTag>> {
        unsafe {
            let result = CTFontCopyAvailableTables(self.0, kCTFontTableOptionsExcludeSynthetic);
            if result.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(result))
            }
        }
    }

    pub fn get_bounding_rects_for_glyphs(
        &self,
        orientation: CTFontOrientation,
        glyphs: &[CGGlyph],
    ) -> CGRect {
        unsafe {
            CTFontGetBoundingRectsForGlyphs(
                self.as_concrete_TypeRef(),
                orientation,
                glyphs.as_ptr(),
                ptr::null_mut(),
                glyphs.len() as CFIndex,
            )
        }
    }

    pub fn draw_glyphs(&self, glyphs: &[CGGlyph], positions: &[CGPoint], context: CGContext) {
        assert_eq!(glyphs.len(), positions.len());
        unsafe {
            CTFontDrawGlyphs(
                self.as_concrete_TypeRef(),
                glyphs.as_ptr(),
                positions.as_ptr(),
                glyphs.len() as size_t,
                context.as_ptr(),
            )
        }
    }

    pub fn get_matrix(&self) -> CGAffineTransform {
        unsafe { CTFontGetMatrix(self.as_concrete_TypeRef()) }
    }

    pub fn url(&self) -> Option<CFURL> {
        unsafe {
            let result = CTFontCopyAttribute(self.0, kCTFontURLAttribute);
            if result.is_null() {
                None
            } else {
                Some(CFURL::wrap_under_create_rule(result as CFURLRef))
            }
        }
    }

    pub fn get_variation_axes(&self) -> Option<CFArray<CFDictionary<CFString, CFType>>> {
        unsafe {
            let axes = CTFontCopyVariationAxes(self.0);
            if axes.is_null() {
                return None;
            }
            Some(TCFType::wrap_under_create_rule(axes))
        }
    }

    pub fn create_path_for_glyph(
        &self,
        glyph: CGGlyph,
        matrix: &CGAffineTransform,
    ) -> Result<CGPath, ()> {
        unsafe {
            let path = CTFontCreatePathForGlyph(self.0, glyph, matrix);
            if path.is_null() {
                Err(())
            } else {
                Ok(CGPath::from_ptr(path))
            }
        }
    }

    #[inline]
    pub fn glyph_count(&self) -> CFIndex {
        unsafe { CTFontGetGlyphCount(self.0) }
    }
}

// Helper methods
pub fn debug_font_names(font: &CTFont) {
    fn get_key(font: &CTFont, key: CTFontNameSpecifier) -> String {
        font.get_string_by_name_key(key).unwrap()
    }

    println!(
        "kCTFontFamilyNameKey: {}",
        get_key(font, CTFontNameSpecifier::Family)
    );
    println!(
        "kCTFontSubFamilyNameKey: {}",
        get_key(font, CTFontNameSpecifier::SubFamily)
    );
    println!(
        "kCTFontStyleNameKey: {}",
        get_key(font, CTFontNameSpecifier::Style)
    );
    println!(
        "kCTFontUniqueNameKey: {}",
        get_key(font, CTFontNameSpecifier::Unique)
    );
    println!(
        "kCTFontFullNameKey: {}",
        get_key(font, CTFontNameSpecifier::Full)
    );
    println!(
        "kCTFontPostScriptNameKey: {}",
        get_key(font, CTFontNameSpecifier::PostScript)
    );
}

pub fn debug_font_traits(font: &CTFont) {
    let sym = font.symbolic_traits();
    println!("kCTFontItalicTrait: {}", sym.is_italic());
    println!("kCTFontBoldTrait: {}", sym.is_bold());
    println!("kCTFontExpandedTrait: {}", sym.is_expanded());
    println!("kCTFontCondensedTrait: {}", sym.is_condensed());
    println!("kCTFontMonoSpaceTrait: {}", sym.is_monospace());

    let traits = font.all_traits();
    println!("kCTFontWeightTrait: {}", traits.normalized_weight());
    println!("kCTFontWidthTrait: {}", traits.normalized_width());
    //    println!("kCTFontSlantTrait: {}", traits.normalized_slant());
}

#[cfg(feature = "mountainlion")]
pub fn cascade_list_for_languages(
    font: &CTFont,
    language_pref_list: &CFArray<CFString>,
) -> CFArray<CTFontDescriptor> {
    unsafe {
        let font_collection_ref = CTFontCopyDefaultCascadeListForLanguages(
            font.as_concrete_TypeRef(),
            language_pref_list.as_concrete_TypeRef(),
        );
        CFArray::wrap_under_create_rule(font_collection_ref)
    }
}

#[cfg_attr(feature = "link", link(name = "CoreText", kind = "framework"))]
extern "C" {
    /*
     * CTFont.h
     */

    /* Name Specifier Constants */
    static kCTFontCopyrightNameKey: CFStringRef;
    static kCTFontFamilyNameKey: CFStringRef;
    static kCTFontSubFamilyNameKey: CFStringRef;
    static kCTFontStyleNameKey: CFStringRef;
    static kCTFontUniqueNameKey: CFStringRef;
    static kCTFontFullNameKey: CFStringRef;
    static kCTFontVersionNameKey: CFStringRef;
    static kCTFontPostScriptNameKey: CFStringRef;
    static kCTFontTrademarkNameKey: CFStringRef;
    static kCTFontManufacturerNameKey: CFStringRef;
    static kCTFontDesignerNameKey: CFStringRef;
    static kCTFontDescriptionNameKey: CFStringRef;
    static kCTFontVendorURLNameKey: CFStringRef;
    static kCTFontDesignerURLNameKey: CFStringRef;
    static kCTFontLicenseNameKey: CFStringRef;
    static kCTFontLicenseURLNameKey: CFStringRef;
    static kCTFontSampleTextNameKey: CFStringRef;
    static kCTFontPostScriptCIDNameKey: CFStringRef;

    #[cfg(test)]
    static kCTFontVariationAxisIdentifierKey: CFStringRef;
    //static kCTFontVariationAxisMinimumValueKey: CFStringRef;
    #[cfg(test)]
    static kCTFontVariationAxisMaximumValueKey: CFStringRef;
    //static kCTFontVariationAxisDefaultValueKey: CFStringRef;
    //static kCTFontVariationAxisNameKey: CFStringRef;

    //static kCTFontFeatureTypeIdentifierKey: CFStringRef;
    //static kCTFontFeatureTypeNameKey: CFStringRef;
    //static kCTFontFeatureTypeExclusiveKey: CFStringRef;
    //static kCTFontFeatureTypeSelectorsKey: CFStringRef;
    //static kCTFontFeatureSelectorIdentifierKey: CFStringRef;
    //static kCTFontFeatureSelectorNameKey: CFStringRef;
    //static kCTFontFeatureSelectorDefaultKey: CFStringRef;
    //static kCTFontFeatureSelectorSettingKey: CFStringRef;

    static kCTFontURLAttribute: CFStringRef;

    // N.B. Unlike most Cocoa bindings, this extern block is organized according
    // to the documentation's Functions By Task listing, because there so many functions.

    /* Creating Fonts */
    fn CTFontCreateWithName(
        name: CFStringRef,
        size: CGFloat,
        matrix: *const CGAffineTransform,
    ) -> CTFontRef;
    fn CTFontCreateWithNameAndOptions(
        name: CFStringRef,
        size: CGFloat,
        matrix: *const CGAffineTransform,
        options: CTFontOptions,
    ) -> CTFontRef;
    fn CTFontCreateWithFontDescriptor(
        descriptor: CTFontDescriptorRef,
        size: CGFloat,
        matrix: *const CGAffineTransform,
    ) -> CTFontRef;
    fn CTFontCreateWithFontDescriptorAndOptions(
        descriptor: CTFontDescriptorRef,
        size: CGFloat,
        matrix: *const CGAffineTransform,
        options: CTFontOptions,
    ) -> CTFontRef;
    fn CTFontCreateUIFontForLanguage(
        uiType: CTFontUIFontType,
        size: CGFloat,
        language: CFStringRef,
    ) -> CTFontRef;
    fn CTFontCreateCopyWithAttributes(
        font: CTFontRef,
        size: CGFloat,
        matrix: *const CGAffineTransform,
        attributes: CTFontDescriptorRef,
    ) -> CTFontRef;
    fn CTFontCreateCopyWithSymbolicTraits(
        font: CTFontRef,
        size: CGFloat,
        matrix: *const CGAffineTransform,
        symTraitValue: CTFontSymbolicTraits,
        symTraitMask: CTFontSymbolicTraits,
    ) -> CTFontRef;
    //fn CTFontCreateCopyWithFamily
    //fn CTFontCreateForString

    /* Getting Font Data */
    fn CTFontCopyFontDescriptor(font: CTFontRef) -> CTFontDescriptorRef;
    fn CTFontCopyAttribute(font: CTFontRef, attribute: CFStringRef) -> CFTypeRef;
    fn CTFontGetSize(font: CTFontRef) -> CGFloat;
    fn CTFontGetMatrix(font: CTFontRef) -> CGAffineTransform;
    fn CTFontGetSymbolicTraits(font: CTFontRef) -> CTFontSymbolicTraits;
    fn CTFontCopyTraits(font: CTFontRef) -> CFDictionaryRef;

    /* Getting Font Names */
    //fn CTFontCopyPostScriptName(font: CTFontRef) -> CFStringRef;
    //fn CTFontCopyFamilyName(font: CTFontRef) -> CFStringRef;
    //fn CTFontCopyFullName(font: CTFontRef) -> CFStringRef;
    //fn CTFontCopyDisplayName(font: CTFontRef) -> CFStringRef;
    fn CTFontCopyName(font: CTFontRef, nameKey: CFStringRef) -> CFStringRef;
    //fn CTFontCopyLocalizedName(font: CTFontRef, nameKey: CFStringRef,
    //                           language: *CFStringRef) -> CFStringRef;
    #[cfg(feature = "mountainlion")]
    fn CTFontCopyDefaultCascadeListForLanguages(
        font: CTFontRef,
        languagePrefList: CFArrayRef,
    ) -> CFArrayRef;

    /* Working With Encoding */
    //fn CTFontCopyCharacterSet
    //fn CTFontGetStringEncoding
    //fn CTFontCopySupportedLanguages

    /* Getting Font Metrics */
    fn CTFontGetAscent(font: CTFontRef) -> CGFloat;
    fn CTFontGetDescent(font: CTFontRef) -> CGFloat;
    fn CTFontGetLeading(font: CTFontRef) -> CGFloat;
    fn CTFontGetUnitsPerEm(font: CTFontRef) -> libc::c_uint;
    fn CTFontGetGlyphCount(font: CTFontRef) -> CFIndex;
    fn CTFontGetBoundingBox(font: CTFontRef) -> CGRect;
    fn CTFontGetUnderlinePosition(font: CTFontRef) -> CGFloat;
    fn CTFontGetUnderlineThickness(font: CTFontRef) -> CGFloat;
    fn CTFontGetSlantAngle(font: CTFontRef) -> CGFloat;
    fn CTFontGetCapHeight(font: CTFontRef) -> CGFloat;
    fn CTFontGetXHeight(font: CTFontRef) -> CGFloat;

    /* Getting Glyph Data */
    fn CTFontCreatePathForGlyph(
        font: CTFontRef,
        glyph: CGGlyph,
        matrix: *const CGAffineTransform,
    ) -> CGPathRef;
    fn CTFontGetGlyphWithName(font: CTFontRef, glyphName: CFStringRef) -> CGGlyph;
    fn CTFontGetBoundingRectsForGlyphs(
        font: CTFontRef,
        orientation: CTFontOrientation,
        glyphs: *const CGGlyph,
        boundingRects: *mut CGRect,
        count: CFIndex,
    ) -> CGRect;
    fn CTFontGetAdvancesForGlyphs(
        font: CTFontRef,
        orientation: CTFontOrientation,
        glyphs: *const CGGlyph,
        advances: *mut CGSize,
        count: CFIndex,
    ) -> libc::c_double;
    fn CTFontGetVerticalTranslationsForGlyphs(
        font: CTFontRef,
        orientation: CTFontOrientation,
        glyphs: *const CGGlyph,
        translations: *mut CGSize,
        count: CFIndex,
    );

    /* Working With Font Variations */
    fn CTFontCopyVariationAxes(font: CTFontRef) -> CFArrayRef;
    //fn CTFontCopyVariation

    /* Getting Font Features */
    //fn CTFontCopyFeatures
    //fn CTFontCopyFeatureSettings

    /* Working with Glyphs */
    fn CTFontGetGlyphsForCharacters(
        font: CTFontRef,
        characters: *const UniChar,
        glyphs: *mut CGGlyph,
        count: CFIndex,
    ) -> bool;
    fn CTFontDrawGlyphs(
        font: CTFontRef,
        glyphs: *const CGGlyph,
        positions: *const CGPoint,
        count: size_t,
        context: CGContextRef,
    );
    //fn CTFontGetLigatureCaretPositions

    /* Converting Fonts */
    fn CTFontCopyGraphicsFont(font: CTFontRef, attributes: *mut CTFontDescriptorRef) -> CGFontRef;
    fn CTFontCreateWithGraphicsFont(
        graphicsFont: CGFontRef,
        size: CGFloat,
        matrix: *const CGAffineTransform,
        attributes: CTFontDescriptorRef,
    ) -> CTFontRef;
    //fn CTFontGetPlatformFont
    //fn CTFontCreateWithPlatformFont
    //fn CTFontCreateWithQuickdrawInstance

    /* Getting Font Table Data */
    fn CTFontCopyAvailableTables(font: CTFontRef, options: CTFontTableOptions) -> CFArrayRef;
    fn CTFontCopyTable(
        font: CTFontRef,
        table: CTFontTableTag,
        options: CTFontTableOptions,
    ) -> CFDataRef;

    fn CTFontGetTypeID() -> CFTypeID;
}

#[test]
fn copy_font() {
    use std::io::Read;
    let mut f = std::fs::File::open("/System/Library/Fonts/ZapfDingbats.ttf").unwrap();
    let mut font_data = Vec::new();
    f.read_to_end(&mut font_data).unwrap();
    let desc = crate::font_manager::create_font_descriptor(&font_data).unwrap();
    let font = new_from_descriptor(&desc, 12.);
    drop(desc);
    let desc = font.copy_descriptor();
    drop(font);
    let font = new_from_descriptor(&desc, 14.);
    assert_eq!(font.family_name(), "Zapf Dingbats");
}

#[cfg(test)]
fn macos_version() -> (i32, i32, i32) {
    use std::io::Read;

    // This is the same approach that Firefox uses for detecting versions
    let file = "/System/Library/CoreServices/SystemVersion.plist";
    let mut f = std::fs::File::open(file).unwrap();
    let mut system_version_data = Vec::new();
    f.read_to_end(&mut system_version_data).unwrap();

    use core_foundation::propertylist;
    let (list, _) = propertylist::create_with_data(
        core_foundation::data::CFData::from_buffer(&system_version_data),
        propertylist::kCFPropertyListImmutable,
    )
    .unwrap();
    let k = unsafe { propertylist::CFPropertyList::wrap_under_create_rule(list) };

    let dict = unsafe {
        std::mem::transmute::<_, CFDictionary<CFType, CFType>>(
            k.downcast::<CFDictionary>().unwrap(),
        )
    };

    let version = dict
        .find(&CFString::new("ProductVersion").as_CFType())
        .as_ref()
        .unwrap()
        .downcast::<CFString>()
        .unwrap()
        .to_string();

    match version
        .split('.')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>()[..]
    {
        [a, b, c] => (a, b, c),
        [a, b] => (a, b, 0),
        _ => panic!(),
    }
}

#[test]
fn copy_system_font() {
    use crate::*;

    let small = new_ui_font_for_language(kCTFontSystemDetailFontType, 19., None);
    let big = small.clone_with_font_size(20.);

    // ensure that we end up with different fonts for the different sizes before 10.15
    if macos_version() < (10, 15, 0) {
        assert_ne!(big.url(), small.url());
    } else {
        assert_eq!(big.url(), small.url());
    }

    let ps = small.postscript_name();
    let desc = small.copy_descriptor();

    // check that we can construct a new version by descriptor
    let ctfont = new_from_descriptor(&desc, 20.);
    assert_eq!(big.postscript_name(), ctfont.postscript_name());

    // check that we can construct a new version by attributes
    let attr = desc.attributes();
    let desc_from_attr = font_descriptor::new_from_attributes(&attr);
    let font_from_attr = new_from_descriptor(&desc_from_attr, 19.);
    assert_eq!(font_from_attr.postscript_name(), small.postscript_name());

    // on newer versions of macos we can't construct by name anymore
    if macos_version() < (10, 13, 0) {
        let ui_font_by_name = new_from_name(&small.postscript_name(), 19.).unwrap();
        assert_eq!(ui_font_by_name.postscript_name(), small.postscript_name());

        let ui_font_by_name = new_from_name(&small.postscript_name(), 20.).unwrap();
        assert_eq!(ui_font_by_name.postscript_name(), small.postscript_name());

        let ui_font_by_name = new_from_name(&big.postscript_name(), 20.).unwrap();
        assert_eq!(ui_font_by_name.postscript_name(), big.postscript_name());
    }

    // but we can still construct the CGFont by name
    let cgfont = CGFont::from_name(&CFString::new(&ps)).unwrap();
    let cgfont = new_from_CGFont(&cgfont, 0.);
    println!("{:?}", cgfont);
    let desc = cgfont.copy_descriptor();
    let matching = unsafe {
        crate::font_descriptor::CTFontDescriptorCreateMatchingFontDescriptor(
            desc.as_concrete_TypeRef(),
            std::ptr::null(),
        )
    };
    let matching = unsafe { CTFontDescriptor::wrap_under_create_rule(matching) };

    println!("{:?}", cgfont.copy_descriptor());
    assert!(desc
        .attributes()
        .find(CFString::from_static_string("NSFontSizeAttribute"))
        .is_some());

    println!("{:?}", matching);
    println!(
        "{:?}",
        matching
            .attributes()
            .find(CFString::from_static_string("NSFontSizeAttribute"))
    );

    assert!(matching
        .attributes()
        .find(CFString::from_static_string("NSFontSizeAttribute"))
        .is_none());

    assert_eq!(small.postscript_name(), cgfont.postscript_name());
}

// Tests what happens when variations have values not inbetween min and max
#[test]
fn out_of_range_variations() {
    use crate::*;
    use core_foundation::base::ItemRef;

    let small = new_ui_font_for_language(kCTFontSystemDetailFontType, 19., None);

    let axes = small.get_variation_axes();
    let version = dbg!(macos_version());
    if version < (10, 12, 0) {
        assert!(axes.is_none());
        return;
    }
    let axes = axes.unwrap();
    let mut vals = Vec::new();
    dbg!(&axes);
    for axis in axes.iter() {
        let tag = axis
            .find(unsafe { kCTFontVariationAxisIdentifierKey })
            .unwrap()
            .downcast::<CFNumber>()
            .unwrap()
            .to_i64()
            .unwrap();
        let max = axis
            .find(unsafe { kCTFontVariationAxisMaximumValueKey })
            .unwrap()
            .downcast::<CFNumber>()
            .unwrap()
            .to_f64()
            .unwrap();
        vals.push((CFNumber::from(tag), CFNumber::from(max + 1.)));
    }
    let vals_dict = CFDictionary::from_CFType_pairs(&vals);
    let variation_attribute =
        unsafe { CFString::wrap_under_get_rule(font_descriptor::kCTFontVariationAttribute) };
    let attrs_dict = CFDictionary::from_CFType_pairs(&[(variation_attribute.clone(), vals_dict)]);
    let ct_var_font_desc = small
        .copy_descriptor()
        .create_copy_with_attributes(attrs_dict.to_untyped())
        .unwrap();
    let variation_font = crate::font::new_from_descriptor(&ct_var_font_desc, 19.);
    let var_desc = variation_font.copy_descriptor();
    let var_attrs = var_desc.attributes();
    dbg!(&var_attrs);

    // Handling of attributes greater than max changed between versions
    let var_attrs = var_attrs.find(variation_attribute);
    if version >= (14, 0, 0) {
        check_attrs(0., var_attrs, axes);
    } else if version >= (12, 0, 0) && version < (13, 0, 0) {
        check_attrs(1., var_attrs, axes);
    } else if version >= (10, 15, 0) {
        assert!(var_attrs.is_none());
    } else {
        let var_attrs = var_attrs.unwrap().downcast::<CFDictionary>().unwrap();
        assert!(var_attrs.is_empty());
    }

    fn check_attrs(
        clamp_diff: f64,
        var_attrs: Option<ItemRef<'_, CFType>>,
        axes: CFArray<CFDictionary<CFString, CFType>>,
    ) {
        let var_attrs = var_attrs.unwrap().downcast::<CFDictionary>().unwrap();
        assert!(!var_attrs.is_empty());
        let var_attrs: CFDictionary<CFType, CFType> = unsafe { std::mem::transmute(var_attrs) };
        // attributes greater than max remain
        for axis in axes.iter() {
            let tag = axis
                .find(unsafe { kCTFontVariationAxisIdentifierKey })
                .unwrap();
            let max = axis
                .find(unsafe { kCTFontVariationAxisMaximumValueKey })
                .unwrap()
                .downcast::<CFNumber>()
                .unwrap()
                .to_f64()
                .unwrap();
            let val = var_attrs
                .find(tag.clone())
                .unwrap()
                .downcast::<CFNumber>()
                .unwrap()
                .to_f64()
                .unwrap();

            let expected = max + clamp_diff;
            assert_eq!(
                val, expected,
                "axis {:?} = {:?} (expected {:?})",
                tag, val, expected
            );
        }
    }
}

#[test]
fn equal_descriptor_different_font() {
    use crate::*;

    let variation_attribute =
        unsafe { CFString::wrap_under_get_rule(font_descriptor::kCTFontVariationAttribute) };
    let size_attribute =
        unsafe { CFString::wrap_under_get_rule(font_descriptor::kCTFontSizeAttribute) };

    let sys_font = new_ui_font_for_language(kCTFontSystemDetailFontType, 19., None);

    // but we can still construct the CGFont by name
    let create_vars = |desc| {
        let vals: Vec<(CFNumber, CFNumber)> =
            vec![(CFNumber::from(0x6f70737a), CFNumber::from(17.))];
        let vals_dict = CFDictionary::from_CFType_pairs(&vals);
        let attrs_dict =
            CFDictionary::from_CFType_pairs(&[(variation_attribute.clone(), vals_dict)]);
        let size_attrs_dict =
            CFDictionary::from_CFType_pairs(&[(size_attribute.clone(), CFNumber::from(120.))]);
        dbg!(&desc);
        let from_font_desc = new_from_descriptor(&desc, 120.).copy_descriptor();
        let resized_font_desc = desc
            .create_copy_with_attributes(size_attrs_dict.to_untyped())
            .unwrap();
        if macos_version() >= (11, 0, 0) {
            assert_eq!(from_font_desc, resized_font_desc);
        } else {
            // we won't have a name if we're using system font desc
            if from_font_desc
                .attributes()
                .find(unsafe { font_descriptor::kCTFontNameAttribute })
                .is_none()
            {
                // it's surprising that desc's are the not equal but the attributes are
                assert_ne!(from_font_desc, resized_font_desc);
                assert_eq!(
                    from_font_desc.attributes().to_untyped(),
                    resized_font_desc.attributes().to_untyped()
                );
            } else if macos_version() >= (10, 13, 0) {
                // this is unsurprising
                assert_ne!(from_font_desc, resized_font_desc);
                assert_ne!(
                    from_font_desc.attributes().to_untyped(),
                    resized_font_desc.attributes().to_untyped()
                );
            } else {
                assert_ne!(from_font_desc, resized_font_desc);
                assert_eq!(
                    from_font_desc.attributes().to_untyped(),
                    resized_font_desc.attributes().to_untyped()
                );
            }
        }

        let from_font_desc = from_font_desc
            .create_copy_with_attributes(attrs_dict.to_untyped())
            .unwrap();
        let resized_font_desc = resized_font_desc
            .create_copy_with_attributes(attrs_dict.to_untyped())
            .unwrap();
        (from_font_desc, resized_font_desc)
    };

    // setting the variation works properly if we use system font desc
    let (from_font_desc, resized_font_desc) = create_vars(sys_font.copy_descriptor());
    assert_eq!(from_font_desc, resized_font_desc);
    assert!(resized_font_desc
        .attributes()
        .find(variation_attribute.clone())
        .is_some());

    // but doesn't if we refer to it by name
    let ps = sys_font.postscript_name();
    let cgfont = CGFont::from_name(&CFString::new(&ps)).unwrap();
    let ctfont = new_from_CGFont(&cgfont, 0.);

    let (from_font_desc, resized_font_desc) = create_vars(ctfont.copy_descriptor());
    if macos_version() >= (10, 15, 0) {
        assert_ne!(from_font_desc, resized_font_desc);
    }

    if macos_version() >= (10, 13, 0) {
        assert!(from_font_desc
            .attributes()
            .find(variation_attribute.clone())
            .is_some());
        if macos_version() >= (11, 0, 0) {
            assert!(resized_font_desc
                .attributes()
                .find(variation_attribute)
                .is_none());
        } else {
            assert!(resized_font_desc
                .attributes()
                .find(variation_attribute)
                .is_some());
        };
    }
}

#[test]
fn system_font_variation() {
    use crate::*;

    let small = new_ui_font_for_language(kCTFontSystemDetailFontType, 19., None);

    // but we can still construct the CGFont by name
    let ps = small.postscript_name();
    let cgfont = CGFont::from_name(&CFString::new(&ps)).unwrap();
    let cgfont = new_from_CGFont(&cgfont, 0.);
    let desc = cgfont.copy_descriptor();

    let vals: Vec<(CFNumber, CFNumber)> =
        vec![(CFNumber::from(0x6f70737a /* opsz */), CFNumber::from(17.))];
    let vals_dict = CFDictionary::from_CFType_pairs(&vals);
    let variation_attribute =
        unsafe { CFString::wrap_under_get_rule(font_descriptor::kCTFontVariationAttribute) };
    let attrs_dict = CFDictionary::from_CFType_pairs(&[(variation_attribute, vals_dict)]);
    let ct_var_font_desc = desc
        .create_copy_with_attributes(attrs_dict.to_untyped())
        .unwrap();
    let attrs = ct_var_font_desc.attributes();
    let var_attr = attrs.find(CFString::from_static_string("NSCTFontVariationAttribute"));
    if macos_version() >= (11, 0, 0) {
        // the variation goes away
        assert!(var_attr.is_none());
    } else {
        assert!(var_attr.is_some());
    }

    dbg!(ct_var_font_desc);
}

#[test]
fn ui_font() {
    // pass some garbagey inputs
    new_ui_font_for_language(
        kCTFontSystemDetailFontType,
        10000009.,
        Some(CFString::from("Gofld")),
    );
}

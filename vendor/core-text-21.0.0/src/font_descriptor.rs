// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_upper_case_globals)]

use core_foundation::array::CFArrayRef;
use core_foundation::base::{CFType, CFTypeID, CFTypeRef, TCFType};
use core_foundation::dictionary::{CFDictionary, CFDictionaryRef};
use core_foundation::number::{CFNumber, CFNumberRef};
use core_foundation::set::CFSetRef;
use core_foundation::string::{CFString, CFStringRef};
use core_foundation::url::{CFURLRef, CFURL};
use core_foundation::{declare_TCFType, impl_CFTypeDescription, impl_TCFType};
use core_graphics::base::CGFloat;

use std::os::raw::c_void;
use std::path::PathBuf;

/*
* CTFontTraits.h
*/
// actually, these are extern enums
pub type CTFontFormat = u32;
pub const kCTFontFormatUnrecognized: CTFontFormat = 0;
pub const kCTFontFormatOpenTypePostScript: CTFontFormat = 1;
pub const kCTFontFormatOpenTypeTrueType: CTFontFormat = 2;
pub const kCTFontFormatTrueType: CTFontFormat = 3;
pub const kCTFontFormatPostScript: CTFontFormat = 4;
pub const kCTFontFormatBitmap: CTFontFormat = 5;

pub const kCTFontClassMaskShift: u32 = 28;

pub type CTFontSymbolicTraits = u32;
pub const kCTFontItalicTrait: CTFontSymbolicTraits = 1 << 0;
pub const kCTFontBoldTrait: CTFontSymbolicTraits = 1 << 1;
pub const kCTFontExpandedTrait: CTFontSymbolicTraits = 1 << 5;
pub const kCTFontCondensedTrait: CTFontSymbolicTraits = 1 << 6;
pub const kCTFontMonoSpaceTrait: CTFontSymbolicTraits = 1 << 10;
pub const kCTFontVerticalTrait: CTFontSymbolicTraits = 1 << 11;
pub const kCTFontUIOptimizedTrait: CTFontSymbolicTraits = 1 << 12;
pub const kCTFontColorGlyphsTrait: CTFontSymbolicTraits = 1 << 13;
pub const kCTFontClassMaskTrait: CTFontSymbolicTraits = 15 << kCTFontClassMaskShift;

pub trait SymbolicTraitAccessors {
    fn is_italic(&self) -> bool;
    fn is_bold(&self) -> bool;
    fn is_expanded(&self) -> bool;
    fn is_condensed(&self) -> bool;
    fn is_monospace(&self) -> bool;
    fn is_vertical(&self) -> bool;
}

impl SymbolicTraitAccessors for CTFontSymbolicTraits {
    fn is_italic(&self) -> bool {
        (*self & kCTFontItalicTrait) != 0
    }
    fn is_bold(&self) -> bool {
        (*self & kCTFontBoldTrait) != 0
    }
    fn is_expanded(&self) -> bool {
        (*self & kCTFontExpandedTrait) != 0
    }
    fn is_condensed(&self) -> bool {
        (*self & kCTFontCondensedTrait) != 0
    }
    fn is_monospace(&self) -> bool {
        (*self & kCTFontMonoSpaceTrait) != 0
    }
    fn is_vertical(&self) -> bool {
        (*self & kCTFontVerticalTrait) != 0
    }
}

pub type CTFontStylisticClass = u32;
pub const kCTFontUnknownClass: CTFontStylisticClass = 0 << kCTFontClassMaskShift;
pub const kCTFontOldStyleSerifsClass: CTFontStylisticClass = 1 << kCTFontClassMaskShift;
pub const kCTFontTransitionalSerifsClass: CTFontStylisticClass = 2 << kCTFontClassMaskShift;
pub const kCTFontModernSerifsClass: CTFontStylisticClass = 3 << kCTFontClassMaskShift;
pub const kCTFontClarendonSerifsClass: CTFontStylisticClass = 4 << kCTFontClassMaskShift;
pub const kCTFontSlabSerifsClass: CTFontStylisticClass = 5 << kCTFontClassMaskShift;
pub const kCTFontFreeformSerifsClass: CTFontStylisticClass = 7 << kCTFontClassMaskShift;
pub const kCTFontSansSerifClass: CTFontStylisticClass = 8 << kCTFontClassMaskShift;
pub const kCTFontOrnamentalsClass: CTFontStylisticClass = 9 << kCTFontClassMaskShift;
pub const kCTFontScriptsClass: CTFontStylisticClass = 10 << kCTFontClassMaskShift;
pub const kCTFontSymbolicClass: CTFontStylisticClass = 12 << kCTFontClassMaskShift;

pub trait StylisticClassAccessors {
    fn is_serif(&self) -> bool;
    fn is_sans_serif(&self) -> bool;
    fn is_script(&self) -> bool;
    fn is_fantasy(&self) -> bool;
    fn is_symbols(&self) -> bool;
}

impl StylisticClassAccessors for CTFontStylisticClass {
    fn is_serif(&self) -> bool {
        let any_serif_class = kCTFontOldStyleSerifsClass
            | kCTFontTransitionalSerifsClass
            | kCTFontModernSerifsClass
            | kCTFontClarendonSerifsClass
            | kCTFontSlabSerifsClass
            | kCTFontFreeformSerifsClass;

        (*self & any_serif_class) != 0
    }

    fn is_sans_serif(&self) -> bool {
        (*self & kCTFontSansSerifClass) != 0
    }

    fn is_script(&self) -> bool {
        (*self & kCTFontScriptsClass) != 0
    }

    fn is_fantasy(&self) -> bool {
        (*self & kCTFontOrnamentalsClass) != 0
    }

    fn is_symbols(&self) -> bool {
        (*self & kCTFontSymbolicClass) != 0
    }
}

pub type CTFontAttributes = CFDictionary;

pub type CTFontTraits = CFDictionary<CFString, CFType>;

pub trait TraitAccessors {
    fn symbolic_traits(&self) -> CTFontSymbolicTraits;
    fn normalized_weight(&self) -> f64;
    fn normalized_width(&self) -> f64;
    fn normalized_slant(&self) -> f64;
}

trait TraitAccessorPrivate {
    fn extract_number_for_key(&self, key: CFStringRef) -> CFNumber;
}

impl TraitAccessorPrivate for CTFontTraits {
    fn extract_number_for_key(&self, key: CFStringRef) -> CFNumber {
        let cftype = self.get(key);
        cftype.downcast::<CFNumber>().unwrap()
    }
}

impl TraitAccessors for CTFontTraits {
    fn symbolic_traits(&self) -> CTFontSymbolicTraits {
        unsafe {
            let number = self.extract_number_for_key(kCTFontSymbolicTrait);
            number.to_i64().unwrap() as u32
        }
    }

    fn normalized_weight(&self) -> f64 {
        unsafe {
            let number = self.extract_number_for_key(kCTFontWeightTrait);
            number.to_f64().unwrap()
        }
    }

    fn normalized_width(&self) -> f64 {
        unsafe {
            let number = self.extract_number_for_key(kCTFontWidthTrait);
            number.to_f64().unwrap()
        }
    }

    fn normalized_slant(&self) -> f64 {
        unsafe {
            let number = self.extract_number_for_key(kCTFontSlantTrait);
            number.to_f64().unwrap()
        }
    }
}

/*
* CTFontDescriptor.h
*/
pub type CTFontOrientation = u32;
pub const kCTFontDefaultOrientation: CTFontOrientation = 0; // deprecated since macOS 10.11, iOS 9
pub const kCTFontOrientationDefault: CTFontOrientation = 0; // introduced in macOS 10.8, iOS 6
pub const kCTFontHorizontalOrientation: CTFontOrientation = 1; // deprecated since macOS 10.11, iOS 9
pub const kCTFontOrientationHorizontal: CTFontOrientation = 1; // introduced in macOS 10.8, iOS 6
pub const kCTFontVerticalOrientation: CTFontOrientation = 2; // deprecated since macOS 10.11, iOS 9
pub const kCTFontOrientationVertical: CTFontOrientation = 2; // introduced in macOS 10.8, iOS 6

pub type CTFontPriority = u32;
pub const kCTFontPrioritySystem: CTFontPriority = 10000;
pub const kCTFontPriorityNetwork: CTFontPriority = 20000;
pub const kCTFontPriorityComputer: CTFontPriority = 30000;
pub const kCTFontPriorityUser: CTFontPriority = 40000;
pub const kCTFontPriorityDynamic: CTFontPriority = 50000;
pub const kCTFontPriorityProcess: CTFontPriority = 60000;

#[repr(C)]
pub struct __CTFontDescriptor(c_void);

pub type CTFontDescriptorRef = *const __CTFontDescriptor;

declare_TCFType! {
    CTFontDescriptor, CTFontDescriptorRef
}
impl_TCFType!(
    CTFontDescriptor,
    CTFontDescriptorRef,
    CTFontDescriptorGetTypeID
);
impl_CFTypeDescription!(CTFontDescriptor);

// "Font objects (CTFont, CTFontDescriptor, and associated objects) can be used
//  simultaneously by multiple operations, work queues, or threads."
unsafe impl Send for CTFontDescriptor {}
unsafe impl Sync for CTFontDescriptor {}

impl CTFontDescriptor {
    fn get_string_attribute(&self, attribute: CFStringRef) -> Option<String> {
        unsafe {
            let value = CTFontDescriptorCopyAttribute(self.0, attribute);
            if value.is_null() {
                return None;
            }

            let value = CFType::wrap_under_create_rule(value);
            assert!(value.instance_of::<CFString>());
            let s = CFString::wrap_under_get_rule(value.as_CFTypeRef() as CFStringRef);
            Some(s.to_string())
        }
    }
}

impl CTFontDescriptor {
    pub fn family_name(&self) -> String {
        unsafe {
            let value = self.get_string_attribute(kCTFontFamilyNameAttribute);
            value.expect("A font must have a non-null family name.")
        }
    }

    pub fn font_name(&self) -> String {
        unsafe {
            let value = self.get_string_attribute(kCTFontNameAttribute);
            value.expect("A font must have a non-null name.")
        }
    }

    pub fn style_name(&self) -> String {
        unsafe {
            let value = self.get_string_attribute(kCTFontStyleNameAttribute);
            value.expect("A font must have a non-null style name.")
        }
    }

    pub fn display_name(&self) -> String {
        unsafe {
            let value = self.get_string_attribute(kCTFontDisplayNameAttribute);
            value.expect("A font must have a non-null display name.")
        }
    }

    pub fn font_format(&self) -> Option<CTFontFormat> {
        unsafe {
            let value = CTFontDescriptorCopyAttribute(self.0, kCTFontFormatAttribute);
            if value.is_null() {
                return None;
            }

            let value = CFType::wrap_under_create_rule(value);
            assert!(value.instance_of::<CFNumber>());
            let format = CFNumber::wrap_under_get_rule(value.as_CFTypeRef() as CFNumberRef);
            format.to_i32().map(|x| x as CTFontFormat)
        }
    }

    pub fn font_path(&self) -> Option<PathBuf> {
        unsafe {
            let value = CTFontDescriptorCopyAttribute(self.0, kCTFontURLAttribute);
            if value.is_null() {
                return None;
            }

            let value = CFType::wrap_under_create_rule(value);
            assert!(value.instance_of::<CFURL>());
            let url = CFURL::wrap_under_get_rule(value.as_CFTypeRef() as CFURLRef);
            url.to_path()
        }
    }

    pub fn traits(&self) -> CTFontTraits {
        unsafe {
            let value = CTFontDescriptorCopyAttribute(self.0, kCTFontTraitsAttribute);
            assert!(!value.is_null());
            let value = CFType::wrap_under_create_rule(value);
            assert!(value.instance_of::<CFDictionary>());
            CFDictionary::wrap_under_get_rule(value.as_CFTypeRef() as CFDictionaryRef)
        }
    }

    pub fn create_copy_with_attributes(&self, attr: CFDictionary) -> Result<CTFontDescriptor, ()> {
        unsafe {
            let desc = CTFontDescriptorCreateCopyWithAttributes(
                self.as_concrete_TypeRef(),
                attr.as_concrete_TypeRef(),
            );
            if desc.is_null() {
                return Err(());
            }
            Ok(CTFontDescriptor::wrap_under_create_rule(desc))
        }
    }

    pub fn attributes(&self) -> CFDictionary<CFString, CFType> {
        unsafe {
            let attrs = CTFontDescriptorCopyAttributes(self.as_concrete_TypeRef());
            CFDictionary::wrap_under_create_rule(attrs)
        }
    }
}

pub fn new_from_attributes(attributes: &CFDictionary<CFString, CFType>) -> CTFontDescriptor {
    unsafe {
        let result: CTFontDescriptorRef =
            CTFontDescriptorCreateWithAttributes(attributes.as_concrete_TypeRef());
        CTFontDescriptor::wrap_under_create_rule(result)
    }
}

pub fn new_from_variations(variations: &CFDictionary<CFString, CFNumber>) -> CTFontDescriptor {
    unsafe {
        let var_key = CFString::wrap_under_get_rule(kCTFontVariationAttribute);
        let var_val = CFType::wrap_under_get_rule(variations.as_CFTypeRef());
        let attributes = CFDictionary::from_CFType_pairs(&[(var_key, var_val)]);
        new_from_attributes(&attributes)
    }
}

pub fn new_from_postscript_name(name: &CFString) -> CTFontDescriptor {
    unsafe {
        let result: CTFontDescriptorRef =
            CTFontDescriptorCreateWithNameAndSize(name.as_concrete_TypeRef(), 0.0);
        CTFontDescriptor::wrap_under_create_rule(result)
    }
}

pub fn debug_descriptor(desc: &CTFontDescriptor) {
    println!("family: {}", desc.family_name());
    println!("name: {}", desc.font_name());
    println!("style: {}", desc.style_name());
    println!("display: {}", desc.display_name());
    println!("path: {:?}", desc.font_path());
    desc.show();
}

extern "C" {
    /*
     * CTFontTraits.h
     */

    // font trait constants
    pub static kCTFontSymbolicTrait: CFStringRef;
    pub static kCTFontWeightTrait: CFStringRef;
    pub static kCTFontWidthTrait: CFStringRef;
    pub static kCTFontSlantTrait: CFStringRef;

    /*
     * CTFontDescriptor.h
     */

    // font attribute constants. Note that the name-related attributes
    // here are somewhat flaky. Servo creates CTFont instances and
    // then uses CTFontCopyName to get more fine-grained names.
    pub static kCTFontURLAttribute: CFStringRef; // value: CFURLRef
    pub static kCTFontNameAttribute: CFStringRef; // value: CFStringRef
    pub static kCTFontDisplayNameAttribute: CFStringRef; // value: CFStringRef
    pub static kCTFontFamilyNameAttribute: CFStringRef; // value: CFStringRef
    pub static kCTFontStyleNameAttribute: CFStringRef; // value: CFStringRef
    pub static kCTFontTraitsAttribute: CFStringRef;
    pub static kCTFontVariationAttribute: CFStringRef;
    pub static kCTFontSizeAttribute: CFStringRef;
    pub static kCTFontMatrixAttribute: CFStringRef;
    pub static kCTFontCascadeListAttribute: CFStringRef;
    pub static kCTFontCharacterSetAttribute: CFStringRef;
    pub static kCTFontLanguagesAttribute: CFStringRef;
    pub static kCTFontBaselineAdjustAttribute: CFStringRef;
    pub static kCTFontMacintoshEncodingsAttribute: CFStringRef;
    pub static kCTFontFeaturesAttribute: CFStringRef;
    pub static kCTFontFeatureSettingsAttribute: CFStringRef;
    pub static kCTFontFixedAdvanceAttribute: CFStringRef;
    pub static kCTFontOrientationAttribute: CFStringRef;
    pub static kCTFontFormatAttribute: CFStringRef;
    pub static kCTFontRegistrationScopeAttribute: CFStringRef;
    pub static kCTFontPriorityAttribute: CFStringRef;
    pub static kCTFontEnabledAttribute: CFStringRef;

    pub fn CTFontDescriptorCopyAttribute(
        descriptor: CTFontDescriptorRef,
        attribute: CFStringRef,
    ) -> CFTypeRef;
    pub fn CTFontDescriptorCopyAttributes(descriptor: CTFontDescriptorRef) -> CFDictionaryRef;
    pub fn CTFontDescriptorCopyLocalizedAttribute(
        descriptor: CTFontDescriptorRef,
        attribute: CFStringRef,
        language: *mut CFStringRef,
    ) -> CFTypeRef;
    pub fn CTFontDescriptorCreateCopyWithAttributes(
        original: CTFontDescriptorRef,
        attributes: CFDictionaryRef,
    ) -> CTFontDescriptorRef;
    pub fn CTFontDescriptorCreateCopyWithFeature(
        original: CTFontDescriptorRef,
        featureTypeIdentifier: CFNumberRef,
        featureSelectorIdentifier: CFNumberRef,
    ) -> CTFontDescriptorRef;
    pub fn CTFontDescriptorCreateCopyWithVariation(
        original: CTFontDescriptorRef,
        variationIdentifier: CFNumberRef,
        variationValue: CGFloat,
    ) -> CTFontDescriptorRef;
    pub fn CTFontDescriptorCreateMatchingFontDescriptor(
        descriptor: CTFontDescriptorRef,
        mandatoryAttributes: CFSetRef,
    ) -> CTFontDescriptorRef;
    pub fn CTFontDescriptorCreateWithAttributes(attributes: CFDictionaryRef)
        -> CTFontDescriptorRef;
    pub fn CTFontDescriptorCreateWithNameAndSize(
        name: CFStringRef,
        size: CGFloat,
    ) -> CTFontDescriptorRef;
    pub fn CTFontDescriptorGetTypeID() -> CFTypeID;
}

extern "C" {
    pub fn CTFontDescriptorCreateMatchingFontDescriptors(
        descriptor: CTFontDescriptorRef,
        mandatoryAttributes: CFSetRef,
    ) -> CFArrayRef;
}

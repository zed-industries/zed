#![allow(unused, non_upper_case_globals)]

use crate::FontFeatures;
use cocoa::appkit::CGFloat;
use core_foundation::{
    array::{
        kCFTypeArrayCallBacks, CFArray, CFArrayAppendValue, CFArrayCreateMutable, CFMutableArrayRef,
    },
    base::{kCFAllocatorDefault, CFRelease, TCFType},
    dictionary::{
        kCFTypeDictionaryKeyCallBacks, kCFTypeDictionaryValueCallBacks, CFDictionaryCreate,
    },
    number::CFNumber,
    string::{CFString, CFStringRef},
};
use core_graphics::{display::CFDictionary, geometry::CGAffineTransform};
use core_text::{
    font::{CTFont, CTFontRef},
    font_descriptor::{
        kCTFontFeatureSettingsAttribute, CTFontDescriptor, CTFontDescriptorCopyAttributes,
        CTFontDescriptorCreateCopyWithFeature, CTFontDescriptorCreateWithAttributes,
        CTFontDescriptorRef,
    },
};
use font_kit::font::Font;
use std::ptr;

pub fn generate_feature_array(features: &FontFeatures) -> CFMutableArrayRef {
    unsafe {
        let mut feature_array =
            CFArrayCreateMutable(kCFAllocatorDefault, 0, &kCFTypeArrayCallBacks);
        for (tag, value) in features.tag_value_list() {
            let keys = [kCTFontOpenTypeFeatureTag, kCTFontOpenTypeFeatureValue];
            let values = [
                CFString::new(&tag).as_CFTypeRef(),
                CFNumber::from(*value as i32).as_CFTypeRef(),
            ];
            let dict = CFDictionaryCreate(
                kCFAllocatorDefault,
                &keys as *const _ as _,
                &values as *const _ as _,
                2,
                &kCFTypeDictionaryKeyCallBacks,
                &kCFTypeDictionaryValueCallBacks,
            );
            values.into_iter().for_each(|value| CFRelease(value));
            CFArrayAppendValue(feature_array, dict as _);
            CFRelease(dict as _);
        }
        feature_array
    }
}

#[link(name = "CoreText", kind = "framework")]
extern "C" {
    static kCTFontOpenTypeFeatureTag: CFStringRef;
    static kCTFontOpenTypeFeatureValue: CFStringRef;

    pub fn CTFontCreateCopyWithAttributes(
        font: CTFontRef,
        size: CGFloat,
        matrix: *const CGAffineTransform,
        attributes: CTFontDescriptorRef,
    ) -> CTFontRef;
}

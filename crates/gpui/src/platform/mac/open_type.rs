#![allow(unused, non_upper_case_globals)]

use crate::{FontFallbacks, FontFeatures};
use cocoa::appkit::CGFloat;
use core_foundation::{
    array::{
        kCFTypeArrayCallBacks, CFArray, CFArrayAppendArray, CFArrayAppendValue,
        CFArrayCreateMutable, CFArrayGetCount, CFArrayGetValueAtIndex, CFArrayRef,
        CFMutableArrayRef,
    },
    base::{kCFAllocatorDefault, CFRelease, TCFType},
    dictionary::{
        kCFTypeDictionaryKeyCallBacks, kCFTypeDictionaryValueCallBacks, CFDictionaryCreate,
    },
    number::CFNumber,
    string::{CFString, CFStringRef},
};
use core_foundation_sys::locale::CFLocaleCopyPreferredLanguages;
use core_graphics::{display::CFDictionary, geometry::CGAffineTransform};
use core_text::{
    font::{cascade_list_for_languages, CTFont, CTFontRef},
    font_descriptor::{
        kCTFontCascadeListAttribute, kCTFontFeatureSettingsAttribute, CTFontDescriptor,
        CTFontDescriptorCopyAttributes, CTFontDescriptorCreateCopyWithFeature,
        CTFontDescriptorCreateWithAttributes, CTFontDescriptorCreateWithNameAndSize,
        CTFontDescriptorRef,
    },
};
use font_kit::font::Font as FontKitFont;
use std::ptr;

pub fn apply_features_and_fallbacks(
    font: &mut FontKitFont,
    features: &FontFeatures,
    fallbacks: Option<&FontFallbacks>,
) -> anyhow::Result<()> {
    unsafe {
        let fallback_array = CFArrayCreateMutable(kCFAllocatorDefault, 0, &kCFTypeArrayCallBacks);

        if let Some(fallbacks) = fallbacks {
            for user_fallback in fallbacks.fallback_list() {
                let name = CFString::from(user_fallback.as_str());
                let fallback_desc =
                    CTFontDescriptorCreateWithNameAndSize(name.as_concrete_TypeRef(), 0.0);
                CFArrayAppendValue(fallback_array, fallback_desc as _);
                CFRelease(fallback_desc as _);
            }
        }

        {
            let preferred_languages: CFArray<CFString> =
                CFArray::wrap_under_create_rule(CFLocaleCopyPreferredLanguages());

            let default_fallbacks = CTFontCopyDefaultCascadeListForLanguages(
                font.native_font().as_concrete_TypeRef(),
                preferred_languages.as_concrete_TypeRef(),
            );
            let default_fallbacks: CFArray<CTFontDescriptor> =
                CFArray::wrap_under_create_rule(default_fallbacks);

            default_fallbacks
                .iter()
                .filter(|desc| desc.font_path().is_some())
                .map(|desc| {
                    CFArrayAppendValue(fallback_array, desc.as_concrete_TypeRef() as _);
                });
        }

        let feature_array = generate_feature_array(features);
        let keys = [kCTFontFeatureSettingsAttribute, kCTFontCascadeListAttribute];
        let values = [feature_array, fallback_array];
        let attrs = CFDictionaryCreate(
            kCFAllocatorDefault,
            keys.as_ptr() as _,
            values.as_ptr() as _,
            2,
            &kCFTypeDictionaryKeyCallBacks,
            &kCFTypeDictionaryValueCallBacks,
        );
        CFRelease(feature_array as *const _ as _);
        CFRelease(fallback_array as *const _ as _);
        let new_descriptor = CTFontDescriptorCreateWithAttributes(attrs);
        CFRelease(attrs as _);
        let new_descriptor = CTFontDescriptor::wrap_under_create_rule(new_descriptor);
        let new_font = CTFontCreateCopyWithAttributes(
            font.native_font().as_concrete_TypeRef(),
            0.0,
            std::ptr::null(),
            new_descriptor.as_concrete_TypeRef(),
        );
        let new_font = CTFont::wrap_under_create_rule(new_font);
        *font = font_kit::font::Font::from_native_font(&new_font);

        Ok(())
    }
}

fn generate_feature_array(features: &FontFeatures) -> CFMutableArrayRef {
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

    fn CTFontCreateCopyWithAttributes(
        font: CTFontRef,
        size: CGFloat,
        matrix: *const CGAffineTransform,
        attributes: CTFontDescriptorRef,
    ) -> CTFontRef;
    fn CTFontCopyDefaultCascadeListForLanguages(
        font: CTFontRef,
        languagePrefList: CFArrayRef,
    ) -> CFArrayRef;
}

// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::font_descriptor::{CTFontDescriptor, CTFontDescriptorRef};
use core_foundation::array::{CFArray, CFArrayRef};
use core_foundation::base::TCFType;
use core_foundation::data::{CFData, CFDataRef};
use core_foundation::string::CFString;
use core_foundation::url::CFURLRef;

pub fn copy_available_font_family_names() -> CFArray<CFString> {
    unsafe { TCFType::wrap_under_create_rule(CTFontManagerCopyAvailableFontFamilyNames()) }
}

pub fn create_font_descriptor(buffer: &[u8]) -> Result<CTFontDescriptor, ()> {
    let cf_data = CFData::from_buffer(buffer);
    unsafe {
        let ct_font_descriptor_ref =
            CTFontManagerCreateFontDescriptorFromData(cf_data.as_concrete_TypeRef());
        if ct_font_descriptor_ref.is_null() {
            return Err(());
        }
        Ok(CTFontDescriptor::wrap_under_create_rule(
            ct_font_descriptor_ref,
        ))
    }
}

pub fn create_font_descriptor_with_data(data: CFData) -> Result<CTFontDescriptor, ()> {
    unsafe {
        let ct_font_descriptor_ref =
            CTFontManagerCreateFontDescriptorFromData(data.as_concrete_TypeRef());
        if ct_font_descriptor_ref.is_null() {
            return Err(());
        }
        Ok(CTFontDescriptor::wrap_under_create_rule(
            ct_font_descriptor_ref,
        ))
    }
}

extern "C" {
    /*
     * CTFontManager.h
     */

    // Incomplete function bindings are mostly related to CoreText font matching, which
    // we implement in a platform-independent manner using FontMatcher.

    //pub fn CTFontManagerCompareFontFamilyNames
    pub fn CTFontManagerCopyAvailableFontURLs() -> CFArrayRef;
    pub fn CTFontManagerCopyAvailableFontFamilyNames() -> CFArrayRef;
    pub fn CTFontManagerCopyAvailablePostScriptNames() -> CFArrayRef;
    pub fn CTFontManagerCreateFontDescriptorsFromURL(fileURL: CFURLRef) -> CFArrayRef;
    pub fn CTFontManagerCreateFontDescriptorFromData(data: CFDataRef) -> CTFontDescriptorRef;
    //pub fn CTFontManagerCreateFontRequestRunLoopSource
    //pub fn CTFontManagerEnableFontDescriptors
    //pub fn CTFontManagerGetAutoActivationSetting
    //pub fn CTFontManagerGetScopeForURL
    //pub fn CTFontManagerGetAutoActivationSetting
    //pub fn CTFontManagerGetScopeForURL
    pub fn CTFontManagerIsSupportedFont(fontURL: CFURLRef) -> bool;
    //pub fn CTFontManagerRegisterFontsForURL
    //pub fn CTFontManagerRegisterFontsForURLs
    //pub fn CTFontManagerRegisterGraphicsFont
    //pub fn CTFontManagerSetAutoActivationSetting
    //pub fn CTFontManagerUnregisterFontsForURL
    //pub fn CTFontManagerUnregisterFontsForURLs
    //pub fn CTFontManagerUnregisterGraphicsFont
}

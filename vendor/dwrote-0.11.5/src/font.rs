/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::cell::UnsafeCell;
use std::mem;
use std::ptr;
use winapi::shared::minwindef::{FALSE, TRUE};
use winapi::shared::winerror::S_OK;
use winapi::um::dwrite::IDWriteFont;
use winapi::um::dwrite::IDWriteFontFace;
use winapi::um::dwrite::IDWriteFontFamily;
use winapi::um::dwrite::IDWriteLocalizedStrings;
use winapi::um::dwrite::DWRITE_FONT_METRICS;
use winapi::um::dwrite::DWRITE_INFORMATIONAL_STRING_COPYRIGHT_NOTICE;
use winapi::um::dwrite::DWRITE_INFORMATIONAL_STRING_DESCRIPTION;
use winapi::um::dwrite::DWRITE_INFORMATIONAL_STRING_DESIGNER;
use winapi::um::dwrite::DWRITE_INFORMATIONAL_STRING_DESIGNER_URL;
use winapi::um::dwrite::DWRITE_INFORMATIONAL_STRING_DESIGN_SCRIPT_LANGUAGE_TAG;
use winapi::um::dwrite::DWRITE_INFORMATIONAL_STRING_FONT_VENDOR_URL;
use winapi::um::dwrite::DWRITE_INFORMATIONAL_STRING_FULL_NAME;
use winapi::um::dwrite::DWRITE_INFORMATIONAL_STRING_ID;
use winapi::um::dwrite::DWRITE_INFORMATIONAL_STRING_LICENSE_DESCRIPTION;
use winapi::um::dwrite::DWRITE_INFORMATIONAL_STRING_LICENSE_INFO_URL;
use winapi::um::dwrite::DWRITE_INFORMATIONAL_STRING_MANUFACTURER;
use winapi::um::dwrite::DWRITE_INFORMATIONAL_STRING_POSTSCRIPT_CID_NAME;
use winapi::um::dwrite::DWRITE_INFORMATIONAL_STRING_POSTSCRIPT_NAME;
use winapi::um::dwrite::DWRITE_INFORMATIONAL_STRING_PREFERRED_FAMILY_NAMES;
use winapi::um::dwrite::DWRITE_INFORMATIONAL_STRING_PREFERRED_SUBFAMILY_NAMES;
use winapi::um::dwrite::DWRITE_INFORMATIONAL_STRING_SAMPLE_TEXT;
use winapi::um::dwrite::DWRITE_INFORMATIONAL_STRING_SUPPORTED_SCRIPT_LANGUAGE_TAG;
use winapi::um::dwrite::DWRITE_INFORMATIONAL_STRING_TRADEMARK;
use winapi::um::dwrite::DWRITE_INFORMATIONAL_STRING_VERSION_STRINGS;
use winapi::um::dwrite::DWRITE_INFORMATIONAL_STRING_WIN32_FAMILY_NAMES;
use winapi::um::dwrite::DWRITE_INFORMATIONAL_STRING_WIN32_SUBFAMILY_NAMES;
use winapi::um::dwrite::DWRITE_INFORMATIONAL_STRING_WWS_FAMILY_NAME;
use winapi::um::dwrite_1::{IDWriteFont1, DWRITE_FONT_METRICS1};
use wio::com::ComPtr;

use super::*;
use helpers::*;

pub struct Font {
    native: UnsafeCell<ComPtr<IDWriteFont>>,
}

impl Font {
    pub fn take(native: ComPtr<IDWriteFont>) -> Font {
        Font {
            native: UnsafeCell::new(native),
        }
    }

    pub unsafe fn as_ptr(&self) -> *mut IDWriteFont {
        (*self.native.get()).as_raw()
    }

    pub fn to_descriptor(&self) -> FontDescriptor {
        FontDescriptor {
            family_name: self.family_name(),
            stretch: self.stretch(),
            style: self.style(),
            weight: self.weight(),
        }
    }

    pub fn stretch(&self) -> FontStretch {
        unsafe { mem::transmute::<u32, FontStretch>((*self.native.get()).GetStretch()) }
    }

    pub fn style(&self) -> FontStyle {
        unsafe { mem::transmute::<u32, FontStyle>((*self.native.get()).GetStyle()) }
    }

    pub fn weight(&self) -> FontWeight {
        unsafe { FontWeight::from_u32((*self.native.get()).GetWeight()) }
    }

    pub fn is_monospace(&self) -> Option<bool> {
        unsafe {
            let font1: Option<ComPtr<IDWriteFont1>> = (*self.native.get()).cast().ok();
            font1.map(|font| font.IsMonospacedFont() == TRUE)
        }
    }

    pub fn simulations(&self) -> FontSimulations {
        unsafe { mem::transmute::<u32, FontSimulations>((*self.native.get()).GetSimulations()) }
    }

    pub fn family_name(&self) -> String {
        unsafe {
            let mut family: *mut IDWriteFontFamily = ptr::null_mut();
            let hr = (*self.native.get()).GetFontFamily(&mut family);
            assert!(hr == 0);

            FontFamily::take(ComPtr::from_raw(family)).name()
        }
    }

    pub fn face_name(&self) -> String {
        unsafe {
            let mut names: *mut IDWriteLocalizedStrings = ptr::null_mut();
            let hr = (*self.native.get()).GetFaceNames(&mut names);
            assert!(hr == 0);

            get_locale_string(&mut ComPtr::from_raw(names))
        }
    }

    pub fn informational_string(&self, id: InformationalStringId) -> Option<String> {
        unsafe {
            let mut names: *mut IDWriteLocalizedStrings = ptr::null_mut();
            let mut exists = FALSE;
            let id = id as DWRITE_INFORMATIONAL_STRING_ID;
            let hr = (*self.native.get()).GetInformationalStrings(id, &mut names, &mut exists);
            assert!(hr == S_OK);
            if exists == TRUE {
                Some(get_locale_string(&mut ComPtr::from_raw(names)))
            } else {
                None
            }
        }
    }

    pub fn create_font_face(&self) -> FontFace {
        // FIXME create_font_face should cache the FontFace and return it,
        // there's a 1:1 relationship
        unsafe {
            let mut face: *mut IDWriteFontFace = ptr::null_mut();
            let hr = (*self.native.get()).CreateFontFace(&mut face);
            assert!(hr == 0);
            FontFace::take(ComPtr::from_raw(face))
        }
    }

    pub fn metrics(&self) -> FontMetrics {
        unsafe {
            let font_1: Option<ComPtr<IDWriteFont1>> = (*self.native.get()).cast().ok();
            match font_1 {
                None => {
                    let mut metrics = mem::zeroed();
                    (*self.native.get()).GetMetrics(&mut metrics);
                    FontMetrics::Metrics0(metrics)
                }
                Some(font_1) => {
                    let mut metrics_1 = mem::zeroed();
                    font_1.GetMetrics(&mut metrics_1);
                    FontMetrics::Metrics1(metrics_1)
                }
            }
        }
    }
}

impl Clone for Font {
    fn clone(&self) -> Font {
        unsafe {
            Font {
                native: UnsafeCell::new((*self.native.get()).clone()),
            }
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InformationalStringId {
    FullName = DWRITE_INFORMATIONAL_STRING_FULL_NAME,
    PostscriptName = DWRITE_INFORMATIONAL_STRING_POSTSCRIPT_NAME,
    PostscriptCidName = DWRITE_INFORMATIONAL_STRING_POSTSCRIPT_CID_NAME,
    CopyrightNotice = DWRITE_INFORMATIONAL_STRING_COPYRIGHT_NOTICE,
    Description = DWRITE_INFORMATIONAL_STRING_DESCRIPTION,
    Designer = DWRITE_INFORMATIONAL_STRING_DESIGNER,
    DesignerUrl = DWRITE_INFORMATIONAL_STRING_DESIGNER_URL,
    DesignScriptLanguageTag = DWRITE_INFORMATIONAL_STRING_DESIGN_SCRIPT_LANGUAGE_TAG,
    VendorUrl = DWRITE_INFORMATIONAL_STRING_FONT_VENDOR_URL,
    LicenseDescription = DWRITE_INFORMATIONAL_STRING_LICENSE_DESCRIPTION,
    LicenseInfoUrl = DWRITE_INFORMATIONAL_STRING_LICENSE_INFO_URL,
    Manufacturer = DWRITE_INFORMATIONAL_STRING_MANUFACTURER,
    PreferredFamilyNames = DWRITE_INFORMATIONAL_STRING_PREFERRED_FAMILY_NAMES,
    PreferredSubfamilyNames = DWRITE_INFORMATIONAL_STRING_PREFERRED_SUBFAMILY_NAMES,
    SampleText = DWRITE_INFORMATIONAL_STRING_SAMPLE_TEXT,
    SupportedScriptLanguageTag = DWRITE_INFORMATIONAL_STRING_SUPPORTED_SCRIPT_LANGUAGE_TAG,
    Trademark = DWRITE_INFORMATIONAL_STRING_TRADEMARK,
    Version = DWRITE_INFORMATIONAL_STRING_VERSION_STRINGS,
    Win32FamilyNames = DWRITE_INFORMATIONAL_STRING_WIN32_FAMILY_NAMES,
    Win32SubfamilyNames = DWRITE_INFORMATIONAL_STRING_WIN32_SUBFAMILY_NAMES,
    WwsFamilyName = DWRITE_INFORMATIONAL_STRING_WWS_FAMILY_NAME,
}

/// A wrapper around the `DWRITE_FONT_METRICS` and `DWRITE_FONT_METRICS1` types.
pub enum FontMetrics {
    /// Windows 7.
    Metrics0(DWRITE_FONT_METRICS),
    /// Windows 8 and up.
    Metrics1(DWRITE_FONT_METRICS1),
}

impl FontMetrics {
    /// Convert self to the Metrics0 arm (throwing away additional information)
    pub fn metrics0(self) -> DWRITE_FONT_METRICS {
        match self {
            FontMetrics::Metrics0(metrics) => metrics,
            FontMetrics::Metrics1(metrics) => DWRITE_FONT_METRICS {
                designUnitsPerEm: metrics.designUnitsPerEm,
                ascent: metrics.ascent,
                descent: metrics.descent,
                lineGap: metrics.lineGap,
                capHeight: metrics.capHeight,
                xHeight: metrics.xHeight,
                underlinePosition: metrics.underlinePosition,
                underlineThickness: metrics.underlineThickness,
                strikethroughPosition: metrics.strikethroughPosition,
                strikethroughThickness: metrics.strikethroughThickness,
            },
        }
    }
}

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::cell::UnsafeCell;
use std::ptr::null_mut;
use winapi::um::dwrite_2::{IDWriteFactory2, IDWriteFontFallback};
use wio::com::ComPtr;

use super::*;

pub struct FontFallback {
    native: UnsafeCell<ComPtr<IDWriteFontFallback>>,
}

pub struct FallbackResult {
    /// Length of mapped substring, in utf-16 code units.
    pub mapped_length: usize,
    /// The font that should be used to render the substring.
    pub mapped_font: Option<Font>,
    /// The scale factor to apply.
    pub scale: f32,
}

impl FontFallback {
    pub fn get_system_fallback() -> Option<FontFallback> {
        unsafe {
            let factory = ComPtr::from_raw(DWriteFactory());
            let factory2: Option<ComPtr<IDWriteFactory2>> = factory.cast().ok();
            std::mem::forget(factory);
            let factory2 = factory2?;
            let mut native = null_mut();
            let hr = factory2.GetSystemFontFallback(&mut native);
            assert_eq!(hr, 0);
            Some(Self::take(ComPtr::from_raw(native)))
        }
    }

    pub fn take(native: ComPtr<IDWriteFontFallback>) -> FontFallback {
        FontFallback {
            native: UnsafeCell::new(native),
        }
    }

    // TODO: I'm following crate conventions for unsafe, but it's bullshit
    pub unsafe fn as_ptr(&self) -> *mut IDWriteFontFallback {
        (*self.native.get()).as_raw()
    }

    // TODO: map_characters (main function)
    pub fn map_characters(
        &self,
        text_analysis_source: &TextAnalysisSource,
        text_position: u32,
        text_length: u32,
        base_font: &FontCollection,
        base_family: Option<&str>,
        base_weight: FontWeight,
        base_style: FontStyle,
        base_stretch: FontStretch,
    ) -> FallbackResult {
        unsafe {
            let mut font = null_mut();
            let mut mapped_length = 0;
            let mut scale = 0.0;
            let hr = (*self.as_ptr()).MapCharacters(
                text_analysis_source.as_ptr(),
                text_position,
                text_length,
                base_font.as_ptr(),
                base_family
                    .map(|s| s.to_wide_null().as_mut_ptr())
                    .unwrap_or(null_mut()),
                base_weight.t(),
                base_style.t(),
                base_stretch.t(),
                &mut mapped_length,
                &mut font,
                &mut scale,
            );
            assert_eq!(hr, 0);
            let mapped_font = if font.is_null() {
                None
            } else {
                Some(Font::take(ComPtr::from_raw(font)))
            };
            FallbackResult {
                mapped_length: mapped_length as usize,
                mapped_font,
                scale,
            }
        }
    }
}

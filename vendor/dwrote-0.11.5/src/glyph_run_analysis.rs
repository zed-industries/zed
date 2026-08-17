/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::cell::UnsafeCell;
use std::mem;
use std::ptr;
use winapi::shared::windef::RECT;
use winapi::um::dcommon::DWRITE_MEASURING_MODE;
use winapi::um::dwrite::DWRITE_TEXTURE_CLEARTYPE_3x1;
use winapi::um::dwrite::IDWriteGlyphRunAnalysis;
use winapi::um::dwrite::{DWRITE_TEXTURE_ALIASED_1x1, DWRITE_GLYPH_RUN, DWRITE_TEXTURE_TYPE};
use winapi::um::dwrite::{DWRITE_MATRIX, DWRITE_RENDERING_MODE};
use winapi::um::winnt::HRESULT;
use wio::com::ComPtr;

use super::DWriteFactory;

pub struct GlyphRunAnalysis {
    native: UnsafeCell<ComPtr<IDWriteGlyphRunAnalysis>>,
}

impl GlyphRunAnalysis {
    pub fn create(
        glyph_run: &DWRITE_GLYPH_RUN,
        pixels_per_dip: f32,
        transform: Option<DWRITE_MATRIX>,
        rendering_mode: DWRITE_RENDERING_MODE,
        measuring_mode: DWRITE_MEASURING_MODE,
        baseline_x: f32,
        baseline_y: f32,
    ) -> Result<GlyphRunAnalysis, HRESULT> {
        unsafe {
            let mut native: *mut IDWriteGlyphRunAnalysis = ptr::null_mut();
            let hr = (*DWriteFactory()).CreateGlyphRunAnalysis(
                glyph_run as *const DWRITE_GLYPH_RUN,
                pixels_per_dip,
                transform
                    .as_ref()
                    .map(|x| x as *const _)
                    .unwrap_or(ptr::null()),
                rendering_mode,
                measuring_mode,
                baseline_x,
                baseline_y,
                &mut native,
            );
            if hr != 0 {
                Err(hr)
            } else {
                Ok(GlyphRunAnalysis::take(ComPtr::from_raw(native)))
            }
        }
    }

    pub fn take(native: ComPtr<IDWriteGlyphRunAnalysis>) -> GlyphRunAnalysis {
        GlyphRunAnalysis {
            native: UnsafeCell::new(native),
        }
    }

    pub fn get_alpha_texture_bounds(
        &self,
        texture_type: DWRITE_TEXTURE_TYPE,
    ) -> Result<RECT, HRESULT> {
        unsafe {
            let mut rect: RECT = mem::zeroed();
            rect.left = 1234;
            rect.top = 1234;
            let hr = (*self.native.get()).GetAlphaTextureBounds(texture_type, &mut rect);
            if hr != 0 {
                Err(hr)
            } else {
                Ok(rect)
            }
        }
    }

    pub fn create_alpha_texture(
        &self,
        texture_type: DWRITE_TEXTURE_TYPE,
        rect: RECT,
    ) -> Result<Vec<u8>, HRESULT> {
        unsafe {
            let rect_pixels = (rect.right - rect.left) * (rect.bottom - rect.top);
            let rect_bytes = rect_pixels
                * match texture_type {
                    DWRITE_TEXTURE_ALIASED_1x1 => 1,
                    DWRITE_TEXTURE_CLEARTYPE_3x1 => 3,
                    _ => panic!("bad texture type specified"),
                };

            let mut out_bytes: Vec<u8> = vec![0; rect_bytes as usize];
            let hr = (*self.native.get()).CreateAlphaTexture(
                texture_type,
                &rect,
                out_bytes.as_mut_ptr(),
                out_bytes.len() as u32,
            );
            if hr != 0 {
                Err(hr)
            } else {
                Ok(out_bytes)
            }
        }
    }
}

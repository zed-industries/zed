/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::cell::UnsafeCell;
use std::mem::{self, zeroed};
use std::slice;
use std::{error, fmt, ptr};
use winapi::ctypes::c_void;
use winapi::shared::minwindef::{BOOL, FALSE, TRUE};
use winapi::shared::winerror::S_OK;
use winapi::um::dcommon::DWRITE_MEASURING_MODE;
use winapi::um::dwrite::IDWriteRenderingParams;
use winapi::um::dwrite::DWRITE_FONT_FACE_TYPE_TRUETYPE;
use winapi::um::dwrite::{IDWriteFontFace, IDWriteFontFile};
use winapi::um::dwrite::{DWRITE_FONT_FACE_TYPE_BITMAP, DWRITE_FONT_FACE_TYPE_CFF};
use winapi::um::dwrite::{DWRITE_FONT_FACE_TYPE_RAW_CFF, DWRITE_FONT_FACE_TYPE_TYPE1};
use winapi::um::dwrite::{DWRITE_FONT_FACE_TYPE_TRUETYPE_COLLECTION, DWRITE_FONT_FACE_TYPE_VECTOR};
use winapi::um::dwrite::{DWRITE_FONT_SIMULATIONS, DWRITE_GLYPH_METRICS};
use winapi::um::dwrite::{DWRITE_GLYPH_OFFSET, DWRITE_MATRIX, DWRITE_RENDERING_MODE};
use winapi::um::dwrite::{DWRITE_RENDERING_MODE_DEFAULT, DWRITE_RENDERING_MODE_NATURAL_SYMMETRIC};
use winapi::um::dwrite_1::IDWriteFontFace1;
use winapi::um::dwrite_3::{
    IDWriteFontFace5, IDWriteFontResource, DWRITE_FONT_AXIS_ATTRIBUTES_VARIABLE,
    DWRITE_FONT_AXIS_VALUE,
};
use winapi::um::winnt::HRESULT;
use winapi::Interface;
use wio::com::ComPtr;

use super::{DWriteFactory, DefaultDWriteRenderParams, FontFile, FontMetrics};
use crate::com_helpers::Com;
use crate::geometry_sink_impl::GeometrySinkImpl;
use crate::outline_builder::OutlineBuilder;
use crate::FontSimulations;

pub struct FontFace {
    native: UnsafeCell<ComPtr<IDWriteFontFace>>,
    face1: UnsafeCell<Option<ComPtr<IDWriteFontFace1>>>,
    face5: UnsafeCell<Option<ComPtr<IDWriteFontFace5>>>,
}

impl FontFace {
    pub fn take(native: ComPtr<IDWriteFontFace>) -> FontFace {
        let cell = UnsafeCell::new(native);
        FontFace {
            native: cell,
            face1: UnsafeCell::new(None),
            face5: UnsafeCell::new(None),
        }
    }

    pub unsafe fn as_ptr(&self) -> *mut IDWriteFontFace {
        (*self.native.get()).as_raw()
    }

    unsafe fn raw_files(&self) -> Result<Vec<*mut IDWriteFontFile>, HRESULT> {
        let mut number_of_files: u32 = 0;
        let hr = (*self.native.get()).GetFiles(&mut number_of_files, ptr::null_mut());
        if hr != S_OK {
            return Err(hr);
        }

        let mut file_ptrs: Vec<*mut IDWriteFontFile> =
            vec![ptr::null_mut(); number_of_files as usize];
        let hr = (*self.native.get()).GetFiles(&mut number_of_files, file_ptrs.as_mut_ptr());
        if hr != S_OK {
            return Err(hr);
        }
        Ok(file_ptrs)
    }

    #[deprecated(note = "Use `files` instead.")]
    pub fn get_files(&self) -> Vec<FontFile> {
        self.files().unwrap()
    }

    pub fn files(&self) -> Result<Vec<FontFile>, HRESULT> {
        unsafe {
            self.raw_files().map(|file_ptrs| {
                file_ptrs
                    .iter()
                    .map(|p| FontFile::take(ComPtr::from_raw(*p)))
                    .collect()
            })
        }
    }

    pub fn create_font_face_with_simulations(
        &self,
        simulations: DWRITE_FONT_SIMULATIONS,
    ) -> FontFace {
        unsafe {
            let file_ptrs = self.raw_files().unwrap();
            let face_type = (*self.native.get()).GetType();
            let face_index = (*self.native.get()).GetIndex();
            let mut face: *mut IDWriteFontFace = ptr::null_mut();
            let hr = (*DWriteFactory()).CreateFontFace(
                face_type,
                file_ptrs.len() as u32,
                file_ptrs.as_ptr(),
                face_index,
                simulations,
                &mut face,
            );
            for p in file_ptrs {
                let _ = ComPtr::<IDWriteFontFile>::from_raw(p);
            }
            assert!(hr == 0);
            FontFace::take(ComPtr::from_raw(face))
        }
    }

    pub fn get_glyph_count(&self) -> u16 {
        unsafe { (*self.native.get()).GetGlyphCount() }
    }

    pub fn metrics(&self) -> FontMetrics {
        unsafe {
            let font_1 = self.get_face1();
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

    #[deprecated(note = "Use `glyph_indices` instead.")]
    pub fn get_glyph_indices(&self, code_points: &[u32]) -> Vec<u16> {
        self.glyph_indices(code_points).unwrap()
    }

    pub fn glyph_indices(&self, code_points: &[u32]) -> Result<Vec<u16>, HRESULT> {
        let mut glyph_indices: Vec<u16> = vec![0; code_points.len()];
        unsafe {
            let hr = (*self.native.get()).GetGlyphIndices(
                code_points.as_ptr(),
                code_points.len() as u32,
                glyph_indices.as_mut_ptr(),
            );
            if hr != S_OK {
                return Err(hr);
            }
            Ok(glyph_indices)
        }
    }

    #[deprecated(note = "Use `design_glyph_metrics` instead.")]
    pub fn get_design_glyph_metrics(
        &self,
        glyph_indices: &[u16],
        is_sideways: bool,
    ) -> Vec<DWRITE_GLYPH_METRICS> {
        self.design_glyph_metrics(glyph_indices, is_sideways)
            .unwrap()
    }

    pub fn design_glyph_metrics(
        &self,
        glyph_indices: &[u16],
        is_sideways: bool,
    ) -> Result<Vec<DWRITE_GLYPH_METRICS>, HRESULT> {
        unsafe {
            let mut metrics: Vec<DWRITE_GLYPH_METRICS> = vec![zeroed(); glyph_indices.len()];
            let hr = (*self.native.get()).GetDesignGlyphMetrics(
                glyph_indices.as_ptr(),
                glyph_indices.len() as u32,
                metrics.as_mut_ptr(),
                is_sideways as BOOL,
            );
            if hr != S_OK {
                return Err(hr);
            }
            Ok(metrics)
        }
    }

    #[deprecated(note = "Use `gdi_compatible_glyph_metrics` instead.")]
    pub fn get_gdi_compatible_glyph_metrics(
        &self,
        em_size: f32,
        pixels_per_dip: f32,
        transform: *const DWRITE_MATRIX,
        use_gdi_natural: bool,
        glyph_indices: &[u16],
        is_sideways: bool,
    ) -> Vec<DWRITE_GLYPH_METRICS> {
        self.gdi_compatible_glyph_metrics(
            em_size,
            pixels_per_dip,
            transform,
            use_gdi_natural,
            glyph_indices,
            is_sideways,
        )
        .unwrap()
    }

    pub fn gdi_compatible_glyph_metrics(
        &self,
        em_size: f32,
        pixels_per_dip: f32,
        transform: *const DWRITE_MATRIX,
        use_gdi_natural: bool,
        glyph_indices: &[u16],
        is_sideways: bool,
    ) -> Result<Vec<DWRITE_GLYPH_METRICS>, HRESULT> {
        unsafe {
            let mut metrics: Vec<DWRITE_GLYPH_METRICS> = vec![zeroed(); glyph_indices.len()];
            let hr = (*self.native.get()).GetGdiCompatibleGlyphMetrics(
                em_size,
                pixels_per_dip,
                transform,
                use_gdi_natural as BOOL,
                glyph_indices.as_ptr(),
                glyph_indices.len() as u32,
                metrics.as_mut_ptr(),
                is_sideways as BOOL,
            );
            if hr != S_OK {
                return Err(hr);
            }
            Ok(metrics)
        }
    }

    #[deprecated(note = "Use `font_table` instead.")]
    pub fn get_font_table(&self, opentype_table_tag: u32) -> Option<Vec<u8>> {
        self.font_table(opentype_table_tag).unwrap()
    }

    /// Returns the contents of the OpenType table with the given tag.
    ///
    /// NB: The bytes of the tag are reversed! You probably want to use the `u32::swap_bytes()`
    /// method on the tag value before calling this method.
    pub fn font_table(&self, opentype_table_tag: u32) -> Result<Option<Vec<u8>>, HRESULT> {
        let mut table_data_ptr: *const u8 = ptr::null_mut();
        let mut table_size: u32 = 0;
        let mut table_context: *mut c_void = ptr::null_mut();
        let mut exists: BOOL = FALSE;
        unsafe {
            let hr = (*self.native.get()).TryGetFontTable(
                opentype_table_tag,
                &mut table_data_ptr as *mut *const _ as *mut *const c_void,
                &mut table_size,
                &mut table_context,
                &mut exists,
            );
            if hr != S_OK {
                return Err(hr);
            }

            if exists == FALSE {
                return Ok(None);
            }

            let table_bytes = slice::from_raw_parts(table_data_ptr, table_size as usize).to_vec();

            (*self.native.get()).ReleaseFontTable(table_context);

            Ok(Some(table_bytes))
        }
    }

    pub fn get_recommended_rendering_mode(
        &self,
        em_size: f32,
        pixels_per_dip: f32,
        measure_mode: DWRITE_MEASURING_MODE,
        rendering_params: *mut IDWriteRenderingParams,
    ) -> DWRITE_RENDERING_MODE {
        unsafe {
            let mut render_mode: DWRITE_RENDERING_MODE = DWRITE_RENDERING_MODE_DEFAULT;
            let hr = (*self.native.get()).GetRecommendedRenderingMode(
                em_size,
                pixels_per_dip,
                measure_mode,
                rendering_params,
                &mut render_mode,
            );

            if hr != S_OK {
                return DWRITE_RENDERING_MODE_NATURAL_SYMMETRIC;
            }

            render_mode
        }
    }

    pub fn get_recommended_rendering_mode_default_params(
        &self,
        em_size: f32,
        pixels_per_dip: f32,
        measure_mode: DWRITE_MEASURING_MODE,
    ) -> DWRITE_RENDERING_MODE {
        self.get_recommended_rendering_mode(
            em_size,
            pixels_per_dip,
            measure_mode,
            DefaultDWriteRenderParams(),
        )
    }

    #[deprecated(note = "Use `glyph_run_outline` instead.")]
    pub fn get_glyph_run_outline(
        &self,
        em_size: f32,
        glyph_indices: &[u16],
        glyph_advances: Option<&[f32]>,
        glyph_offsets: Option<&[DWRITE_GLYPH_OFFSET]>,
        is_sideways: bool,
        is_right_to_left: bool,
        outline_builder: Box<dyn OutlineBuilder>,
    ) {
        self.glyph_run_outline(
            em_size,
            glyph_indices,
            glyph_advances,
            glyph_offsets,
            is_sideways,
            is_right_to_left,
            outline_builder,
        )
        .unwrap()
    }

    pub fn glyph_run_outline(
        &self,
        em_size: f32,
        glyph_indices: &[u16],
        glyph_advances: Option<&[f32]>,
        glyph_offsets: Option<&[DWRITE_GLYPH_OFFSET]>,
        is_sideways: bool,
        is_right_to_left: bool,
        outline_builder: Box<dyn OutlineBuilder>,
    ) -> Result<(), GlyphRunOutlineError> {
        let glyph_advances = match glyph_advances {
            None => ptr::null(),
            Some(glyph_advances) => {
                if glyph_advances.len() != glyph_indices.len() {
                    return Err(GlyphRunOutlineError::InvalidInput);
                }
                glyph_advances.as_ptr()
            }
        };
        let glyph_offsets = match glyph_offsets {
            None => ptr::null(),
            Some(glyph_offsets) => {
                if glyph_offsets.len() != glyph_indices.len() {
                    return Err(GlyphRunOutlineError::InvalidInput);
                }
                glyph_offsets.as_ptr()
            }
        };
        let is_sideways = if is_sideways { TRUE } else { FALSE };
        let is_right_to_left = if is_right_to_left { TRUE } else { FALSE };
        let geometry_sink = GeometrySinkImpl::new(outline_builder);
        let geometry_sink = geometry_sink.into_interface();
        unsafe {
            let hr = (*self.native.get()).GetGlyphRunOutline(
                em_size,
                glyph_indices.as_ptr(),
                glyph_advances,
                glyph_offsets,
                glyph_indices.len() as u32,
                is_sideways,
                is_right_to_left,
                geometry_sink,
            );
            if hr != S_OK {
                return Err(GlyphRunOutlineError::Win32Error(hr));
            }
        }
        Ok(())
    }

    pub fn has_kerning_pairs(&self) -> bool {
        unsafe {
            match self.get_face1() {
                Some(face1) => face1.HasKerningPairs() == TRUE,
                None => false,
            }
        }
    }

    #[deprecated(note = "Use `glyph_pair_kerning_adjustment` instead.")]
    pub fn get_glyph_pair_kerning_adjustment(&self, first_glyph: u16, second_glyph: u16) -> i32 {
        self.glyph_pair_kerning_adjustment(first_glyph, second_glyph)
            .unwrap()
    }

    pub fn glyph_pair_kerning_adjustment(
        &self,
        first_glyph: u16,
        second_glyph: u16,
    ) -> Result<i32, HRESULT> {
        unsafe {
            match self.get_face1() {
                Some(face1) => {
                    let mut adjustments = [0; 2];
                    let hr = face1.GetKerningPairAdjustments(
                        2,
                        [first_glyph, second_glyph].as_ptr(),
                        adjustments.as_mut_ptr(),
                    );
                    if hr != S_OK {
                        return Err(hr);
                    }

                    Ok(adjustments[0])
                }
                None => Ok(0),
            }
        }
    }

    #[inline]
    pub fn get_type(&self) -> FontFaceType {
        unsafe {
            match (*self.native.get()).GetType() {
                DWRITE_FONT_FACE_TYPE_CFF => FontFaceType::Cff,
                DWRITE_FONT_FACE_TYPE_RAW_CFF => FontFaceType::RawCff,
                DWRITE_FONT_FACE_TYPE_TRUETYPE => FontFaceType::TrueType,
                DWRITE_FONT_FACE_TYPE_TRUETYPE_COLLECTION => FontFaceType::TrueTypeCollection,
                DWRITE_FONT_FACE_TYPE_TYPE1 => FontFaceType::Type1,
                DWRITE_FONT_FACE_TYPE_VECTOR => FontFaceType::Vector,
                DWRITE_FONT_FACE_TYPE_BITMAP => FontFaceType::Bitmap,
                _ => FontFaceType::Unknown,
            }
        }
    }

    #[inline]
    pub fn get_index(&self) -> u32 {
        unsafe { (*self.native.get()).GetIndex() }
    }

    #[inline]
    unsafe fn get_face1(&self) -> Option<ComPtr<IDWriteFontFace1>> {
        self.get_interface(&self.face1)
    }

    #[inline]
    unsafe fn get_face5(&self) -> Option<ComPtr<IDWriteFontFace5>> {
        self.get_interface(&self.face5)
    }

    #[inline]
    unsafe fn get_interface<I: Interface>(
        &self,
        interface: &UnsafeCell<Option<ComPtr<I>>>,
    ) -> Option<ComPtr<I>> {
        if (*interface.get()).is_none() {
            *interface.get() = (*self.native.get()).cast().ok()
        }
        (*interface.get()).clone()
    }

    pub fn has_variations(&self) -> bool {
        unsafe {
            match self.get_face5() {
                Some(face5) => face5.HasVariations() == TRUE,
                None => false,
            }
        }
    }

    /// If this font has variations, return a [`Vec<DWRITE_FONT_AXIS_VALUE`] of the
    /// variation axes and their values. If the font does not have variations,
    /// return an empty `Vec`.
    pub fn variations(&self) -> Result<Vec<DWRITE_FONT_AXIS_VALUE>, HRESULT> {
        let face5 = unsafe { self.get_face5() };
        let Some(face5) = face5 else {
            return Ok(vec![]);
        };
        if unsafe { face5.HasVariations() != TRUE } {
            return Ok(vec![]);
        }
        let axis_count = unsafe { face5.GetFontAxisValueCount() as usize };
        if axis_count == 0 {
            return Ok(vec![]);
        }

        let mut resource: *mut IDWriteFontResource = ptr::null_mut();
        let hr = unsafe { face5.GetFontResource(&mut resource) };
        if hr != S_OK || resource.is_null() {
            return Err(hr);
        }

        let mut axis_values = Vec::with_capacity(axis_count);
        axis_values.resize(
            axis_count,
            DWRITE_FONT_AXIS_VALUE {
                axisTag: 0,
                value: 0.,
            },
        );

        let hr = unsafe { face5.GetFontAxisValues(axis_values.as_mut_ptr(), axis_count as u32) };
        if hr != S_OK {
            return Err(hr);
        }

        let resource = unsafe { &*resource };
        Ok(axis_values
            .iter()
            .enumerate()
            .filter_map(|(index, axis_value)| {
                let attributes = unsafe { resource.GetFontAxisAttributes(index as u32) };
                if attributes & DWRITE_FONT_AXIS_ATTRIBUTES_VARIABLE == 0 {
                    None
                } else {
                    Some(*axis_value)
                }
            })
            .collect())
    }

    pub fn create_font_face_with_variations(
        &self,
        simulations: DWRITE_FONT_SIMULATIONS,
        axis_values: &[DWRITE_FONT_AXIS_VALUE],
    ) -> Option<FontFace> {
        unsafe {
            if let Some(face5) = self.get_face5() {
                let mut resource: *mut IDWriteFontResource = ptr::null_mut();
                let hr = face5.GetFontResource(&mut resource);
                if hr == S_OK && !resource.is_null() {
                    let resource = ComPtr::from_raw(resource);
                    let mut var_face: *mut IDWriteFontFace5 = ptr::null_mut();
                    let hr = resource.CreateFontFace(
                        simulations,
                        axis_values.as_ptr(),
                        axis_values.len() as u32,
                        &mut var_face,
                    );
                    if hr == S_OK && !var_face.is_null() {
                        let var_face = ComPtr::from_raw(var_face).cast().unwrap();
                        return Some(FontFace::take(var_face));
                    }
                }
            }
            None
        }
    }

    pub fn simulations(&self) -> FontSimulations {
        unsafe {
            std::mem::transmute::<DWRITE_FONT_SIMULATIONS, FontSimulations>(
                (*self.native.get()).GetSimulations(),
            )
        }
    }
}

impl Clone for FontFace {
    fn clone(&self) -> FontFace {
        unsafe {
            FontFace {
                native: UnsafeCell::new((*self.native.get()).clone()),
                face1: UnsafeCell::new(None),
                face5: UnsafeCell::new(None),
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FontFaceType {
    Unknown,
    Cff,
    RawCff,
    TrueType,
    TrueTypeCollection,
    Type1,
    Vector,
    Bitmap,
}

#[derive(Debug)]
pub enum GlyphRunOutlineError {
    InvalidInput,
    Win32Error(HRESULT),
}

impl fmt::Display for GlyphRunOutlineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput => write!(f, "Invalid input"),
            Self::Win32Error(code) => write!(f, "{:#x}", code),
        }
    }
}

impl error::Error for GlyphRunOutlineError {}

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#![allow(non_upper_case_globals)]

#[cfg(feature = "serde_serialization")]
extern crate serde;
#[cfg_attr(feature = "serde_serialization", macro_use)]
#[cfg(feature = "serde_serialization")]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate winapi;

include!("types.rs");

use std::ffi::CString;
use std::ptr;
use winapi::shared::guiddef::REFIID;
use winapi::shared::winerror::S_OK;
use winapi::um::dwrite::IDWriteFactory;
use winapi::um::dwrite::IDWriteRenderingParams;
use winapi::um::dwrite::DWRITE_FACTORY_TYPE;
use winapi::um::dwrite::DWRITE_FACTORY_TYPE_SHARED;
use winapi::um::unknwnbase::IUnknown;
use winapi::um::winnt::LPCSTR;
use winapi::Interface;

pub use winapi::um::winnt::HRESULT;

mod helpers;
use helpers::ToWide;
use std::os::raw::c_void;

#[cfg(test)]
mod test;

// We still use the DWrite structs for things like metrics; re-export them
// here
pub use winapi::shared::windef::RECT;
pub use winapi::um::dcommon::DWRITE_MEASURING_MODE;
pub use winapi::um::dcommon::{
    DWRITE_MEASURING_MODE_GDI_CLASSIC, DWRITE_MEASURING_MODE_GDI_NATURAL,
    DWRITE_MEASURING_MODE_NATURAL,
};
pub use winapi::um::dwrite::DWRITE_FONT_METRICS as FontMetrics0;
pub use winapi::um::dwrite::DWRITE_FONT_SIMULATIONS;
pub use winapi::um::dwrite::DWRITE_GLYPH_OFFSET as GlyphOffset;
pub use winapi::um::dwrite::DWRITE_RENDERING_MODE;
pub use winapi::um::dwrite::DWRITE_TEXTURE_TYPE;
pub use winapi::um::dwrite::{DWRITE_TEXTURE_ALIASED_1x1, DWRITE_TEXTURE_CLEARTYPE_3x1};
pub use winapi::um::dwrite::{
    DWRITE_FONT_SIMULATIONS_BOLD, DWRITE_FONT_SIMULATIONS_NONE, DWRITE_FONT_SIMULATIONS_OBLIQUE,
};
pub use winapi::um::dwrite::{DWRITE_GLYPH_RUN, DWRITE_MATRIX};
pub use winapi::um::dwrite::{
    DWRITE_RENDERING_MODE_ALIASED, DWRITE_RENDERING_MODE_CLEARTYPE_GDI_CLASSIC,
    DWRITE_RENDERING_MODE_CLEARTYPE_GDI_NATURAL, DWRITE_RENDERING_MODE_CLEARTYPE_NATURAL,
    DWRITE_RENDERING_MODE_CLEARTYPE_NATURAL_SYMMETRIC, DWRITE_RENDERING_MODE_DEFAULT,
    DWRITE_RENDERING_MODE_GDI_CLASSIC, DWRITE_RENDERING_MODE_GDI_NATURAL,
    DWRITE_RENDERING_MODE_NATURAL, DWRITE_RENDERING_MODE_NATURAL_SYMMETRIC,
    DWRITE_RENDERING_MODE_OUTLINE,
};
pub use winapi::um::dwrite_1::DWRITE_FONT_METRICS1 as FontMetrics1;
pub use winapi::um::dwrite_3::DWRITE_FONT_AXIS_VALUE;
use winapi::um::libloaderapi::{GetProcAddress, LoadLibraryW};

#[macro_use]
mod com_helpers;

mod bitmap_render_target;
pub use bitmap_render_target::BitmapRenderTarget;
mod font;
pub use font::{Font, FontMetrics, InformationalStringId};
mod font_collection;
pub use font_collection::FontCollection;
mod font_face;
pub use font_face::{FontFace, FontFaceType};
mod font_fallback;
pub use font_fallback::{FallbackResult, FontFallback};
mod font_family;
pub use font_family::FontFamily;
mod font_file;
pub use font_file::FontFile;
mod gdi_interop;
pub use gdi_interop::GdiInterop;
mod outline_builder;
pub use outline_builder::OutlineBuilder;
mod rendering_params;
pub use rendering_params::RenderingParams;
mod text_analysis_source;
pub use text_analysis_source::TextAnalysisSource;
mod glyph_run_analysis;
pub use glyph_run_analysis::GlyphRunAnalysis;

// This is an internal implementation of FontFileLoader, for our utility
// functions.  We don't wrap the DWriteFontFileLoader interface and
// related things.
mod font_file_loader_impl;

// This is an implementation of `FontCollectionLoader` for client code.
mod font_collection_impl;
pub use font_collection_impl::CustomFontCollectionLoaderImpl;

// This is an implementation of `TextAnalysisSource` for client code.
mod text_analysis_source_impl;
pub use text_analysis_source_impl::{
    CustomTextAnalysisSourceImpl, NumberSubstitution, TextAnalysisSourceMethods,
};

// This is an internal implementation of `GeometrySink` so that we can
// expose `IDWriteGeometrySink` in an idiomatic way.
mod geometry_sink_impl;

lazy_static! {
    static ref DWRITE_FACTORY_RAW_PTR: usize = {
        unsafe {
            type DWriteCreateFactoryType =
                extern "system" fn(DWRITE_FACTORY_TYPE, REFIID, *mut *mut IUnknown) -> HRESULT;

            let dwrite_dll = LoadLibraryW("dwrite.dll".to_wide_null().as_ptr());
            assert!(!dwrite_dll.is_null());
            let create_factory_name = CString::new("DWriteCreateFactory").unwrap();
            let dwrite_create_factory_ptr =
                GetProcAddress(dwrite_dll, create_factory_name.as_ptr() as LPCSTR);
            assert!(!dwrite_create_factory_ptr.is_null());

            let dwrite_create_factory = mem::transmute::<*const c_void, DWriteCreateFactoryType>(
                dwrite_create_factory_ptr as *const _,
            );

            let mut factory: *mut IDWriteFactory = ptr::null_mut();
            let hr = dwrite_create_factory(
                DWRITE_FACTORY_TYPE_SHARED,
                &IDWriteFactory::uuidof(),
                &mut factory as *mut *mut IDWriteFactory as *mut *mut IUnknown,
            );
            assert!(hr == S_OK);
            factory as usize
        }
    };
    static ref DEFAULT_DWRITE_RENDERING_PARAMS_RAW_PTR: usize = {
        unsafe {
            let mut default_rendering_params: *mut IDWriteRenderingParams = ptr::null_mut();
            let hr = (*DWriteFactory()).CreateRenderingParams(&mut default_rendering_params);
            assert!(hr == S_OK);
            default_rendering_params as usize
        }
    };
} // end lazy static

// FIXME vlad would be nice to return, say, FactoryPtr<IDWriteFactory>
// that has a DerefMut impl, so that we can write
// DWriteFactory().SomeOperation() as opposed to
// (*DWriteFactory()).SomeOperation()
#[allow(non_snake_case)]
fn DWriteFactory() -> *mut IDWriteFactory {
    (*DWRITE_FACTORY_RAW_PTR) as *mut IDWriteFactory
}

#[allow(non_snake_case)]
fn DefaultDWriteRenderParams() -> *mut IDWriteRenderingParams {
    (*DEFAULT_DWRITE_RENDERING_PARAMS_RAW_PTR) as *mut IDWriteRenderingParams
}

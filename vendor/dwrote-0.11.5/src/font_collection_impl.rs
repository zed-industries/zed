/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// A temporary custom font collection that exists solely for the face-to-font mapping to work.

use std::mem;
use std::sync::atomic::AtomicUsize;
use winapi::ctypes::c_void;
use winapi::shared::guiddef::REFIID;
use winapi::shared::minwindef::{BOOL, FALSE, TRUE, ULONG};
use winapi::shared::winerror::{E_INVALIDARG, S_OK};
use winapi::um::dwrite::IDWriteFactory;
use winapi::um::dwrite::IDWriteFontCollectionLoader;
use winapi::um::dwrite::IDWriteFontCollectionLoaderVtbl;
use winapi::um::dwrite::IDWriteFontFile;
use winapi::um::dwrite::IDWriteFontFileEnumerator;
use winapi::um::dwrite::IDWriteFontFileEnumeratorVtbl;
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};
use winapi::um::winnt::HRESULT;
use wio::com::ComPtr;

use crate::com_helpers::Com;
use crate::FontFile;

static FONT_COLLECTION_LOADER_VTBL: IDWriteFontCollectionLoaderVtbl =
    IDWriteFontCollectionLoaderVtbl {
        parent: implement_iunknown!(static IDWriteFontCollectionLoader,
                                    CustomFontCollectionLoaderImpl),
        CreateEnumeratorFromKey: CustomFontCollectionLoaderImpl_CreateEnumeratorFromKey,
    };

#[repr(C)]
pub struct CustomFontCollectionLoaderImpl {
    // NB: This must be the first field.
    _refcount: AtomicUsize,
    font_files: Vec<ComPtr<IDWriteFontFile>>,
}

impl Com<IDWriteFontCollectionLoader> for CustomFontCollectionLoaderImpl {
    type Vtbl = IDWriteFontCollectionLoaderVtbl;
    #[inline]
    fn vtbl() -> &'static IDWriteFontCollectionLoaderVtbl {
        &FONT_COLLECTION_LOADER_VTBL
    }
}

impl Com<IUnknown> for CustomFontCollectionLoaderImpl {
    type Vtbl = IUnknownVtbl;
    #[inline]
    fn vtbl() -> &'static IUnknownVtbl {
        &FONT_COLLECTION_LOADER_VTBL.parent
    }
}

impl CustomFontCollectionLoaderImpl {
    pub fn new(font_files: &[FontFile]) -> ComPtr<IDWriteFontCollectionLoader> {
        unsafe {
            ComPtr::from_raw(
                CustomFontCollectionLoaderImpl {
                    _refcount: AtomicUsize::new(1),
                    font_files: font_files.iter().map(|file| file.as_com_ptr()).collect(),
                }
                .into_interface(),
            )
        }
    }
}

#[allow(non_snake_case)]
unsafe extern "system" fn CustomFontCollectionLoaderImpl_CreateEnumeratorFromKey(
    this: *mut IDWriteFontCollectionLoader,
    _: *mut IDWriteFactory,
    _: *const c_void,
    _: u32,
    out_enumerator: *mut *mut IDWriteFontFileEnumerator,
) -> HRESULT {
    let this = CustomFontCollectionLoaderImpl::from_interface(this);
    let enumerator = CustomFontFileEnumeratorImpl::new(this.font_files.clone());
    let enumerator = ComPtr::<IDWriteFontFileEnumerator>::from_raw(enumerator.into_interface());
    *out_enumerator = enumerator.as_raw();
    mem::forget(enumerator);
    S_OK
}

#[repr(C)]
struct CustomFontFileEnumeratorImpl {
    // NB(pcwalton): This must be the first field.
    _refcount: AtomicUsize,
    font_files: Vec<ComPtr<IDWriteFontFile>>,
    index: isize,
}

impl Com<IDWriteFontFileEnumerator> for CustomFontFileEnumeratorImpl {
    type Vtbl = IDWriteFontFileEnumeratorVtbl;
    #[inline]
    fn vtbl() -> &'static IDWriteFontFileEnumeratorVtbl {
        &FONT_FILE_ENUMERATOR_VTBL
    }
}

impl Com<IUnknown> for CustomFontFileEnumeratorImpl {
    type Vtbl = IUnknownVtbl;
    #[inline]
    fn vtbl() -> &'static IUnknownVtbl {
        &FONT_FILE_ENUMERATOR_VTBL.parent
    }
}

static FONT_FILE_ENUMERATOR_VTBL: IDWriteFontFileEnumeratorVtbl = IDWriteFontFileEnumeratorVtbl {
    parent: implement_iunknown!(static IDWriteFontFileEnumerator, CustomFontFileEnumeratorImpl),
    GetCurrentFontFile: CustomFontFileEnumeratorImpl_GetCurrentFontFile,
    MoveNext: CustomFontFileEnumeratorImpl_MoveNext,
};

impl CustomFontFileEnumeratorImpl {
    pub fn new(font_files: Vec<ComPtr<IDWriteFontFile>>) -> CustomFontFileEnumeratorImpl {
        CustomFontFileEnumeratorImpl {
            _refcount: AtomicUsize::new(1),
            font_files,
            index: -1,
        }
    }
}

#[allow(non_snake_case)]
unsafe extern "system" fn CustomFontFileEnumeratorImpl_GetCurrentFontFile(
    this: *mut IDWriteFontFileEnumerator,
    out_font_file: *mut *mut IDWriteFontFile,
) -> HRESULT {
    let this = CustomFontFileEnumeratorImpl::from_interface(this);
    if this.index < 0 || this.index >= this.font_files.len() as isize {
        return E_INVALIDARG;
    }
    let new_font_file = this.font_files[this.index as usize].clone();
    *out_font_file = new_font_file.as_raw();
    mem::forget(new_font_file);
    S_OK
}

#[allow(non_snake_case)]
unsafe extern "system" fn CustomFontFileEnumeratorImpl_MoveNext(
    this: *mut IDWriteFontFileEnumerator,
    has_current_file: *mut BOOL,
) -> HRESULT {
    let this = CustomFontFileEnumeratorImpl::from_interface(this);
    let font_file_count = this.font_files.len() as isize;
    if this.index < font_file_count {
        this.index += 1
    }
    *has_current_file = if this.index >= 0 && this.index < font_file_count {
        TRUE
    } else {
        FALSE
    };
    S_OK
}

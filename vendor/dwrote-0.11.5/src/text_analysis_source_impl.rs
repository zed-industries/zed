/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! A custom implementation of the "text analysis source" interface so that
//! we can convey data to the `FontFallback::map_characters` method.

#![allow(non_snake_case)]

use std::borrow::Cow;
use std::ffi::OsStr;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use std::ptr::{self, null};
use std::sync::atomic::AtomicUsize;
use winapi::ctypes::wchar_t;
use winapi::shared::basetsd::UINT32;
use winapi::shared::guiddef::REFIID;
use winapi::shared::minwindef::{FALSE, TRUE, ULONG};
use winapi::shared::ntdef::LOCALE_NAME_MAX_LENGTH;
use winapi::shared::winerror::{E_INVALIDARG, S_OK};
use winapi::um::dwrite::IDWriteNumberSubstitution;
use winapi::um::dwrite::IDWriteTextAnalysisSource;
use winapi::um::dwrite::IDWriteTextAnalysisSourceVtbl;
use winapi::um::dwrite::DWRITE_NUMBER_SUBSTITUTION_METHOD;
use winapi::um::dwrite::DWRITE_READING_DIRECTION;
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};
use winapi::um::winnt::HRESULT;
use wio::com::ComPtr;

use super::DWriteFactory;
use crate::com_helpers::Com;
use crate::helpers::ToWide;

/// The Rust side of a custom text analysis source implementation.
pub trait TextAnalysisSourceMethods {
    /// Determine the locale for a range of text.
    ///
    /// Return locale and length of text (in utf-16 code units) for which the
    /// locale is valid.
    fn get_locale_name(&self, text_position: u32) -> (Cow<'_, str>, u32);

    /// Get the text direction for the paragraph.
    fn get_paragraph_reading_direction(&self) -> DWRITE_READING_DIRECTION;
}

#[repr(C)]
pub struct CustomTextAnalysisSourceImpl<'a> {
    // NB: This must be the first field.
    _refcount: AtomicUsize,
    inner: Box<dyn TextAnalysisSourceMethods + 'a>,
    text: Cow<'a, [wchar_t]>,
    number_subst: Option<NumberSubstitution>,
    locale_buf: [wchar_t; LOCALE_NAME_MAX_LENGTH],
}

/// A wrapped version of an `IDWriteNumberSubstitution` object.
pub struct NumberSubstitution {
    native: ComPtr<IDWriteNumberSubstitution>,
}

// TODO: implement Clone, for convenience and efficiency?

static TEXT_ANALYSIS_SOURCE_VTBL: IDWriteTextAnalysisSourceVtbl = IDWriteTextAnalysisSourceVtbl {
    parent: implement_iunknown!(static IDWriteTextAnalysisSource, CustomTextAnalysisSourceImpl),
    GetLocaleName: CustomTextAnalysisSourceImpl_GetLocaleName,
    GetNumberSubstitution: CustomTextAnalysisSourceImpl_GetNumberSubstitution,
    GetParagraphReadingDirection: CustomTextAnalysisSourceImpl_GetParagraphReadingDirection,
    GetTextAtPosition: CustomTextAnalysisSourceImpl_GetTextAtPosition,
    GetTextBeforePosition: CustomTextAnalysisSourceImpl_GetTextBeforePosition,
};

impl<'a> CustomTextAnalysisSourceImpl<'a> {
    /// Create a new custom TextAnalysisSource for the given text and a trait
    /// implementation.
    ///
    /// Note: this method has no NumberSubsitution specified. See
    /// `from_text_and_number_subst_native` if you need number substitution.
    pub fn from_text_native(
        inner: Box<dyn TextAnalysisSourceMethods + 'a>,
        text: Cow<'a, [wchar_t]>,
    ) -> CustomTextAnalysisSourceImpl<'a> {
        assert!(text.len() <= (u32::MAX as usize));
        CustomTextAnalysisSourceImpl {
            _refcount: AtomicUsize::new(1),
            inner,
            text,
            number_subst: None,
            locale_buf: [0u16; LOCALE_NAME_MAX_LENGTH],
        }
    }

    /// Create a new custom TextAnalysisSource for the given text and a trait
    /// implementation.
    ///
    /// Note: this method only supports a single `NumberSubstitution` for the
    /// entire string.
    pub fn from_text_and_number_subst_native(
        inner: Box<dyn TextAnalysisSourceMethods + 'a>,
        text: Cow<'a, [wchar_t]>,
        number_subst: NumberSubstitution,
    ) -> CustomTextAnalysisSourceImpl<'a> {
        assert!(text.len() <= (u32::MAX as usize));
        CustomTextAnalysisSourceImpl {
            _refcount: AtomicUsize::new(1),
            inner,
            text,
            number_subst: Some(number_subst),
            locale_buf: [0u16; LOCALE_NAME_MAX_LENGTH],
        }
    }
}

impl Com<IDWriteTextAnalysisSource> for CustomTextAnalysisSourceImpl<'_> {
    type Vtbl = IDWriteTextAnalysisSourceVtbl;
    #[inline]
    fn vtbl() -> &'static IDWriteTextAnalysisSourceVtbl {
        &TEXT_ANALYSIS_SOURCE_VTBL
    }
}

impl Com<IUnknown> for CustomTextAnalysisSourceImpl<'_> {
    type Vtbl = IUnknownVtbl;
    #[inline]
    fn vtbl() -> &'static IUnknownVtbl {
        &TEXT_ANALYSIS_SOURCE_VTBL.parent
    }
}

unsafe extern "system" fn CustomTextAnalysisSourceImpl_GetLocaleName(
    this: *mut IDWriteTextAnalysisSource,
    text_position: UINT32,
    text_length: *mut UINT32,
    locale_name: *mut *const wchar_t,
) -> HRESULT {
    let this = CustomTextAnalysisSourceImpl::from_interface(this);
    let (locale, text_len) = this.inner.get_locale_name(text_position);

    // Copy the locale data into the buffer
    for (i, c) in OsStr::new(&*locale)
        .encode_wide()
        .chain(Some(0))
        .enumerate()
    {
        // -1 here is deliberate: it ensures that we never write to the last character in
        // this.locale_buf, so that the buffer is always null-terminated.
        if i >= this.locale_buf.len() - 1 {
            break;
        }

        *this.locale_buf.get_unchecked_mut(i) = c;
    }

    *text_length = text_len;
    *locale_name = this.locale_buf.as_ptr();
    S_OK
}

unsafe extern "system" fn CustomTextAnalysisSourceImpl_GetNumberSubstitution(
    this: *mut IDWriteTextAnalysisSource,
    text_position: UINT32,
    text_length: *mut UINT32,
    number_substitution: *mut *mut IDWriteNumberSubstitution,
) -> HRESULT {
    let this = CustomTextAnalysisSourceImpl::from_interface(this);
    if text_position >= (this.text.len() as u32) {
        return E_INVALIDARG;
    }

    *text_length = (this.text.len() as UINT32) - text_position;
    *number_substitution = match &this.number_subst {
        Some(number_subst) => {
            let com_ptr = &number_subst.native;
            com_ptr.AddRef();
            com_ptr.as_raw()
        }
        None => std::ptr::null_mut(),
    };

    S_OK
}

unsafe extern "system" fn CustomTextAnalysisSourceImpl_GetParagraphReadingDirection(
    this: *mut IDWriteTextAnalysisSource,
) -> DWRITE_READING_DIRECTION {
    let this = CustomTextAnalysisSourceImpl::from_interface(this);
    this.inner.get_paragraph_reading_direction()
}

unsafe extern "system" fn CustomTextAnalysisSourceImpl_GetTextAtPosition(
    this: *mut IDWriteTextAnalysisSource,
    text_position: UINT32,
    text_string: *mut *const wchar_t,
    text_length: *mut UINT32,
) -> HRESULT {
    let this = CustomTextAnalysisSourceImpl::from_interface(this);
    if text_position >= (this.text.len() as u32) {
        *text_string = null();
        *text_length = 0;
        return S_OK;
    }
    *text_string = this.text.as_ptr().add(text_position as usize);
    *text_length = (this.text.len() as UINT32) - text_position;
    S_OK
}

unsafe extern "system" fn CustomTextAnalysisSourceImpl_GetTextBeforePosition(
    this: *mut IDWriteTextAnalysisSource,
    text_position: UINT32,
    text_string: *mut *const wchar_t,
    text_length: *mut UINT32,
) -> HRESULT {
    let this = CustomTextAnalysisSourceImpl::from_interface(this);
    if text_position == 0 || text_position > (this.text.len() as u32) {
        *text_string = null();
        *text_length = 0;
        return S_OK;
    }
    *text_string = this.text.as_ptr();
    *text_length = text_position;
    S_OK
}

impl NumberSubstitution {
    pub fn new(
        subst_method: DWRITE_NUMBER_SUBSTITUTION_METHOD,
        locale: &str,
        ignore_user_overrides: bool,
    ) -> NumberSubstitution {
        unsafe {
            let mut native: *mut IDWriteNumberSubstitution = ptr::null_mut();
            let hr = (*DWriteFactory()).CreateNumberSubstitution(
                subst_method,
                locale.to_wide_null().as_ptr(),
                if ignore_user_overrides { TRUE } else { FALSE },
                &mut native,
            );
            assert_eq!(hr, 0, "error creating number substitution");
            NumberSubstitution {
                native: ComPtr::from_raw(native),
            }
        }
    }
}

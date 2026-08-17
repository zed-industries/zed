/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::borrow::Cow;
use std::marker::PhantomData;
use winapi::ctypes::wchar_t;
use winapi::um::dwrite::IDWriteTextAnalysisSource;
use wio::com::ComPtr;

use super::*;
use crate::com_helpers::Com;

pub struct TextAnalysisSource<'a> {
    native: ComPtr<IDWriteTextAnalysisSource>,
    phantom: PhantomData<CustomTextAnalysisSourceImpl<'a>>,
}

impl<'a> TextAnalysisSource<'a> {
    /// Create a new custom TextAnalysisSource for the given text and a trait
    /// implementation.
    ///
    /// Note: this method has no NumberSubsitution specified. See
    /// `from_text_and_number_subst` if you need number substitution.
    pub fn from_text(
        inner: Box<dyn TextAnalysisSourceMethods + 'a>,
        text: Cow<'a, [wchar_t]>,
    ) -> TextAnalysisSource<'a> {
        let native = unsafe {
            ComPtr::from_raw(
                CustomTextAnalysisSourceImpl::from_text_native(inner, text).into_interface(),
            )
        };
        TextAnalysisSource {
            native,
            phantom: PhantomData,
        }
    }

    /// Create a new custom TextAnalysisSource for the given text and a trait
    /// implementation.
    ///
    /// Note: this method only supports a single `NumberSubstitution` for the
    /// entire string.
    pub fn from_text_and_number_subst(
        inner: Box<dyn TextAnalysisSourceMethods + 'a>,
        text: Cow<'a, [wchar_t]>,
        number_subst: NumberSubstitution,
    ) -> TextAnalysisSource<'a> {
        let native = unsafe {
            ComPtr::from_raw(
                CustomTextAnalysisSourceImpl::from_text_and_number_subst_native(
                    inner,
                    text,
                    number_subst,
                )
                .into_interface(),
            )
        };
        TextAnalysisSource {
            native,
            phantom: PhantomData,
        }
    }

    pub fn as_ptr(&self) -> *mut IDWriteTextAnalysisSource {
        self.native.as_raw()
    }
}

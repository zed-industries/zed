/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::cell::UnsafeCell;
use std::mem;
use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering};
use winapi::shared::minwindef::{BOOL, FALSE, TRUE};
use winapi::shared::winerror::S_OK;
use winapi::um::dwrite::IDWriteFontCollectionLoader;
use winapi::um::dwrite::{IDWriteFont, IDWriteFontCollection, IDWriteFontFamily};
use winapi::um::winnt::HRESULT;
use wio::com::ComPtr;

use super::{DWriteFactory, Font, FontDescriptor, FontFace, FontFamily};
use crate::helpers::*;

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

pub struct FontCollectionFamilyIterator {
    collection: ComPtr<IDWriteFontCollection>,
    curr: u32,
    count: u32,
}

impl Iterator for FontCollectionFamilyIterator {
    type Item = FontFamily;
    fn next(&mut self) -> Option<FontFamily> {
        if self.curr == self.count {
            return None;
        }

        unsafe {
            let mut family: *mut IDWriteFontFamily = ptr::null_mut();
            let hr = self.collection.GetFontFamily(self.curr, &mut family);
            assert!(hr == 0);
            self.curr += 1;
            Some(FontFamily::take(ComPtr::from_raw(family)))
        }
    }
}

pub struct FontCollection {
    native: UnsafeCell<ComPtr<IDWriteFontCollection>>,
}

impl FontCollection {
    pub fn get_system(update: bool) -> FontCollection {
        unsafe {
            let mut native: *mut IDWriteFontCollection = ptr::null_mut();
            let hr = (*DWriteFactory())
                .GetSystemFontCollection(&mut native, if update { TRUE } else { FALSE });
            assert!(hr == 0);

            FontCollection {
                native: UnsafeCell::new(ComPtr::from_raw(native)),
            }
        }
    }

    pub fn system() -> FontCollection {
        FontCollection::get_system(false)
    }

    pub fn take(native: ComPtr<IDWriteFontCollection>) -> FontCollection {
        FontCollection {
            native: UnsafeCell::new(native),
        }
    }

    pub fn from_loader(collection_loader: ComPtr<IDWriteFontCollectionLoader>) -> FontCollection {
        unsafe {
            let factory = DWriteFactory();
            assert_eq!(
                (*factory).RegisterFontCollectionLoader(collection_loader.clone().into_raw()),
                S_OK
            );
            let mut collection: *mut IDWriteFontCollection = ptr::null_mut();
            let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
            assert_eq!(
                (*factory).CreateCustomFontCollection(
                    collection_loader.clone().into_raw(),
                    &id as *const usize as *const _,
                    mem::size_of::<AtomicUsize>() as u32,
                    &mut collection
                ),
                S_OK
            );
            FontCollection::take(ComPtr::from_raw(collection))
        }
    }

    pub unsafe fn as_ptr(&self) -> *mut IDWriteFontCollection {
        (*self.native.get()).as_raw()
    }

    pub fn families_iter(&self) -> FontCollectionFamilyIterator {
        unsafe {
            FontCollectionFamilyIterator {
                collection: (*self.native.get()).clone(),
                curr: 0,
                count: (*self.native.get()).GetFontFamilyCount(),
            }
        }
    }

    pub fn get_font_family_count(&self) -> u32 {
        unsafe { (*self.native.get()).GetFontFamilyCount() }
    }

    #[deprecated(note = "Use `font_family` instead.")]
    pub fn get_font_family(&self, index: u32) -> FontFamily {
        self.font_family(index).unwrap()
    }

    /// Returns the [`FontFamily`] at the given index.
    pub fn font_family(&self, index: u32) -> Result<FontFamily, HRESULT> {
        let mut family: *mut IDWriteFontFamily = ptr::null_mut();
        unsafe {
            let hr = (*self.native.get()).GetFontFamily(index, &mut family);
            if hr != S_OK {
                return Err(hr);
            }
            Ok(FontFamily::take(ComPtr::from_raw(family)))
        }
    }

    #[deprecated(note = "Use `font_from_descriptor` instead.")]
    pub fn get_font_from_descriptor(&self, desc: &FontDescriptor) -> Option<Font> {
        self.font_from_descriptor(desc).unwrap()
    }

    /// Find a font matching the given font descriptor in this [`FontCollection`].
    pub fn font_from_descriptor(&self, desc: &FontDescriptor) -> Result<Option<Font>, HRESULT> {
        if let Some(family) = self.font_family_by_name(&desc.family_name)? {
            let font = family.first_matching_font(desc.weight, desc.stretch, desc.style)?;
            // Exact matches only here
            if font.weight() == desc.weight
                && font.stretch() == desc.stretch
                && font.style() == desc.style
            {
                return Ok(Some(font));
            }
        }

        Ok(None)
    }

    #[deprecated(note = "Use `font_from_face` instead.")]
    pub fn get_font_from_face(&self, face: &FontFace) -> Option<Font> {
        self.font_from_face(face).ok()
    }

    /// Get a [`Font`] from the given [`FontFace`].
    pub fn font_from_face(&self, face: &FontFace) -> Result<Font, HRESULT> {
        let mut font: *mut IDWriteFont = ptr::null_mut();
        unsafe {
            let hr = (*self.native.get()).GetFontFromFontFace(face.as_ptr(), &mut font);
            if hr != S_OK {
                return Err(hr);
            }
            Ok(Font::take(ComPtr::from_raw(font)))
        }
    }

    #[deprecated(note = "Use `font_family_by_name` instead.")]
    pub fn get_font_family_by_name(&self, family_name: &str) -> Option<FontFamily> {
        self.font_family_by_name(family_name).unwrap()
    }

    /// Find a [`FontFamily`] with the given name. Returns `None` if no family
    /// with that name is found.
    pub fn font_family_by_name(&self, family_name: &str) -> Result<Option<FontFamily>, HRESULT> {
        let mut index: u32 = 0;
        let mut exists: BOOL = FALSE;
        unsafe {
            let hr = (*self.native.get()).FindFamilyName(
                family_name.to_wide_null().as_ptr(),
                &mut index,
                &mut exists,
            );
            if hr != S_OK {
                return Err(hr);
            }
            if exists == FALSE {
                return Ok(None);
            }

            let mut family: *mut IDWriteFontFamily = ptr::null_mut();
            let hr = (*self.native.get()).GetFontFamily(index, &mut family);
            if hr != S_OK {
                return Err(hr);
            }

            Ok(Some(FontFamily::take(ComPtr::from_raw(family))))
        }
    }
}

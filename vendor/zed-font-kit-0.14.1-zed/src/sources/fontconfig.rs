// font-kit/src/sources/fontconfig.rs
//
// Copyright © 2018 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A source that contains the fonts installed on the system, as reported by the Fontconfig
//! library.
//!
//! On macOS and Windows, the Cargo feature `source-fontconfig` can be used to opt into fontconfig
//! support. To prefer it over the native font source (only if you know what you're doing), use the
//! `source-fontconfig-default` feature.

use crate::error::SelectionError;
use crate::family_handle::FamilyHandle;
use crate::family_name::FamilyName;
use crate::handle::Handle;
use crate::properties::Properties;
use crate::source::Source;
use std::any::Any;

/// A source that contains the fonts installed on the system, as reported by the Fontconfig
/// library.
///
/// On macOS and Windows, the Cargo feature `source-fontconfig` can be used to opt into fontconfig
/// support. To prefer it over the native font source (only if you know what you're doing), use the
/// `source-fontconfig-default` feature.
#[allow(missing_debug_implementations)]
pub struct FontconfigSource {
    config: fc::Config,
}

impl Default for FontconfigSource {
    fn default() -> Self {
        Self::new()
    }
}

impl FontconfigSource {
    /// Initializes Fontconfig and prepares it for queries.
    pub fn new() -> FontconfigSource {
        FontconfigSource {
            config: fc::Config::new(),
        }
    }

    /// Returns paths of all fonts installed on the system.
    pub fn all_fonts(&self) -> Result<Vec<Handle>, SelectionError> {
        let pattern = fc::Pattern::new();

        // We want the family name.
        let mut object_set = fc::ObjectSet::new();
        object_set.push_string(fc::Object::File);
        object_set.push_string(fc::Object::Index);

        let patterns = pattern
            .list(&self.config, object_set)
            .map_err(|_| SelectionError::NotFound)?;

        let mut handles = vec![];
        for patt in patterns {
            let path = match patt.get_string(fc::Object::File) {
                Some(v) => v,
                None => continue,
            };

            let index = match patt.get_integer(fc::Object::Index) {
                Some(v) => v,
                None => continue,
            };

            handles.push(Handle::Path {
                path: path.into(),
                font_index: index as u32,
            });
        }

        if !handles.is_empty() {
            Ok(handles)
        } else {
            Err(SelectionError::NotFound)
        }
    }

    /// Returns the names of all families installed on the system.
    pub fn all_families(&self) -> Result<Vec<String>, SelectionError> {
        let pattern = fc::Pattern::new();

        // We want the family name.
        let mut object_set = fc::ObjectSet::new();
        object_set.push_string(fc::Object::Family);

        let patterns = pattern
            .list(&self.config, object_set)
            .map_err(|_| SelectionError::NotFound)?;

        let mut result_families = vec![];
        for patt in patterns {
            if let Some(family) = patt.get_string(fc::Object::Family) {
                result_families.push(family);
            }
        }

        result_families.sort();
        result_families.dedup();

        if !result_families.is_empty() {
            Ok(result_families)
        } else {
            Err(SelectionError::NotFound)
        }
    }

    /// Looks up a font family by name and returns the handles of all the fonts in that family.
    pub fn select_family_by_name(&self, family_name: &str) -> Result<FamilyHandle, SelectionError> {
        use std::borrow::Cow;

        let family_name = match family_name {
            "serif" | "sans-serif" | "monospace" | "cursive" | "fantasy" => {
                Cow::from(self.select_generic_font(family_name)?)
            }
            _ => Cow::from(family_name),
        };

        let pattern = fc::Pattern::from_name(family_name.as_ref());

        let mut object_set = fc::ObjectSet::new();
        object_set.push_string(fc::Object::File);
        object_set.push_string(fc::Object::Index);

        let patterns = pattern
            .list(&self.config, object_set)
            .map_err(|_| SelectionError::NotFound)?;

        let mut handles = vec![];
        for patt in patterns {
            let font_path = patt.get_string(fc::Object::File).unwrap();
            let font_index = patt.get_integer(fc::Object::Index).unwrap() as u32;
            let handle = Handle::from_path(std::path::PathBuf::from(font_path), font_index);
            handles.push(handle);
        }

        if !handles.is_empty() {
            Ok(FamilyHandle::from_font_handles(handles.into_iter()))
        } else {
            Err(SelectionError::NotFound)
        }
    }

    /// Selects a font by a generic name.
    ///
    /// Accepts: serif, sans-serif, monospace, cursive and fantasy.
    fn select_generic_font(&self, name: &str) -> Result<String, SelectionError> {
        let mut pattern = fc::Pattern::from_name(name);
        pattern.config_substitute(fc::MatchKind::Pattern);
        pattern.default_substitute();

        let patterns = pattern
            .sorted(&self.config)
            .map_err(|_| SelectionError::NotFound)?;

        if let Some(patt) = patterns.into_iter().next() {
            if let Some(family) = patt.get_string(fc::Object::Family) {
                return Ok(family);
            }
        }

        Err(SelectionError::NotFound)
    }

    /// Selects a font by PostScript name, which should be a unique identifier.
    ///
    /// The default implementation, which is used by the DirectWrite and the filesystem backends,
    /// does a brute-force search of installed fonts to find the one that matches.
    pub fn select_by_postscript_name(
        &self,
        postscript_name: &str,
    ) -> Result<Handle, SelectionError> {
        let mut pattern = fc::Pattern::new();
        pattern.push_string(fc::Object::PostScriptName, postscript_name.to_owned());

        // We want the file path and the font index.
        let mut object_set = fc::ObjectSet::new();
        object_set.push_string(fc::Object::File);
        object_set.push_string(fc::Object::Index);

        let patterns = pattern
            .list(&self.config, object_set)
            .map_err(|_| SelectionError::NotFound)?;

        if let Some(patt) = patterns.into_iter().next() {
            let font_path = patt.get_string(fc::Object::File).unwrap();
            let font_index = patt.get_integer(fc::Object::Index).unwrap() as u32;
            let handle = Handle::from_path(std::path::PathBuf::from(font_path), font_index);
            Ok(handle)
        } else {
            Err(SelectionError::NotFound)
        }
    }

    /// Performs font matching according to the CSS Fonts Level 3 specification and returns the
    /// handle.
    #[inline]
    pub fn select_best_match(
        &self,
        family_names: &[FamilyName],
        properties: &Properties,
    ) -> Result<Handle, SelectionError> {
        <Self as Source>::select_best_match(self, family_names, properties)
    }
}

impl Source for FontconfigSource {
    #[inline]
    fn all_fonts(&self) -> Result<Vec<Handle>, SelectionError> {
        self.all_fonts()
    }

    #[inline]
    fn all_families(&self) -> Result<Vec<String>, SelectionError> {
        self.all_families()
    }

    #[inline]
    fn select_family_by_name(&self, family_name: &str) -> Result<FamilyHandle, SelectionError> {
        self.select_family_by_name(family_name)
    }

    #[inline]
    fn select_by_postscript_name(&self, postscript_name: &str) -> Result<Handle, SelectionError> {
        self.select_by_postscript_name(postscript_name)
    }

    #[inline]
    fn as_any(&self) -> &dyn Any {
        self
    }

    #[inline]
    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

// A minimal fontconfig wrapper.
mod fc {
    #![allow(dead_code)]

    use fontconfig_sys as ffi;
    use fontconfig_sys::ffi_dispatch;

    #[cfg(feature = "source-fontconfig-dlopen")]
    use ffi::statics::LIB;
    #[cfg(not(feature = "source-fontconfig-dlopen"))]
    use ffi::*;

    use std::ffi::{CStr, CString};
    use std::os::raw::{c_char, c_uchar};
    use std::ptr;

    #[derive(Clone, Copy)]
    pub enum Error {
        NoMatch,
        TypeMismatch,
        NoId,
        OutOfMemory,
    }

    #[derive(Clone, Copy)]
    pub enum MatchKind {
        Pattern,
        Font,
        Scan,
    }

    impl MatchKind {
        fn to_u32(self) -> u32 {
            match self {
                MatchKind::Pattern => ffi::FcMatchPattern,
                MatchKind::Font => ffi::FcMatchFont,
                MatchKind::Scan => ffi::FcMatchScan,
            }
        }
    }

    // https://www.freedesktop.org/software/fontconfig/fontconfig-devel/x19.html
    #[derive(Clone, Copy)]
    pub enum Object {
        Family,
        File,
        Index,
        PostScriptName,
    }

    impl Object {
        fn as_bytes(&self) -> &[u8] {
            match self {
                Object::Family => b"family\0",
                Object::File => b"file\0",
                Object::Index => b"index\0",
                Object::PostScriptName => b"postscriptname\0",
            }
        }

        fn as_ptr(&self) -> *const libc::c_char {
            self.as_bytes().as_ptr() as *const libc::c_char
        }
    }

    pub struct Config {
        d: *mut ffi::FcConfig,
    }

    impl Config {
        // FcInitLoadConfigAndFonts
        pub fn new() -> Self {
            unsafe {
                Config {
                    d: ffi_dispatch!(
                        feature = "source-fontconfig-dlopen",
                        LIB,
                        FcInitLoadConfigAndFonts,
                    ),
                }
            }
        }
    }

    impl Drop for Config {
        fn drop(&mut self) {
            unsafe {
                ffi_dispatch!(
                    feature = "source-fontconfig-dlopen",
                    LIB,
                    FcConfigDestroy,
                    self.d
                );
            }
        }
    }

    pub struct Pattern {
        d: *mut ffi::FcPattern,
        c_strings: Vec<CString>,
    }

    impl Pattern {
        fn from_ptr(d: *mut ffi::FcPattern) -> Self {
            Pattern {
                d,
                c_strings: vec![],
            }
        }

        // FcPatternCreate
        pub fn new() -> Self {
            unsafe {
                Pattern::from_ptr(ffi_dispatch!(
                    feature = "source-fontconfig-dlopen",
                    LIB,
                    FcPatternCreate,
                ))
            }
        }

        // FcNameParse
        pub fn from_name(name: &str) -> Self {
            let c_name = CString::new(name).unwrap();
            unsafe {
                Pattern::from_ptr(ffi_dispatch!(
                    feature = "source-fontconfig-dlopen",
                    LIB,
                    FcNameParse,
                    c_name.as_ptr() as *mut c_uchar
                ))
            }
        }

        // FcPatternAddString
        pub fn push_string(&mut self, object: Object, value: String) {
            unsafe {
                let c_string = CString::new(value).unwrap();
                ffi_dispatch!(
                    feature = "source-fontconfig-dlopen",
                    LIB,
                    FcPatternAddString,
                    self.d,
                    object.as_ptr(),
                    c_string.as_ptr() as *const c_uchar
                );

                // We have to keep this string, because `FcPattern` has a pointer to it now.
                self.c_strings.push(c_string)
            }
        }

        // FcConfigSubstitute
        pub fn config_substitute(&mut self, match_kind: MatchKind) {
            unsafe {
                ffi_dispatch!(
                    feature = "source-fontconfig-dlopen",
                    LIB,
                    FcConfigSubstitute,
                    ptr::null_mut(),
                    self.d,
                    match_kind.to_u32()
                );
            }
        }

        // FcDefaultSubstitute
        pub fn default_substitute(&mut self) {
            unsafe {
                ffi_dispatch!(
                    feature = "source-fontconfig-dlopen",
                    LIB,
                    FcDefaultSubstitute,
                    self.d
                );
            }
        }

        // FcFontSort
        pub fn sorted(&self, config: &Config) -> Result<FontSet, Error> {
            let mut res = ffi::FcResultMatch;
            let d = unsafe {
                ffi_dispatch!(
                    feature = "source-fontconfig-dlopen",
                    LIB,
                    FcFontSort,
                    config.d,
                    self.d,
                    1,
                    ptr::null_mut(),
                    &mut res
                )
            };

            match res {
                ffi::FcResultMatch => Ok(FontSet { d, idx: 0 }),
                ffi::FcResultTypeMismatch => Err(Error::TypeMismatch),
                ffi::FcResultNoId => Err(Error::NoId),
                ffi::FcResultOutOfMemory => Err(Error::OutOfMemory),
                _ => Err(Error::NoMatch),
            }
        }

        // FcFontList
        pub fn list(&self, config: &Config, set: ObjectSet) -> Result<FontSet, Error> {
            let d = unsafe {
                ffi_dispatch!(
                    feature = "source-fontconfig-dlopen",
                    LIB,
                    FcFontList,
                    config.d,
                    self.d,
                    set.d
                )
            };
            if !d.is_null() {
                Ok(FontSet { d, idx: 0 })
            } else {
                Err(Error::NoMatch)
            }
        }
    }

    impl Drop for Pattern {
        #[inline]
        fn drop(&mut self) {
            unsafe {
                ffi_dispatch!(
                    feature = "source-fontconfig-dlopen",
                    LIB,
                    FcPatternDestroy,
                    self.d
                )
            }
        }
    }

    // A read-only `FcPattern` without a destructor.
    pub struct PatternRef {
        d: *mut ffi::FcPattern,
    }

    impl PatternRef {
        // FcPatternGetString
        pub fn get_string(&self, object: Object) -> Option<String> {
            unsafe {
                let mut string = ptr::null_mut();
                let res = ffi_dispatch!(
                    feature = "source-fontconfig-dlopen",
                    LIB,
                    FcPatternGetString,
                    self.d,
                    object.as_ptr(),
                    0,
                    &mut string
                );
                if res != ffi::FcResultMatch {
                    return None;
                }

                if string.is_null() {
                    return None;
                }

                CStr::from_ptr(string as *const c_char)
                    .to_str()
                    .ok()
                    .map(|string| string.to_owned())
            }
        }

        // FcPatternGetInteger
        pub fn get_integer(&self, object: Object) -> Option<i32> {
            unsafe {
                let mut integer = 0;
                let res = ffi_dispatch!(
                    feature = "source-fontconfig-dlopen",
                    LIB,
                    FcPatternGetInteger,
                    self.d,
                    object.as_ptr(),
                    0,
                    &mut integer
                );
                if res != ffi::FcResultMatch {
                    return None;
                }

                Some(integer)
            }
        }
    }

    pub struct FontSet {
        d: *mut ffi::FcFontSet,
        idx: usize,
    }

    impl FontSet {
        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        pub fn len(&self) -> usize {
            unsafe { (*self.d).nfont as usize }
        }
    }

    impl Iterator for FontSet {
        type Item = PatternRef;

        fn next(&mut self) -> Option<Self::Item> {
            if self.idx == self.len() {
                return None;
            }

            let idx = self.idx;
            self.idx += 1;

            let d = unsafe { *(*self.d).fonts.add(idx) };
            Some(PatternRef { d })
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (0, Some(self.len()))
        }
    }

    impl Drop for FontSet {
        fn drop(&mut self) {
            unsafe {
                ffi_dispatch!(
                    feature = "source-fontconfig-dlopen",
                    LIB,
                    FcFontSetDestroy,
                    self.d
                )
            }
        }
    }

    pub struct ObjectSet {
        d: *mut ffi::FcObjectSet,
    }

    impl ObjectSet {
        // FcObjectSetCreate
        pub fn new() -> Self {
            unsafe {
                ObjectSet {
                    d: ffi_dispatch!(feature = "source-fontconfig-dlopen", LIB, FcObjectSetCreate,),
                }
            }
        }

        // FcObjectSetAdd
        pub fn push_string(&mut self, object: Object) {
            unsafe {
                // Returns `false` if the property name cannot be inserted
                // into the set (due to allocation failure).
                assert_eq!(
                    ffi_dispatch!(
                        feature = "source-fontconfig-dlopen",
                        LIB,
                        FcObjectSetAdd,
                        self.d,
                        object.as_ptr()
                    ),
                    1
                );
            }
        }
    }

    impl Drop for ObjectSet {
        fn drop(&mut self) {
            unsafe {
                ffi_dispatch!(
                    feature = "source-fontconfig-dlopen",
                    LIB,
                    FcObjectSetDestroy,
                    self.d
                )
            }
        }
    }
}

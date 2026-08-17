// font-kit/src/sources/directwrite.rs
//
// Copyright © 2018 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A source that contains the installed fonts on Windows.

use dwrote::Font as DWriteFont;
use dwrote::FontCollection as DWriteFontCollection;
use std::any::Any;

use crate::error::SelectionError;
use crate::family_handle::FamilyHandle;
use crate::family_name::FamilyName;
use crate::handle::Handle;
use crate::properties::Properties;
use crate::source::Source;

/// A source that contains the installed fonts on Windows.
#[allow(missing_debug_implementations)]
pub struct DirectWriteSource {
    system_font_collection: DWriteFontCollection,
}

impl DirectWriteSource {
    /// Opens the system font collection.
    pub fn new() -> DirectWriteSource {
        DirectWriteSource {
            system_font_collection: DWriteFontCollection::system(),
        }
    }

    /// Returns paths of all fonts installed on the system.
    pub fn all_fonts(&self) -> Result<Vec<Handle>, SelectionError> {
        let mut handles = Vec::new();

        for dwrite_family in self.system_font_collection.families_iter() {
            for font_index in 0..dwrite_family.get_font_count() {
                let dwrite_font = dwrite_family.get_font(font_index);
                handles.push(self.create_handle_from_dwrite_font(dwrite_font))
            }
        }

        Ok(handles)
    }

    /// Returns the names of all families installed on the system.
    pub fn all_families(&self) -> Result<Vec<String>, SelectionError> {
        Ok(self
            .system_font_collection
            .families_iter()
            .map(|dwrite_family| dwrite_family.name())
            .collect())
    }

    /// Looks up a font family by name and returns the handles of all the fonts in that family.
    ///
    /// TODO(pcwalton): Case-insensitivity.
    pub fn select_family_by_name(&self, family_name: &str) -> Result<FamilyHandle, SelectionError> {
        let mut family = FamilyHandle::new();
        let dwrite_family = match self
            .system_font_collection
            .get_font_family_by_name(family_name)
        {
            Some(dwrite_family) => dwrite_family,
            None => return Err(SelectionError::NotFound),
        };
        for font_index in 0..dwrite_family.get_font_count() {
            let dwrite_font = dwrite_family.get_font(font_index);
            family.push(self.create_handle_from_dwrite_font(dwrite_font))
        }
        Ok(family)
    }

    /// Selects a font by PostScript name, which should be a unique identifier.
    ///
    /// On the DirectWrite backend, this does a brute-force search of installed fonts to find the
    /// one that matches.
    pub fn select_by_postscript_name(
        &self,
        postscript_name: &str,
    ) -> Result<Handle, SelectionError> {
        <Self as Source>::select_by_postscript_name(self, postscript_name)
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

    fn create_handle_from_dwrite_font(&self, dwrite_font: DWriteFont) -> Handle {
        let dwrite_font_face = dwrite_font.create_font_face();
        let dwrite_font_files = dwrite_font_face.get_files();
        Handle::Path {
            path: dwrite_font_files[0].get_font_file_path().unwrap(),
            font_index: dwrite_font_face.get_index(),
        }
    }
}

impl Source for DirectWriteSource {
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
    fn as_any(&self) -> &dyn Any {
        self
    }

    #[inline]
    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

// font-kit/src/family_handle.rs
//
// Copyright © 2018 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Encapsulates the information needed to locate and open the fonts in a family.

use crate::handle::Handle;

/// Encapsulates the information needed to locate and open the fonts in a family.
#[derive(Debug)]
pub struct FamilyHandle {
    pub(crate) fonts: Vec<Handle>,
}

impl Default for FamilyHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl FamilyHandle {
    /// Creates an empty set of family handles.
    #[inline]
    pub fn new() -> FamilyHandle {
        FamilyHandle { fonts: vec![] }
    }

    /// Creates a set of font family handles.
    #[inline]
    pub fn from_font_handles<I>(fonts: I) -> FamilyHandle
    where
        I: Iterator<Item = Handle>,
    {
        FamilyHandle {
            fonts: fonts.collect::<Vec<Handle>>(),
        }
    }

    /// Adds a new handle to this set.
    #[inline]
    pub fn push(&mut self, font: Handle) {
        self.fonts.push(font)
    }

    /// Returns true if and only if this set has no fonts in it.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.fonts.is_empty()
    }

    /// Returns all the handles in this set.
    #[inline]
    pub fn fonts(&self) -> &[Handle] {
        &self.fonts
    }
}

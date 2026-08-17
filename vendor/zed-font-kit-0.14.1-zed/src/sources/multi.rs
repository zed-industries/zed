// font-kit/src/sources/multi.rs
//
// Copyright © 2018 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A source that encapsulates multiple sources and allows them to be queried as a group.
//!
//! This is useful when an application wants a library of fonts consisting of the installed system
//! fonts plus some other application-supplied fonts.

use crate::error::SelectionError;
use crate::family_handle::FamilyHandle;
use crate::family_name::FamilyName;
use crate::handle::Handle;
use crate::properties::Properties;
use crate::source::Source;
use std::{
    any::Any,
    fmt,
    ops::{Index, IndexMut},
    slice,
};

/// A source that encapsulates multiple sources and allows them to be queried as a group.
///
/// This is useful when an application wants a library of fonts consisting of the installed system
/// fonts plus some other application-supplied fonts.
#[allow(missing_debug_implementations)]
pub struct MultiSource {
    subsources: Vec<Box<dyn Source>>,
}

impl MultiSource {
    /// Creates a new source that contains all the fonts in the supplied sources.
    pub fn from_sources(subsources: Vec<Box<dyn Source>>) -> MultiSource {
        MultiSource { subsources }
    }

    /// Returns paths of all fonts installed on the system.
    pub fn all_fonts(&self) -> Result<Vec<Handle>, SelectionError> {
        let mut handles = vec![];
        for subsource in &self.subsources {
            handles.extend(subsource.all_fonts()?.into_iter())
        }
        Ok(handles)
    }

    /// Returns the names of all families installed on the system.
    pub fn all_families(&self) -> Result<Vec<String>, SelectionError> {
        let mut families = vec![];
        for subsource in &self.subsources {
            families.extend(subsource.all_families()?.into_iter())
        }
        Ok(families)
    }

    /// Looks up a font family by name and returns the handles of all the fonts in that family.
    pub fn select_family_by_name(&self, family_name: &str) -> Result<FamilyHandle, SelectionError> {
        for subsource in &self.subsources {
            match subsource.select_family_by_name(family_name) {
                Ok(family) => return Ok(family),
                Err(SelectionError::NotFound) => {}
                Err(err) => return Err(err),
            }
        }
        Err(SelectionError::NotFound)
    }

    /// Selects a font by PostScript name, which should be a unique identifier.
    pub fn select_by_postscript_name(
        &self,
        postscript_name: &str,
    ) -> Result<Handle, SelectionError> {
        for subsource in &self.subsources {
            match subsource.select_by_postscript_name(postscript_name) {
                Ok(font) => return Ok(font),
                Err(SelectionError::NotFound) => {}
                Err(err) => return Err(err),
            }
        }
        Err(SelectionError::NotFound)
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

    /// Returns an iterator over the contained sources.
    #[inline]
    pub fn iter(&self) -> MultiIter<'_> {
        MultiIter(self.subsources.iter())
    }

    /// Returns an iterator over the contained sources with mutable access.
    #[inline]
    pub fn iter_mut(&mut self) -> MultiIterMut<'_> {
        MultiIterMut(self.subsources.iter_mut())
    }

    /// A convenience method to get the first source with the given type.
    ///
    /// Returns `None` if no source of the given type was found.
    pub fn find_source<T: Source + 'static>(&self) -> Option<&T> {
        self.iter().find_map(|v| v.as_any().downcast_ref())
    }

    /// A convenience method to get the first source with the given type.
    ///
    /// Returns `None` if no source of the given type was found.
    pub fn find_source_mut<T: Source + 'static>(&mut self) -> Option<&mut T> {
        self.iter_mut().find_map(|v| v.as_mut_any().downcast_mut())
    }
}

impl Source for MultiSource {
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

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Index<usize> for MultiSource {
    type Output = dyn Source;

    fn index(&self, idx: usize) -> &Self::Output {
        &*self.subsources[idx]
    }
}

impl IndexMut<usize> for MultiSource {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut *self.subsources[idx]
    }
}

/// An iterator over the sources in a [`MultiSource`].
pub struct MultiIter<'a>(slice::Iter<'a, Box<dyn Source>>);

impl<'a> Iterator for MultiIter<'a> {
    type Item = &'a dyn Source;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|v| &**v)
    }
}

impl fmt::Debug for MultiIter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MultiIter").finish()
    }
}

/// An iterator over the mutable sources in a [`MultiSource`].
pub struct MultiIterMut<'a>(slice::IterMut<'a, Box<dyn Source>>);

impl<'a> Iterator for MultiIterMut<'a> {
    type Item = &'a mut dyn Source;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|v| &mut **v)
    }
}

impl fmt::Debug for MultiIterMut<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MultiIterMut").finish()
    }
}

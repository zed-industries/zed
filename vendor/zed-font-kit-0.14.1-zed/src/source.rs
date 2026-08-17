// font-kit/src/source.rs
//
// Copyright © 2018 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A database of installed fonts that can be queried.

use crate::error::SelectionError;
use crate::family::Family;
use crate::family_handle::FamilyHandle;
use crate::family_name::FamilyName;
use crate::font::Font;
use crate::handle::Handle;
use crate::matching;
use crate::properties::Properties;
use std::any::Any;

#[cfg(all(
    any(target_os = "macos", target_os = "ios"),
    not(feature = "loader-freetype-default")
))]
pub use crate::sources::core_text::CoreTextSource as SystemSource;
#[cfg(all(target_family = "windows", not(feature = "source-fontconfig-default")))]
pub use crate::sources::directwrite::DirectWriteSource as SystemSource;
#[cfg(all(
    any(
        not(any(
            target_os = "android",
            target_os = "macos",
            target_os = "ios",
            target_family = "windows",
            target_arch = "wasm32",
        )),
        feature = "source-fontconfig-default"
    ),
    not(target_env = "ohos")
))]
pub use crate::sources::fontconfig::FontconfigSource as SystemSource;
#[cfg(all(target_os = "android", not(feature = "source-fontconfig-default")))]
pub use crate::sources::fs::FsSource as SystemSource;

#[cfg(target_env = "ohos")]
pub use crate::sources::fs::FsSource as SystemSource;

// FIXME(pcwalton): These could expand to multiple fonts, and they could be language-specific.
#[cfg(any(target_family = "windows", target_os = "macos", target_os = "ios"))]
const DEFAULT_FONT_FAMILY_SERIF: &'static str = "Times New Roman";
#[cfg(any(target_family = "windows", target_os = "macos", target_os = "ios"))]
const DEFAULT_FONT_FAMILY_SANS_SERIF: &'static str = "Arial";
#[cfg(target_env = "ohos")]
const DEFAULT_FONT_FAMILY_SANS_SERIF: &str = "HarmonyOS Sans";
#[cfg(any(target_family = "windows", target_os = "macos", target_os = "ios"))]
const DEFAULT_FONT_FAMILY_MONOSPACE: &'static str = "Courier New";
#[cfg(target_env = "ohos")]
const DEFAULT_FONT_FAMILY_MONOSPACE: &str = "HarmonyOS Sans";
#[cfg(any(target_family = "windows", target_os = "macos", target_os = "ios"))]
const DEFAULT_FONT_FAMILY_CURSIVE: &'static str = "Comic Sans MS";
#[cfg(target_family = "windows")]
const DEFAULT_FONT_FAMILY_FANTASY: &'static str = "Impact";
#[cfg(any(target_os = "macos", target_os = "ios"))]
const DEFAULT_FONT_FAMILY_FANTASY: &'static str = "Papyrus";

#[cfg(not(any(target_family = "windows", target_os = "macos", target_os = "ios")))]
const DEFAULT_FONT_FAMILY_SERIF: &str = "serif";
#[cfg(not(any(
    target_family = "windows",
    target_os = "macos",
    target_os = "ios",
    target_env = "ohos"
)))]
const DEFAULT_FONT_FAMILY_SANS_SERIF: &str = "sans-serif";
#[cfg(not(any(
    target_family = "windows",
    target_os = "macos",
    target_os = "ios",
    target_env = "ohos"
)))]
const DEFAULT_FONT_FAMILY_MONOSPACE: &str = "monospace";
#[cfg(not(any(target_family = "windows", target_os = "macos", target_os = "ios")))]
const DEFAULT_FONT_FAMILY_CURSIVE: &str = "cursive";
#[cfg(not(any(target_family = "windows", target_os = "macos", target_os = "ios")))]
const DEFAULT_FONT_FAMILY_FANTASY: &str = "fantasy";

/// A database of installed fonts that can be queried.
///
/// This trait is object-safe.
pub trait Source: Any {
    /// Returns paths of all fonts installed on the system.
    fn all_fonts(&self) -> Result<Vec<Handle>, SelectionError>;

    /// Returns the names of all families installed on the system.
    fn all_families(&self) -> Result<Vec<String>, SelectionError>;

    /// Looks up a font family by name and returns the handles of all the fonts in that family.
    fn select_family_by_name(&self, family_name: &str) -> Result<FamilyHandle, SelectionError>;

    /// Selects a font by PostScript name, which should be a unique identifier.
    ///
    /// The default implementation, which is used by the DirectWrite and the filesystem backends,
    /// does a brute-force search of installed fonts to find the one that matches.
    fn select_by_postscript_name(&self, postscript_name: &str) -> Result<Handle, SelectionError> {
        // TODO(pcwalton): Optimize this by searching for families with similar names first.
        for family_name in self.all_families()? {
            if let Ok(family_handle) = self.select_family_by_name(&family_name) {
                if let Ok(family) = Family::<Font>::from_handle(&family_handle) {
                    for (handle, font) in family_handle.fonts().iter().zip(family.fonts().iter()) {
                        if let Some(font_postscript_name) = font.postscript_name() {
                            if font_postscript_name == postscript_name {
                                return Ok((*handle).clone());
                            }
                        }
                    }
                }
            }
        }
        Err(SelectionError::NotFound)
    }

    // FIXME(pcwalton): This only returns one family instead of multiple families for the generic
    // family names.
    #[doc(hidden)]
    fn select_family_by_generic_name(
        &self,
        family_name: &FamilyName,
    ) -> Result<FamilyHandle, SelectionError> {
        match *family_name {
            FamilyName::Title(ref title) => self.select_family_by_name(title),
            FamilyName::Serif => self.select_family_by_name(DEFAULT_FONT_FAMILY_SERIF),
            FamilyName::SansSerif => self.select_family_by_name(DEFAULT_FONT_FAMILY_SANS_SERIF),
            FamilyName::Monospace => self.select_family_by_name(DEFAULT_FONT_FAMILY_MONOSPACE),
            FamilyName::Cursive => self.select_family_by_name(DEFAULT_FONT_FAMILY_CURSIVE),
            FamilyName::Fantasy => self.select_family_by_name(DEFAULT_FONT_FAMILY_FANTASY),
        }
    }

    /// Performs font matching according to the CSS Fonts Level 3 specification and returns the
    /// handle.
    #[inline]
    fn select_best_match(
        &self,
        family_names: &[FamilyName],
        properties: &Properties,
    ) -> Result<Handle, SelectionError> {
        for family_name in family_names {
            if let Ok(family_handle) = self.select_family_by_generic_name(family_name) {
                let candidates = self.select_descriptions_in_family(&family_handle)?;
                if let Ok(index) = matching::find_best_match(&candidates, properties) {
                    return Ok(family_handle.fonts[index].clone());
                }
            }
        }
        Err(SelectionError::NotFound)
    }

    #[doc(hidden)]
    fn select_descriptions_in_family(
        &self,
        family: &FamilyHandle,
    ) -> Result<Vec<Properties>, SelectionError> {
        let mut fields = vec![];
        for font_handle in family.fonts() {
            match Font::from_handle(font_handle) {
                Ok(font) => fields.push(font.properties()),
                Err(e) => log::warn!("Error loading font from handle: {:?}", e),
            }
        }
        Ok(fields)
    }

    /// Accesses this `Source` as `Any`, which allows downcasting back to a concrete type from a
    /// trait object.
    fn as_any(&self) -> &dyn Any;

    /// Accesses this `Source` as `Any`, which allows downcasting back to a concrete type from a
    /// trait object.
    fn as_mut_any(&mut self) -> &mut dyn Any;
}

// font-kit/src/sources/fs.rs
//
// Copyright © 2018 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A source that loads fonts from a directory or directories on disk.
//!
//! This source uses the WalkDir abstraction from the `walkdir` crate to locate fonts.
//!
//! This is the native source on Android and OpenHarmony.

use std::any::Any;
use std::fs::File;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[cfg(not(any(target_os = "android", target_family = "windows", target_env = "ohos")))]
use dirs;
#[cfg(target_family = "windows")]
use std::ffi::OsString;
#[cfg(target_family = "windows")]
use std::os::windows::ffi::OsStringExt;
#[cfg(target_family = "windows")]
use winapi::shared::minwindef::{MAX_PATH, UINT};
#[cfg(target_family = "windows")]
use winapi::um::sysinfoapi;

use crate::error::SelectionError;
use crate::family_handle::FamilyHandle;
use crate::family_name::FamilyName;
use crate::file_type::FileType;
use crate::font::Font;
use crate::handle::Handle;
use crate::properties::Properties;
use crate::source::Source;
use crate::sources::mem::MemSource;

/// A source that loads fonts from a directory or directories on disk.
///
/// This source uses the WalkDir abstraction from the `walkdir` crate to locate fonts.
///
/// This is the native source on Android and OpenHarmony.
#[allow(missing_debug_implementations)]
pub struct FsSource {
    mem_source: MemSource,
}

impl Default for FsSource {
    fn default() -> Self {
        Self::new()
    }
}

impl FsSource {
    /// Opens the default set of directories on this platform and indexes the fonts found within.
    ///
    /// Do not rely on this function for systems other than Android or OpenHarmony. It makes a best
    /// effort to locate fonts in the typical platform directories, but it is too simple to pick up
    /// fonts that are stored in unusual locations but nevertheless properly installed.
    pub fn new() -> FsSource {
        let mut fonts = vec![];
        for font_directory in default_font_directories() {
            fonts.extend(Self::discover_fonts(&font_directory));
        }

        FsSource {
            mem_source: MemSource::from_fonts(fonts.into_iter()).unwrap(),
        }
    }

    fn discover_fonts(path: &Path) -> Vec<Handle> {
        let mut fonts = vec![];
        for directory_entry in WalkDir::new(path).into_iter() {
            let directory_entry = match directory_entry {
                Ok(directory_entry) => directory_entry,
                Err(_) => continue,
            };
            let path = directory_entry.path();
            let mut file = match File::open(path) {
                Err(_) => continue,
                Ok(file) => file,
            };
            match Font::analyze_file(&mut file) {
                Err(_) => continue,
                Ok(FileType::Single) => fonts.push(Handle::from_path(path.to_owned(), 0)),
                Ok(FileType::Collection(font_count)) => {
                    for font_index in 0..font_count {
                        fonts.push(Handle::from_path(path.to_owned(), font_index))
                    }
                }
            }
        }
        fonts
    }

    /// Indexes all fonts found in `path`
    pub fn in_path<P>(path: P) -> FsSource
    where
        P: AsRef<Path>,
    {
        let fonts = Self::discover_fonts(path.as_ref());
        FsSource {
            mem_source: MemSource::from_fonts(fonts.into_iter()).unwrap(),
        }
    }

    /// Returns paths of all fonts installed on the system.
    pub fn all_fonts(&self) -> Result<Vec<Handle>, SelectionError> {
        self.mem_source.all_fonts()
    }

    /// Returns the names of all families installed on the system.
    pub fn all_families(&self) -> Result<Vec<String>, SelectionError> {
        self.mem_source.all_families()
    }

    /// Looks up a font family by name and returns the handles of all the fonts in that family.
    pub fn select_family_by_name(&self, family_name: &str) -> Result<FamilyHandle, SelectionError> {
        self.mem_source.select_family_by_name(family_name)
    }

    /// Selects a font by PostScript name, which should be a unique identifier.
    ///
    /// This implementation does a brute-force search of installed fonts to find the one that
    /// matches.
    pub fn select_by_postscript_name(
        &self,
        postscript_name: &str,
    ) -> Result<Handle, SelectionError> {
        self.mem_source.select_by_postscript_name(postscript_name)
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

impl Source for FsSource {
    #[inline]
    fn all_fonts(&self) -> Result<Vec<Handle>, SelectionError> {
        self.all_fonts()
    }

    #[inline]
    fn all_families(&self) -> Result<Vec<String>, SelectionError> {
        self.all_families()
    }

    fn select_family_by_name(&self, family_name: &str) -> Result<FamilyHandle, SelectionError> {
        self.select_family_by_name(family_name)
    }

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

#[cfg(any(target_os = "android", target_env = "ohos"))]
fn default_font_directories() -> Vec<PathBuf> {
    vec![PathBuf::from("/system/fonts")]
}

#[cfg(target_family = "windows")]
fn default_font_directories() -> Vec<PathBuf> {
    unsafe {
        let mut buffer = vec![0; MAX_PATH];
        let len = sysinfoapi::GetWindowsDirectoryW(buffer.as_mut_ptr(), buffer.len() as UINT);
        assert!(len != 0);
        buffer.truncate(len as usize);

        let mut path = PathBuf::from(OsString::from_wide(&buffer));
        path.push("Fonts");
        vec![path]
    }
}

#[cfg(target_os = "macos")]
fn default_font_directories() -> Vec<PathBuf> {
    let mut directories = vec![
        PathBuf::from("/System/Library/Fonts"),
        PathBuf::from("/Library/Fonts"),
        PathBuf::from("/Network/Library/Fonts"),
    ];
    if let Some(mut path) = dirs::home_dir() {
        path.push("Library");
        path.push("Fonts");
        directories.push(path);
    }
    directories
}

#[cfg(not(any(
    target_os = "android",
    target_family = "windows",
    target_os = "macos",
    target_env = "ohos"
)))]
fn default_font_directories() -> Vec<PathBuf> {
    let mut directories = vec![
        PathBuf::from("/usr/share/fonts"),
        PathBuf::from("/usr/local/share/fonts"),
        PathBuf::from("/var/run/host/usr/share/fonts"), // Flatpak specific
        PathBuf::from("/var/run/host/usr/local/share/fonts"),
    ];
    if let Some(path) = dirs::home_dir() {
        directories.push(path.join(".fonts")); // ~/.fonts is deprecated
        directories.push(path.join("local").join("share").join("fonts")); // Flatpak specific
    }
    if let Some(mut path) = dirs::data_dir() {
        path.push("fonts");
        directories.push(path);
    }
    directories
}

// font-kit/src/error.rs
//
// Copyright © 2018 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Various types of errors that `font-kit` can return.

use std::borrow::Cow;
use std::convert::From;
use std::error::Error;
use std::io;

macro_rules! impl_display {
    ($enum:ident, {$($variant:pat => $fmt_string:expr),+$(,)* }) => {

        impl ::std::fmt::Display for $enum {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                use self::$enum::*;
                match &self {
                    $(
                        $variant => write!(f, "{}", $fmt_string),
                    )+
                }
            }
        }
    };
}

/// Reasons why a loader might fail to load a font.
#[derive(Debug)]
pub enum FontLoadingError {
    /// The data was of a format the loader didn't recognize.
    UnknownFormat,
    /// Attempted to load an invalid index in a TrueType or OpenType font collection.
    ///
    /// For example, if a `.ttc` file has 2 fonts in it, and you ask for the 5th one, you'll get
    /// this error.
    NoSuchFontInCollection,
    /// Attempted to load a malformed or corrupted font.
    Parse,
    /// Attempted to load a font from the filesystem, but there is no filesystem (e.g. in
    /// WebAssembly).
    NoFilesystem,
    /// A disk or similar I/O error occurred while attempting to load the font.
    Io(io::Error),
}

impl Error for FontLoadingError {}

impl_display! { FontLoadingError, {
        UnknownFormat => "unknown format",
        NoSuchFontInCollection => "no such font in the collection",
        Parse => "parse error",
        NoFilesystem => "no filesystem present",
        Io(e) => format!("I/O error: {}", e),
    }
}

impl From<io::Error> for FontLoadingError {
    fn from(error: io::Error) -> FontLoadingError {
        FontLoadingError::Io(error)
    }
}

/// Reasons why a font might fail to load a glyph.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GlyphLoadingError {
    /// The font didn't contain a glyph with that ID.
    NoSuchGlyph,
    /// A platform function returned an error.
    PlatformError,
}

impl Error for GlyphLoadingError {}

impl_display! { GlyphLoadingError, {
        NoSuchGlyph => "no such glyph",
        PlatformError => "platform error",
    }
}

#[cfg(target_family = "windows")]
impl From<winapi::um::winnt::HRESULT> for GlyphLoadingError {
    fn from(_err: winapi::um::winnt::HRESULT) -> GlyphLoadingError {
        GlyphLoadingError::PlatformError
    }
}

/// Reasons why a source might fail to look up a font or fonts.
#[derive(Clone, PartialEq, Debug)]
pub enum SelectionError {
    /// No font matching the given query was found.
    NotFound,
    /// The source was inaccessible because of an I/O or similar error.
    CannotAccessSource {
        /// Additional diagnostic information may include file name
        reason: Option<Cow<'static, str>>,
    },
}

impl Error for SelectionError {}

impl_display! { SelectionError, {
        NotFound => "no font found",
        CannotAccessSource { reason: ref maybe_cow } => maybe_cow.as_deref().unwrap_or("failed to access source")
    }
}

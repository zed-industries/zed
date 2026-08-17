// font-kit/src/hinting.rs
//
// Copyright © 2018 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Specifies how hinting (grid fitting) is to be performed (or not performed) for a glyph.
//!
//! This affects both outlines and rasterization.

/// Specifies how hinting (grid fitting) is to be performed (or not performed) for a glyph.
///
/// This affects both outlines and rasterization.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HintingOptions {
    /// No hinting is performed unless absolutely necessary to assemble the glyph.
    ///
    /// This corresponds to what macOS and FreeType in its "no hinting" mode do.
    None,

    /// Hinting is performed only in the vertical direction. The specified point size is used for
    /// grid fitting.
    ///
    /// This corresponds to what DirectWrite and FreeType in its light hinting mode do.
    Vertical(f32),

    /// Hinting is performed only in the vertical direction, and further tweaks are applied to make
    /// subpixel antialiasing look better. The specified point size is used for grid fitting.
    ///
    /// This matches DirectWrite, GDI in its ClearType mode, and FreeType in its LCD hinting mode.
    VerticalSubpixel(f32),

    /// Hinting is performed in both horizontal and vertical directions. The specified point size
    /// is used for grid fitting.
    ///
    /// This corresponds to what GDI in non-ClearType modes and FreeType in its normal hinting mode
    /// do.
    Full(f32),
}

impl HintingOptions {
    /// Returns the point size that will be used for grid fitting, if any.
    #[inline]
    pub fn grid_fitting_size(&self) -> Option<f32> {
        match *self {
            HintingOptions::None => None,
            HintingOptions::Vertical(size)
            | HintingOptions::VerticalSubpixel(size)
            | HintingOptions::Full(size) => Some(size),
        }
    }
}

// Copyright 2006 The Android Open Source Project
// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use crate::geom::ScreenIntRect;

use crate::alpha_runs::AlphaRun;
use crate::color::AlphaU8;
use crate::LengthU32;

/// Mask is used to describe alpha bitmaps.
pub struct Mask {
    pub image: [u8; 2],
    pub bounds: ScreenIntRect,
    pub row_bytes: u32,
}

/// Blitter is responsible for actually writing pixels into memory.
///
/// Besides efficiency, they handle clipping and antialiasing.
/// An object that implements Blitter contains all the context needed to generate pixels
/// for the destination and how src/generated pixels map to the destination.
/// The coordinates passed to the `blit_*` calls are in destination pixel space.
pub trait Blitter {
    /// Blits a horizontal run of one or more pixels.
    fn blit_h(&mut self, _x: u32, _y: u32, _width: LengthU32) {
        unreachable!()
    }

    /// Blits a horizontal run of antialiased pixels.
    ///
    /// runs[] is a *sparse* zero-terminated run-length encoding of spans of constant alpha values.
    ///
    /// The runs[] and antialias[] work together to represent long runs of pixels with the same
    /// alphas. The runs[] contains the number of pixels with the same alpha, and antialias[]
    /// contain the coverage value for that number of pixels. The runs[] (and antialias[]) are
    /// encoded in a clever way. The runs array is zero terminated, and has enough entries for
    /// each pixel plus one, in most cases some of the entries will not contain valid data. An entry
    /// in the runs array contains the number of pixels (np) that have the same alpha value. The
    /// next np value is found np entries away. For example, if runs[0] = 7, then the next valid
    /// entry will by at runs[7]. The runs array and antialias[] are coupled by index. So, if the
    /// np entry is at runs[45] = 12 then the alpha value can be found at antialias[45] = 0x88.
    /// This would mean to use an alpha value of 0x88 for the next 12 pixels starting at pixel 45.
    fn blit_anti_h(
        &mut self,
        _x: u32,
        _y: u32,
        _antialias: &mut [AlphaU8],
        _runs: &mut [AlphaRun],
    ) {
        unreachable!()
    }

    /// Blits a vertical run of pixels with a constant alpha value.
    fn blit_v(&mut self, _x: u32, _y: u32, _height: LengthU32, _alpha: AlphaU8) {
        unreachable!()
    }

    fn blit_anti_h2(&mut self, _x: u32, _y: u32, _alpha0: AlphaU8, _alpha1: AlphaU8) {
        unreachable!()
    }

    fn blit_anti_v2(&mut self, _x: u32, _y: u32, _alpha0: AlphaU8, _alpha1: AlphaU8) {
        unreachable!()
    }

    /// Blits a solid rectangle one or more pixels wide.
    fn blit_rect(&mut self, _rect: &ScreenIntRect) {
        unreachable!()
    }

    /// Blits a pattern of pixels defined by a rectangle-clipped mask.
    fn blit_mask(&mut self, _mask: &Mask, _clip: &ScreenIntRect) {
        unreachable!()
    }
}

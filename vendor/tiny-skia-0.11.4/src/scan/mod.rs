// Copyright 2011 The Android Open Source Project
// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod hairline;
pub mod hairline_aa;
pub mod path;
pub mod path_aa;

use crate::{IntRect, Rect};

use crate::blitter::Blitter;
use crate::geom::{IntRectExt, ScreenIntRect};

pub fn fill_rect(rect: &Rect, clip: &ScreenIntRect, blitter: &mut dyn Blitter) {
    if let Some(rect) = rect.round() {
        fill_int_rect(&rect, clip, blitter);
    }
}

fn fill_int_rect(rect: &IntRect, clip: &ScreenIntRect, blitter: &mut dyn Blitter) {
    let rect = match rect.intersect(&clip.to_int_rect()) {
        Some(v) => v,
        None => return, // everything was clipped out
    };

    let rect = match rect.to_screen_int_rect() {
        Some(v) => v,
        None => return,
    };

    blitter.blit_rect(&rect);
}

pub fn fill_rect_aa(rect: &Rect, clip: &ScreenIntRect, blitter: &mut dyn Blitter) {
    hairline_aa::fill_rect(rect, clip, blitter);
}

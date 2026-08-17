// Copyright (c) 2018-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use num_derive::FromPrimitive;

use crate::api::{Opaque, T35};
use crate::context::SB_SIZE;
use crate::mc::SUBPEL_FILTER_SIZE;
use crate::util::*;

use crate::tiling::*;

mod plane;
pub use plane::*;

const FRAME_MARGIN: usize = 16 + SUBPEL_FILTER_SIZE;
const LUMA_PADDING: usize = SB_SIZE + FRAME_MARGIN;

/// Override the frame type decision
///
/// Only certain frame types can be selected.
#[derive(Debug, PartialEq, Eq, Clone, Copy, FromPrimitive, Default)]
#[repr(C)]
pub enum FrameTypeOverride {
  /// Do not force any decision.
  #[default]
  No,
  /// Force the frame to be a Keyframe.
  Key,
}

/// Optional per-frame encoder parameters
#[derive(Debug, Default)]
pub struct FrameParameters {
  /// Force emitted frame to be of the type selected
  pub frame_type_override: FrameTypeOverride,
  /// Output the provided data in the matching encoded Packet
  pub opaque: Option<Opaque>,
  /// List of t35 metadata associated with this frame
  pub t35_metadata: Box<[T35]>,
}

pub use v_frame::frame::Frame;

/// Public Trait Interface for Frame Allocation
pub(crate) trait FrameAlloc {
  /// Initialise new frame default type
  fn new(width: usize, height: usize, chroma_sampling: ChromaSampling)
    -> Self;
}

impl<T: Pixel> FrameAlloc for Frame<T> {
  /// Creates a new frame with the given parameters.
  /// new function calls `new_with_padding` function which takes `luma_padding`
  /// as parameter
  fn new(
    width: usize, height: usize, chroma_sampling: ChromaSampling,
  ) -> Self {
    v_frame::frame::Frame::new_with_padding(
      width,
      height,
      chroma_sampling,
      LUMA_PADDING,
    )
  }
}

/// Public Trait for calculating Padding
pub(crate) trait FramePad {
  fn pad(&mut self, w: usize, h: usize, planes: usize);
}

impl<T: Pixel> FramePad for Frame<T> {
  fn pad(&mut self, w: usize, h: usize, planes: usize) {
    for pli in 0..planes {
      self.planes[pli].pad(w, h);
    }
  }
}

/// Public Trait for new Tile of a frame
pub(crate) trait AsTile<T: Pixel> {
  fn as_tile(&self) -> Tile<'_, T>;
  fn as_tile_mut(&mut self) -> TileMut<'_, T>;
}

impl<T: Pixel> AsTile<T> for Frame<T> {
  #[inline(always)]
  fn as_tile(&self) -> Tile<'_, T> {
    let PlaneConfig { width, height, .. } = self.planes[0].cfg;
    Tile::new(self, TileRect { x: 0, y: 0, width, height })
  }
  #[inline(always)]
  fn as_tile_mut(&mut self) -> TileMut<'_, T> {
    let PlaneConfig { width, height, .. } = self.planes[0].cfg;
    TileMut::new(self, TileRect { x: 0, y: 0, width, height })
  }
}

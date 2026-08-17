// Copyright (c) 2019-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use super::*;

use crate::context::*;
use crate::frame::*;
use crate::util::*;

/// Rectangle of a tile, in pixels
///
/// This is similar to Rect, but with unsigned (x, y) for convenience.
#[derive(Debug, Clone, Copy)]
pub struct TileRect {
  pub x: usize,
  pub y: usize,
  pub width: usize,
  pub height: usize,
}

impl TileRect {
  #[inline(always)]
  pub const fn decimated(&self, xdec: usize, ydec: usize) -> Self {
    Self {
      x: self.x >> xdec,
      y: self.y >> ydec,
      width: self.width >> xdec,
      height: self.height >> ydec,
    }
  }

  #[inline(always)]
  pub const fn to_frame_plane_offset(
    &self, tile_po: PlaneOffset,
  ) -> PlaneOffset {
    PlaneOffset {
      x: self.x as isize + tile_po.x,
      y: self.y as isize + tile_po.y,
    }
  }

  #[inline(always)]
  pub fn to_frame_block_offset(
    &self, tile_bo: TileBlockOffset, xdec: usize, ydec: usize,
  ) -> PlaneBlockOffset {
    debug_assert!(self.x % (MI_SIZE >> xdec) == 0);
    debug_assert!(self.y % (MI_SIZE >> ydec) == 0);
    let bx = self.x >> (MI_SIZE_LOG2 - xdec);
    let by = self.y >> (MI_SIZE_LOG2 - ydec);
    PlaneBlockOffset(BlockOffset { x: bx + tile_bo.0.x, y: by + tile_bo.0.y })
  }

  #[inline(always)]
  pub fn to_frame_super_block_offset(
    &self, tile_sbo: TileSuperBlockOffset, sb_size_log2: usize, xdec: usize,
    ydec: usize,
  ) -> PlaneSuperBlockOffset {
    debug_assert!(sb_size_log2 == 6 || sb_size_log2 == 7);
    debug_assert!(self.x % (1 << (sb_size_log2 - xdec)) == 0);
    debug_assert!(self.y % (1 << (sb_size_log2 - ydec)) == 0);
    let sbx = self.x >> (sb_size_log2 - xdec);
    let sby = self.y >> (sb_size_log2 - ydec);
    PlaneSuperBlockOffset(SuperBlockOffset {
      x: sbx + tile_sbo.0.x,
      y: sby + tile_sbo.0.y,
    })
  }
}

impl From<TileRect> for Rect {
  #[inline(always)]
  fn from(tile_rect: TileRect) -> Rect {
    Rect {
      x: tile_rect.x as isize,
      y: tile_rect.y as isize,
      width: tile_rect.width,
      height: tile_rect.height,
    }
  }
}

/// Tiled view of a frame
#[derive(Debug)]
pub struct Tile<'a, T: Pixel> {
  pub planes: [PlaneRegion<'a, T>; MAX_PLANES],
}

/// Mutable tiled view of a frame
#[derive(Debug)]
pub struct TileMut<'a, T: Pixel> {
  pub planes: [PlaneRegionMut<'a, T>; MAX_PLANES],
}

// common impl for Tile and TileMut
macro_rules! tile_common {
  // $name: Tile or TileMut
  // $pr_type: PlaneRegion or PlaneRegionMut
  // $iter: iter or iter_mut
  //opt_mut: nothing or mut
  ($name:ident, $pr_type:ident, $iter:ident $(,$opt_mut:tt)?) => {
    impl<'a, T: Pixel> $name<'a, T> {

      #[inline(always)]
      pub fn new(
        frame: &'a $($opt_mut)? Frame<T>,
        luma_rect: TileRect,
      ) -> Self {
        let mut planes_iter = frame.planes.$iter();
        Self {
          planes: [
            {
              let plane = planes_iter.next().unwrap();
              $pr_type::new(plane, luma_rect.into())
            },
            {
              let plane = planes_iter.next().unwrap();
              let rect = luma_rect.decimated(plane.cfg.xdec, plane.cfg.ydec);
              $pr_type::new(plane, rect.into())
            },
            {
              let plane = planes_iter.next().unwrap();
              let rect = luma_rect.decimated(plane.cfg.xdec, plane.cfg.ydec);
              $pr_type::new(plane, rect.into())
            },
          ],
        }
      }

      /// Return a view to a subregion of a Tile
      ///
      /// The subregion must be included in (i.e. must not exceed) this Tile.
      ///
      /// It is described by an `Area`, relative to the luma plane of
      /// this region.
      ///
      /// # Panics
      ///
      /// - If the requested dimensions are larger than the plane size
      #[inline(always)]
      pub fn subregion(&self, area: Area) -> Tile<'_, T> {
        let tile_rect = area.to_rect(
          0,
          0,
          self.planes[0].rect().width,
          self.planes[0].rect().height,
        );
        Tile {
          planes: {
            let sub_plane = |pli: usize| {
              let plane = &self.planes[pli];
              let &PlaneConfig { xdec, ydec, .. } = plane.plane_cfg;
              let rect = tile_rect.decimated(xdec, ydec);
              if !plane.is_null() {
                assert!(rect.x >= 0 && rect.x as usize <= plane.rect().width);
                assert!(rect.y >= 0 && rect.y as usize <= plane.rect().height);
                assert!(rect.x as usize + rect.width <=
                        plane.rect().x as usize + plane.rect().width);
                assert!(rect.y as usize + rect.height <=
                        plane.rect().y as usize + plane.rect().height);
              }
              plane.subregion(rect.to_area())
            };
            [sub_plane(0), sub_plane(1), sub_plane(2)]
          },
        }
      }

      // Return an equivalent Tile with origin homed to 0,0.  Data
      // pointer is not moved (0,0 points to the same pixel previously
      // pointed to by old x,y).
      #[inline(always)]
      pub fn home(&self) -> Self {
        Self {
          planes: [
            self.planes[0].home(),
            self.planes[1].home(),
            self.planes[2].home(),
          ],
        }
      }

      // Return a copy of this tile's contents in a new backing frame.
      #[inline(always)]
      pub(crate) fn scratch_copy(&self) -> Frame<T> {
        Frame {
          planes: [
            self.planes[0].scratch_copy(),
            self.planes[1].scratch_copy(),
            self.planes[2].scratch_copy(),
          ],
        }
      }
    }
  }
}

tile_common!(Tile, PlaneRegion, iter);
tile_common!(TileMut, PlaneRegionMut, iter_mut, mut);

impl<T: Pixel> TileMut<'_, T> {
  #[inline(always)]
  pub fn as_const(&self) -> Tile<'_, T> {
    Tile {
      planes: [
        self.planes[0].as_const(),
        self.planes[1].as_const(),
        self.planes[2].as_const(),
      ],
    }
  }
}

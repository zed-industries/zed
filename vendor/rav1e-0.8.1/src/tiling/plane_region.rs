// Copyright (c) 2019-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

#![allow(clippy::iter_nth_zero)]

use crate::context::*;
use crate::frame::*;
use crate::util::*;

use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use std::slice;

/// Rectangle of a plane region, in pixels
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Rect {
  // coordinates relative to the plane origin (xorigin, yorigin)
  pub x: isize,
  pub y: isize,
  pub width: usize,
  pub height: usize,
}

impl Rect {
  #[inline(always)]
  pub const fn decimated(&self, xdec: usize, ydec: usize) -> Self {
    Self {
      x: self.x >> xdec,
      y: self.y >> ydec,
      width: self.width >> xdec,
      height: self.height >> ydec,
    }
  }
  pub const fn to_area(&self) -> Area {
    Area::Rect { x: self.x, y: self.y, width: self.width, height: self.height }
  }
}

// Structure to describe a rectangle area in several ways
//
// To retrieve a subregion from a region, we need to provide the subregion
// bounds, relative to its parent region. The subregion must always be included
// in its parent region.
//
// For that purpose, we could just use a rectangle (x, y, width, height), but
// this would be too cumbersome to use in practice. For example, we often need
// to pass a subregion from an offset, using the same bottom-right corner as
// its parent, or to pass a subregion expressed in block offset instead of
// pixel offset.
//
// Area provides a flexible way to describe a subregion.
#[derive(Debug, Clone, Copy)]
pub enum Area {
  /// A well-defined rectangle
  Rect { x: isize, y: isize, width: usize, height: usize },
  /// A rectangle starting at offset (x, y) and ending at the bottom-right
  /// corner of the parent
  StartingAt { x: isize, y: isize },
  /// A well-defined rectangle with offset expressed in blocks
  BlockRect { bo: BlockOffset, width: usize, height: usize },
  /// a rectangle starting at given block offset until the bottom-right corner
  /// of the parent
  BlockStartingAt { bo: BlockOffset },
}

impl Area {
  #[inline(always)]
  /// Convert to a rectangle of pixels.
  /// For a `BlockRect` and `BlockStartingAt`, for subsampled chroma planes,
  /// the returned rect will be aligned to a 4x4 chroma block.
  /// This is necessary for `compute_distortion` and `rdo_cfl_alpha` as
  /// the subsampled chroma block covers multiple luma blocks.
  pub const fn to_rect(
    &self, xdec: usize, ydec: usize, parent_width: usize, parent_height: usize,
  ) -> Rect {
    match *self {
      Area::Rect { x, y, width, height } => Rect { x, y, width, height },
      Area::StartingAt { x, y } => Rect {
        x,
        y,
        width: (parent_width as isize - x) as usize,
        height: (parent_height as isize - y) as usize,
      },
      Area::BlockRect { bo, width, height } => Rect {
        x: (bo.x >> xdec << BLOCK_TO_PLANE_SHIFT) as isize,
        y: (bo.y >> ydec << BLOCK_TO_PLANE_SHIFT) as isize,
        width,
        height,
      },
      Area::BlockStartingAt { bo } => {
        let x = (bo.x >> xdec << BLOCK_TO_PLANE_SHIFT) as isize;
        let y = (bo.y >> ydec << BLOCK_TO_PLANE_SHIFT) as isize;
        Rect {
          x,
          y,
          width: (parent_width as isize - x) as usize,
          height: (parent_height as isize - y) as usize,
        }
      }
    }
  }
}

/// Bounded region of a plane
///
/// This allows to give access to a rectangular area of a plane without
/// giving access to the whole plane.
#[derive(Debug)]
pub struct PlaneRegion<'a, T: Pixel> {
  data: *const T, // points to (plane_cfg.x, plane_cfg.y)
  pub plane_cfg: &'a PlaneConfig,
  // private to guarantee borrowing rules
  rect: Rect,
  phantom: PhantomData<&'a T>,
}

/// Mutable bounded region of a plane
///
/// This allows to give mutable access to a rectangular area of the plane
/// without giving access to the whole plane.
#[derive(Debug)]
pub struct PlaneRegionMut<'a, T: Pixel> {
  data: *mut T, // points to (plane_cfg.x, plane_cfg.y)
  pub plane_cfg: &'a PlaneConfig,
  rect: Rect,
  phantom: PhantomData<&'a mut T>,
}

// common impl for PlaneRegion and PlaneRegionMut
macro_rules! plane_region_common {
  // $name: PlaneRegion or PlaneRegionMut
  // $as_ptr: as_ptr or as_mut_ptr
  // $opt_mut: nothing or mut
  ($name:ident, $as_ptr:ident $(,$opt_mut:tt)?) => {
    impl<'a, T: Pixel> $name<'a, T> {
      #[inline(always)]
      pub fn is_null(&self) -> bool {
        self.data.is_null()
      }

      #[cold]
      pub fn empty(plane_cfg : &'a PlaneConfig) -> Self {
        return Self {
          // SAFETY: This is actually pretty unsafe.
          // This means we need to ensure that no other method on this struct
          // can access data if the dimensions are 0.
          data: unsafe { std::ptr::null_mut::<T>() },
          plane_cfg,
          rect: Rect::default(),
          phantom: PhantomData,
        }
      }

      /// # Panics
      ///
      /// - If the configured dimensions are invalid
      #[inline(always)]
      pub fn from_slice(data: &'a $($opt_mut)? [T], cfg: &'a PlaneConfig, rect: Rect) -> Self {
        if cfg.width == 0 || cfg.height == 0 {
          return Self::empty(&cfg);
        }
        assert!(rect.x >= -(cfg.xorigin as isize));
        assert!(rect.y >= -(cfg.yorigin as isize));
        assert!(cfg.xorigin as isize + rect.x + rect.width as isize <= cfg.stride as isize);
        assert!(cfg.yorigin as isize + rect.y + rect.height as isize <= cfg.alloc_height as isize);

        // SAFETY: The above asserts ensure we do not go OOB.
        unsafe { Self::from_slice_unsafe(data, cfg, rect)}
      }

      #[inline(always)]
      pub unsafe fn from_slice_unsafe(data: &'a $($opt_mut)? [T], cfg: &'a PlaneConfig, rect: Rect) -> Self {
        debug_assert!(rect.x >= -(cfg.xorigin as isize));
        debug_assert!(rect.y >= -(cfg.yorigin as isize));
        debug_assert!(cfg.xorigin as isize + rect.x + rect.width as isize <= cfg.stride as isize);
        debug_assert!(cfg.yorigin as isize + rect.y + rect.height as isize <= cfg.alloc_height as isize);

        let origin = (cfg.yorigin as isize + rect.y) * cfg.stride as isize + cfg.xorigin as isize + rect.x;
        Self {
          data: data.$as_ptr().offset(origin),
          plane_cfg: cfg,
          rect,
          phantom: PhantomData,
        }
      }

      #[inline(always)]
      pub fn new(plane: &'a $($opt_mut)? Plane<T>, rect: Rect) -> Self {
        Self::from_slice(& $($opt_mut)? plane.data, &plane.cfg, rect)
      }

      #[inline(always)]
      pub fn new_from_plane(plane: &'a $($opt_mut)? Plane<T>) -> Self {
        let rect = Area::StartingAt { x: 0, y: 0 }.to_rect(
          plane.cfg.xdec,
          plane.cfg.ydec,
          plane.cfg.stride - plane.cfg.xorigin,
          plane.cfg.alloc_height - plane.cfg.yorigin,
        );

        // SAFETY: Area::StartingAt{}.to_rect is guaranteed to be the entire plane
        unsafe { Self::from_slice_unsafe(& $($opt_mut)? plane.data, &plane.cfg, rect) }
      }

      #[inline(always)]
      pub fn data_ptr(&self) -> *const T {
        self.data
      }

      #[inline(always)]
      pub fn rect(&self) -> &Rect {
        &self.rect
      }

      #[inline(always)]
      pub fn rows_iter(&self) -> RowsIter<'_, T> {
        RowsIter {
          data: self.data,
          stride: self.plane_cfg.stride,
          width: self.rect.width,
          remaining: self.rect.height,
          phantom: PhantomData,
        }
      }

      pub fn vert_windows(&self, h: usize) -> VertWindows<'_, T> {
        VertWindows {
          data: self.data,
          plane_cfg: self.plane_cfg,
          remaining: (self.rect.height as isize - h as isize + 1).max(0) as usize,
          output_rect: Rect {
            x: self.rect.x,
            y: self.rect.y,
            width: self.rect.width,
            height: h
          }
        }
      }

      pub fn horz_windows(&self, w: usize) -> HorzWindows<'_, T> {
        HorzWindows {
          data: self.data,
          plane_cfg: self.plane_cfg,
          remaining: (self.rect.width as isize - w as isize + 1).max(0) as usize,
          output_rect: Rect {
            x: self.rect.x,
            y: self.rect.y,
            width: w,
            height: self.rect.height
          }
        }
      }

      /// Return a view to a subregion of the plane
      ///
      /// The subregion must be included in (i.e. must not exceed) this region.
      ///
      /// It is described by an `Area`, relative to this region.
      ///
      /// # Panics
      ///
      /// - If the requested dimensions are larger than the plane region size
      ///
      /// # Example
      ///
      /// ``` ignore
      /// # use rav1e::tiling::*;
      /// # fn f(region: &PlaneRegion<'_, u16>) {
      /// // a subregion from (10, 8) to the end of the region
      /// let subregion = region.subregion(Area::StartingAt { x: 10, y: 8 });
      /// # }
      /// ```
      ///
      /// ``` ignore
      /// # use rav1e::context::*;
      /// # use rav1e::tiling::*;
      /// # fn f(region: &PlaneRegion<'_, u16>) {
      /// // a subregion from the top-left of block (2, 3) having size (64, 64)
      /// let bo = BlockOffset { x: 2, y: 3 };
      /// let subregion = region.subregion(Area::BlockRect { bo, width: 64, height: 64 });
      /// # }
      /// ```
      #[inline(always)]
      pub fn subregion(&self, area: Area) -> PlaneRegion<'_, T> {
        if self.data.is_null() {
          return PlaneRegion::empty(&self.plane_cfg);
        }
        let rect = area.to_rect(
          self.plane_cfg.xdec,
          self.plane_cfg.ydec,
          self.rect.width,
          self.rect.height,
        );
        assert!(rect.x >= 0 && rect.x as usize <= self.rect.width);
        assert!(rect.y >= 0 && rect.y as usize <= self.rect.height);
        // SAFETY: The above asserts ensure we do not go outside the original rectangle.
        let data = unsafe {
          self.data.add(rect.y as usize * self.plane_cfg.stride + rect.x as usize)
        };
        let absolute_rect = Rect {
          x: self.rect.x + rect.x,
          y: self.rect.y + rect.y,
          width: rect.width,
          height: rect.height,
        };
        PlaneRegion {
          data,
          plane_cfg: &self.plane_cfg,
          rect: absolute_rect,
          phantom: PhantomData,
        }
      }

      // Return an equivalent PlaneRegion with origin homed to 0,0.  Data
      // pointer is not moved (0,0 points to the same pixel previously
      // pointed to by old x,y).
      #[inline(always)]
      pub fn home(&self) -> Self {
        let home_rect = Rect {
          x: 0,
          y: 0,
          width: self.rect.width,
          height: self.rect.height,
        };

        Self {
          data: self.data,
          plane_cfg: &self.plane_cfg,
          rect: home_rect,
          phantom: PhantomData,
        }
      }

      #[inline(always)]
      pub fn to_frame_plane_offset(&self, tile_po: PlaneOffset) -> PlaneOffset {
        PlaneOffset {
          x: self.rect.x + tile_po.x,
          y: self.rect.y + tile_po.y,
        }
      }

      #[inline(always)]
      pub fn to_frame_block_offset(&self, tile_bo: TileBlockOffset) -> PlaneBlockOffset {
        debug_assert!(self.rect.x >= 0);
        debug_assert!(self.rect.y >= 0);
        let PlaneConfig { xdec, ydec, .. } = self.plane_cfg;
        debug_assert!(self.rect.x as usize % (MI_SIZE >> xdec) == 0);
        debug_assert!(self.rect.y as usize % (MI_SIZE >> ydec) == 0);
        let bx = self.rect.x as usize >> MI_SIZE_LOG2 - xdec;
        let by = self.rect.y as usize >> MI_SIZE_LOG2 - ydec;
        PlaneBlockOffset(BlockOffset {
          x: bx + tile_bo.0.x,
          y: by + tile_bo.0.y,
        })
      }

      #[inline(always)]
      pub fn to_frame_super_block_offset(
        &self,
        tile_sbo: TileSuperBlockOffset,
        sb_size_log2: usize
      ) -> PlaneSuperBlockOffset {
        debug_assert!(sb_size_log2 == 6 || sb_size_log2 == 7);
        debug_assert!(self.rect.x >= 0);
        debug_assert!(self.rect.y >= 0);
        let PlaneConfig { xdec, ydec, .. } = self.plane_cfg;
        debug_assert!(self.rect.x as usize % (1 << sb_size_log2 - xdec) == 0);
        debug_assert!(self.rect.y as usize % (1 << sb_size_log2 - ydec) == 0);
        let sbx = self.rect.x as usize >> sb_size_log2 - xdec;
        let sby = self.rect.y as usize >> sb_size_log2 - ydec;
        PlaneSuperBlockOffset(SuperBlockOffset {
          x: sbx + tile_sbo.0.x,
          y: sby + tile_sbo.0.y,
        })
      }

      /// Returns the frame block offset of the subregion.
      #[inline(always)]
      pub fn frame_block_offset(&self) -> PlaneBlockOffset {
        self.to_frame_block_offset(TileBlockOffset(BlockOffset { x: 0, y: 0 }))
      }

      pub(crate) fn scratch_copy(&self) -> Plane<T> {
        let &Rect { width, height, .. } = self.rect();
        let &PlaneConfig { xdec, ydec, .. } = self.plane_cfg;
        let mut ret: Plane<T> = Plane::new(width, height, xdec, ydec, 0, 0);
        let mut dst: PlaneRegionMut<T> = ret.as_region_mut();
        for (dst_row, src_row) in dst.rows_iter_mut().zip(self.rows_iter()) {
          for (out, input) in dst_row.iter_mut().zip(src_row) {
            *out = *input;
          }
        }
        ret
      }
    }

    unsafe impl<T: Pixel> Send for $name<'_, T> {}
    unsafe impl<T: Pixel> Sync for $name<'_, T> {}

    impl<T: Pixel> Index<usize> for $name<'_, T> {
      type Output = [T];

      #[inline(always)]
      fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.rect.height);
        // SAFETY: The above assert ensures we do not access OOB data.
        unsafe {
          let ptr = self.data.add(index * self.plane_cfg.stride);
          slice::from_raw_parts(ptr, self.rect.width)
        }
      }
    }
  }
}

plane_region_common!(PlaneRegion, as_ptr);
plane_region_common!(PlaneRegionMut, as_mut_ptr, mut);

impl<T: Pixel> PlaneRegionMut<'_, T> {
  #[inline(always)]
  pub fn data_ptr_mut(&mut self) -> *mut T {
    self.data
  }

  #[inline(always)]
  pub fn rows_iter_mut(&mut self) -> RowsIterMut<'_, T> {
    RowsIterMut {
      data: self.data,
      stride: self.plane_cfg.stride,
      width: self.rect.width,
      remaining: self.rect.height,
      phantom: PhantomData,
    }
  }

  /// Return a mutable view to a subregion of the plane
  ///
  /// The subregion must be included in (i.e. must not exceed) this region.
  ///
  /// It is described by an `Area`, relative to this region.
  ///
  /// # Panics
  ///
  /// - If the targeted `area` is outside of the bounds of this plane region.
  ///
  /// # Example
  ///
  /// ``` ignore
  /// # use rav1e::tiling::*;
  /// # fn f(region: &mut PlaneRegionMut<'_, u16>) {
  /// // a mutable subregion from (10, 8) having size (32, 32)
  /// let subregion = region.subregion_mut(Area::Rect { x: 10, y: 8, width: 32, height: 32 });
  /// # }
  /// ```
  ///
  /// ``` ignore
  /// # use rav1e::context::*;
  /// # use rav1e::tiling::*;
  /// # fn f(region: &mut PlaneRegionMut<'_, u16>) {
  /// // a mutable subregion from the top-left of block (2, 3) to the end of the region
  /// let bo = BlockOffset { x: 2, y: 3 };
  /// let subregion = region.subregion_mut(Area::BlockStartingAt { bo });
  /// # }
  /// ```
  #[inline(always)]
  pub fn subregion_mut(&mut self, area: Area) -> PlaneRegionMut<'_, T> {
    let rect = area.to_rect(
      self.plane_cfg.xdec,
      self.plane_cfg.ydec,
      self.rect.width,
      self.rect.height,
    );
    assert!(rect.x >= 0 && rect.x as usize <= self.rect.width);
    assert!(rect.y >= 0 && rect.y as usize <= self.rect.height);
    // SAFETY: The above asserts ensure we do not go outside the original rectangle.
    let data = unsafe {
      self.data.add(rect.y as usize * self.plane_cfg.stride + rect.x as usize)
    };
    let absolute_rect = Rect {
      x: self.rect.x + rect.x,
      y: self.rect.y + rect.y,
      width: rect.width,
      height: rect.height,
    };
    PlaneRegionMut {
      data,
      plane_cfg: self.plane_cfg,
      rect: absolute_rect,
      phantom: PhantomData,
    }
  }

  #[inline(always)]
  pub fn as_const(&self) -> PlaneRegion<'_, T> {
    PlaneRegion {
      data: self.data,
      plane_cfg: self.plane_cfg,
      rect: self.rect,
      phantom: PhantomData,
    }
  }
}

impl<T: Pixel> IndexMut<usize> for PlaneRegionMut<'_, T> {
  #[inline(always)]
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    assert!(index < self.rect.height);
    // SAFETY: The above assert ensures we do not access OOB data.
    unsafe {
      let ptr = self.data.add(index * self.plane_cfg.stride);
      slice::from_raw_parts_mut(ptr, self.rect.width)
    }
  }
}

/// Iterator over plane region rows
pub struct RowsIter<'a, T: Pixel> {
  data: *const T,
  stride: usize,
  width: usize,
  remaining: usize,
  phantom: PhantomData<&'a T>,
}

/// Mutable iterator over plane region rows
pub struct RowsIterMut<'a, T: Pixel> {
  data: *mut T,
  stride: usize,
  width: usize,
  remaining: usize,
  phantom: PhantomData<&'a mut T>,
}

impl<'a, T: Pixel> Iterator for RowsIter<'a, T> {
  type Item = &'a [T];

  #[inline(always)]
  fn next(&mut self) -> Option<Self::Item> {
    if self.remaining > 0 {
      // SAFETY: We verified that we have enough data left to not go OOB,
      // assuming that `self.stride` and `self.width` are set correctly.
      let row = unsafe {
        let ptr = self.data;
        self.data = self.data.add(self.stride);
        slice::from_raw_parts(ptr, self.width)
      };
      self.remaining -= 1;
      Some(row)
    } else {
      None
    }
  }

  #[inline(always)]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.remaining, Some(self.remaining))
  }
}

impl<'a, T: Pixel> Iterator for RowsIterMut<'a, T> {
  type Item = &'a mut [T];

  #[inline(always)]
  fn next(&mut self) -> Option<Self::Item> {
    if self.remaining > 0 {
      // SAFETY: We verified that we have enough data left to not go OOB,
      // assuming that `self.stride` and `self.width` are set correctly.
      let row = unsafe {
        let ptr = self.data;
        self.data = self.data.add(self.stride);
        slice::from_raw_parts_mut(ptr, self.width)
      };
      self.remaining -= 1;
      Some(row)
    } else {
      None
    }
  }

  #[inline(always)]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.remaining, Some(self.remaining))
  }
}

impl<T: Pixel> ExactSizeIterator for RowsIter<'_, T> {}
impl<T: Pixel> FusedIterator for RowsIter<'_, T> {}
impl<T: Pixel> ExactSizeIterator for RowsIterMut<'_, T> {}
impl<T: Pixel> FusedIterator for RowsIterMut<'_, T> {}

pub struct VertWindows<'a, T: Pixel> {
  data: *const T,
  plane_cfg: &'a PlaneConfig,
  remaining: usize,
  output_rect: Rect,
}

pub struct HorzWindows<'a, T: Pixel> {
  data: *const T,
  plane_cfg: &'a PlaneConfig,
  remaining: usize,
  output_rect: Rect,
}

impl<'a, T: Pixel> Iterator for VertWindows<'a, T> {
  type Item = PlaneRegion<'a, T>;

  #[inline(always)]
  fn next(&mut self) -> Option<Self::Item> {
    self.nth(0)
  }

  #[inline(always)]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.remaining, Some(self.remaining))
  }

  #[inline(always)]
  fn nth(&mut self, n: usize) -> Option<Self::Item> {
    if self.remaining > n {
      // SAFETY: We verified that we have enough data left to not go OOB.
      self.data = unsafe { self.data.add(self.plane_cfg.stride * n) };
      self.output_rect.y += n as isize;
      let output = PlaneRegion {
        data: self.data,
        plane_cfg: self.plane_cfg,
        rect: self.output_rect,
        phantom: PhantomData,
      };
      // SAFETY: We verified that we have enough data left to not go OOB.
      self.data = unsafe { self.data.add(self.plane_cfg.stride) };
      self.output_rect.y += 1;
      self.remaining -= (n + 1);
      Some(output)
    } else {
      None
    }
  }
}

impl<'a, T: Pixel> Iterator for HorzWindows<'a, T> {
  type Item = PlaneRegion<'a, T>;

  #[inline(always)]
  fn next(&mut self) -> Option<Self::Item> {
    self.nth(0)
  }

  #[inline(always)]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.remaining, Some(self.remaining))
  }

  #[inline(always)]
  fn nth(&mut self, n: usize) -> Option<Self::Item> {
    if self.remaining > n {
      // SAFETY: We verified that we have enough data left to not go OOB.
      self.data = unsafe { self.data.add(n) };
      self.output_rect.x += n as isize;
      let output = PlaneRegion {
        data: self.data,
        plane_cfg: self.plane_cfg,
        rect: self.output_rect,
        phantom: PhantomData,
      };
      // SAFETY: We verified that we have enough data left to not go OOB.
      self.data = unsafe { self.data.add(1) };
      self.output_rect.x += 1;
      self.remaining -= (n + 1);
      Some(output)
    } else {
      None
    }
  }
}

impl<T: Pixel> ExactSizeIterator for VertWindows<'_, T> {}
impl<T: Pixel> FusedIterator for VertWindows<'_, T> {}
impl<T: Pixel> ExactSizeIterator for HorzWindows<'_, T> {}
impl<T: Pixel> FusedIterator for HorzWindows<'_, T> {}

#[test]
fn area_test() {
  assert_eq!(
    (Area::BlockStartingAt { bo: BlockOffset { x: 0, y: 0 } })
      .to_rect(0, 0, 100, 100),
    Rect { x: 0, y: 0, width: 100, height: 100 }
  );
  assert_eq!(
    (Area::BlockStartingAt { bo: BlockOffset { x: 1, y: 1 } })
      .to_rect(0, 0, 100, 100),
    Rect { x: 4, y: 4, width: 96, height: 96 }
  );
  assert_eq!(
    (Area::BlockStartingAt { bo: BlockOffset { x: 1, y: 1 } })
      .to_rect(1, 1, 50, 50),
    Rect { x: 0, y: 0, width: 50, height: 50 }
  );
  assert_eq!(
    (Area::BlockStartingAt { bo: BlockOffset { x: 2, y: 2 } })
      .to_rect(1, 1, 50, 50),
    Rect { x: 4, y: 4, width: 46, height: 46 }
  );
  assert_eq!(
    (Area::BlockRect { bo: BlockOffset { x: 0, y: 0 }, width: 1, height: 1 })
      .to_rect(0, 0, 100, 100),
    Rect { x: 0, y: 0, width: 1, height: 1 }
  );
  assert_eq!(
    (Area::BlockRect { bo: BlockOffset { x: 1, y: 1 }, width: 1, height: 1 })
      .to_rect(0, 0, 100, 100),
    Rect { x: 4, y: 4, width: 1, height: 1 }
  );
  assert_eq!(
    (Area::BlockRect { bo: BlockOffset { x: 1, y: 1 }, width: 1, height: 1 })
      .to_rect(1, 1, 50, 50),
    Rect { x: 0, y: 0, width: 1, height: 1 }
  );
  assert_eq!(
    (Area::BlockRect { bo: BlockOffset { x: 2, y: 2 }, width: 1, height: 1 })
      .to_rect(1, 1, 50, 50),
    Rect { x: 4, y: 4, width: 1, height: 1 }
  );
}

#[test]
fn frame_block_offset() {
  {
    let p = Plane::<u8>::new(100, 100, 0, 0, 0, 0);
    let pr =
      PlaneRegion::new(&p, Rect { x: 0, y: 0, width: 100, height: 100 });
    let bo = BlockOffset { x: 0, y: 0 };
    assert_eq!(
      pr.to_frame_block_offset(TileBlockOffset(bo)),
      PlaneBlockOffset(bo)
    );
    assert_eq!(
      pr.to_frame_block_offset(TileBlockOffset(bo)),
      pr.subregion(Area::BlockStartingAt { bo }).frame_block_offset()
    );
  }
  {
    let p = Plane::<u8>::new(100, 100, 0, 0, 0, 0);
    let pr =
      PlaneRegion::new(&p, Rect { x: 0, y: 0, width: 100, height: 100 });
    let bo = BlockOffset { x: 1, y: 1 };
    assert_eq!(
      pr.to_frame_block_offset(TileBlockOffset(bo)),
      PlaneBlockOffset(bo)
    );
    assert_eq!(
      pr.to_frame_block_offset(TileBlockOffset(bo)),
      pr.subregion(Area::BlockStartingAt { bo }).frame_block_offset()
    );
  }
  {
    let p = Plane::<u8>::new(100, 100, 1, 1, 0, 0);
    let pr =
      PlaneRegion::new(&p, Rect { x: 0, y: 0, width: 100, height: 100 });
    let bo = BlockOffset { x: 1, y: 1 };
    assert_eq!(
      pr.to_frame_block_offset(TileBlockOffset(bo)),
      PlaneBlockOffset(bo)
    );
  }
  {
    let p = Plane::<u8>::new(100, 100, 1, 1, 0, 0);
    let pr =
      PlaneRegion::new(&p, Rect { x: 0, y: 0, width: 100, height: 100 });
    let bo = BlockOffset { x: 2, y: 2 };
    assert_eq!(
      pr.to_frame_block_offset(TileBlockOffset(bo)),
      PlaneBlockOffset(bo)
    );
    assert_eq!(
      pr.to_frame_block_offset(TileBlockOffset(bo)),
      pr.subregion(Area::BlockStartingAt { bo }).frame_block_offset()
    );
  }
}

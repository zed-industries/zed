use std::{
    iter::FusedIterator,
    marker::PhantomData,
    ops::{Index, IndexMut},
    slice,
};

use v_frame::{
    pixel::Pixel,
    plane::{Plane, PlaneConfig, PlaneOffset},
};

use super::block::{BlockOffset, BLOCK_TO_PLANE_SHIFT};

/// Bounded region of a plane
///
/// This allows giving access to a rectangular area of a plane without
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
      #[cold]
      pub fn empty(plane_cfg : &'a PlaneConfig) -> Self {
        return Self {
          // SAFETY: This is actually pretty unsafe.
          // This means we need to ensure that no other method on this struct
          // can access data if the dimensions are 0.
          data: std::ptr::null_mut::<T>(),
          plane_cfg,
          rect: Rect::default(),
          phantom: PhantomData,
        }
      }

      /// # Panics
      ///
      /// - If the configured dimensions are invalid
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

      unsafe fn from_slice_unsafe(data: &'a $($opt_mut)? [T], cfg: &'a PlaneConfig, rect: Rect) -> Self {
        let origin = (cfg.yorigin as isize + rect.y) * cfg.stride as isize + cfg.xorigin as isize + rect.x;
        Self {
          data: data.$as_ptr().offset(origin),
          plane_cfg: cfg,
          rect,
          phantom: PhantomData,
        }
      }

      pub fn new(plane: &'a $($opt_mut)? Plane<T>, rect: Rect) -> Self {
        Self::from_slice(& $($opt_mut)? plane.data, &plane.cfg, rect)
      }

      #[allow(dead_code)]
      pub fn new_from_plane(plane: &'a $($opt_mut)? Plane<T>) -> Self {
        let rect = Rect {
            x: 0,
            y: 0,
            width: plane.cfg.stride - plane.cfg.xorigin,
            height: plane.cfg.alloc_height - plane.cfg.yorigin,
        };

        // SAFETY: Area::StartingAt{}.to_rect is guaranteed to be the entire plane
        unsafe { Self::from_slice_unsafe(& $($opt_mut)? plane.data, &plane.cfg, rect) }
      }

      #[allow(dead_code)]
      pub fn data_ptr(&self) -> *const T {
        self.data
      }

      pub fn rect(&self) -> &Rect {
        &self.rect
      }

      #[allow(dead_code)]
      pub fn rows_iter(&self) -> PlaneRegionRowsIter<'_, T> {
        PlaneRegionRowsIter {
          data: self.data,
          stride: self.plane_cfg.stride,
          width: self.rect.width,
          remaining: self.rect.height,
          phantom: PhantomData,
        }
      }

      #[allow(dead_code)]
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

      #[allow(dead_code)]
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
      #[allow(dead_code)]
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
    }

    unsafe impl<T: Pixel> Send for $name<'_, T> {}
    unsafe impl<T: Pixel> Sync for $name<'_, T> {}

    impl<T: Pixel> Index<usize> for $name<'_, T> {
      type Output = [T];

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
    pub fn data_ptr_mut(&mut self) -> *mut T {
        self.data
    }

    pub fn rows_iter_mut(&mut self) -> PlaneRegionRowsIterMut<'_, T> {
        PlaneRegionRowsIterMut {
            data: self.data,
            stride: self.plane_cfg.stride,
            width: self.rect.width,
            remaining: self.rect.height,
            phantom: PhantomData,
        }
    }

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
pub struct PlaneRegionRowsIter<'a, T: Pixel> {
    data: *const T,
    stride: usize,
    width: usize,
    remaining: usize,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: Pixel> Iterator for PlaneRegionRowsIter<'a, T> {
    type Item = &'a [T];

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

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

/// Mutable iterator over plane region rows
pub struct PlaneRegionRowsIterMut<'a, T: Pixel> {
    data: *mut T,
    stride: usize,
    width: usize,
    remaining: usize,
    phantom: PhantomData<&'a mut T>,
}

impl<'a, T: Pixel> Iterator for PlaneRegionRowsIterMut<'a, T> {
    type Item = &'a mut [T];

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

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<T: Pixel> ExactSizeIterator for PlaneRegionRowsIter<'_, T> {
}
impl<T: Pixel> FusedIterator for PlaneRegionRowsIter<'_, T> {
}
impl<T: Pixel> ExactSizeIterator for PlaneRegionRowsIterMut<'_, T> {
}
impl<T: Pixel> FusedIterator for PlaneRegionRowsIterMut<'_, T> {
}

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

    fn next(&mut self) -> Option<Self::Item> {
        self.nth(0)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }

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
            self.remaining -= n + 1;
            Some(output)
        } else {
            None
        }
    }
}

impl<'a, T: Pixel> Iterator for HorzWindows<'a, T> {
    type Item = PlaneRegion<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.nth(0)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }

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
            self.remaining -= n + 1;
            Some(output)
        } else {
            None
        }
    }
}

impl<T: Pixel> ExactSizeIterator for VertWindows<'_, T> {
}
impl<T: Pixel> FusedIterator for VertWindows<'_, T> {
}
impl<T: Pixel> ExactSizeIterator for HorzWindows<'_, T> {
}
impl<T: Pixel> FusedIterator for HorzWindows<'_, T> {
}

/// Rectangle of a plane region, in pixels
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Rect {
    // coordinates relative to the plane origin (xorigin, yorigin)
    pub x: isize,
    pub y: isize,
    pub width: usize,
    pub height: usize,
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
    Rect(Rect),
    /// A rectangle starting at offset (x, y) and ending at the bottom-right
    /// corner of the parent
    StartingAt { x: isize, y: isize },
    /// a rectangle starting at given block offset until the bottom-right corner
    /// of the parent
    BlockStartingAt { bo: BlockOffset },
}

impl Area {
    /// Convert to a rectangle of pixels.
    /// For a `BlockRect` and `BlockStartingAt`, for subsampled chroma planes,
    /// the returned rect will be aligned to a 4x4 chroma block.
    /// This is necessary for `compute_distortion` and `rdo_cfl_alpha` as
    /// the subsampled chroma block covers multiple luma blocks.
    pub const fn to_rect(
        self,
        xdec: usize,
        ydec: usize,
        parent_width: usize,
        parent_height: usize,
    ) -> Rect {
        match self {
            Area::Rect(rect) => rect,
            Area::StartingAt { x, y } => Rect {
                x,
                y,
                width: (parent_width as isize - x) as usize,
                height: (parent_height as isize - y) as usize,
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

pub trait AsRegion<T: Pixel> {
    fn as_region(&self) -> PlaneRegion<'_, T>;
    fn region(&self, area: Area) -> PlaneRegion<'_, T>;
    fn region_mut(&mut self, area: Area) -> PlaneRegionMut<'_, T>;
}

impl<T: Pixel> AsRegion<T> for Plane<T> {
    fn as_region(&self) -> PlaneRegion<'_, T> {
        PlaneRegion::new_from_plane(self)
    }

    fn region(&self, area: Area) -> PlaneRegion<'_, T> {
        let rect = area.to_rect(
            self.cfg.xdec,
            self.cfg.ydec,
            self.cfg.stride - self.cfg.xorigin,
            self.cfg.alloc_height - self.cfg.yorigin,
        );
        PlaneRegion::new(self, rect)
    }

    fn region_mut(&mut self, area: Area) -> PlaneRegionMut<'_, T> {
        let rect = area.to_rect(
            self.cfg.xdec,
            self.cfg.ydec,
            self.cfg.stride - self.cfg.xorigin,
            self.cfg.alloc_height - self.cfg.yorigin,
        );
        PlaneRegionMut::new(self, rect)
    }
}

/// Absolute offset in blocks inside a plane, where a block is defined
/// to be an `N*N` square where `N == (1 << BLOCK_TO_PLANE_SHIFT)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlaneBlockOffset(pub BlockOffset);

impl PlaneBlockOffset {
    /// Convert to plane offset without decimation.
    pub const fn to_luma_plane_offset(self) -> PlaneOffset {
        self.0.to_luma_plane_offset()
    }
}

//! In graphics code it's very common to pass `width` and `height` along with a `Vec` of pixels,
//! all as separate arguments. This is tedious, and can lead to errors.
//!
//! This crate is a simple struct that adds dimensions to the underlying buffer. This makes it easier to correctly keep track
//! of the image size and allows passing images with just one function argument instead three or four.
//!
//! Additionally, it has a concept of a `stride`, which allows defining sub-regions of images without copying,
//! as well as handling padding (e.g. buffers for video frames may require to be a multiple of 8, regardless of logical image size).
//!
//! For convenience, there are iterators over rows or all pixels of a (sub)image and
//! pixel-based indexing directly with `img[(x,y)]` (where `x`/`y` can be `u32` as well as `usize`).
//!
//! `Img<Container>` type has aliases for common uses:
//!
//! * Owned: `ImgVec<T>` → `Img<Vec<T>>`  (use it in `struct`s and return types)
//! * Reference: `ImgRef<T>` → `Img<&[T]>` (use it in function arguments)
//! * Mutable reference: `ImgRefMut<T>` → `Img<&mut [T]>`
//!
//! It is assumed that the container is [one element per pixel](https://crates.io/crates/rgb/), e.g. `Vec<RGBA>`,
//! and _not_ a `Vec<u8>` where 4 `u8` elements are interpreted as one pixel.
//!
//!
//!  ```rust
//!  use imgref::*;
//!  # fn some_image_processing_function(img: ImgRef<u8>) -> ImgVec<u8> { img.new_buf(img.buf().to_vec()) }
//!
//!  fn main() {
//!      let img = Img::new(vec![0; 1000], 50, 20); // 1000 pixels of a 50×20 image
//!
//!      let new_image = some_image_processing_function(img.as_ref()); // Use imgvec.as_ref() instead of &imgvec for better efficiency
//!
//!      println!("New size is {}×{}", new_image.width(), new_image.height());
//!      println!("And the top left pixel is {:?}", new_image[(0u32,0u32)]);
//!
//!      let first_row_slice = &new_image[0];
//!
//!      for row in new_image.rows() {
//!          // …
//!      }
//!      for px in new_image.pixels() {
//!          // …
//!      }
//!
//!      // slice (x, y, width, height) by reference - no copy!
//!      let fragment = img.sub_image(5, 5, 15, 15);
//!
//!      //
//!      let (vec, width, height) = fragment.to_contiguous_buf();
//!  }
//!  ```

#![no_std]

extern crate alloc;
#[cfg(test)]
extern crate std;

use alloc::borrow::{Cow, ToOwned};
use alloc::vec::Vec;
use core::slice;

mod traits;

mod iter;
mod ops;
pub use iter::*;

/// Image owning its pixels.
///
/// A 2D array of pixels. The pixels are oriented top-left first and rows are `stride` pixels wide.
///
/// If size of the `buf` is larger than `width`*`height`, then any excess space is a padding (see `width_padded()`/`height_padded()`).
pub type ImgVec<Pixel> = Img<Vec<Pixel>>;

/// Reference to pixels inside another image.
/// Pass this structure by value (i.e. `ImgRef`, not `&ImgRef`).
///
/// Only `width` of pixels of every `stride` can be modified. The `buf` may be longer than `height`*`stride`, but the extra space should be ignored.
pub type ImgRef<'slice, Pixel> = Img<&'slice [Pixel]>;

/// Same as `ImgRef`, but mutable
/// Pass this structure by value (i.e. `ImgRef`, not `&ImgRef`).
///
pub type ImgRefMut<'slice, Pixel> = Img<&'slice mut [Pixel]>;

/// Additional methods that depend on buffer size
///
/// To use these methods you need:
///
/// ```rust
/// use imgref::*;
/// ```
pub trait ImgExt<Pixel> {
    /// Maximum possible width of the data, including the stride.
    ///
    /// # Panics
    ///
    /// This method may panic if the underlying buffer is not at least `height()*stride()` pixels large.
    #[cfg(feature = "deprecated")]
    fn width_padded(&self) -> usize;

    /// Height in number of full strides.
    /// If the underlying buffer is not an even multiple of strides, the last row is ignored.
    ///
    /// # Panics
    ///
    /// This method may panic if the underlying buffer is not at least `height()*stride()` pixels large.
    #[cfg(feature = "deprecated")]
    fn height_padded(&self) -> usize;

    /// Iterate over the entire buffer as rows, including all padding
    ///
    /// Rows will have up to `stride` width, but the last row may be shorter.
    #[cfg(feature = "deprecated")]
    fn rows_padded(&self) -> slice::Chunks<'_, Pixel>;

    /// Borrow the container
    fn as_ref(&self) -> ImgRef<'_, Pixel>;
}

/// Additional methods that depend on buffer size
///
/// To use these methods you need:
///
/// ```rust
/// use imgref::*;
/// ```
pub trait ImgExtMut<Pixel> {
    /// Iterate over the entire buffer as rows, including all padding
    ///
    /// Rows will have up to `stride` width, but the last row may be shorter.
    #[cfg(feature = "deprecated")]
    fn rows_padded_mut(&mut self) -> slice::ChunksMut<'_, Pixel>;

    /// Borrow the container mutably
    fn as_mut(&mut self) -> ImgRefMut<'_, Pixel>;
}

/// Basic struct used for both owned (alias `ImgVec`) and borrowed (alias `ImgRef`) image fragments.
///
/// Note: the fields are `pub` only because of borrow checker limitations. Please consider them as read-only.
#[derive(Debug, Copy, Clone)]
pub struct Img<Container> {
    /// Storage for the pixels. Usually `Vec<Pixel>` or `&[Pixel]`. See `ImgVec` and `ImgRef`.
    ///
    /// Note that future version will make this field private. Use `.rows()` and `.pixels()` iterators where possible, or `buf()`/`buf_mut()`/`into_buf()`.
    #[deprecated(note = "Don't access struct fields directly. Use buf(), buf_mut() or into_buf()")]
    #[cfg(feature = "deprecated")]
    pub buf: Container,

    #[cfg(not(feature = "deprecated"))]
    buf: Container,

    /// Number of pixels to skip in the container to advance to the next row.
    ///
    /// Note: pixels between `width` and `stride` may not be usable, and may not even exist in the last row.
    #[deprecated(note = "Don't access struct fields directly. Use stride()")]
    #[cfg(feature = "deprecated")]
    pub stride: usize,

    #[cfg(not(feature = "deprecated"))]
    stride: usize,

    /// Width of the image in pixels.
    ///
    /// Note that this isn't same as the width of the row in the `buf`, see `stride`
    #[deprecated(note = "Don't access struct fields directly. Use width()")]
    #[cfg(feature = "deprecated")]
    pub width: u32,

    #[cfg(not(feature = "deprecated"))]
    width: u32,

    /// Height of the image in pixels.
    #[deprecated(note = "Don't access struct fields directly. Use height()")]
    #[cfg(feature = "deprecated")]
    pub height: u32,

    #[cfg(not(feature = "deprecated"))]
    height: u32,
}

impl<Container> Img<Container> {
    /// Width of the image in pixels.
    ///
    /// Note that this isn't same as the width of the row in image data, see `stride()`
    #[inline(always)]
    #[allow(deprecated)]
    pub const fn width(&self) -> usize { self.width as usize }

    /// Height of the image in pixels.
    #[inline(always)]
    #[allow(deprecated)]
    pub const fn height(&self) -> usize { self.height as usize }

    /// Number of _pixels_ to skip in the container to advance to the next row.
    ///
    /// Note the last row may have fewer pixels than the stride.
    /// Some APIs use number of *bytes* for a stride. You may need to multiply this one by number of pixels.
    #[inline(always)]
    #[allow(deprecated)]
    pub const fn stride(&self) -> usize { self.stride }

    /// Immutable reference to the pixel storage. Warning: exposes stride. Use `pixels()` or `rows()` instead.
    ///
    /// See also `into_contiguous_buf()`.
    #[inline(always)]
    #[allow(deprecated)]
    pub const fn buf(&self) -> &Container { &self.buf }

    /// Mutable reference to the pixel storage. Warning: exposes stride. Use `pixels_mut()` or `rows_mut()` instead.
    ///
    /// See also `into_contiguous_buf()`.
    #[inline(always)]
    #[allow(deprecated)]
    pub fn buf_mut(&mut self) -> &mut Container { &mut self.buf }

    /// Get the pixel storage by consuming the image. Be careful about stride — see `into_contiguous_buf()` for a safe version.
    #[inline(always)]
    #[allow(deprecated)]
    pub fn into_buf(self) -> Container { self.buf }

    #[deprecated(note = "this was meant to be private, use new_buf() and/or rows()")]
    #[cfg(feature = "deprecated")]
    #[doc(hidden)]
    pub fn rows_buf<'buf, T: 'buf>(&self, buf: &'buf [T]) -> RowsIter<'buf, T> {
        self.rows_buf_internal(buf)
    }

    #[inline]
    #[track_caller]
    fn rows_buf_internal<'buf, T: 'buf>(&self, buf: &'buf [T]) -> RowsIter<'buf, T> {
        let stride = self.stride();
        debug_assert!(self.width() <= self.stride());
        debug_assert!(buf.len() >= self.width() * self.height());
        assert!(stride > 0);
        let non_padded = &buf[0..stride * self.height() + self.width() - stride];
        RowsIter {
            width: self.width(),
            inner: non_padded.chunks(stride),
        }
    }
}

impl<Pixel,Container> ImgExt<Pixel> for Img<Container> where Container: AsRef<[Pixel]> {
    #[inline(always)]
    #[cfg(feature = "deprecated")]
    fn width_padded(&self) -> usize {
        self.stride()
    }

    #[inline(always)]
    #[cfg(feature = "deprecated")]
    fn height_padded(&self) -> usize {
        let len = self.buf().as_ref().len();
        assert_eq!(0, len % self.stride());
        len / self.stride()
    }

    /// Iterate over the entire buffer as rows, including all padding
    ///
    /// Rows will have up to `stride` width, but the last row may be shorter.
    #[inline(always)]
    #[cfg(feature = "deprecated")]
    fn rows_padded(&self) -> slice::Chunks<'_, Pixel> {
        self.buf().as_ref().chunks(self.stride())
    }

    #[inline(always)]
    #[allow(deprecated)]
    fn as_ref(&self) -> ImgRef<'_, Pixel> {
        Img {
            buf: self.buf.as_ref(),
            width: self.width,
            height: self.height,
            stride: self.stride,
        }
    }
}

impl<Pixel,Container> ImgExtMut<Pixel> for Img<Container> where Container: AsMut<[Pixel]> {
    /// Iterate over the entire buffer as rows, including all padding
    ///
    /// Rows will have up to `stride` width, but the last row may be shorter.
    ///
    /// # Panics
    ///
    /// If stride is 0
    #[cfg(feature = "deprecated")]
    fn rows_padded_mut(&mut self) -> slice::ChunksMut<'_, Pixel> {
        let stride = self.stride();
        self.buf_mut().as_mut().chunks_mut(stride)
    }

    #[inline(always)]
    #[allow(deprecated)]
    fn as_mut(&mut self) -> ImgRefMut<'_, Pixel> {
        Img {
            buf: self.buf.as_mut(),
            width: self.width,
            height: self.height,
            stride: self.stride,
        }
    }
}

#[inline]
fn sub_image(left: usize, top: usize, width: usize, height: usize, stride: usize, buf_len: usize) -> (usize, usize, usize) {
    let start = stride * top + left;
    let full_strides_end = start + stride * height;
    // when left > 0 and height is full, the last line is shorter than the stride
    let end = if buf_len >= full_strides_end {
        full_strides_end
    } else {
        debug_assert!(height > 0);
        let min_strides_len = full_strides_end + width - stride;
        debug_assert!(buf_len >= min_strides_len, "the buffer is too small to fit the subimage");
        // if can't use full buffer, then shrink to min required (last line having exact width)
        min_strides_len
    };
    (start, end, stride)
}

impl<'slice, T> ImgRef<'slice, T> {
    /// Make a reference for a part of the image, without copying any pixels.
    ///
    /// # Panics
    ///
    /// It will panic if `sub_image` is outside of the image area
    /// (left + width must be <= container width, etc.)
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn sub_image(&self, left: usize, top: usize, width: usize, height: usize) -> Self {
        assert!(top + height <= self.height());
        assert!(left + width <= self.width());
        let (start, end, stride) = sub_image(left, top, width, height, self.stride(), self.buf().len());
        let buf = &self.buf()[start..end];
        Self::new_stride(buf, width, height, stride)
    }

    #[inline]
    /// Iterate over whole rows of pixels as slices
    ///
    /// # Panics
    ///
    /// If stride is 0
    ///
    /// See also `pixels()`
    pub fn rows(&self) -> RowsIter<'slice, T> {
        self.rows_buf_internal(self.buf())
    }

    /// Deprecated
    ///
    /// Note: it iterates **all** pixels in the underlying buffer, not just limited by width/height.
    #[deprecated(note = "Size of this buffer is unpredictable. Use .rows() instead")]
    #[cfg(feature = "deprecated")]
    #[doc(hidden)]
    pub fn iter(&self) -> slice::Iter<'slice, T> {
        self.buf().iter()
    }
}

impl<'a, T: Clone> ImgRef<'a, T> {
    /// Returns a reference to the buffer, width, height. Guarantees that the buffer is contiguous,
    /// i.e. it's `width*height` elements long, and `[x + y*width]` addresses each pixel.
    ///
    /// It will create a copy if the buffer isn't contiguous (width != stride).
    /// For a more efficient version, see `into_contiguous_buf()`
    #[allow(deprecated)]
    #[must_use]
    pub fn to_contiguous_buf(&self) -> (Cow<'a, [T]>, usize, usize) {
        let width = self.width();
        let height = self.height();
        let stride = self.stride();
        if width == stride {
            return (Cow::Borrowed(self.buf), width, height);
        }
        let mut buf = Vec::with_capacity(width * height);
        for row in self.rows() {
            buf.extend_from_slice(row);
        }
        (Cow::Owned(buf), width, height)
    }
}

impl<'slice, T> ImgRefMut<'slice, T> {
    /// Turn this into immutable reference, and slice a subregion of it
    #[inline]
    #[allow(deprecated)]
    #[must_use]
    pub fn sub_image(&'slice self, left: usize, top: usize, width: usize, height: usize) -> ImgRef<'slice, T> {
        self.as_ref().sub_image(left, top, width, height)
    }

    /// Slices this image reference to produce another reference to a subregion of it.
    ///
    /// Note that mutable borrows are exclusive, so it's not possible to have more than
    /// one mutable subimage at a time.
    ///
    /// ## Panics
    ///
    /// If the coordinates are out of bounds
    #[inline]
    #[allow(deprecated)]
    #[must_use]
    #[track_caller]
    pub fn sub_image_mut(&mut self, left: usize, top: usize, width: usize, height: usize) -> ImgRefMut<'_, T> {
        assert!(top + height <= self.height());
        assert!(left + width <= self.width());
        let (start, end, stride) = sub_image(left, top, width, height, self.stride(), self.buf.len());
        let buf = &mut self.buf[start..end];
        ImgRefMut::new_stride(buf, width, height, stride)
    }

    /// Transforms this image reference to refer to a subregion.
    /// 
    /// This is identical in behavior to [`ImgRefMut::sub_image_mut()`], except that it returns an
    /// [`ImgRefMut`] with the same lifetime, rather than a reborrow with a shorter lifetime.
    ///
    /// ## Panics
    ///
    /// If the coordinates are out of bounds
    #[allow(deprecated)]
    #[must_use]
    #[track_caller]
    pub fn into_sub_image_mut(self, left: usize, top: usize, width: usize, height: usize) -> Self {
        assert!(top + height <= self.height());
        assert!(left + width <= self.width());
        let (start, end, stride) = sub_image(left, top, width, height, self.stride(), self.buf.len());
        let buf = &mut self.buf[start..end];
        ImgRefMut::new_stride(buf, width, height, stride)
    }

    /// Make mutable reference immutable
    #[inline]
    #[must_use]
    pub fn as_ref(&self) -> ImgRef<'_, T> {
        self.new_buf(self.buf().as_ref())
    }
}

impl<'slice, T: Copy> ImgRef<'slice, T> {
    /// Iterate `width*height` pixels in the `Img`, ignoring padding area
    ///
    /// If you want to iterate in parallel, parallelize `rows()` instead.
    ///
    /// # Panics
    ///
    /// if width is 0
    #[inline]
    #[track_caller]
    pub fn pixels(&self) -> PixelsIter<'slice, T> {
        PixelsIter::new(*self)
    }
}

impl<'slice, T> ImgRef<'slice, T> {
    /// Iterate `width*height` pixels in the `Img`, by reference, ignoring padding area
    ///
    /// If you want to iterate in parallel, parallelize `rows()` instead.
    ///
    /// # Panics
    ///
    /// if width is 0
    #[inline]
    pub fn pixels_ref(&self) -> PixelsRefIter<'slice, T> {
        PixelsRefIter::new(*self)
    }
}

impl<T: Copy> ImgRefMut<'_, T> {
    /// # Panics
    ///
    /// If you want to iterate in parallel, parallelize `rows()` instead.
    ///
    /// if width is 0
    #[inline]
    pub fn pixels(&self) -> PixelsIter<'_, T> {
        PixelsIter::new(self.as_ref())
    }
}

impl<T> ImgRefMut<'_, T> {
    /// If you want to iterate in parallel, parallelize `rows()` instead.
    /// # Panics
    ///
    /// if width is 0
    #[inline]
    pub fn pixels_mut(&mut self) -> PixelsIterMut<'_, T> {
        PixelsIterMut::new(self)
    }
}

impl<T: Copy> ImgVec<T> {
    /// If you want to iterate in parallel, parallelize `rows()` instead.
    /// # Panics
    ///
    /// if width is 0
    #[inline]
    pub fn pixels(&self) -> PixelsIter<'_, T> {
        PixelsIter::new(self.as_ref())
    }
}

impl<T> ImgVec<T> {
    /// If you want to iterate in parallel, parallelize `rows()` instead.
    /// # Panics
    ///
    /// if width is 0
    #[inline]
    pub fn pixels_mut(&mut self) -> PixelsIterMut<'_, T> {
        PixelsIterMut::new(&mut self.as_mut())
    }
}

impl<T> ImgRefMut<'_, T> {
    /// Iterate over whole rows as slices
    ///
    /// # Panics
    ///
    /// if stride is 0
    #[inline]
    pub fn rows(&self) -> RowsIter<'_, T> {
        self.rows_buf_internal(&self.buf()[..])
    }

    /// Iterate over whole rows as slices
    ///
    /// # Panics
    ///
    /// if stride is 0
    #[inline]
    #[allow(deprecated)]
    pub fn rows_mut(&mut self) -> RowsIterMut<'_, T> {
        let stride = self.stride();
        let width = self.width();
        let height = self.height();
        let non_padded = &mut self.buf[0..stride * height + width - stride];
        RowsIterMut {
            width,
            inner: non_padded.chunks_mut(stride),
        }
    }
}

/// Deprecated. Use .`rows()` or .`pixels()` iterators which are more predictable
#[cfg(feature = "deprecated")]
impl<Container> IntoIterator for Img<Container> where Container: IntoIterator {
    type Item = Container::Item;
    type IntoIter = Container::IntoIter;
    /// Deprecated. Use .`rows()` or .`pixels()` iterators which are more predictable
    fn into_iter(self) -> Container::IntoIter {
        self.into_buf().into_iter()
    }
}

impl<T> ImgVec<T> {
    /// Create a mutable view into a region within the image. See `sub_image()` for read-only views.
    ///
    /// ## Panics
    ///
    /// If the coordinates are out of bounds
    #[allow(deprecated)]
    #[must_use]
    #[track_caller]
    pub fn sub_image_mut(&mut self, left: usize, top: usize, width: usize, height: usize) -> ImgRefMut<'_, T> {
        assert!(top + height <= self.height());
        assert!(left + width <= self.width());
        let start = self.stride * top + left;
        let min_buf_size = if self.height > 0 { self.stride * height + width - self.stride } else {0};
        let buf = &mut self.buf[start .. start + min_buf_size];
        Img::new_stride(buf, width, height, self.stride)
    }

    #[inline]
    #[must_use]
    /// Make a reference for a part of the image, without copying any pixels.
    pub fn sub_image(&self, left: usize, top: usize, width: usize, height: usize) -> ImgRef<'_, T> {
        self.as_ref().sub_image(left, top, width, height)
    }

    /// Make a reference to this image to pass it to functions without giving up ownership
    ///
    /// The reference should be passed by value (`ImgRef`, not `&ImgRef`).
    ///
    /// If you need a mutable reference, see `as_mut()` and `sub_image_mut()`
    #[inline]
    #[must_use]
    pub fn as_ref(&self) -> ImgRef<'_, T> {
        self.new_buf(self.buf().as_ref())
    }

    /// Make a mutable reference to the entire image
    ///
    /// The reference should be passed by value (`ImgRefMut`, not `&mut ImgRefMut`).
    ///
    /// See also `sub_image_mut()` and `rows_mut()`
    #[inline]
    pub fn as_mut(&mut self) -> ImgRefMut<'_, T> {
        let width = self.width();
        let height = self.height();
        let stride = self.stride();
        Img::new_stride(self.buf_mut().as_mut(), width, height, stride)
    }

    #[deprecated(note = "Size of this buffer may be unpredictable. Use .rows() instead")]
    #[cfg(feature = "deprecated")]
    #[doc(hidden)]
    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.buf().iter()
    }

    /// Iterate over rows of the image as slices
    ///
    /// Each slice is guaranteed to be exactly `width` pixels wide.
    ///
    /// This iterator is a good candidate for parallelization (e.g. rayon's `par_bridge()`)
    #[inline]
    pub fn rows(&self) -> RowsIter<'_, T> {
        self.rows_buf_internal(self.buf())
    }

    /// Iterate over rows of the image as mutable slices
    ///
    /// Each slice is guaranteed to be exactly `width` pixels wide.
    ///
    /// This iterator is a good candidate for parallelization (e.g. rayon's `par_bridge()`)
    #[inline]
    #[allow(deprecated)]
    pub fn rows_mut(&mut self) -> RowsIterMut<'_, T> {
        let stride = self.stride();
        let width = self.width();
        let height = self.height();
        let non_padded = &mut self.buf[0..stride * height + width - stride];
        RowsIterMut {
            width,
            inner: non_padded.chunks_mut(stride),
        }
    }
}

impl<Container> Img<Container> {
    /// Same as `new()`, except each row is located `stride` number of pixels after the previous one.
    ///
    /// Stride can be equal to `width` or larger. If it's larger, then pixels between end of previous row and start of the next are considered a padding, and may be ignored.
    ///
    /// The `Container` is usually a `Vec` or a slice.
    ///
    /// ## Panics
    ///
    /// If stride is 0.
    #[inline]
    #[allow(deprecated)]
    #[track_caller]
    pub fn new_stride(buf: Container, width: usize, height: usize, stride: usize) -> Self {
        assert!(stride > 0);
        assert!(stride >= width);
        debug_assert!(height < u32::MAX as usize);
        debug_assert!(width < u32::MAX as usize);
        Self {
            buf,
            width: width as u32,
            height: height as u32,
            stride,
        }
    }

    /// Create new image with `Container` (which can be `Vec`, `&[]` or something else) with given `width` and `height` in pixels.
    ///
    /// Assumes the pixels in container are contiguous, layed out row by row with `width` pixels per row and at least `height` rows.
    ///
    /// If the container is larger than `width`×`height` pixels, the extra rows are a considered a padding and may be ignored.
    #[inline]
    pub fn new(buf: Container, width: usize, height: usize) -> Self {
        Self::new_stride(buf, width, height, width)
    }
}

impl<T: Copy> Img<Vec<T>> {
    /// Returns the buffer, width, height. Guarantees that the buffer is contiguous,
    /// i.e. it's `width*height` elements long, and `[x + y*width]` addresses each pixel.
    ///
    /// Efficiently performs operation in-place. For other containers use `pixels().collect()`.
    #[allow(deprecated)]
    #[must_use]
    pub fn into_contiguous_buf(mut self) -> (Vec<T>, usize, usize) {
        let (_, w, h) = self.as_contiguous_buf();
        (self.buf, w, h)
    }

    /// Returns a reference to the buffer, width, height. Guarantees that the buffer is contiguous,
    /// i.e. it's `width*height` elements long, and `[x + y*width]` addresses each pixel.
    ///
    /// Efficiently performs operation in-place. For other containers use `pixels().collect()`.
    #[allow(deprecated)]
    #[must_use]
    pub fn as_contiguous_buf(&mut self) -> (&[T], usize, usize) {
        let width = self.width();
        let height = self.height();
        let stride = self.stride();
        if width != stride {
            for row in 1..height {
                self.buf.copy_within(row * stride .. row * stride + width, row * width);
            }
        }
        self.buf.truncate(width * height);
        (&mut self.buf, width, height)
    }
}

impl<OldContainer> Img<OldContainer> {
    /// A convenience method for creating an image of the same size and stride, but with a new buffer.
    #[inline]
    #[track_caller]
    pub fn map_buf<NewContainer, OldPixel, NewPixel, F>(self, callback: F) -> Img<NewContainer>
        where NewContainer: AsRef<[NewPixel]>, OldContainer: AsRef<[OldPixel]>, F: FnOnce(OldContainer) -> NewContainer {
        let width = self.width();
        let height = self.height();
        let stride = self.stride();
        let old_buf_len = self.buf().as_ref().len();
        #[allow(deprecated)]
        let new_buf = callback(self.buf);
        assert_eq!(old_buf_len, new_buf.as_ref().len());
        Img::new_stride(new_buf, width, height, stride)
    }

    /// A convenience method for creating an image of the same size and stride, but with a new buffer.
    #[inline]
    #[track_caller]
    pub fn new_buf<NewContainer, OldPixel, NewPixel>(&self, new_buf: NewContainer) -> Img<NewContainer>
        where NewContainer: AsRef<[NewPixel]>, OldContainer: AsRef<[OldPixel]> {
        assert_eq!(self.buf().as_ref().len(), new_buf.as_ref().len());
        Img::new_stride(new_buf, self.width(), self.height(), self.stride())
    }
}

impl<T: Clone> From<Img<Cow<'_, [T]>>> for Img<Vec<T>> {
    #[allow(deprecated)]
    fn from(img: Img<Cow<'_, [T]>>) -> Self {
        Self {
            width: img.width,
            height: img.height,
            stride: img.stride,
            buf: img.buf.into_owned(),
        }
    }
}

impl<T: Clone> From<ImgVec<T>> for Img<Cow<'static, [T]>> {
    #[allow(deprecated)]
    fn from(img: ImgVec<T>) -> Self {
        Img {
            width: img.width,
            height: img.height,
            stride: img.stride,
            buf: img.buf.into(),
        }
    }
}

impl<'a, T: Clone> From<ImgRef<'a, T>> for Img<Cow<'a, [T]>> {
    #[allow(deprecated)]
    fn from(img: ImgRef<'a, T>) -> Self {
        Img {
            buf: img.buf.into(),
            width: img.width,
            height: img.height,
            stride: img.stride,
        }
    }
}

impl<T: Clone> Img<Cow<'_, [T]>> {
    /// Convert underlying buffer to owned (e.g. slice to vec)
    ///
    /// See also `to_contiguous_buf().0.into_owned()`
    #[allow(deprecated)]
    #[must_use]
    pub fn into_owned(self) -> ImgVec<T> {
        match self.buf {
            Cow::Borrowed(_) => {
                let tmp = self.as_ref();
                let (buf, w, h) = tmp.to_contiguous_buf();
                ImgVec::new(buf.into_owned(), w, h)
            },
            Cow::Owned(buf) => Img {
                buf,
                width: self.width,
                height: self.height,
                stride: self.stride,
            },
        }
    }
}

impl<T> Img<T> where T: ToOwned {
    /// Convert underlying buffer to owned (e.g. slice to vec)
    ///
    /// See also `to_contiguous_buf().0.into_owned()`
    #[allow(deprecated)]
    pub fn to_owned(&self) -> Img<T::Owned> {
        Img {
            buf: self.buf.to_owned(),
            width: self.width,
            height: self.height,
            stride: self.stride,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    mod with_opinionated_container {
        use super::*;

        struct IDontDeriveAnything;

        #[test]
        fn compiles() {
            let _ = Img::new(IDontDeriveAnything, 1, 1);
        }
    }

    #[test]
    fn with_vec() {
        let bytes = vec![0u8;20];
        let old = Img::new_stride(bytes, 10,2,10);
        let _ = old.new_buf(vec![6u16;20]);
    }

    #[test]
    fn zero() {
        let bytes = vec![0u8];
        let mut img = Img::new_stride(bytes,0,0,1);
        let _ = img.sub_image(0,0,0,0);
        let _ = img.sub_image_mut(0,0,0,0);
        let _ = img.as_ref();
    }

    #[test]
    fn zero_width() {
        let bytes = vec![0u8];
        let mut img = Img::new_stride(bytes,0,1,1);
        let _ = img.sub_image(0,1,0,0);
        let _ = img.sub_image_mut(0,0,0,1);
    }

    #[test]
    fn zero_height() {
        let bytes = vec![0u8];
        let mut img = Img::new_stride(bytes,1,0,1);
        assert_eq!(0, img.rows().count());
        let _ = img.sub_image(1,0,0,0);
        let _ = img.sub_image_mut(0,0,1,0);
    }

    #[test]
    #[allow(deprecated)]
    fn with_slice() {
        let bytes = vec![0u8;20];
        let _ = Img::new_stride(bytes.as_slice(), 10,2,10);
        let vec = ImgVec::new_stride(bytes, 10,2,10);

        #[cfg(feature = "deprecated")]
        for _ in vec.iter() {}

        assert_eq!(2, vec.rows().count());
        for _ in *vec.as_ref().buf() {}

        #[cfg(feature = "deprecated")]
        for _ in vec {}
    }

    #[test]
    fn sub() {
        let img = Img::new_stride(vec![1,2,3,4,
                       5,6,7,8,
                       9], 3, 2, 4);
        assert_eq!(img.buf()[img.stride()], 5);
        assert_eq!(img.buf()[img.stride() + img.width()-1], 7);

        assert_eq!(img.pixels().count(), img.width() * img.height());
        assert_eq!(img.pixels().sum::<i32>(), 24);

        {
        let refimg = img.as_ref();
        let refimg2 = refimg; // Test is Copy

        // sub-image with stride hits end of the buffer
        let s1 = refimg.sub_image(1, 0, refimg.width()-1, refimg.height());
        let _ = s1.sub_image(1, 0, s1.width()-1, s1.height());

        let subimg = refimg.sub_image(1, 1, 2, 1);
        assert_eq!(subimg.pixels().count(), subimg.width() * subimg.height());

        assert_eq!(subimg.buf()[0], 6);
        assert_eq!(subimg.stride(), refimg2.stride());
        assert!(subimg.stride() * subimg.height() + subimg.width() - subimg.stride() <= subimg.buf().len());
        assert_eq!(refimg.buf()[0], 1);
        assert_eq!(1, subimg.rows().count());
        }

        // 3 different methods for constructing mutable sub-images
        let mut img_for_mut_1 = img.clone();
        let mut img_for_mut_2 = img.clone();
        let mut img_for_mut_3 = img;
        for mut subimg in [
            img_for_mut_1.sub_image_mut(1, 1, 2, 1),
            img_for_mut_2.as_mut().sub_image_mut(1, 1, 2, 1),
            img_for_mut_3.as_mut().into_sub_image_mut(1, 1, 2, 1),
        ] {
            assert_eq!(1, subimg.rows().count());
            assert_eq!(1, subimg.rows_mut().count());
            assert_eq!(1, subimg.rows_mut().rev().count());
            assert_eq!(1, subimg.rows_mut().fuse().rev().count());
            assert_eq!(subimg.buf()[0], 6);
        }
    }

    #[test]
    fn rows() {
        let img = ImgVec::new_stride(vec![0u8; 10000], 10, 15, 100);
        assert_eq!(img.height(), img.rows().count());
        assert_eq!(img.height(), img.rows().rev().count());
        assert_eq!(img.height(), img.rows().fuse().rev().count());
    }

    #[test]
    fn mut_pixels() {
        for y in 1..15 {
            for x in 1..10 {
                let mut img = ImgVec::new_stride(vec![0u8; 10000], x, y, 100);
                assert_eq!(x*y, img.pixels_mut().count());
                assert_eq!(x*y, img.as_mut().pixels().count());
                assert_eq!(x*y, img.as_mut().pixels_mut().count());
                assert_eq!(x*y, img.as_mut().as_ref().pixels().count());
            }
        }
    }

    #[test]
    fn into_contiguous_buf() {
        for in_h in [1, 2, 3, 38, 39, 40, 41].iter().copied() {
            for in_w in [1, 2, 3, 120, 121].iter().copied() {
                for stride in [in_w, 121, 122, 166, 242, 243].iter().copied() {
                    let img = ImgVec::new_stride((0..10000).map(|x| x as u8).collect(), in_w, in_h, stride)
                        .map_buf(|x| x);
                    let pixels: Vec<_> = img.pixels().collect();
                    let (buf, w, h) = img.into_contiguous_buf();
                    assert_eq!(pixels, buf);
                    assert_eq!(in_w*in_h, buf.len());
                    assert_eq!(10000, buf.capacity());
                    assert_eq!(in_w, w);
                    assert_eq!(in_h, h);
                }
            }
        }

        let img = ImgVec::new((0..55*33).map(|x| x as u8).collect(), 55, 33);
        let pixels: Vec<_> = img.pixels().collect();
        let tmp = img.as_ref();
        let (buf, ..) = tmp.to_contiguous_buf();
        assert_eq!(&pixels[..], &buf[..]);
        let (buf, ..) = img.into_contiguous_buf();
        assert_eq!(pixels, buf);
    }
}

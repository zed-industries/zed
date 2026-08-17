// Copyright (c) 2017-2021, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use std::fmt::{Debug, Display, Formatter};
use std::iter::{self, FusedIterator};
use std::marker::PhantomData;
use std::mem::size_of;
use std::ops::{Index, IndexMut, Range};

use aligned_vec::{ABox, AVec, ConstAlign};

use crate::math::*;
use crate::pixel::*;

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

/// Plane-specific configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct PlaneConfig {
    /// Data stride.
    pub stride: usize,
    /// Allocated height in pixels.
    pub alloc_height: usize,
    /// Width in pixels.
    pub width: usize,
    /// Height in pixels.
    pub height: usize,
    /// Decimator along the X axis.
    ///
    /// For example, for chroma planes in a 4:2:0 configuration this would be 1.
    pub xdec: usize,
    /// Decimator along the Y axis.
    ///
    /// For example, for chroma planes in a 4:2:0 configuration this would be 1.
    pub ydec: usize,
    /// Number of padding pixels on the right.
    pub xpad: usize,
    /// Number of padding pixels on the bottom.
    pub ypad: usize,
    /// X where the data starts.
    pub xorigin: usize,
    /// Y where the data starts.
    pub yorigin: usize,
}

impl PlaneConfig {
    /// Stride alignment in bytes.
    const STRIDE_ALIGNMENT_LOG2: usize = 6;

    #[inline]
    pub fn new(
        width: usize,
        height: usize,
        xdec: usize,
        ydec: usize,
        xpad: usize,
        ypad: usize,
        type_size: usize,
    ) -> Self {
        let xorigin = xpad.align_power_of_two(Self::STRIDE_ALIGNMENT_LOG2 + 1 - type_size);
        let yorigin = ypad;
        let stride = (xorigin + width + xpad)
            .align_power_of_two(Self::STRIDE_ALIGNMENT_LOG2 + 1 - type_size);
        let alloc_height = yorigin + height + ypad;

        PlaneConfig {
            stride,
            alloc_height,
            width,
            height,
            xdec,
            ydec,
            xpad,
            ypad,
            xorigin,
            yorigin,
        }
    }
}

/// Absolute offset in pixels inside a plane
#[derive(Clone, Copy, Debug, Default)]
pub struct PlaneOffset {
    pub x: isize,
    pub y: isize,
}

/// Backing buffer for the Plane data
///
/// The buffer is padded and aligned according to the architecture-specific
/// SIMD constraints.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct PlaneData<T: Pixel> {
    #[cfg(not(target_arch = "wasm32"))]
    data: ABox<[T], ConstAlign<{ 1 << 6 }>>,
    #[cfg(target_arch = "wasm32")]
    data: ABox<[T], ConstAlign<{ 1 << 3 }>>,
}

unsafe impl<T: Pixel + Send> Send for PlaneData<T> {}
unsafe impl<T: Pixel + Sync> Sync for PlaneData<T> {}

impl<T: Pixel> std::ops::Deref for PlaneData<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.data.as_ref()
    }
}

impl<T: Pixel> std::ops::DerefMut for PlaneData<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.data.as_mut()
    }
}

impl<T: Pixel> PlaneData<T> {
    #[cfg(target_arch = "wasm32")]
    // FIXME: wasm32 allocator fails for alignment larger than 3
    const DATA_ALIGNMENT: usize = 1 << 3;
    #[cfg(not(target_arch = "wasm32"))]
    const DATA_ALIGNMENT: usize = 1 << 6;

    pub fn new(len: usize) -> Self {
        Self {
            data: AVec::from_iter(
                Self::DATA_ALIGNMENT,
                iter::repeat(T::cast_from(128)).take(len),
            )
            .into_boxed_slice(),
        }
    }

    fn from_slice(data: &[T]) -> Self {
        Self {
            data: AVec::from_slice(Self::DATA_ALIGNMENT, data).into_boxed_slice(),
        }
    }
}

/// One data plane of a frame.
///
/// For example, a plane can be a Y luma plane or a U or V chroma plane.
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Plane<T: Pixel> {
    // TODO: it is used by encoder to copy by plane and by tiling, make it
    // private again once tiling is moved and a copy_plane fn is added.
    //
    pub data: PlaneData<T>,
    /// Plane configuration.
    pub cfg: PlaneConfig,
}

impl<T: Pixel> Debug for Plane<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Plane {{ data: [{}, ...], cfg: {:?} }}",
            self.data[0], self.cfg
        )
    }
}

impl<T: Pixel> Plane<T> {
    /// Allocates and returns a new plane.
    pub fn new(
        width: usize,
        height: usize,
        xdec: usize,
        ydec: usize,
        xpad: usize,
        ypad: usize,
    ) -> Self {
        let cfg = PlaneConfig::new(width, height, xdec, ydec, xpad, ypad, size_of::<T>());
        let data = PlaneData::new(cfg.stride * cfg.alloc_height);

        Plane { data, cfg }
    }

    /// # Panics
    ///
    /// - If `len` is not a multiple of `stride`
    pub fn from_slice(data: &[T], stride: usize) -> Self {
        let len = data.len();

        assert!(len % stride == 0);

        Self {
            data: PlaneData::from_slice(data),
            cfg: PlaneConfig {
                stride,
                alloc_height: len / stride,
                width: stride,
                height: len / stride,
                xdec: 0,
                ydec: 0,
                xpad: 0,
                ypad: 0,
                xorigin: 0,
                yorigin: 0,
            },
        }
    }

    pub fn pad(&mut self, w: usize, h: usize) {
        let xorigin = self.cfg.xorigin;
        let yorigin = self.cfg.yorigin;
        let stride = self.cfg.stride;
        let alloc_height = self.cfg.alloc_height;
        let width = (w + self.cfg.xdec) >> self.cfg.xdec;
        let height = (h + self.cfg.ydec) >> self.cfg.ydec;

        if xorigin > 0 {
            for y in 0..height {
                let base = (yorigin + y) * stride;
                let fill_val = self.data[base + xorigin];
                for val in &mut self.data[base..base + xorigin] {
                    *val = fill_val;
                }
            }
        }

        if xorigin + width < stride {
            for y in 0..height {
                let base = (yorigin + y) * stride + xorigin + width;
                let fill_val = self.data[base - 1];
                for val in &mut self.data[base..base + stride - (xorigin + width)] {
                    *val = fill_val;
                }
            }
        }

        if yorigin > 0 {
            let (top, bottom) = self.data.split_at_mut(yorigin * stride);
            let src = &bottom[..stride];
            for y in 0..yorigin {
                let dst = &mut top[y * stride..(y + 1) * stride];
                dst.copy_from_slice(src);
            }
        }

        if yorigin + height < self.cfg.alloc_height {
            let (top, bottom) = self.data.split_at_mut((yorigin + height) * stride);
            let src = &top[(yorigin + height - 1) * stride..];
            for y in 0..alloc_height - (yorigin + height) {
                let dst = &mut bottom[y * stride..(y + 1) * stride];
                dst.copy_from_slice(src);
            }
        }
    }

    /// Minimally test that the plane has been padded.
    pub fn probe_padding(&self, w: usize, h: usize) -> bool {
        let PlaneConfig {
            xorigin,
            yorigin,
            stride,
            alloc_height,
            xdec,
            ydec,
            ..
        } = self.cfg;
        let width = (w + xdec) >> xdec;
        let height = (h + ydec) >> ydec;
        let corner = (yorigin + height - 1) * stride + xorigin + width - 1;
        let corner_value = self.data[corner];

        self.data[(yorigin + height) * stride - 1] == corner_value
            && self.data[(alloc_height - 1) * stride + xorigin + width - 1] == corner_value
            && self.data[alloc_height * stride - 1] == corner_value
    }

    pub fn slice(&self, po: PlaneOffset) -> PlaneSlice<'_, T> {
        PlaneSlice {
            plane: self,
            x: po.x,
            y: po.y,
        }
    }

    pub fn mut_slice(&mut self, po: PlaneOffset) -> PlaneMutSlice<'_, T> {
        PlaneMutSlice {
            plane: self,
            x: po.x,
            y: po.y,
        }
    }

    #[inline]
    fn index(&self, x: usize, y: usize) -> usize {
        (y + self.cfg.yorigin) * self.cfg.stride + (x + self.cfg.xorigin)
    }

    /// This version of the function crops off the padding on the right side of the image
    #[inline]
    pub fn row_range_cropped(&self, x: isize, y: isize) -> Range<usize> {
        debug_assert!(self.cfg.yorigin as isize + y >= 0);
        debug_assert!(self.cfg.xorigin as isize + x >= 0);
        let base_y = (self.cfg.yorigin as isize + y) as usize;
        let base_x = (self.cfg.xorigin as isize + x) as usize;
        let base = base_y * self.cfg.stride + base_x;
        let width = (self.cfg.width as isize - x) as usize;
        base..base + width
    }

    /// This version of the function includes the padding on the right side of the image
    #[inline]
    pub fn row_range(&self, x: isize, y: isize) -> Range<usize> {
        debug_assert!(self.cfg.yorigin as isize + y >= 0);
        debug_assert!(self.cfg.xorigin as isize + x >= 0);
        let base_y = (self.cfg.yorigin as isize + y) as usize;
        let base_x = (self.cfg.xorigin as isize + x) as usize;
        let base = base_y * self.cfg.stride + base_x;
        let width = self.cfg.stride - base_x;
        base..base + width
    }

    /// Returns the pixel at the given coordinates.
    pub fn p(&self, x: usize, y: usize) -> T {
        self.data[self.index(x, y)]
    }

    /// Returns plane data starting from the origin.
    pub fn data_origin(&self) -> &[T] {
        &self.data[self.index(0, 0)..]
    }

    /// Returns mutable plane data starting from the origin.
    pub fn data_origin_mut(&mut self) -> &mut [T] {
        let i = self.index(0, 0);
        &mut self.data[i..]
    }

    /// Copies data into the plane from a pixel array.
    ///
    /// # Panics
    ///
    /// - If `source_bytewidth` does not match the generic `T` of `Plane`
    pub fn copy_from_raw_u8(
        &mut self,
        source: &[u8],
        source_stride: usize,
        source_bytewidth: usize,
    ) {
        let stride = self.cfg.stride;

        assert!(stride != 0);
        assert!(source_stride != 0);

        for (self_row, source_row) in self
            .data_origin_mut()
            .chunks_exact_mut(stride)
            .zip(source.chunks_exact(source_stride))
        {
            match source_bytewidth {
                1 => {
                    for (self_pixel, source_pixel) in self_row.iter_mut().zip(source_row.iter()) {
                        *self_pixel = T::cast_from(*source_pixel);
                    }
                }
                2 => {
                    assert!(
                        size_of::<T>() == 2,
                        "source bytewidth ({}) cannot fit in Plane<u8>",
                        source_bytewidth
                    );

                    debug_assert!(T::type_enum() == PixelType::U16);

                    // SAFETY: because of the assert it is safe to assume that T == u16
                    let self_row: &mut [u16] = unsafe { std::mem::transmute(self_row) };
                    // SAFETY: we reinterpret the slice of bytes as a slice of elements of
                    // [u8; 2] to allow for more efficient codegen with from_le_bytes
                    let source_row: &[[u8; 2]] = unsafe {
                        std::slice::from_raw_parts(source_row.as_ptr().cast(), source_row.len() / 2)
                    };

                    for (self_pixel, bytes) in self_row.iter_mut().zip(source_row) {
                        *self_pixel = u16::from_le_bytes(*bytes);
                    }
                }

                _ => {}
            }
        }
    }

    /// Copies data from a plane into a pixel array.
    ///
    /// # Panics
    ///
    /// - If `dest_bytewidth` does not match the generic `T` of `Plane`
    pub fn copy_to_raw_u8(&self, dest: &mut [u8], dest_stride: usize, dest_bytewidth: usize) {
        let stride = self.cfg.stride;
        for (self_row, dest_row) in self
            .data_origin()
            .chunks_exact(stride)
            .zip(dest.chunks_exact_mut(dest_stride))
        {
            match dest_bytewidth {
                1 => {
                    for (self_pixel, dest_pixel) in
                        self_row[..self.cfg.width].iter().zip(dest_row.iter_mut())
                    {
                        *dest_pixel = u8::cast_from(*self_pixel);
                    }
                }
                2 => {
                    assert!(
                        size_of::<T>() >= 2,
                        "dest bytewidth ({}) cannot fit in Plane<u8>",
                        dest_bytewidth
                    );

                    // SAFETY: we reinterpret the slice of bytes as a slice
                    // of [u8; 2] with half the elements
                    let dest_row: &mut [[u8; 2]] = unsafe {
                        std::slice::from_raw_parts_mut(
                            dest_row.as_mut_ptr().cast(),
                            dest_row.len() / 2,
                        )
                    };

                    for (self_pixel, bytes) in self_row[..self.cfg.width].iter().zip(dest_row) {
                        *bytes = u16::cast_from(*self_pixel).to_le_bytes();
                    }
                }

                _ => {}
            }
        }
    }

    /// Returns plane with half the resolution for width and height.
    /// Downscaled with 2x2 box filter.
    /// Padded to dimensions with `frame_width` and `frame_height`.
    ///
    /// # Panics
    ///
    /// - If the requested width and height are > half the input width or height
    pub fn downsampled(&self, frame_width: usize, frame_height: usize) -> Plane<T> {
        let src = self;
        let mut new = Plane::new(
            (src.cfg.width + 1) / 2,
            (src.cfg.height + 1) / 2,
            src.cfg.xdec + 1,
            src.cfg.ydec + 1,
            src.cfg.xpad / 2,
            src.cfg.ypad / 2,
        );

        let width = new.cfg.width;
        let height = new.cfg.height;

        assert!(width * 2 <= src.cfg.stride - src.cfg.xorigin);
        assert!(height * 2 <= src.cfg.alloc_height - src.cfg.yorigin);

        let data_origin = src.data_origin();
        for (row_idx, dst_row) in new
            .mut_slice(PlaneOffset::default())
            .rows_iter_mut()
            .enumerate()
            .take(height)
        {
            let src_top_row = &data_origin[(src.cfg.stride * row_idx * 2)..][..(2 * width)];
            let src_bottom_row =
                &data_origin[(src.cfg.stride * (row_idx * 2 + 1))..][..(2 * width)];

            for ((dst, a), b) in dst_row
                .iter_mut()
                .zip(src_top_row.chunks_exact(2))
                .zip(src_bottom_row.chunks_exact(2))
            {
                let sum = u32::cast_from(a[0])
                    + u32::cast_from(a[1])
                    + u32::cast_from(b[0])
                    + u32::cast_from(b[1]);
                let avg = (sum + 2) >> 2;
                *dst = T::cast_from(avg);
            }
        }

        new.pad(frame_width, frame_height);
        new
    }

    /// Returns a plane downscaled from the source plane by a factor of `scale` (not padded)
    pub fn downscale<const SCALE: usize>(&self) -> Plane<T> {
        let mut new_plane = Plane::new(self.cfg.width / SCALE, self.cfg.height / SCALE, 0, 0, 0, 0);

        self.downscale_in_place::<SCALE>(&mut new_plane);

        new_plane
    }

    /// Downscales the source plane by a factor of `scale`, writing the result to `in_plane` (not padded)
    ///
    /// # Panics
    ///
    /// - If the current plane's width and height are not at least `SCALE` times the `in_plane`'s
    #[cfg_attr(feature = "profiling", profiling::function(downscale_in_place))]
    pub fn downscale_in_place<const SCALE: usize>(&self, in_plane: &mut Plane<T>) {
        let stride = in_plane.cfg.stride;
        let width = in_plane.cfg.width;
        let height = in_plane.cfg.height;

        if stride == 0 || self.cfg.stride == 0 {
            panic!("stride cannot be 0");
        }

        assert!(width * SCALE <= self.cfg.stride - self.cfg.xorigin);
        assert!(height * SCALE <= self.cfg.alloc_height - self.cfg.yorigin);

        // SAFETY: Bounds checks have been removed for performance reasons
        unsafe {
            let src = self;
            let box_pixels = SCALE * SCALE;
            let half_box_pixels = box_pixels as u32 / 2; // Used for rounding int division

            let data_origin = src.data_origin();
            let plane_data_mut_slice = &mut *in_plane.data;

            // Iter dst rows
            for row_idx in 0..height {
                let dst_row = plane_data_mut_slice.get_unchecked_mut(row_idx * stride..);
                // Iter dst cols
                for (col_idx, dst) in dst_row.get_unchecked_mut(..width).iter_mut().enumerate() {
                    macro_rules! generate_inner_loop {
                        ($x:ty) => {
                            let mut sum = half_box_pixels as $x;
                            // Sum box of size scale * scale

                            // Iter src row
                            for y in 0..SCALE {
                                let src_row_idx = row_idx * SCALE + y;
                                let src_row =
                                    data_origin.get_unchecked((src_row_idx * src.cfg.stride)..);

                                // Iter src col
                                for x in 0..SCALE {
                                    let src_col_idx = col_idx * SCALE + x;
                                    sum += <$x>::cast_from(*src_row.get_unchecked(src_col_idx));
                                }
                            }

                            // Box average
                            let avg = sum as usize / box_pixels;
                            *dst = T::cast_from(avg);
                        };
                    }

                    // Use 16 bit precision if overflow would not happen
                    if T::type_enum() == PixelType::U8
                        && SCALE as u128 * SCALE as u128 * (u8::MAX as u128)
                            + half_box_pixels as u128
                            <= u16::MAX as u128
                    {
                        generate_inner_loop!(u16);
                    } else {
                        generate_inner_loop!(u32);
                    }
                }
            }
        }
    }

    /// Iterates over the pixels in the plane, skipping the padding.
    pub fn iter(&self) -> PlaneIter<'_, T> {
        PlaneIter::new(self)
    }

    /// Iterates over the lines of the plane
    pub fn rows_iter(&self) -> RowsIter<'_, T> {
        RowsIter {
            plane: self,
            x: 0,
            y: 0,
        }
    }

    pub fn rows_iter_mut(&mut self) -> RowsIterMut<'_, T> {
        RowsIterMut {
            plane: self as *mut Plane<T>,
            x: 0,
            y: 0,
            phantom: PhantomData,
        }
    }

    /// Return a line
    pub fn row(&self, y: isize) -> &[T] {
        let range = self.row_range(0, y);

        &self.data[range]
    }
}

/// Iterator over plane pixels, skipping padding.
#[derive(Debug)]
pub struct PlaneIter<'a, T: Pixel> {
    plane: &'a Plane<T>,
    y: usize,
    x: usize,
}

impl<'a, T: Pixel> PlaneIter<'a, T> {
    /// Creates a new iterator.
    pub fn new(plane: &'a Plane<T>) -> Self {
        Self { plane, y: 0, x: 0 }
    }

    fn width(&self) -> usize {
        self.plane.cfg.width
    }

    fn height(&self) -> usize {
        self.plane.cfg.height
    }
}

impl<T: Pixel> Iterator for PlaneIter<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.y == self.height() {
            return None;
        }
        let pixel = self.plane.p(self.x, self.y);
        if self.x == self.width() - 1 {
            self.x = 0;
            self.y += 1;
        } else {
            self.x += 1;
        }
        Some(pixel)
    }
}

impl<T: Pixel> FusedIterator for PlaneIter<'_, T> {}

// A Plane, PlaneSlice, or PlaneRegion is assumed to include or be able to include
// padding on the edge of the frame
#[derive(Clone, Copy, Debug)]
pub struct PlaneSlice<'a, T: Pixel> {
    pub plane: &'a Plane<T>,
    pub x: isize,
    pub y: isize,
}

// A RowsIter or RowsIterMut is assumed to crop the padding from the frame edges
pub struct RowsIter<'a, T: Pixel> {
    plane: &'a Plane<T>,
    x: isize,
    y: isize,
}

impl<'a, T: Pixel> Iterator for RowsIter<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.plane.cfg.height as isize > self.y {
            // cannot directly return self.ps.row(row) due to lifetime issue
            let range = self.plane.row_range_cropped(self.x, self.y);
            self.y += 1;
            Some(&self.plane.data[range])
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.plane.cfg.height as isize - self.y;
        debug_assert!(remaining >= 0);
        let remaining = remaining as usize;

        (remaining, Some(remaining))
    }
}

impl<T: Pixel> ExactSizeIterator for RowsIter<'_, T> {}
impl<T: Pixel> FusedIterator for RowsIter<'_, T> {}

impl<'a, T: Pixel> PlaneSlice<'a, T> {
    #[allow(unused)]
    pub fn as_ptr(&self) -> *const T {
        self[0].as_ptr()
    }

    pub fn rows_iter(&self) -> RowsIter<'_, T> {
        RowsIter {
            plane: self.plane,
            x: self.x,
            y: self.y,
        }
    }

    pub fn clamp(&self) -> PlaneSlice<'a, T> {
        PlaneSlice {
            plane: self.plane,
            x: self.x.clamp(
                -(self.plane.cfg.xorigin as isize),
                self.plane.cfg.width as isize,
            ),
            y: self.y.clamp(
                -(self.plane.cfg.yorigin as isize),
                self.plane.cfg.height as isize,
            ),
        }
    }

    pub fn subslice(&self, xo: usize, yo: usize) -> PlaneSlice<'a, T> {
        PlaneSlice {
            plane: self.plane,
            x: self.x + xo as isize,
            y: self.y + yo as isize,
        }
    }

    pub fn reslice(&self, xo: isize, yo: isize) -> PlaneSlice<'a, T> {
        PlaneSlice {
            plane: self.plane,
            x: self.x + xo,
            y: self.y + yo,
        }
    }

    /// A slice starting i pixels above the current one.
    pub fn go_up(&self, i: usize) -> PlaneSlice<'a, T> {
        PlaneSlice {
            plane: self.plane,
            x: self.x,
            y: self.y - i as isize,
        }
    }

    /// A slice starting i pixels to the left of the current one.
    pub fn go_left(&self, i: usize) -> PlaneSlice<'a, T> {
        PlaneSlice {
            plane: self.plane,
            x: self.x - i as isize,
            y: self.y,
        }
    }

    pub fn p(&self, add_x: usize, add_y: usize) -> T {
        let new_y = (self.y + add_y as isize + self.plane.cfg.yorigin as isize) as usize;
        let new_x = (self.x + add_x as isize + self.plane.cfg.xorigin as isize) as usize;
        self.plane.data[new_y * self.plane.cfg.stride + new_x]
    }

    /// Checks if `add_y` and `add_x` lies in the allocated bounds of the
    /// underlying plane.
    pub fn accessible(&self, add_x: usize, add_y: usize) -> bool {
        let y = (self.y + add_y as isize + self.plane.cfg.yorigin as isize) as usize;
        let x = (self.x + add_x as isize + self.plane.cfg.xorigin as isize) as usize;
        y < self.plane.cfg.alloc_height && x < self.plane.cfg.stride
    }

    /// Checks if -`sub_x` and -`sub_y` lies in the allocated bounds of the
    /// underlying plane.
    pub fn accessible_neg(&self, sub_x: usize, sub_y: usize) -> bool {
        let y = self.y - sub_y as isize + self.plane.cfg.yorigin as isize;
        let x = self.x - sub_x as isize + self.plane.cfg.xorigin as isize;
        y >= 0 && x >= 0
    }

    /// This version of the function crops off the padding on the right side of the image
    pub fn row_cropped(&self, y: usize) -> &[T] {
        let y = (self.y + y as isize + self.plane.cfg.yorigin as isize) as usize;
        let x = (self.x + self.plane.cfg.xorigin as isize) as usize;
        let start = y * self.plane.cfg.stride + x;
        let width = (self.plane.cfg.width as isize - self.x) as usize;
        &self.plane.data[start..start + width]
    }

    /// This version of the function includes the padding on the right side of the image
    pub fn row(&self, y: usize) -> &[T] {
        let y = (self.y + y as isize + self.plane.cfg.yorigin as isize) as usize;
        let x = (self.x + self.plane.cfg.xorigin as isize) as usize;
        let start = y * self.plane.cfg.stride + x;
        let width = self.plane.cfg.stride - x;
        &self.plane.data[start..start + width]
    }
}

impl<T: Pixel> Index<usize> for PlaneSlice<'_, T> {
    type Output = [T];
    fn index(&self, index: usize) -> &Self::Output {
        let range = self.plane.row_range(self.x, self.y + index as isize);
        &self.plane.data[range]
    }
}

pub struct PlaneMutSlice<'a, T: Pixel> {
    pub plane: &'a mut Plane<T>,
    pub x: isize,
    pub y: isize,
}

pub struct RowsIterMut<'a, T: Pixel> {
    plane: *mut Plane<T>,
    x: isize,
    y: isize,
    phantom: PhantomData<&'a mut Plane<T>>,
}

impl<'a, T: Pixel> Iterator for RowsIterMut<'a, T> {
    type Item = &'a mut [T];

    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: there could not be a concurrent call using a mutable reference to the plane
        let plane = unsafe { &mut *self.plane };
        if plane.cfg.height as isize > self.y {
            // cannot directly return self.ps.row(row) due to lifetime issue
            let range = plane.row_range_cropped(self.x, self.y);
            self.y += 1;
            Some(&mut plane.data[range])
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // SAFETY: there could not be a concurrent call using a mutable reference to the plane
        let plane = unsafe { &mut *self.plane };
        let remaining = plane.cfg.height as isize - self.y;
        debug_assert!(remaining >= 0);
        let remaining = remaining as usize;

        (remaining, Some(remaining))
    }
}

impl<T: Pixel> ExactSizeIterator for RowsIterMut<'_, T> {}
impl<T: Pixel> FusedIterator for RowsIterMut<'_, T> {}

impl<T: Pixel> PlaneMutSlice<'_, T> {
    #[allow(unused)]
    pub fn rows_iter(&self) -> RowsIter<'_, T> {
        RowsIter {
            plane: self.plane,
            x: self.x,
            y: self.y,
        }
    }

    pub fn rows_iter_mut(&mut self) -> RowsIterMut<'_, T> {
        RowsIterMut {
            plane: self.plane as *mut Plane<T>,
            x: self.x,
            y: self.y,
            phantom: PhantomData,
        }
    }

    #[allow(unused)]
    pub fn subslice(&mut self, xo: usize, yo: usize) -> PlaneMutSlice<'_, T> {
        PlaneMutSlice {
            plane: self.plane,
            x: self.x + xo as isize,
            y: self.y + yo as isize,
        }
    }
}

impl<T: Pixel> Index<usize> for PlaneMutSlice<'_, T> {
    type Output = [T];
    fn index(&self, index: usize) -> &Self::Output {
        let range = self.plane.row_range(self.x, self.y + index as isize);
        &self.plane.data[range]
    }
}

impl<T: Pixel> IndexMut<usize> for PlaneMutSlice<'_, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let range = self.plane.row_range(self.x, self.y + index as isize);
        &mut self.plane.data[range]
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    use wasm_bindgen_test::*;

    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    wasm_bindgen_test_configure!(run_in_browser);

    #[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), wasm_bindgen_test)]
    #[test]
    fn copy_from_raw_u8() {
        #[rustfmt::skip]
        let mut plane = Plane::from_slice(&[
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 1, 2, 3, 4, 0, 0,
            0, 0, 8, 7, 6, 5, 0, 0,
            0, 0, 9, 8, 7, 6, 0, 0,
            0, 0, 2, 3, 4, 5, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
        ], 8);

        let input = vec![42u8; 64];

        plane.copy_from_raw_u8(&input, 8, 1);

        println!("{:?}", &plane.data[..10]);

        assert_eq!(&input[..64], &plane.data[..64]);
    }

    #[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), wasm_bindgen_test)]
    #[test]
    fn copy_to_raw_u8() {
        #[rustfmt::skip]
        let plane = Plane::from_slice(&[
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 1, 2, 3, 4, 0, 0,
            0, 0, 8, 7, 6, 5, 0, 0,
            0, 0, 9, 8, 7, 6, 0, 0,
            0, 0, 2, 3, 4, 5, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
        ], 8);

        let mut output = vec![42u8; 64];

        plane.copy_to_raw_u8(&mut output, 8, 1);

        println!("{:?}", &plane.data[..10]);

        assert_eq!(&output[..64], &plane.data[..64]);
    }

    #[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), wasm_bindgen_test)]
    #[test]
    fn test_plane_downsample() {
        #[rustfmt::skip]
        let plane = Plane::<u8> {
            data: PlaneData::from_slice(&[
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 1, 2, 3, 4, 0, 0,
                0, 0, 8, 7, 6, 5, 0, 0,
                0, 0, 9, 8, 7, 6, 0, 0,
                0, 0, 2, 3, 4, 5, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
            ]),
            cfg: PlaneConfig {
                stride: 8,
                alloc_height: 9,
                width: 4,
                height: 4,
                xdec: 0,
                ydec: 0,
                xpad: 0,
                ypad: 0,
                xorigin: 2,
                yorigin: 3,
            },
        };
        let downsampled = plane.downsampled(4, 4);

        #[rustfmt::skip]
        let expected = &[
            5, 5,
            6, 6,
        ];

        let v: Vec<_> = downsampled.iter().collect();

        assert_eq!(&expected[..], &v[..]);
    }

    #[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), wasm_bindgen_test)]
    #[test]
    fn test_plane_downsample_odd() {
        #[rustfmt::skip]
        let plane = Plane::<u8> {
            data: PlaneData::from_slice(&[
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 1, 2, 3, 4, 0, 0,
                0, 0, 8, 7, 6, 5, 0, 0,
                0, 0, 9, 8, 7, 6, 0, 0,
                0, 0, 2, 3, 4, 5, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
            ]),
            cfg: PlaneConfig {
                stride: 8,
                alloc_height: 9,
                width: 3,
                height: 3,
                xdec: 0,
                ydec: 0,
                xpad: 0,
                ypad: 0,
                xorigin: 2,
                yorigin: 3,
            },
        };
        let downsampled = plane.downsampled(3, 3);

        #[rustfmt::skip]
        let expected = &[
            5, 5,
            6, 6,
        ];

        let v: Vec<_> = downsampled.iter().collect();
        assert_eq!(&expected[..], &v[..]);
    }

    #[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), wasm_bindgen_test)]
    #[test]
    fn test_plane_downscale() {
        #[rustfmt::skip]
        let plane = Plane::<u8> {
            data: PlaneData::from_slice(&[
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1, 4, 5, 0, 0,
                0, 0, 2, 3, 6, 7, 0, 0,
                0, 0, 8, 9, 7, 5, 0, 0,
                0, 0, 9, 8, 3, 1, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
            ]),
            cfg: PlaneConfig {
                stride: 8,
                alloc_height: 9,
                width: 4,
                height: 4,
                xdec: 0,
                ydec: 0,
                xpad: 0,
                ypad: 0,
                xorigin: 2,
                yorigin: 3,
            },
        };
        let downscaled = plane.downscale::<2>();

        #[rustfmt::skip]
        let expected = &[
            2, 6,
            9, 4
        ];

        let v: Vec<_> = downscaled.iter().collect();
        assert_eq!(&expected[..], &v[..]);
    }

    #[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), wasm_bindgen_test)]
    #[test]
    fn test_plane_downscale_odd() {
        #[rustfmt::skip]
        let plane = Plane::<u8> {
            data: PlaneData::from_slice(&[
                1, 2, 3, 4, 1, 2, 3, 4,
                0, 0, 8, 7, 6, 5, 8, 7,
                6, 5, 8, 7, 6, 5, 8, 7,
                6, 5, 8, 7, 0, 0, 2, 3,
                4, 5, 0, 0, 9, 8, 7, 6,
                0, 0, 0, 0, 2, 3, 4, 5,
                0, 0, 0, 0, 2, 3, 4, 5,
            ]),
            cfg: PlaneConfig {
                stride: 8,
                alloc_height: 7,
                width: 8,
                height: 7,
                xdec: 0,
                ydec: 0,
                xpad: 0,
                ypad: 0,
                xorigin: 0,
                yorigin: 0,
            },
        };

        let downscaled = plane.downscale::<3>();

        #[rustfmt::skip]
        let expected = &[
            4, 5,
            3, 3
        ];

        let v: Vec<_> = downscaled.iter().collect();
        assert_eq!(&expected[..], &v[..]);
    }

    #[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), wasm_bindgen_test)]
    #[test]
    fn test_plane_downscale_odd_2() {
        #[rustfmt::skip]
        let plane = Plane::<u8> {
            data: PlaneData::from_slice(&[
                9, 8, 3, 1, 0, 1, 4, 5, 0, 0,
                0, 1, 4, 5, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 9, 0,
                0, 2, 3, 6, 7, 0, 0, 0, 0, 0,
                0, 0, 8, 9, 7, 5, 0, 0, 0, 0,
                9, 8, 3, 1, 0, 1, 4, 5, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 2, 3, 6, 7, 0,
                0, 0, 0, 0, 0, 0, 8, 9, 7, 5,
                0, 0, 0, 0, 9, 8, 3, 1, 0, 0
            ]),
            cfg: PlaneConfig {
                stride: 10,
                alloc_height: 10,
                width: 10,
                height: 10,
                xdec: 0,
                ydec: 0,
                xpad: 0,
                ypad: 0,
                xorigin: 0,
                yorigin: 0,
            },
        };
        let downscaled = plane.downscale::<3>();

        #[rustfmt::skip]
        let expected = &[
            3, 1, 2,
            4, 4, 1,
            0, 0, 4,
        ];

        let v: Vec<_> = downscaled.iter().collect();
        assert_eq!(&expected[..], &v[..]);
    }

    #[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), wasm_bindgen_test)]
    #[test]
    fn test_plane_pad() {
        #[rustfmt::skip]
        let mut plane = Plane::<u8> {
            data: PlaneData::from_slice(&[
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 1, 2, 3, 4, 0, 0,
                0, 0, 8, 7, 6, 5, 0, 0,
                0, 0, 9, 8, 7, 6, 0, 0,
                0, 0, 2, 3, 4, 5, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
            ]),
            cfg: PlaneConfig {
                stride: 8,
                alloc_height: 9,
                width: 4,
                height: 4,
                xdec: 0,
                ydec: 0,
                xpad: 0,
                ypad: 0,
                xorigin: 2,
                yorigin: 3,
            },
        };
        plane.pad(4, 4);

        #[rustfmt::skip]
        assert_eq!(&[
            1, 1, 1, 2, 3, 4, 4, 4,
            1, 1, 1, 2, 3, 4, 4, 4,
            1, 1, 1, 2, 3, 4, 4, 4,
            1, 1, 1, 2, 3, 4, 4, 4,
            8, 8, 8, 7, 6, 5, 5, 5,
            9, 9, 9, 8, 7, 6, 6, 6,
            2, 2, 2, 3, 4, 5, 5, 5,
            2, 2, 2, 3, 4, 5, 5, 5,
            2, 2, 2, 3, 4, 5, 5, 5,
        ], &plane.data[..]);
    }

    #[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), wasm_bindgen_test)]
    #[test]
    fn test_pixel_iterator() {
        #[rustfmt::skip]
        let plane = Plane::<u8> {
            data: PlaneData::from_slice(&[
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 1, 2, 3, 4, 0, 0,
                0, 0, 8, 7, 6, 5, 0, 0,
                0, 0, 9, 8, 7, 6, 0, 0,
                0, 0, 2, 3, 4, 5, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
            ]),
            cfg: PlaneConfig {
                stride: 8,
                alloc_height: 9,
                width: 4,
                height: 4,
                xdec: 0,
                ydec: 0,
                xpad: 0,
                ypad: 0,
                xorigin: 2,
                yorigin: 3,
            },
        };

        let pixels: Vec<u8> = plane.iter().collect();

        assert_eq!(
            &[1, 2, 3, 4, 8, 7, 6, 5, 9, 8, 7, 6, 2, 3, 4, 5,][..],
            &pixels[..]
        );
    }
}

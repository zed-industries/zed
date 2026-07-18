// font-kit/src/canvas.rs
//
// Copyright © 2018 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! An in-memory bitmap surface for glyph rasterization.

use lazy_static::lazy_static;
use pathfinder_geometry::rect::RectI;
use pathfinder_geometry::vector::Vector2I;
use std::cmp;
use std::fmt;

use crate::utils;

lazy_static! {
    static ref BITMAP_1BPP_TO_8BPP_LUT: [[u8; 8]; 256] = {
        let mut lut = [[0; 8]; 256];
        for byte in 0..0x100 {
            let mut value = [0; 8];
            for bit in 0..8 {
                if (byte & (0x80 >> bit)) != 0 {
                    value[bit] = 0xff;
                }
            }
            lut[byte] = value
        }
        lut
    };
}

/// An in-memory bitmap surface for glyph rasterization.
pub struct Canvas {
    /// The raw pixel data.
    pub pixels: Vec<u8>,
    /// The size of the buffer, in pixels.
    pub size: Vector2I,
    /// The number of *bytes* between successive rows.
    pub stride: usize,
    /// The image format of the canvas.
    pub format: Format,
}

impl Canvas {
    /// Creates a new blank canvas with the given pixel size and format.
    ///
    /// Stride is automatically calculated from width.
    ///
    /// The canvas is initialized with transparent black (all values 0).
    #[inline]
    pub fn new(size: Vector2I, format: Format) -> Canvas {
        Canvas::with_stride(
            size,
            size.x() as usize * format.bytes_per_pixel() as usize,
            format,
        )
    }

    /// Creates a new blank canvas with the given pixel size, stride (number of bytes between
    /// successive rows), and format.
    ///
    /// The canvas is initialized with transparent black (all values 0).
    pub fn with_stride(size: Vector2I, stride: usize, format: Format) -> Canvas {
        Canvas {
            pixels: vec![0; stride * size.y() as usize],
            size,
            stride,
            format,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn blit_from_canvas(&mut self, src: &Canvas) {
        self.blit_from(
            Vector2I::default(),
            &src.pixels,
            src.size,
            src.stride,
            src.format,
        )
    }

    /// Blits to a rectangle with origin at `dst_point` and size according to `src_size`.
    /// If the target area overlaps the boundaries of the canvas, only the drawable region is blitted.
    /// `dst_point` and `src_size` are specified in pixels. `src_stride` is specified in bytes.
    /// `src_stride` must be equal or larger than the actual data length.
    #[allow(dead_code)]
    pub(crate) fn blit_from(
        &mut self,
        dst_point: Vector2I,
        src_bytes: &[u8],
        src_size: Vector2I,
        src_stride: usize,
        src_format: Format,
    ) {
        assert_eq!(
            src_stride * src_size.y() as usize,
            src_bytes.len(),
            "Number of pixels in src_bytes does not match stride and size."
        );
        assert!(
            src_stride >= src_size.x() as usize * src_format.bytes_per_pixel() as usize,
            "src_stride must be >= than src_size.x()"
        );

        let dst_rect = RectI::new(dst_point, src_size);
        let dst_rect = dst_rect.intersection(RectI::new(Vector2I::default(), self.size));
        let dst_rect = match dst_rect {
            Some(dst_rect) => dst_rect,
            None => return,
        };

        match (self.format, src_format) {
            (Format::A8, Format::A8)
            | (Format::Rgb24, Format::Rgb24)
            | (Format::Rgba32, Format::Rgba32) => {
                self.blit_from_with::<BlitMemcpy>(dst_rect, src_bytes, src_stride, src_format)
            }
            (Format::A8, Format::Rgb24) => {
                self.blit_from_with::<BlitRgb24ToA8>(dst_rect, src_bytes, src_stride, src_format)
            }
            (Format::Rgb24, Format::A8) => {
                self.blit_from_with::<BlitA8ToRgb24>(dst_rect, src_bytes, src_stride, src_format)
            }
            (Format::Rgb24, Format::Rgba32) => self
                .blit_from_with::<BlitRgba32ToRgb24>(dst_rect, src_bytes, src_stride, src_format),
            (Format::Rgba32, Format::Rgb24) => self
                .blit_from_with::<BlitRgb24ToRgba32>(dst_rect, src_bytes, src_stride, src_format),
            (Format::Rgba32, Format::A8) | (Format::A8, Format::Rgba32) => unimplemented!(),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn blit_from_bitmap_1bpp(
        &mut self,
        dst_point: Vector2I,
        src_bytes: &[u8],
        src_size: Vector2I,
        src_stride: usize,
    ) {
        if self.format != Format::A8 {
            unimplemented!()
        }

        let dst_rect = RectI::new(dst_point, src_size);
        let dst_rect = dst_rect.intersection(RectI::new(Vector2I::default(), self.size));
        let dst_rect = match dst_rect {
            Some(dst_rect) => dst_rect,
            None => return,
        };

        let size = dst_rect.size();

        let dest_bytes_per_pixel = self.format.bytes_per_pixel() as usize;
        let dest_row_stride = size.x() as usize * dest_bytes_per_pixel;
        let src_row_stride = utils::div_round_up(size.x() as usize, 8);

        for y in 0..size.y() {
            let (dest_row_start, src_row_start) = (
                (y + dst_rect.origin_y()) as usize * self.stride
                    + dst_rect.origin_x() as usize * dest_bytes_per_pixel,
                y as usize * src_stride,
            );
            let dest_row_end = dest_row_start + dest_row_stride;
            let src_row_end = src_row_start + src_row_stride;
            let dest_row_pixels = &mut self.pixels[dest_row_start..dest_row_end];
            let src_row_pixels = &src_bytes[src_row_start..src_row_end];
            for x in 0..src_row_stride {
                let pattern = &BITMAP_1BPP_TO_8BPP_LUT[src_row_pixels[x] as usize];
                let dest_start = x * 8;
                let dest_end = cmp::min(dest_start + 8, dest_row_stride);
                let src = &pattern[0..(dest_end - dest_start)];
                dest_row_pixels[dest_start..dest_end].clone_from_slice(src);
            }
        }
    }

    /// Blits to area `rect` using the data given in the buffer `src_bytes`.
    /// `src_stride` must be specified in bytes.
    /// The dimensions of `rect` must be in pixels.
    fn blit_from_with<B: Blit>(
        &mut self,
        rect: RectI,
        src_bytes: &[u8],
        src_stride: usize,
        src_format: Format,
    ) {
        let src_bytes_per_pixel = src_format.bytes_per_pixel() as usize;
        let dest_bytes_per_pixel = self.format.bytes_per_pixel() as usize;

        for y in 0..rect.height() {
            let (dest_row_start, src_row_start) = (
                (y + rect.origin_y()) as usize * self.stride
                    + rect.origin_x() as usize * dest_bytes_per_pixel,
                y as usize * src_stride,
            );
            let dest_row_end = dest_row_start + rect.width() as usize * dest_bytes_per_pixel;
            let src_row_end = src_row_start + rect.width() as usize * src_bytes_per_pixel;
            let dest_row_pixels = &mut self.pixels[dest_row_start..dest_row_end];
            let src_row_pixels = &src_bytes[src_row_start..src_row_end];
            B::blit(dest_row_pixels, src_row_pixels)
        }
    }
}

impl fmt::Debug for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Canvas")
            .field("pixels", &self.pixels.len()) // Do not dump a vector content.
            .field("size", &self.size)
            .field("stride", &self.stride)
            .field("format", &self.format)
            .finish()
    }
}

/// The image format for the canvas.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Format {
    /// Premultiplied R8G8B8A8, little-endian.
    Rgba32,
    /// R8G8B8, little-endian.
    Rgb24,
    /// A8.
    A8,
}

impl Format {
    /// Returns the number of bits per pixel that this image format corresponds to.
    #[inline]
    pub fn bits_per_pixel(self) -> u8 {
        match self {
            Format::Rgba32 => 32,
            Format::Rgb24 => 24,
            Format::A8 => 8,
        }
    }

    /// Returns the number of color channels per pixel that this image format corresponds to.
    #[inline]
    pub fn components_per_pixel(self) -> u8 {
        match self {
            Format::Rgba32 => 4,
            Format::Rgb24 => 3,
            Format::A8 => 1,
        }
    }

    /// Returns the number of bits per color channel that this image format contains.
    #[inline]
    pub fn bits_per_component(self) -> u8 {
        self.bits_per_pixel() / self.components_per_pixel()
    }

    /// Returns the number of bytes per pixel that this image format corresponds to.
    #[inline]
    pub fn bytes_per_pixel(self) -> u8 {
        self.bits_per_pixel() / 8
    }
}

/// The antialiasing strategy that should be used when rasterizing glyphs.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RasterizationOptions {
    /// "Black-and-white" rendering. Each pixel is either entirely on or off.
    Bilevel,
    /// Grayscale antialiasing. Only one channel is used.
    GrayscaleAa,
    /// Subpixel RGB antialiasing, for LCD screens.
    SubpixelAa,
}

trait Blit {
    fn blit(dest: &mut [u8], src: &[u8]);
}

struct BlitMemcpy;

impl Blit for BlitMemcpy {
    #[inline]
    fn blit(dest: &mut [u8], src: &[u8]) {
        dest.clone_from_slice(src)
    }
}

struct BlitRgb24ToA8;

impl Blit for BlitRgb24ToA8 {
    #[inline]
    fn blit(dest: &mut [u8], src: &[u8]) {
        // TODO(pcwalton): SIMD.
        for (dest, src) in dest.iter_mut().zip(src.chunks(3)) {
            *dest = src[1]
        }
    }
}

struct BlitA8ToRgb24;

impl Blit for BlitA8ToRgb24 {
    #[inline]
    fn blit(dest: &mut [u8], src: &[u8]) {
        for (dest, src) in dest.chunks_mut(3).zip(src.iter()) {
            dest[0] = *src;
            dest[1] = *src;
            dest[2] = *src;
        }
    }
}

struct BlitRgba32ToRgb24;

impl Blit for BlitRgba32ToRgb24 {
    #[inline]
    fn blit(dest: &mut [u8], src: &[u8]) {
        // TODO(pcwalton): SIMD.
        for (dest, src) in dest.chunks_mut(3).zip(src.chunks(4)) {
            dest.copy_from_slice(&src[0..3])
        }
    }
}

struct BlitRgb24ToRgba32;

impl Blit for BlitRgb24ToRgba32 {
    fn blit(dest: &mut [u8], src: &[u8]) {
        for (dest, src) in dest.chunks_mut(4).zip(src.chunks(3)) {
            dest[0] = src[0];
            dest[1] = src[1];
            dest[2] = src[2];
            dest[3] = 255;
        }
    }
}

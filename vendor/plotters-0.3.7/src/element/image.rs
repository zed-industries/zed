#[cfg(all(
    not(all(target_arch = "wasm32", not(target_os = "wasi"))),
    feature = "image"
))]
use image::{DynamicImage, GenericImageView};

use super::{Drawable, PointCollection};
use plotters_backend::{BackendCoord, DrawingBackend, DrawingErrorKind};

use plotters_bitmap::bitmap_pixel::{PixelFormat, RGBPixel};

#[cfg(all(
    not(all(target_arch = "wasm32", not(target_os = "wasi"))),
    feature = "image"
))]
use plotters_bitmap::bitmap_pixel::BGRXPixel;

use plotters_bitmap::BitMapBackend;

use std::borrow::Borrow;
use std::marker::PhantomData;

enum Buffer<'a> {
    Owned(Vec<u8>),
    Borrowed(&'a [u8]),
    BorrowedMut(&'a mut [u8]),
}

impl<'a> Borrow<[u8]> for Buffer<'a> {
    fn borrow(&self) -> &[u8] {
        self.as_ref()
    }
}

impl AsRef<[u8]> for Buffer<'_> {
    fn as_ref(&self) -> &[u8] {
        match self {
            Buffer::Owned(owned) => owned.as_ref(),
            Buffer::Borrowed(target) => target,
            Buffer::BorrowedMut(target) => target,
        }
    }
}

impl<'a> Buffer<'a> {
    fn to_mut(&mut self) -> &mut [u8] {
        let owned = match self {
            Buffer::Owned(owned) => return &mut owned[..],
            Buffer::BorrowedMut(target) => return target,
            Buffer::Borrowed(target) => {
                let mut value = vec![];
                value.extend_from_slice(target);
                value
            }
        };

        *self = Buffer::Owned(owned);
        self.to_mut()
    }
}

/// The element that contains a bitmap on it
pub struct BitMapElement<'a, Coord, P: PixelFormat = RGBPixel> {
    image: Buffer<'a>,
    size: (u32, u32),
    pos: Coord,
    phantom: PhantomData<P>,
}

impl<'a, Coord, P: PixelFormat> BitMapElement<'a, Coord, P> {
    /// Create a new empty bitmap element. This can be use as
    /// the draw and blit pattern.
    ///
    /// - `pos`: The left upper coordinate for the element
    /// - `size`: The size of the bitmap
    pub fn new(pos: Coord, size: (u32, u32)) -> Self {
        Self {
            image: Buffer::Owned(vec![0; (size.0 * size.1) as usize * P::PIXEL_SIZE]),
            size,
            pos,
            phantom: PhantomData,
        }
    }

    /// Create a new bitmap element with an pre-allocated owned buffer, this function will
    /// take the ownership of the buffer.
    ///
    /// - `pos`: The left upper coordinate of the elelent
    /// - `size`: The size of the bitmap
    /// - `buf`: The buffer to use
    /// - **returns**: The newly created image element, if the buffer isn't fit the image
    ///   dimension, this will returns an `None`.
    pub fn with_owned_buffer(pos: Coord, size: (u32, u32), buf: Vec<u8>) -> Option<Self> {
        if buf.len() < (size.0 * size.1) as usize * P::PIXEL_SIZE {
            return None;
        }

        Some(Self {
            image: Buffer::Owned(buf),
            size,
            pos,
            phantom: PhantomData,
        })
    }

    /// Create a new bitmap element with a mut borrow to an existing buffer
    ///
    /// - `pos`: The left upper coordinate of the elelent
    /// - `size`: The size of the bitmap
    /// - `buf`: The buffer to use
    /// - **returns**: The newly created image element, if the buffer isn't fit the image
    ///   dimension, this will returns an `None`.
    pub fn with_mut(pos: Coord, size: (u32, u32), buf: &'a mut [u8]) -> Option<Self> {
        if buf.len() < (size.0 * size.1) as usize * P::PIXEL_SIZE {
            return None;
        }

        Some(Self {
            image: Buffer::BorrowedMut(buf),
            size,
            pos,
            phantom: PhantomData,
        })
    }

    /// Create a new bitmap element with a shared borrowed buffer. This means if we want to modify
    /// the content of the image, the buffer is automatically copied
    ///
    /// - `pos`: The left upper coordinate of the elelent
    /// - `size`: The size of the bitmap
    /// - `buf`: The buffer to use
    /// - **returns**: The newly created image element, if the buffer isn't fit the image
    ///   dimension, this will returns an `None`.
    pub fn with_ref(pos: Coord, size: (u32, u32), buf: &'a [u8]) -> Option<Self> {
        if buf.len() < (size.0 * size.1) as usize * P::PIXEL_SIZE {
            return None;
        }

        Some(Self {
            image: Buffer::Borrowed(buf),
            size,
            pos,
            phantom: PhantomData,
        })
    }

    /// Copy the existing bitmap element to another location
    ///
    /// - `pos`: The new location to copy
    pub fn copy_to<Coord2>(&self, pos: Coord2) -> BitMapElement<Coord2, P> {
        BitMapElement {
            image: Buffer::Borrowed(self.image.borrow()),
            size: self.size,
            pos,
            phantom: PhantomData,
        }
    }

    /// Move the existing bitmap element to a new position
    ///
    /// - `pos`: The new position
    pub fn move_to(&mut self, pos: Coord) {
        self.pos = pos;
    }

    /// Make the bitmap element as a bitmap backend, so that we can use
    /// plotters drawing functionality on the bitmap element
    pub fn as_bitmap_backend(&mut self) -> BitMapBackend<P> {
        BitMapBackend::with_buffer_and_format(self.image.to_mut(), self.size).unwrap()
    }
}

#[cfg(all(
    not(all(target_arch = "wasm32", not(target_os = "wasi"))),
    feature = "image"
))]
impl<'a, Coord> From<(Coord, DynamicImage)> for BitMapElement<'a, Coord, RGBPixel> {
    fn from((pos, image): (Coord, DynamicImage)) -> Self {
        let (w, h) = image.dimensions();
        let rgb_image = image.to_rgb8().into_raw();
        Self {
            pos,
            image: Buffer::Owned(rgb_image),
            size: (w, h),
            phantom: PhantomData,
        }
    }
}

#[cfg(all(
    not(all(target_arch = "wasm32", not(target_os = "wasi"))),
    feature = "image"
))]
impl<'a, Coord> From<(Coord, DynamicImage)> for BitMapElement<'a, Coord, BGRXPixel> {
    fn from((pos, image): (Coord, DynamicImage)) -> Self {
        let (w, h) = image.dimensions();
        let rgb_image = image.to_rgb8().into_raw();
        Self {
            pos,
            image: Buffer::Owned(rgb_image),
            size: (w, h),
            phantom: PhantomData,
        }
    }
}

impl<'a, 'b, Coord> PointCollection<'a, Coord> for &'a BitMapElement<'b, Coord> {
    type Point = &'a Coord;
    type IntoIter = std::iter::Once<&'a Coord>;
    fn point_iter(self) -> Self::IntoIter {
        std::iter::once(&self.pos)
    }
}

impl<'a, Coord, DB: DrawingBackend> Drawable<DB> for BitMapElement<'a, Coord> {
    fn draw<I: Iterator<Item = BackendCoord>>(
        &self,
        mut points: I,
        backend: &mut DB,
        _: (u32, u32),
    ) -> Result<(), DrawingErrorKind<DB::ErrorType>> {
        if let Some((x, y)) = points.next() {
            // TODO: convert the pixel format when needed
            return backend.blit_bitmap((x, y), self.size, self.image.as_ref());
        }
        Ok(())
    }
}

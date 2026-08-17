
//! Provides a predefined pixel storage.
//! Currently only contains a simple flattened vector storage.
//! Use the functions `create_pixel_vec::<YourPixelTuple>` and
//! `set_pixel_in_vec::<YourPixelTuple>` for reading a predefined pixel vector.
//! Use the function `PixelVec::new` to create a pixel vector which can be written to a file.

use super::*;

/// Store all samples in a single array.
/// All samples will be converted to the type `T`.
/// This supports all the sample types, `f16`, `f32`, and `u32`.
///
/// The flattened vector contains all rows one after another.
/// In each row, for each pixel, its red, green, blue, and then alpha
/// samples are stored one after another.
///
/// Use `PixelVec.compute_pixel_index(position)`
/// to compute the flat index of a specific pixel.
#[derive(Eq, PartialEq, Clone)]
pub struct PixelVec<T> {

    /// The resolution of this layer.
    pub resolution: Vec2<usize>,

    /// The flattened vector contains all rows one after another.
    /// In each row, for each pixel, its red, green, blue, and then alpha
    /// samples are stored one after another.
    ///
    /// Use `Flattened::compute_pixel_index(image, position)`
    /// to compute the flat index of a specific pixel.
    pub pixels: Vec<T>,
}

impl<Pixel> PixelVec<Pixel> {

    /// Create a new flattened pixel storage, filled with default pixels.
    /// Accepts a `Channels` parameter, which is not used, so that it can be passed as a function pointer instead of calling it.
    pub fn constructor<Channels>(resolution: Vec2<usize>, _: &Channels) -> Self where Pixel: Default + Clone {
        PixelVec { resolution, pixels: vec![Pixel::default(); resolution.area()] }
    }

    /// Examine a pixel of a `PixelVec<T>` image.
    /// Can usually be used as a function reference instead of calling it directly.
    #[inline]
    pub fn get_pixel(&self, position: Vec2<usize>) -> &Pixel where Pixel: Sync {
        &self.pixels[self.compute_pixel_index(position)]
    }

    /// Update a pixel of a `PixelVec<T>` image.
    /// Can usually be used as a function reference instead of calling it directly.
    #[inline]
    pub fn set_pixel(&mut self, position: Vec2<usize>, pixel: Pixel) {
        let index = self.compute_pixel_index(position);
        self.pixels[index] = pixel;
    }

    /// Create a new flattened pixel storage, checking the length of the provided pixels vector.
    pub fn new(resolution: impl Into<Vec2<usize>>, pixels: Vec<Pixel>) -> Self {
        let size = resolution.into();
        assert_eq!(size.area(), pixels.len(), "expected {} samples, but vector length is {}", size.area(), pixels.len());
        Self { resolution: size, pixels }
    }

    /// Compute the flat index of a specific pixel. Returns a range of either 3 or 4 samples.
    /// The computed index can be used with `PixelVec.samples[index]`.
    /// Panics for invalid sample coordinates.
    #[inline]
    pub fn compute_pixel_index(&self, position: Vec2<usize>) -> usize {
        position.flat_index_for_size(self.resolution)
    }
}

use crate::image::validate_results::{ValidateResult, ValidationResult};

impl<Px> ValidateResult for PixelVec<Px> where Px: ValidateResult {
    fn validate_result(&self, other: &Self, options: ValidationOptions, location: impl Fn() -> String) -> ValidationResult {
        if self.resolution != other.resolution { Err(location() + " > resolution") }
        else { self.pixels.as_slice().validate_result(&other.pixels.as_slice(), options, || location() + " > pixels") }
    }
}

impl<Px> GetPixel for PixelVec<Px> where Px: Clone + Sync {
    type Pixel = Px;
    fn get_pixel(&self, position: Vec2<usize>) -> Self::Pixel {
        self.get_pixel(position).clone()
    }
}

use std::fmt::*;

impl<T> Debug for PixelVec<T> {
    #[inline] fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "[{}; {}]", std::any::type_name::<T>(), self.pixels.len())
    }
}


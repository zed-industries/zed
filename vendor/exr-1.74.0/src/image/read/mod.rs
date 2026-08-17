
//! Read an exr image.
//!
//! For great flexibility and customization, use the `read()` function.
//! The return value of the `read()` function must be further customized before reading a file.

//!
//! For very simple applications, you can alternatively use one of these functions:
//!
//! 1. `read_first_rgba_layer_from_file(path, your_constructor, your_pixel_setter)`:
//!     You specify how to store the pixels.
//!     The first layer containing rgba channels is then loaded from the file.
//!     Fails if no rgba layer can be found.
//!
//! 1. `read_all_rgba_layers_from_file(path, your_constructor, your_pixel_setter)`:
//!     You specify how to store the pixels.
//!     All layers containing rgba channels are then loaded from the file.
//!     Fails if any layer in the image does not contain rgba channels.
//!
//! 1. `read_first_flat_layer_from_file(path)`:
//!     The first layer containing non-deep data with arbitrary channels is loaded from the file.
//!     Fails if no non-deep layer can be found.
//!
//! 1. `read_all_flat_layers_from_file(path)`:
//!     All layers containing non-deep data with arbitrary channels are loaded from the file.
//!     Fails if any layer in the image contains deep data.
//!
//! 1. `read_all_data_from_file(path)`:
//!     All layers with arbitrary channels and all resolution levels are extracted from the file.
//!
//!     Note: Currently does not support deep data, and currently fails
//!     if any layer in the image contains deep data.
//!

// The following three stages are internally used to read an image.
// 1. `ReadImage` - The specification. Contains everything the user wants to tell us about loading an image.
//    The data in this structure will be instantiated and might be borrowed.
// 2. `ImageReader` - The temporary reader. Based on the specification of the blueprint,
//    a reader is instantiated, once for each layer.
//    This data structure accumulates the image data from the file.
//    It also owns temporary data and references the blueprint.
// 3. `Image` - The clean image. The accumulated data from the Reader
//    is converted to the clean image structure, without temporary data.

pub mod image;
pub mod layers;
pub mod any_channels;
pub mod levels;
pub mod samples;
pub mod specific_channels;

use crate::error::{Result};
use crate::image::read::samples::{ReadFlatSamples};
use std::path::Path;
use crate::image::{AnyImage, AnyChannels, FlatSamples, Image, Layer, FlatImage, PixelLayersImage, RgbaChannels};
use crate::image::read::image::ReadLayers;
use crate::image::read::layers::ReadChannels;
use crate::math::Vec2;
use crate::prelude::{PixelImage};
use crate::block::samples::FromNativeSample;


/// All resolution levels, all channels, all layers.
/// Does not support deep data yet. Uses parallel decompression and relaxed error handling.
/// Inspect the source code of this function if you need customization.
pub fn read_all_data_from_file(path: impl AsRef<Path>) -> Result<AnyImage> {
    read()
        .no_deep_data() // TODO deep data
        .all_resolution_levels()
        .all_channels()
        .all_layers()
        .all_attributes()
        .from_file(path)
}

// FIXME do not throw error on deep data but just skip it!
/// No deep data, no resolution levels, all channels, all layers.
/// Uses parallel decompression and relaxed error handling.
/// Inspect the source code of this function if you need customization.
pub fn read_all_flat_layers_from_file(path: impl AsRef<Path>) -> Result<FlatImage> {
    read()
        .no_deep_data()
        .largest_resolution_level()
        .all_channels()
        .all_layers()
        .all_attributes()
        .from_file(path)
}

/// No deep data, no resolution levels, all channels, first layer.
/// Uses parallel decompression and relaxed error handling.
/// Inspect the source code of this function if you need customization.
pub fn read_first_flat_layer_from_file(path: impl AsRef<Path>) -> Result<Image<Layer<AnyChannels<FlatSamples>>>> {
    read()
        .no_deep_data()
        .largest_resolution_level()
        .all_channels()
        .first_valid_layer()
        .all_attributes()
        .from_file(path)
}

/// No deep data, no resolution levels, rgba channels, all layers.
/// If a single layer does not contain rgba data, this method returns an error.
/// Uses parallel decompression and relaxed error handling.
/// `Create` and `Set` can be closures, see the examples for more information.
/// Inspect the source code of this function if you need customization.
/// The alpha channel will contain the value `1.0` if no alpha channel can be found in the image.
///
/// Using two closures, define how to store the pixels.
/// The first closure creates an image, and the second closure inserts a single pixel.
/// The type of the pixel can be defined by the second closure;
/// it must be a tuple containing four values, each being either `f16`, `f32`, `u32` or `Sample`.
// FIXME Set and Create should not need to be static
pub fn read_all_rgba_layers_from_file<R,G,B,A, Set:'static, Create:'static, Pixels: 'static>(
    path: impl AsRef<Path>, create: Create, set_pixel: Set
)
    -> Result<PixelLayersImage<Pixels, RgbaChannels>>
    where
        R: FromNativeSample, G: FromNativeSample, B: FromNativeSample, A: FromNativeSample,
        Create: Fn(Vec2<usize>, &RgbaChannels) -> Pixels, // TODO type alias? CreateRgbaPixels<Pixels=Pixels>,
        Set: Fn(&mut Pixels, Vec2<usize>, (R,G,B,A)),
{
    read()
        .no_deep_data()
        .largest_resolution_level()
        .rgba_channels(create, set_pixel)
        .all_layers()
        .all_attributes()
        .from_file(path)
}

/// No deep data, no resolution levels, rgba channels, choosing the first layer with rgba channels.
/// Uses parallel decompression and relaxed error handling.
/// `Create` and `Set` can be closures, see the examples for more information.
/// Inspect the source code of this function if you need customization.
/// The alpha channel will contain the value `1.0` if no alpha channel can be found in the image.
///
/// Using two closures, define how to store the pixels.
/// The first closure creates an image, and the second closure inserts a single pixel.
/// The type of the pixel can be defined by the second closure;
/// it must be a tuple containing four values, each being either `f16`, `f32`, `u32` or `Sample`.
// FIXME Set and Create should not need to be static
pub fn read_first_rgba_layer_from_file<R,G,B,A, Set:'static, Create:'static, Pixels: 'static>(
    path: impl AsRef<Path>, create: Create, set_pixel: Set
)
    -> Result<PixelImage<Pixels, RgbaChannels>>
    where
        R: FromNativeSample, G: FromNativeSample, B: FromNativeSample, A: FromNativeSample,
        Create: Fn(Vec2<usize>, &RgbaChannels) -> Pixels, // TODO type alias? CreateRgbaPixels<Pixels=Pixels>,
        Set: Fn(&mut Pixels, Vec2<usize>, (R,G,B,A)),
{
    read()
        .no_deep_data()
        .largest_resolution_level()
        .rgba_channels(create, set_pixel)
        .first_valid_layer()
        .all_attributes()
        .from_file(path)
}


/// Utilizes the builder pattern to configure an image reader. This is the initial struct.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ReadBuilder;

/// Create a reader which can be used to load an exr image.
/// Allows you to exactly specify how to load the image, for example:
///
/// ```no_run
///     use exr::prelude::*;
///
///     // the type of the this image depends on the chosen options
///     let image = read()
///         .no_deep_data() // (currently required)
///         .largest_resolution_level() // or `all_resolution_levels()`
///         .all_channels() // or `rgba_channels(constructor, setter)`
///         .all_layers() // or `first_valid_layer()`
///         .all_attributes() // (currently required)
///         .on_progress(|progress| println!("progress: {:.1}", progress*100.0)) // optional
///         .from_file("image.exr").unwrap(); // or `from_buffered(my_byte_slice)`
/// ```
///
/// You can alternatively use one of the following simpler functions:
/// 1. `read_first_flat_layer_from_file`
/// 1. `read_all_rgba_layers_from_file`
/// 1. `read_all_flat_layers_from_file`
/// 1. `read_all_data_from_file`
///
// TODO not panic but skip deep layers!
pub fn read() -> ReadBuilder { ReadBuilder }

impl ReadBuilder {

    /// Specify to handle only one sample per channel, disabling "deep data".
    // TODO not panic but skip deep layers!
    pub fn no_deep_data(self) -> ReadFlatSamples { ReadFlatSamples }

    // pub fn any_resolution_levels() -> ReadBuilder<> {}

    // TODO
    // e. g. `let sum = reader.any_channels_with(|sample, sum| sum += sample)`
    // e. g. `let floats = reader.any_channels_with(|sample, f32_samples| f32_samples[index] = sample as f32)`
    // pub fn no_deep_data_with <S> (self, storage: S) -> FlatSamplesWith<S> {  }

    // pub fn flat_and_deep_data(self) -> ReadAnySamples { ReadAnySamples }
}

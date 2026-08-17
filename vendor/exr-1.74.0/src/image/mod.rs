
//! Data structures that represent a complete exr image.
//! Contains generic structs that must be nested to obtain a complete image type.
//!
//!
//! For example, an rgba image containing multiple layers
//! can be represented using `Image<Layers<SpecificChannels<MyPixelStorage>>>`.
//! An image containing a single layer with arbitrary channels and no deep data
//! can be represented using `Image<Layer<AnyChannels<FlatSamples>>>`.
//!
//!
//! These and other predefined types are included in this module as
//! 1. `PixelImage`: A single layer, fixed set of arbitrary channels.
//! 1. `PixelLayersImage`: Multiple layers, fixed set of arbitrary channels.
//! 1. `RgbaImage`: A single layer, fixed set of channels: rgb, optional a.
//! 1. `RgbaLayersImage`: Multiple layers, fixed set of channels: rgb, optional a.
//! 1. `FlatImage`: Multiple layers, any channels, no deep data.
//! 1. `AnyImage`: All supported data (multiple layers, arbitrary channels, no deep data yet)
//!
//! You can also use your own types inside an image,
//! for example if you want to use a custom sample storage.
//!
//! This is the high-level interface for the pixels of an image.
//! See `exr::blocks` module for a low-level interface.

pub mod read;
pub mod write;
pub mod crop;
pub mod pixel_vec;
pub mod recursive;
// pub mod channel_groups;


use crate::meta::header::{ImageAttributes, LayerAttributes};
use crate::meta::attribute::{Text, LineOrder};
use half::f16;
use crate::math::{Vec2, RoundingMode};
use crate::compression::Compression;
use smallvec::{SmallVec};
use crate::error::Error;

/// Don't do anything
pub(crate) fn ignore_progress(_progress: f64){}

/// This image type contains all supported exr features and can represent almost any image.
/// It currently does not support deep data yet.
pub type AnyImage = Image<Layers<AnyChannels<Levels<FlatSamples>>>>;

/// This image type contains the most common exr features and can represent almost any plain image.
/// Does not contain resolution levels. Does not support deep data.
pub type FlatImage = Image<Layers<AnyChannels<FlatSamples>>>;

/// This image type contains multiple layers, with each layer containing a user-defined type of pixels.
pub type PixelLayersImage<Storage, Channels> = Image<Layers<SpecificChannels<Storage, Channels>>>;

/// This image type contains a single layer containing a user-defined type of pixels.
pub type PixelImage<Storage, Channels> = Image<Layer<SpecificChannels<Storage, Channels>>>;

/// This image type contains multiple layers, with each layer containing a user-defined type of rgba pixels.
pub type RgbaLayersImage<Storage> = PixelLayersImage<Storage, RgbaChannels>;

/// This image type contains a single layer containing a user-defined type of rgba pixels.
pub type RgbaImage<Storage> = PixelImage<Storage, RgbaChannels>;

/// Contains information about the channels in an rgba image, in the order `(red, green, blue, alpha)`.
/// The alpha channel is not required. May be `None` if the image did not contain an alpha channel.
pub type RgbaChannels = (ChannelDescription, ChannelDescription, ChannelDescription, Option<ChannelDescription>);

/// Contains information about the channels in an rgb image, in the order `(red, green, blue)`.
pub type RgbChannels = (ChannelDescription, ChannelDescription, ChannelDescription);

/// The complete exr image.
/// `Layers` can be either a single `Layer` or `Layers`.
#[derive(Debug, Clone, PartialEq)]
pub struct Image<Layers> {

    /// Attributes that apply to the whole image file.
    /// These attributes appear in each layer of the file.
    /// Excludes technical meta data.
    /// Each layer in this image also has its own attributes.
    pub attributes: ImageAttributes,

    /// The layers contained in the image file.
    /// Can be either a single `Layer` or a list of layers.
    pub layer_data: Layers,
}

/// A list of layers. `Channels` can be `SpecificChannels` or `AnyChannels`.
pub type Layers<Channels> = SmallVec<[Layer<Channels>; 2]>;

/// A single Layer, including fancy attributes and compression settings.
/// `Channels` can be either `SpecificChannels` or `AnyChannels`
#[derive(Debug, Clone, PartialEq)]
pub struct Layer<Channels> {

    /// The actual pixel data. Either `SpecificChannels` or `AnyChannels`
    pub channel_data: Channels,

    /// Attributes that apply to this layer.
    /// May still contain attributes that should be considered global for an image file.
    /// Excludes technical meta data: Does not contain data window size, line order, tiling, or compression attributes.
    /// The image also has attributes, which do not differ per layer.
    pub attributes: LayerAttributes,

    /// The pixel resolution of this layer.
    /// See `layer.attributes` for more attributes, like for example layer position.
    pub size: Vec2<usize>,

    /// How the pixels are split up and compressed.
    pub encoding: Encoding
}

/// How the pixels are split up and compressed.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Encoding {

    /// How the pixel data of all channels in this layer is compressed. May be `Compression::Uncompressed`.
    /// See `layer.attributes` for more attributes.
    pub compression: Compression,

    /// Describes how the pixels of this layer are divided into smaller blocks.
    /// Either splits the image into its scan lines or splits the image into tiles of the specified size.
    /// A single block can be loaded without processing all bytes of a file.
    pub blocks: Blocks,

    /// In what order the tiles of this header occur in the file.
    /// Does not change any actual image orientation.
    /// See `layer.attributes` for more attributes.
    pub line_order: LineOrder,
}

/// How the image pixels are split up into separate blocks.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Blocks {

    /// The image is divided into scan line blocks.
    /// The number of scan lines in a block depends on the compression method.
    ScanLines,

    /// The image is divided into tile blocks.
    /// Also specifies the size of each tile in the image
    /// and whether this image contains multiple resolution levels.
    ///
    /// The inner `Vec2` describes the size of each tile.
    /// Stays the same number of pixels across all levels.
    Tiles (Vec2<usize>)
}


/// A grid of pixels. The pixels are written to your custom pixel storage.
/// `PixelStorage` can be anything, from a flat `Vec<f16>` to `Vec<Vec<AnySample>>`, as desired.
/// In order to write this image to a file, your `PixelStorage` must implement [`GetPixel`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecificChannels<Pixels, ChannelsDescription> {

    /// A description of the channels in the file, as opposed to the channels in memory.
    /// Should always be a tuple containing `ChannelDescription`s, one description for each channel.
    pub channels: ChannelsDescription, // TODO this is awkward. can this be not a type parameter please? maybe vec<option<chan_info>> ??

    /// Your custom pixel storage
    // TODO should also support `Levels<YourStorage>`, where levels are desired!
    pub pixels: Pixels, // TODO rename to "pixels"?
}


/// A dynamic list of arbitrary channels.
/// `Samples` can currently only be `FlatSamples` or `Levels<FlatSamples>`.
#[derive(Debug, Clone, PartialEq)]
pub struct AnyChannels<Samples> {

    /// This list must be sorted alphabetically, by channel name.
    /// Use `AnyChannels::sorted` for automatic sorting.
    pub list: SmallVec<[AnyChannel<Samples>; 4]>
}

/// A single arbitrary channel.
/// `Samples` can currently only be `FlatSamples` or `Levels<FlatSamples>`
#[derive(Debug, Clone, PartialEq)]
pub struct AnyChannel<Samples> {

    /// One of "R", "G", or "B" most of the time.
    pub name: Text,

    /// The actual pixel data.
    /// Can be `FlatSamples` or `Levels<FlatSamples>`.
    pub sample_data: Samples,

    /// This attribute only tells lossy compression methods
    /// whether this value should be quantized exponentially or linearly.
    ///
    /// Should be `false` for red, green, blue and luma channels, as they are not perceived linearly.
    /// Should be `true` for hue, chroma, saturation, and alpha channels.
    pub quantize_linearly: bool,

    /// How many of the samples are skipped compared to the other channels in this layer.
    ///
    /// Can be used for chroma subsampling for manual lossy data compression.
    /// Values other than 1 are allowed only in flat, scan-line based images.
    /// If an image is deep or tiled, the sampling rates for all of its channels must be 1.
    pub sampling: Vec2<usize>,
}

/// One or multiple resolution levels of the same image.
/// `Samples` can be `FlatSamples`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Levels<Samples> {

    /// A single image without smaller versions of itself.
    /// If you only want to handle exclusively this case, use `Samples` directly, and not `Levels<Samples>`.
    Singular(Samples),

    /// Contains uniformly scaled smaller versions of the original.
    Mip
    {
        /// Whether to round up or down when calculating Mip/Rip levels.
        rounding_mode: RoundingMode,

        /// The smaller versions of the original.
        level_data: LevelMaps<Samples>
    },

    /// Contains any possible combination of smaller versions of the original.
    Rip
    {
        /// Whether to round up or down when calculating Mip/Rip levels.
        rounding_mode: RoundingMode,

        /// The smaller versions of the original.
        level_data: RipMaps<Samples>
    },
}

/// A list of resolution levels. `Samples` can currently only be `FlatSamples`.
// or `DeepAndFlatSamples` (not yet implemented).
pub type LevelMaps<Samples> = Vec<Samples>;

/// In addition to the full resolution image,
/// this layer also contains smaller versions,
/// and each smaller version has further versions with varying aspect ratios.
/// `Samples` can currently only be `FlatSamples`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RipMaps<Samples> {

    /// A flattened list containing the individual levels
    pub map_data: LevelMaps<Samples>,

    /// The number of levels that were generated along the x-axis and y-axis.
    pub level_count: Vec2<usize>,
}


// TODO deep data
/*#[derive(Clone, PartialEq)]
pub enum DeepAndFlatSamples {
    Deep(DeepSamples),
    Flat(FlatSamples)
}*/

/// A vector of non-deep values (one value per pixel per channel).
/// Stores row after row in a single vector.
/// The precision of all values is either `f16`, `f32` or `u32`.
///
/// Since this is close to the pixel layout in the byte file,
/// this will most likely be the fastest storage.
/// Using a different storage, for example `SpecificChannels`,
/// will probably be slower.
#[derive(Clone, PartialEq)] // debug is implemented manually
pub enum FlatSamples {

    /// A vector of non-deep `f16` values.
    F16(Vec<f16>),

    /// A vector of non-deep `f32` values.
    F32(Vec<f32>),

    /// A vector of non-deep `u32` values.
    U32(Vec<u32>),
}


/*#[derive(Clone, PartialEq)]
pub enum DeepSamples {
    F16(Vec<Vec<f16>>),
    F32(Vec<Vec<f32>>),
    U32(Vec<Vec<u32>>),
}*/

use crate::block::samples::*;
use crate::meta::attribute::*;
use crate::error::Result;
use crate::block::samples::Sample;
use crate::image::write::channels::*;
use crate::image::write::layers::WritableLayers;
use crate::image::write::samples::{WritableSamples};
use crate::meta::{mip_map_levels, rip_map_levels};
use crate::io::Data;
use crate::image::recursive::{NoneMore, Recursive, IntoRecursive};
use std::marker::PhantomData;
use std::ops::Not;
use crate::image::validate_results::{ValidationOptions};


impl<Channels> Layer<Channels> {
    /// Sometimes called "data window"
    pub fn absolute_bounds(&self) -> IntegerBounds {
        IntegerBounds::new(self.attributes.layer_position, self.size)
    }
}


impl<SampleStorage, Channels> SpecificChannels<SampleStorage, Channels> {
    /// Create some pixels with channel information.
    /// The `Channels` must be a tuple containing either `ChannelDescription` or `Option<ChannelDescription>`.
    /// The length of the tuple dictates the number of channels in the sample storage.
    pub fn new(channels: Channels, source_samples: SampleStorage) -> Self
        where
            SampleStorage: GetPixel,
            SampleStorage::Pixel: IntoRecursive,
            Channels: Sync + Clone + IntoRecursive,
            <Channels as IntoRecursive>::Recursive: WritableChannelsDescription<<SampleStorage::Pixel as IntoRecursive>::Recursive>,
    {
        SpecificChannels { channels, pixels: source_samples }
    }
}

/// Convert this type into one of the known sample types.
/// Also specify the preferred native type, which dictates the default sample type in the image.
pub trait IntoSample: IntoNativeSample {

    /// The native sample types that this type should be converted to.
    const PREFERRED_SAMPLE_TYPE: SampleType;
}

impl IntoSample for f16 { const PREFERRED_SAMPLE_TYPE: SampleType = SampleType::F16; }
impl IntoSample for f32 { const PREFERRED_SAMPLE_TYPE: SampleType = SampleType::F32; }
impl IntoSample for u32 { const PREFERRED_SAMPLE_TYPE: SampleType = SampleType::U32; }

/// Used to construct a `SpecificChannels`.
/// Call `with_named_channel` as many times as desired,
/// and then call `with_pixels` to define the colors.
#[derive(Debug)]
pub struct SpecificChannelsBuilder<RecursiveChannels, RecursivePixel> {
    channels: RecursiveChannels,
    px: PhantomData<RecursivePixel>
}

/// This check can be executed at compile time
/// if the channel names are `&'static str` and the compiler is smart enough.
pub trait CheckDuplicates {

    /// Check for duplicate channel names.
    fn already_contains(&self, name: &Text) -> bool;
}

impl CheckDuplicates for NoneMore {
    fn already_contains(&self, _: &Text) -> bool { false }
}

impl<Inner: CheckDuplicates> CheckDuplicates for Recursive<Inner, ChannelDescription> {
    fn already_contains(&self, name: &Text) -> bool {
        &self.value.name == name || self.inner.already_contains(name)
    }
}

impl SpecificChannels<(),()>
{
    /// Start building some specific channels. On the result of this function,
    /// call `with_named_channel` as many times as desired,
    /// and then call `with_pixels` to define the colors.
    pub fn build() -> SpecificChannelsBuilder<NoneMore, NoneMore> {
        SpecificChannelsBuilder { channels: NoneMore, px: Default::default() }
    }
}

impl<RecursiveChannels: CheckDuplicates, RecursivePixel> SpecificChannelsBuilder<RecursiveChannels, RecursivePixel>
{
    /// Add another channel to this image. Does not add the actual pixels,
    /// but instead only declares the presence of the channel.
    /// Panics if the name contains unsupported characters.
    /// Panics if a channel with the same name already exists.
    /// Use `Text::new_or_none()` to manually handle these cases.
    /// Use `with_channel_details` instead if you want to specify more options than just the name of the channel.
    /// The generic parameter can usually be inferred from the closure in `with_pixels`.
    pub fn with_channel<Sample: IntoSample>(self, name: impl Into<Text>)
                                            -> SpecificChannelsBuilder<Recursive<RecursiveChannels, ChannelDescription>, Recursive<RecursivePixel, Sample>>
    {
        self.with_channel_details::<Sample>(ChannelDescription::named(name, Sample::PREFERRED_SAMPLE_TYPE))
    }

    /// Add another channel to this image. Does not add the actual pixels,
    /// but instead only declares the presence of the channel.
    /// Use `with_channel` instead if you only want to specify the name of the channel.
    /// Panics if a channel with the same name already exists.
    /// The generic parameter can usually be inferred from the closure in `with_pixels`.
    pub fn with_channel_details<Sample: Into<Sample>>(self, channel: ChannelDescription)
        -> SpecificChannelsBuilder<Recursive<RecursiveChannels, ChannelDescription>, Recursive<RecursivePixel, Sample>>
    {
        // duplicate channel names are checked later, but also check now to make sure there are no problems with the `SpecificChannelsWriter`
        assert!(self.channels.already_contains(&channel.name).not(), "channel name `{}` is duplicate", channel.name);

        SpecificChannelsBuilder {
            channels: Recursive::new(self.channels, channel),
            px: PhantomData::default()
        }
    }

    /// Specify the actual pixel contents of the image.
    /// You can pass a closure that returns a color for each pixel (`Fn(Vec2<usize>) -> Pixel`),
    /// or you can pass your own image if it implements `GetPixel`.
    /// The pixel type must be a tuple with the correct number of entries, depending on the number of channels.
    /// The tuple entries can be either `f16`, `f32`, `u32` or `Sample`.
    /// Use `with_pixel_fn` instead of this function, to get extra type safety for your pixel closure.
    pub fn with_pixels<Pixels>(self, get_pixel: Pixels) -> SpecificChannels<Pixels, RecursiveChannels>
        where Pixels: GetPixel, <Pixels as GetPixel>::Pixel: IntoRecursive<Recursive=RecursivePixel>,
    {
        SpecificChannels {
            channels: self.channels,
            pixels: get_pixel
        }
    }

    /// Specify the contents of the image.
    /// The pixel type must be a tuple with the correct number of entries, depending on the number of channels.
    /// The tuple entries can be either `f16`, `f32`, `u32` or `Sample`.
    /// Use `with_pixels` instead of this function, if you want to pass an object that is not a closure.
    ///
    /// Usually, the compiler can infer the type of the pixel (for example, `f16,f32,f32`) from the closure.
    /// If that's not possible, you can specify the type of the channels
    /// when declaring the channel (for example, `with_named_channel::<f32>("R")`).
    pub fn with_pixel_fn<Pixel, Pixels>(self, get_pixel: Pixels) -> SpecificChannels<Pixels, RecursiveChannels>
        where Pixels: Sync + Fn(Vec2<usize>) -> Pixel, Pixel: IntoRecursive<Recursive=RecursivePixel>,
    {
        SpecificChannels {
            channels: self.channels,
            pixels: get_pixel
        }
    }
}

impl<SampleStorage> SpecificChannels<
    SampleStorage, (ChannelDescription, ChannelDescription, ChannelDescription, ChannelDescription)
>
{

    /// Create an image with red, green, blue, and alpha channels.
    /// You can pass a closure that returns a color for each pixel (`Fn(Vec2<usize>) -> (R,G,B,A)`),
    /// or you can pass your own image if it implements `GetPixel<Pixel=(R,G,B,A)>`.
    /// Each of `R`, `G`, `B` and `A` can be either `f16`, `f32`, `u32`, or `Sample`.
    pub fn rgba<R, G, B, A>(source_samples: SampleStorage) -> Self
        where R: IntoSample, G: IntoSample,
              B: IntoSample, A: IntoSample,
              SampleStorage: GetPixel<Pixel=(R, G, B, A)>
    {
        SpecificChannels {
            channels: (
                ChannelDescription::named("R", R::PREFERRED_SAMPLE_TYPE),
                ChannelDescription::named("G", G::PREFERRED_SAMPLE_TYPE),
                ChannelDescription::named("B", B::PREFERRED_SAMPLE_TYPE),
                ChannelDescription::named("A", A::PREFERRED_SAMPLE_TYPE),
            ),
            pixels: source_samples
        }
    }
}

impl<SampleStorage> SpecificChannels<
    SampleStorage, (ChannelDescription, ChannelDescription, ChannelDescription)
>
{

    /// Create an image with red, green, and blue channels.
    /// You can pass a closure that returns a color for each pixel (`Fn(Vec2<usize>) -> (R,G,B)`),
    /// or you can pass your own image if it implements `GetPixel<Pixel=(R,G,B)>`.
    /// Each of `R`, `G` and `B` can be either `f16`, `f32`, `u32`, or `Sample`.
    pub fn rgb<R, G, B>(source_samples: SampleStorage) -> Self
        where R: IntoSample, G: IntoSample, B: IntoSample,
              SampleStorage: GetPixel<Pixel=(R, G, B)>
    {
        SpecificChannels {
            channels: (
                ChannelDescription::named("R", R::PREFERRED_SAMPLE_TYPE),
                ChannelDescription::named("G", G::PREFERRED_SAMPLE_TYPE),
                ChannelDescription::named("B", B::PREFERRED_SAMPLE_TYPE),
            ),
            pixels: source_samples
        }
    }
}


/// A list of samples representing a single pixel.
/// Does not heap allocate for images with 8 or fewer channels.
pub type FlatSamplesPixel = SmallVec<[Sample; 8]>;

// TODO also deep samples?
impl Layer<AnyChannels<FlatSamples>> {

    /// Use `samples_at` if you can borrow from this layer
    pub fn sample_vec_at(&self, position: Vec2<usize>) -> FlatSamplesPixel {
        self.samples_at(position).collect()
    }

    /// Lookup all channels of a single pixel in the image
    pub fn samples_at(&self, position: Vec2<usize>) -> FlatSampleIterator<'_> {
        FlatSampleIterator {
            layer: self,
            channel_index: 0,
            position
        }
    }
}

/// Iterate over all channels of a single pixel in the image
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FlatSampleIterator<'s> {
    layer: &'s Layer<AnyChannels<FlatSamples>>,
    channel_index: usize,
    position: Vec2<usize>,
}

impl Iterator for FlatSampleIterator<'_> {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        if self.channel_index < self.layer.channel_data.list.len() {
            let channel = &self.layer.channel_data.list[self.channel_index];
            let sample = channel.sample_data.value_by_flat_index(self.position.flat_index_for_size(self.layer.size));
            self.channel_index += 1;
            Some(sample)
        }
        else { None }
    }

    fn nth(&mut self, pos: usize) -> Option<Self::Item> {
        self.channel_index += pos;
        self.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.layer.channel_data.list.len().saturating_sub(self.channel_index);
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for FlatSampleIterator<'_> {}

impl<SampleData> AnyChannels<SampleData>{

    /// A new list of arbitrary channels. Sorts the list to make it alphabetically stable.
    pub fn sort(mut list: SmallVec<[AnyChannel<SampleData>; 4]>) -> Self {
        list.sort_unstable_by_key(|channel| channel.name.clone()); // TODO no clone?
        Self { list }
    }
}

// FIXME check content size of layer somewhere??? before writing?
impl<LevelSamples> Levels<LevelSamples> {

    /// Get a resolution level by index, sorted by size, decreasing.
    pub fn get_level(&self, level: Vec2<usize>) -> Result<&LevelSamples> {
        match self {
            Levels::Singular(block) => {
                debug_assert_eq!(level, Vec2(0,0), "singular image cannot write leveled blocks bug");
                Ok(block)
            },

            Levels::Mip { level_data, .. } => {
                debug_assert_eq!(level.x(), level.y(), "mip map levels must be equal on x and y bug");
                level_data.get(level.x()).ok_or(Error::invalid("block mip level index"))
            },

            Levels::Rip { level_data, .. } => {
                level_data.get_by_level(level).ok_or(Error::invalid("block rip level index"))
            }
        }
    }

    /// Get a resolution level by index, sorted by size, decreasing.
    // TODO storage order for RIP maps?
    pub fn get_level_mut(&mut self, level: Vec2<usize>) -> Result<&mut LevelSamples> {
        match self {
            Levels::Singular(ref mut block) => {
                debug_assert_eq!(level, Vec2(0,0), "singular image cannot write leveled blocks bug");
                Ok(block)
            },

            Levels::Mip { level_data, .. } => {
                debug_assert_eq!(level.x(), level.y(), "mip map levels must be equal on x and y bug");
                level_data.get_mut(level.x()).ok_or(Error::invalid("block mip level index"))
            },

            Levels::Rip { level_data, .. } => {
                level_data.get_by_level_mut(level).ok_or(Error::invalid("block rip level index"))
            }
        }
    }

    /// Get a slice of all resolution levels, sorted by size, decreasing.
    pub fn levels_as_slice(&self) -> &[LevelSamples] {
        match self {
            Levels::Singular(data) => std::slice::from_ref(data),
            Levels::Mip { level_data, .. } => level_data,
            Levels::Rip { level_data, .. } => &level_data.map_data,
        }
    }

    /// Get a mutable slice of all resolution levels, sorted by size, decreasing.
    pub fn levels_as_slice_mut(&mut self) -> &mut [LevelSamples] {
        match self {
            Levels::Singular(data) => std::slice::from_mut(data),
            Levels::Mip { level_data, .. } => level_data,
            Levels::Rip { level_data, .. } => &mut level_data.map_data,
        }
    }

    // TODO simplify working with levels in general! like level_size_by_index and such

    /*pub fn levels_with_size(&self, rounding: RoundingMode, max_resolution: Vec2<usize>) -> Vec<(Vec2<usize>, &S)> {
        match self {
            Levels::Singular(ref data) => vec![ (max_resolution, data) ],
            Levels::Mip(ref maps) => mip_map_levels(rounding, max_resolution).map(|(_index, size)| size).zip(maps).collect(),
            Levels::Rip(ref rip_maps) => rip_map_levels(rounding, max_resolution).map(|(_index, size)| size).zip(&rip_maps.map_data).collect(),
        }
    }*/

    /// Whether this stores multiple resolution levels.
    pub fn level_mode(&self) -> LevelMode {
        match self {
            Levels::Singular(_) => LevelMode::Singular,
            Levels::Mip { .. } => LevelMode::MipMap,
            Levels::Rip { .. } => LevelMode::RipMap,
        }
    }
}

impl<Samples> RipMaps<Samples> {

    /// Flatten the 2D level index to a one dimensional index.
    pub fn get_level_index(&self, level: Vec2<usize>) -> usize {
        level.flat_index_for_size(self.level_count)
    }

    /// Return a level by level index. Level `0` has the largest resolution.
    pub fn get_by_level(&self, level: Vec2<usize>) -> Option<&Samples> {
        self.map_data.get(self.get_level_index(level))
    }

    /// Return a mutable level reference by level index. Level `0` has the largest resolution.
    pub fn get_by_level_mut(&mut self, level: Vec2<usize>) -> Option<&mut Samples> {
        let index = self.get_level_index(level);
        self.map_data.get_mut(index)
    }
}

impl FlatSamples {

    /// The number of samples in the image. Should be the width times the height.
    /// Might vary when subsampling is used.
    pub fn len(&self) -> usize {
        match self {
            FlatSamples::F16(vec) => vec.len(),
            FlatSamples::F32(vec) => vec.len(),
            FlatSamples::U32(vec) => vec.len(),
        }
    }

    /// Views all samples in this storage as f32.
    /// Matches the underlying sample type again for every sample,
    /// match yourself if performance is critical! Does not allocate.
    pub fn values_as_f32<'s>(&'s self) -> impl 's + Iterator<Item = f32> {
        self.values().map(|sample| sample.to_f32())
    }

    /// All samples in this storage as iterator.
    /// Matches the underlying sample type again for every sample,
    /// match yourself if performance is critical! Does not allocate.
    pub fn values<'s>(&'s self) -> impl 's + Iterator<Item = Sample> {
        (0..self.len()).map(move |index| self.value_by_flat_index(index))
    }

    /// Lookup a single value, by flat index.
    /// The flat index can be obtained using `Vec2::flatten_for_width`
    /// which computes the index in a flattened array of pixel rows.
    pub fn value_by_flat_index(&self, index: usize) -> Sample {
        match self {
            FlatSamples::F16(vec) => Sample::F16(vec[index]),
            FlatSamples::F32(vec) => Sample::F32(vec[index]),
            FlatSamples::U32(vec) => Sample::U32(vec[index]),
        }
    }
}


impl<'s, ChannelData:'s> Layer<ChannelData> {

    /// Create a layer with the specified size, attributes, encoding and channels.
    /// The channels can be either `SpecificChannels` or `AnyChannels`.
    pub fn new(
        dimensions: impl Into<Vec2<usize>>,
        attributes: LayerAttributes,
        encoding: Encoding,
        channels: ChannelData
    ) -> Self
        where ChannelData: WritableChannels<'s>
    {
        Layer { channel_data: channels, attributes, size: dimensions.into(), encoding }
    }

    // TODO test pls wtf
    /// Panics for images with Scanline encoding.
    pub fn levels_with_resolution<'l, L>(&self, levels: &'l Levels<L>) -> Box<dyn 'l + Iterator<Item=(&'l L, Vec2<usize>)>> {
        match levels {
            Levels::Singular(level) => Box::new(std::iter::once((level, self.size))),

            Levels::Mip { rounding_mode, level_data } => Box::new(level_data.iter().zip(
                mip_map_levels(*rounding_mode, self.size)
                    .map(|(_index, size)| size)
            )),

            Levels::Rip { rounding_mode, level_data } => Box::new(level_data.map_data.iter().zip(
                rip_map_levels(*rounding_mode, self.size)
                    .map(|(_index, size)| size)
            )),
        }
    }
}

impl Encoding {

    /// No compression. Massive space requirements.
    /// Fast, because it minimizes data shuffling and reallocation.
    pub const UNCOMPRESSED: Encoding = Encoding {
        compression: Compression::Uncompressed,
        blocks: Blocks::ScanLines, // longest lines, faster memcpy
        line_order: LineOrder::Increasing // presumably fastest?
    };

    /// Run-length encoding with tiles of 64x64 pixels. This is the recommended default encoding.
    /// Almost as fast as uncompressed data, but optimizes single-colored areas such as mattes and masks.
    pub const FAST_LOSSLESS: Encoding = Encoding {
        compression: Compression::RLE,
        blocks: Blocks::Tiles(Vec2(64, 64)), // optimize for RLE compression
        line_order: LineOrder::Unspecified
    };

    /// ZIP compression with blocks of 16 lines. Slow, but produces small files without visible artefacts.
    pub const SMALL_LOSSLESS: Encoding = Encoding {
        compression: Compression::ZIP16,
        blocks: Blocks::ScanLines, // largest possible, but also with high probability of parallel workers
        line_order: LineOrder::Increasing
    };

    /// PIZ compression with tiles of 256x256 pixels. Small images, not too slow.
    pub const SMALL_FAST_LOSSLESS: Encoding = Encoding {
        compression: Compression::PIZ,
        blocks: Blocks::Tiles(Vec2(256, 256)),
        line_order: LineOrder::Unspecified
    };
}

impl Default for Encoding {
    fn default() -> Self { Encoding::FAST_LOSSLESS }
}

impl<'s, LayerData: 's> Image<LayerData> where LayerData: WritableLayers<'s> {
    /// Create an image with one or multiple layers. The layer can be a `Layer`, or `Layers` small vector, or `Vec<Layer>` or `&[Layer]`.
    pub fn new(image_attributes: ImageAttributes, layer_data: LayerData) -> Self {
        Image { attributes: image_attributes, layer_data }
    }
}

// explorable constructor alias
impl<'s, Channels: 's> Image<Layers<Channels>> where Channels: WritableChannels<'s> {
    /// Create an image with multiple layers. The layer can be a `Vec<Layer>` or `Layers` (a small vector).
    pub fn from_layers(image_attributes: ImageAttributes, layer_data: impl Into<Layers<Channels>>) -> Self {
        Self::new(image_attributes, layer_data.into())
    }
}


impl<'s, ChannelData:'s> Image<Layer<ChannelData>> where ChannelData: WritableChannels<'s> {

    /// Uses the display position and size to the channel position and size of the layer.
    pub fn from_layer(layer: Layer<ChannelData>) -> Self {
        let bounds = IntegerBounds::new(layer.attributes.layer_position, layer.size);
        Self::new(ImageAttributes::new(bounds), layer)
    }

    /// Uses empty attributes.
    pub fn from_encoded_channels(size: impl Into<Vec2<usize>>, encoding: Encoding, channels: ChannelData) -> Self {
        // layer name is not required for single-layer images
        Self::from_layer(Layer::new(size, LayerAttributes::default(), encoding, channels))
    }

    /// Uses empty attributes and fast compression.
    pub fn from_channels(size: impl Into<Vec2<usize>>, channels: ChannelData) -> Self {
        Self::from_encoded_channels(size, Encoding::default(), channels)
    }
}


impl Image<NoneMore> {

    /// Create an empty image, to be filled with layers later on. Add at least one layer to obtain a valid image.
    /// Call `with_layer(another_layer)` for each layer you want to add to this image.
    pub fn empty(attributes: ImageAttributes) -> Self { Self { attributes, layer_data: NoneMore } }
}

impl<'s, InnerLayers: 's> Image<InnerLayers> where
    InnerLayers: WritableLayers<'s>,
{
    /// Add another layer to this image. The layer type does
    /// not have to equal the existing layers in this image.
    pub fn with_layer<NewChannels>(self, layer: Layer<NewChannels>)
        -> Image<Recursive<InnerLayers, Layer<NewChannels>>>
        where NewChannels: 's + WritableChannels<'s>
    {
        Image {
            attributes: self.attributes,
            layer_data: Recursive::new(self.layer_data, layer)
        }
    }
}


impl<'s, SampleData: 's> AnyChannel<SampleData> {

    /// Create a new channel without subsampling.
    ///
    /// Automatically flags this channel for specialized compression
    /// if the name is "R", "G", "B", "Y", or "L",
    /// as they typically encode values that are perceived non-linearly.
    /// Construct the value yourself using `AnyChannel { .. }`, if you want to control this flag.
    pub fn new(name: impl Into<Text>, sample_data: SampleData) -> Self where SampleData: WritableSamples<'s> {
        let name: Text = name.into();

        AnyChannel {
            quantize_linearly: ChannelDescription::guess_quantization_linearity(&name),
            name, sample_data,
            sampling: Vec2(1, 1),
        }
    }

    /*/// This is the same as `AnyChannel::new()`, but additionally ensures that the closure type is correct.
    pub fn from_closure<V>(name: Text, sample_data: S) -> Self
        where S: Sync + Fn(Vec2<usize>) -> V, V: InferSampleType + Data
    {
        Self::new(name, sample_data)
    }*/
}

impl std::fmt::Debug for FlatSamples {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.len() <= 6 {
            match self {
                FlatSamples::F16(vec) => vec.fmt(formatter),
                FlatSamples::F32(vec) => vec.fmt(formatter),
                FlatSamples::U32(vec) => vec.fmt(formatter),
            }
        }
        else {
            match self {
                FlatSamples::F16(vec) => write!(formatter, "[f16; {}]", vec.len()),
                FlatSamples::F32(vec) => write!(formatter, "[f32; {}]", vec.len()),
                FlatSamples::U32(vec) => write!(formatter, "[u32; {}]", vec.len()),
            }
        }
    }
}



/// Compare the result of a round trip test with the original method.
/// Supports lossy compression methods.
// #[cfg(test)] TODO do not ship this code
pub mod validate_results {
    use crate::prelude::*;
    use smallvec::Array;
    use crate::prelude::recursive::*;
    use crate::image::write::samples::WritableSamples;
    use std::ops::Not;
    use crate::block::samples::IntoNativeSample;

    /// Compare two objects, but with a few special quirks.
    /// Intended mainly for unit testing.
    pub trait ValidateResult {

        /// Compare self with the other. Panics if not equal.
        ///
        /// Exceptional behaviour:
        /// This does not work the other way around! This method is not symmetrical!
        /// Returns whether the result is correct for this image.
        /// For lossy compression methods, uses approximate equality.
        /// Intended for unit testing.
        ///
        /// Warning: If you use `SpecificChannels`, the comparison might be inaccurate
        /// for images with mixed compression methods. This is to be used with `AnyChannels` mainly.
        fn assert_equals_result(&self, result: &Self) {
            self.validate_result(result, ValidationOptions::default(), || String::new()).unwrap();
        }

        /// Compare self with the other.
        /// Exceptional behaviour:
        /// - Any two NaN values are considered equal, regardless of bit representation.
        /// - If a `lossy` is specified, any two values that differ only by a small amount will be considered equal.
        /// - If `nan_to_zero` is true, and __self is NaN/Infinite and the other value is zero, they are considered equal__
        ///   (because some compression methods replace nan with zero)
        ///
        /// This does not work the other way around! This method is not symmetrical!
        fn validate_result(
            &self, lossy_result: &Self,
            options: ValidationOptions,
            // this is a lazy string, because constructing a string is only necessary in the case of an error,
            // but eats up memory and allocation time every time. this was measured.
            context: impl Fn() -> String
        ) -> ValidationResult;
    }

    /// Whether to do accurate or approximate comparison.
    #[derive(Default, Debug, Eq, PartialEq, Hash, Copy, Clone)]
    pub struct ValidationOptions {
        allow_lossy: bool,
        nan_converted_to_zero: bool,
    }

    /// If invalid, contains the error message.
    pub type ValidationResult = std::result::Result<(), String>;


    impl<C> ValidateResult for Image<C> where C: ValidateResult {
        fn validate_result(&self, other: &Self, options: ValidationOptions, location: impl Fn()->String) -> ValidationResult {
            if self.attributes != other.attributes { Err(location() + "| image > attributes") }
            else { self.layer_data.validate_result(&other.layer_data, options, || location() + "| image > layer data") }
        }
    }

    impl<S> ValidateResult for Layer<AnyChannels<S>>
        where AnyChannel<S>: ValidateResult, S: for<'a> WritableSamples<'a>
    {
        fn validate_result(&self, other: &Self, _overridden: ValidationOptions, location: impl Fn()->String) -> ValidationResult {
            let location = || format!("{} (layer `{:?}`)", location(), self.attributes.layer_name);
            if self.attributes != other.attributes { Err(location() + " > attributes") }
            else if self.encoding != other.encoding { Err(location() + " > encoding") }
            else if self.size != other.size { Err(location() + " > size") }
            else if self.channel_data.list.len() != other.channel_data.list.len() { Err(location() + " > channel count") }
            else {
                for (own_chan, other_chan) in self.channel_data.list.iter().zip(other.channel_data.list.iter()) {
                    own_chan.validate_result(
                        other_chan,

                        ValidationOptions {
                            // no tolerance for lossless channels
                            allow_lossy: other.encoding.compression
                                .is_lossless_for(other_chan.sample_data.sample_type()).not(),

                            // consider nan and zero equal if the compression method does not support nan
                            nan_converted_to_zero: other.encoding.compression.supports_nan().not()
                        },

                        || format!("{} > channel `{}`", location(), own_chan.name)
                    )?;
                }
                Ok(())
            }
        }
    }

    impl<Px, Desc> ValidateResult for Layer<SpecificChannels<Px, Desc>>
        where SpecificChannels<Px, Desc>: ValidateResult
    {
        /// This does an approximate comparison for all channels,
        /// even if some channels can be compressed without loss.
        fn validate_result(&self, other: &Self, _overridden: ValidationOptions, location: impl Fn()->String) -> ValidationResult {
            let location = || format!("{} (layer `{:?}`)", location(), self.attributes.layer_name);

            // TODO dedup with above
            if self.attributes != other.attributes { Err(location() + " > attributes") }
            else if self.encoding != other.encoding { Err(location() + " > encoding") }
            else if self.size != other.size { Err(location() + " > size") }
            else {
                let options = ValidationOptions {
                    // no tolerance for lossless channels
                    // pxr only looses data for f32 values, B44 only for f16, not other any other types
                    allow_lossy: other.encoding.compression.may_loose_data(),// TODO check specific channels sample types

                    // consider nan and zero equal if the compression method does not support nan
                    nan_converted_to_zero: other.encoding.compression.supports_nan().not()
                };

                self.channel_data.validate_result(&other.channel_data, options, || location() + " > channel_data")?;
                Ok(())
            }
        }
    }

    impl<S> ValidateResult for AnyChannels<S> where S: ValidateResult {
        fn validate_result(&self, other: &Self, options: ValidationOptions, location: impl Fn()->String) -> ValidationResult {
            self.list.validate_result(&other.list, options, location)
        }
    }

    impl<S> ValidateResult for AnyChannel<S> where S: ValidateResult {
        fn validate_result(&self, other: &Self, options: ValidationOptions, location: impl Fn()->String) -> ValidationResult {
            if self.name != other.name { Err(location() + " > name") }
            else if self.quantize_linearly != other.quantize_linearly { Err(location() + " > quantize_linearly") }
            else if self.sampling != other.sampling { Err(location() + " > sampling") }
            else {
                self.sample_data.validate_result(&other.sample_data, options, || location() + " > sample_data")
            }
        }
    }

    impl<Pxs, Chans> ValidateResult for SpecificChannels<Pxs, Chans> where Pxs: ValidateResult, Chans: Eq {
        fn validate_result(&self, other: &Self, options: ValidationOptions, location: impl Fn()->String) -> ValidationResult {
            if self.channels != other.channels { Err(location() + " > specific channels") }
            else { self.pixels.validate_result(&other.pixels, options, || location() + " > specific pixels") }
        }
    }

    impl<S> ValidateResult for Levels<S> where S: ValidateResult {
        fn validate_result(&self, other: &Self, options: ValidationOptions, location: impl Fn()->String) -> ValidationResult {
            self.levels_as_slice().validate_result(&other.levels_as_slice(), options, || location() + " > levels")
        }
    }

    impl ValidateResult for FlatSamples {
        fn validate_result(&self, other: &Self, options: ValidationOptions, location: impl Fn()->String) -> ValidationResult {
            use FlatSamples::*;
            match (self, other) {
                (F16(values), F16(other_values)) => values.as_slice().validate_result(&other_values.as_slice(), options, ||location() + " > f16 samples"),
                (F32(values), F32(other_values)) => values.as_slice().validate_result(&other_values.as_slice(), options, ||location() + " > f32 samples"),
                (U32(values), U32(other_values)) => values.as_slice().validate_result(&other_values.as_slice(), options, ||location() + " > u32 samples"),
                (own, other) => Err(format!("{}: samples type mismatch. expected {:?}, found {:?}", location(), own.sample_type(), other.sample_type()))
            }
        }
    }

    impl<T> ValidateResult for &[T] where T: ValidateResult {
        fn validate_result(&self, other: &Self, options: ValidationOptions, location: impl Fn()->String) -> ValidationResult {
            if self.len() != other.len() { Err(location() + " count") }
            else {
                for (index, (slf, other)) in self.iter().zip(other.iter()).enumerate() {
                    slf.validate_result(other, options, ||format!("{} element [{}] of {}", location(), index, self.len()))?;
                }
                Ok(())
            }
        }
    }

    impl<A: Array> ValidateResult for SmallVec<A> where A::Item: ValidateResult {
        fn validate_result(&self, other: &Self, options: ValidationOptions, location: impl Fn()->String) -> ValidationResult {
            self.as_slice().validate_result(&other.as_slice(), options, location)
        }
    }

    impl<A> ValidateResult for Vec<A> where A: ValidateResult {
        fn validate_result(&self, other: &Self, options: ValidationOptions, location: impl Fn()->String) -> ValidationResult {
            self.as_slice().validate_result(&other.as_slice(), options, location)
        }
    }

    impl<A,B,C,D> ValidateResult for (A, B, C, D) where A: Clone+ ValidateResult, B: Clone+ ValidateResult, C: Clone+ ValidateResult, D: Clone+ ValidateResult {
        fn validate_result(&self, other: &Self, options: ValidationOptions, location: impl Fn()->String) -> ValidationResult {
            self.clone().into_recursive().validate_result(&other.clone().into_recursive(), options, location)
        }
    }

    impl<A,B,C> ValidateResult for (A, B, C) where A: Clone+ ValidateResult, B: Clone+ ValidateResult, C: Clone+ ValidateResult {
        fn validate_result(&self, other: &Self, options: ValidationOptions, location: impl Fn()->String) -> ValidationResult {
            self.clone().into_recursive().validate_result(&other.clone().into_recursive(), options, location)
        }
    }

    // // (low priority because it is only used in the tests)
    /*TODO
    impl<Tuple> SimilarToLossy for Tuple where
        Tuple: Clone + IntoRecursive,
        <Tuple as IntoRecursive>::Recursive: SimilarToLossy,
    {
        fn similar_to_lossy(&self, other: &Self, max_difference: f32) -> bool {
            self.clone().into_recursive().similar_to_lossy(&other.clone().into_recursive(), max_difference)
        } // TODO no clone?
    }*/


    // implement for recursive types
    impl ValidateResult for NoneMore {
        fn validate_result(&self, _: &Self, _: ValidationOptions, _: impl Fn()->String) -> ValidationResult { Ok(()) }
    }

    impl<Inner, T> ValidateResult for Recursive<Inner, T> where Inner: ValidateResult, T: ValidateResult {
        fn validate_result(&self, other: &Self, options: ValidationOptions, location: impl Fn()->String) -> ValidationResult {
            self.value.validate_result(&other.value, options, &location).and_then(|()|
                self.inner.validate_result(&other.inner, options, &location)
            )
        }
    }

    impl<S> ValidateResult for Option<S> where S: ValidateResult {
        fn validate_result(&self, other: &Self, options: ValidationOptions, location: impl Fn()->String) -> ValidationResult {
            match (self, other) {
                (None, None) => Ok(()),
                (Some(value), Some(other)) => value.validate_result(other, options, location),
                _ => Err(location() + ": option mismatch")
            }
        }
    }

    impl ValidateResult for f32 {
        fn validate_result(&self, other: &Self, options: ValidationOptions, location: impl Fn()->String) -> ValidationResult {
            if self == other || (self.is_nan() && other.is_nan()) || (options.nan_converted_to_zero && !self.is_normal() && *other == 0.0) {
                return Ok(());
            }

            if options.allow_lossy {
                let epsilon = 0.06;
                let max_difference = 0.1;

                let adaptive_threshold = epsilon * (self.abs() + other.abs());
                let tolerance = adaptive_threshold.max(max_difference);
                let difference = (self - other).abs();

                return if difference <= tolerance { Ok(()) }
                else { Err(format!("{}: expected ~{}, found {} (adaptive tolerance {})", location(), self, other, tolerance)) };
            }

            Err(format!("{}: expected exactly {}, found {}", location(), self, other))
        }
    }

    impl ValidateResult for f16 {
        fn validate_result(&self, other: &Self, options: ValidationOptions, location: impl Fn()->String) -> ValidationResult {
            if self.to_bits() == other.to_bits() { Ok(()) } else {
                self.to_f32().validate_result(&other.to_f32(), options, location)
            }
        }
    }

    impl ValidateResult for u32 {
        fn validate_result(&self, other: &Self, options: ValidationOptions, location: impl Fn()->String) -> ValidationResult {
            if self == other { Ok(()) } else { // todo to float conversion resulting in nan/infinity?
                self.to_f32().validate_result(&other.to_f32(), options, location)
            }
        }
    }

    impl ValidateResult for Sample {
        fn validate_result(&self, other: &Self, options: ValidationOptions, location: impl Fn()->String) -> ValidationResult {
            use Sample::*;
            match (self, other) {
                (F16(a), F16(b)) => a.validate_result(b, options, ||location() + " (f16)"),
                (F32(a), F32(b)) => a.validate_result(b, options, ||location() + " (f32)"),
                (U32(a), U32(b)) => a.validate_result(b, options, ||location() + " (u32)"),
                (_,_) => Err(location() + ": sample type mismatch")
            }
        }
    }


    #[cfg(test)]
    mod test_value_result {
        use std::f32::consts::*;
        use std::io::Cursor;
        use crate::image::pixel_vec::PixelVec;
        use crate::image::validate_results::{ValidateResult, ValidationOptions};
        use crate::meta::attribute::LineOrder::Increasing;
        use crate::image::{FlatSamples};

        fn expect_valid<T>(original: &T, result: &T, allow_lossy: bool, nan_converted_to_zero: bool) where T: ValidateResult {
            original.validate_result(
                result,
                ValidationOptions { allow_lossy, nan_converted_to_zero },
                || String::new()
            ).unwrap();
        }

        fn expect_invalid<T>(original: &T, result: &T, allow_lossy: bool, nan_converted_to_zero: bool) where T: ValidateResult {
            assert!(original.validate_result(
                result,
                ValidationOptions { allow_lossy, nan_converted_to_zero },
                || String::new()
            ).is_err());
        }

        #[test]
        fn test_f32(){
            let original:&[f32] = &[0.0, 0.1, 0.2, 0.3, 0.4, 0.5, -20.4, f32::NAN];
            let lossy:&[f32] = &[0.0, 0.2, 0.2, 0.3, 0.4, 0.5, -20.5, f32::NAN];

            expect_valid(&original, &original, true, true);
            expect_valid(&original, &original, true, false);
            expect_valid(&original, &original, false, true);
            expect_valid(&original, &original, false, false);

            expect_invalid(&original, &lossy, false, false);
            expect_valid(&original, &lossy, true, false);

            expect_invalid(&original, &&original[..original.len()-2], true, true);

            // test relative comparison with some large values
            expect_valid(&1_000_f32, &1_001_f32, true, false);
            expect_invalid(&1_000_f32, &1_200_f32, true, false);

            expect_valid(&10_000_f32, &10_100_f32, true, false);
            expect_invalid(&10_000_f32, &12_000_f32, true, false);

            expect_valid(&33_120_f32, &30_120_f32, true, false);
            expect_invalid(&33_120_f32, &20_120_f32, true, false);
        }

        #[test]
        fn test_nan(){
            let original:&[f32] = &[ 0.0, f32::NAN, f32::NAN ];
            let lossy:&[f32] = &[ 0.0, f32::NAN, 0.0 ];

            expect_valid(&original, &lossy, true, true);
            expect_invalid(&lossy, &original, true, true);

            expect_valid(&lossy, &lossy, true, true);
            expect_valid(&lossy, &lossy, false, true);
        }

        #[test]
        fn test_error(){

            fn print_error<T: ValidateResult>(original: &T, lossy: &T, allow_lossy: bool){
                let message = original
                    .validate_result(
                        &lossy,
                        ValidationOptions { allow_lossy, .. Default::default() },
                        || String::new() // type_name::<T>().to_string()
                    )
                    .unwrap_err();

                println!("message: {}", message);
            }

            let original:&[f32] = &[ 0.0, f32::NAN, f32::NAN ];
            let lossy:&[f32] = &[ 0.0, f32::NAN, 0.0 ];
            print_error(&original, &lossy, false);

            print_error(&2.0, &1.0, true);
            print_error(&2.0, &1.0, false);

            print_error(&FlatSamples::F32(vec![0.1,0.1]), &FlatSamples::F32(vec![0.1,0.2]), false);
            print_error(&FlatSamples::U32(vec![0,0]), &FlatSamples::F32(vec![0.1,0.2]), false);

            {
                let image = crate::prelude::read_all_data_from_file("tests/images/valid/openexr/MultiResolution/Kapaa.exr").unwrap();

                let mut mutated = image.clone();
                let samples = mutated.layer_data.first_mut().unwrap()
                    .channel_data.list.first_mut().unwrap().sample_data.levels_as_slice_mut().first_mut().unwrap();

                match samples {
                    FlatSamples::F16(vals) => vals[100] = vals[1],
                    FlatSamples::F32(vals) => vals[100] = vals[1],
                    FlatSamples::U32(vals) => vals[100] = vals[1],
                }

                print_error(&image, &mutated, false);
            }

            // TODO check out more nested behaviour!
        }

        #[test]
        fn test_uncompressed(){
            use crate::prelude::*;

            let original_pixels: [(f32,f32,f32); 4] = [
                (0.0, -1.1, PI),
                (0.0, -1.1, TAU),
                (0.0, -1.1, f32::EPSILON),
                (f32::NAN, 10000.1, -1024.009),
            ];

            let mut file_bytes = Vec::new();
            let original_image = Image::from_encoded_channels(
                (2,2),
                Encoding {
                    compression: Compression::Uncompressed,
                    line_order: Increasing, // FIXME unspecified may be optimized to increasing, which destroys test eq
                    .. Encoding::default()
                },
                SpecificChannels::rgb(PixelVec::new(Vec2(2,2), original_pixels.to_vec()))
            );

            original_image.write().to_buffered(Cursor::new(&mut file_bytes)).unwrap();

            let lossy_image = read().no_deep_data().largest_resolution_level()
                .rgb_channels(PixelVec::<(f32,f32,f32)>::constructor, PixelVec::set_pixel)
                .first_valid_layer().all_attributes().from_buffered(Cursor::new(&file_bytes)).unwrap();

            original_image.assert_equals_result(&original_image);
            lossy_image.assert_equals_result(&lossy_image);
            original_image.assert_equals_result(&lossy_image);
            lossy_image.assert_equals_result(&original_image);
        }

        #[test]
        fn test_compiles(){
            use crate::prelude::*;

            fn accepts_validatable_value(_: &impl ValidateResult){}

            let object: Levels<FlatSamples> = Levels::Singular(FlatSamples::F32(Vec::default()));
            accepts_validatable_value(&object);

            let object: AnyChannels<Levels<FlatSamples>> = AnyChannels::sort(SmallVec::default());
            accepts_validatable_value(&object);

            let layer: Layer<AnyChannels<Levels<FlatSamples>>> = Layer::new((0,0), Default::default(), Default::default(), object);
            accepts_validatable_value(&layer);

            let layers: Layers<AnyChannels<Levels<FlatSamples>>> = Default::default();
            accepts_validatable_value(&layers);

            let object: Image<Layer<AnyChannels<Levels<FlatSamples>>>> = Image::from_layer(layer);
            object.assert_equals_result(&object);
        }
    }


    #[test]
    fn test_nan_compression_attribute(){
        use crate::prelude::*;
        use crate::prelude::Compression::*;
        use std::io::Cursor;
        use crate::image::pixel_vec::PixelVec;
        use crate::prelude::LineOrder::Increasing;

        let all_compression_methods = [
            Uncompressed, RLE, ZIP1, ZIP16, PXR24, PIZ, B44, B44A,
        ];

        let original_pixels: [(f32,f32,f16); 4] = [
            (f32::NAN, f32::from_bits(0x7fc01234), f16::from_bits(0x7E01)),
            (f32::NAN, f32::from_bits(0xffcabcde), f16::from_bits(0x7FFF)),
            (f32::NAN, f32::from_bits(0x7f800001), f16::from_bits(0xFE01)),
            (f32::NAN, f32::NAN, f16::NAN),
        ];

        assert!(
            original_pixels.iter()
                .all(|&(a,b,c)| a.is_nan() && b.is_nan() && c.is_nan()),
            "test case has a bug"
        );

        for compression in all_compression_methods {
            let mut file_bytes = Vec::new();

            let original_image = Image::from_encoded_channels(
                (2, 2),
                Encoding {
                    compression,
                    line_order: Increasing,
                    ..Encoding::default()
                },
                SpecificChannels::rgb(PixelVec::new((2, 2), original_pixels.to_vec()))
            );

            let result = original_image.write().to_buffered(Cursor::new(&mut file_bytes));
            if let Err(Error::NotSupported(_)) = result { continue; }

            let reconstructed_image =
                read().no_deep_data().largest_resolution_level()
                    .rgb_channels(PixelVec::<(f32, f32, f16)>::constructor, PixelVec::set_pixel)
                    .first_valid_layer().all_attributes().from_buffered(Cursor::new(&file_bytes)).unwrap();

            assert_eq!(
                original_image.layer_data.channel_data.pixels.pixels.len(),
                reconstructed_image.layer_data.channel_data.pixels.pixels.len()
            );

            let was_nanness_preserved = reconstructed_image.layer_data.channel_data.pixels.pixels
                .iter().all(|(r,g,b)| r.is_nan() && g.is_nan() && b.is_nan());

            assert_eq!(
                was_nanness_preserved, compression.supports_nan(),
                "{} nanness claims do not match real output", compression
            );

            let was_nan_pattern_preserved = reconstructed_image.layer_data.channel_data.pixels.pixels
                .iter().zip(original_pixels.iter())
                .all(|((r2, g2, b2), (r1, g1, b1))|
                    r2.to_bits() == r1.to_bits() &&
                    g2.to_bits() == g1.to_bits() &&
                    b2.to_bits() == b1.to_bits()
                );

            assert_eq!(
                was_nan_pattern_preserved, compression.preserves_nan_bits(),
                "{} nan bit claims do not match real output", compression
            );
        }
    }
}



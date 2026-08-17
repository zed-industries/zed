//! How to read a set of resolution levels.

use crate::meta::*;
use crate::image::*;
use crate::error::*;
use crate::meta::attribute::*;
use crate::image::read::any_channels::*;
use crate::block::chunk::TileCoordinates;
use crate::image::read::specific_channels::*;
use crate::image::recursive::*;
use crate::math::Vec2;
use crate::block::lines::LineRef;
use crate::block::samples::*;
use crate::meta::header::{Header};


// Note: In the resulting image, the `FlatSamples` are placed
// directly inside the channels, without `LargestLevel<>` indirection
/// Specify to read only the highest resolution level, skipping all smaller variations.
/// The sample storage can be [`ReadFlatSamples`].
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ReadLargestLevel<DeepOrFlatSamples> {

    /// The sample reading specification
    pub read_samples: DeepOrFlatSamples
}


// FIXME rgba levels???

// Read the largest level, directly, without intermediate structs
impl<DeepOrFlatSamples> ReadLargestLevel<DeepOrFlatSamples> {

    /// Read all arbitrary channels in each layer.
    pub fn all_channels(self) -> ReadAnyChannels<DeepOrFlatSamples> { ReadAnyChannels { read_samples: self.read_samples } } // Instead of Self, the `FlatSamples` are used directly

    /// Read only layers that contain rgba channels. Skips any other channels in the layer.
    /// The alpha channel will contain the value `1.0` if no alpha channel can be found in the image.
    ///
    /// Using two closures, define how to store the pixels.
    /// The first closure creates an image, and the second closure inserts a single pixel.
    /// The type of the pixel can be defined by the second closure;
    /// it must be a tuple containing four values, each being either `f16`, `f32`, `u32` or `Sample`.
    ///
    /// Throws an error for images with deep data or subsampling.
    /// Use `specific_channels` or `all_channels` if you want to read something other than rgba.
    pub fn rgba_channels<R,G,B,A, Create, Set, Pixels>(
        self, create_pixels: Create, set_pixel: Set
    ) -> CollectPixels<
        ReadOptionalChannel<ReadRequiredChannel<ReadRequiredChannel<ReadRequiredChannel<NoneMore, R>, G>, B>, A>,
        (R, G, B, A), Pixels, Create, Set
    >
        where
            R: FromNativeSample, G: FromNativeSample, B: FromNativeSample, A: FromNativeSample,
            Create: Fn(Vec2<usize>, &RgbaChannels) -> Pixels,
            Set: Fn(&mut Pixels, Vec2<usize>, (R,G,B,A)),
    {
        self.specific_channels()
            .required("R").required("G").required("B")
            .optional("A", A::from_f32(1.0))
            .collect_pixels(create_pixels, set_pixel)
    }

    /// Read only layers that contain rgb channels. Skips any other channels in the layer.
    ///
    /// Using two closures, define how to store the pixels.
    /// The first closure creates an image, and the second closure inserts a single pixel.
    /// The type of the pixel can be defined by the second closure;
    /// it must be a tuple containing three values, each being either `f16`, `f32`, `u32` or `Sample`.
    ///
    /// Throws an error for images with deep data or subsampling.
    /// Use `specific_channels` or `all_channels` if you want to read something other than rgb.
    pub fn rgb_channels<R,G,B, Create, Set, Pixels>(
        self, create_pixels: Create, set_pixel: Set
    ) -> CollectPixels<
        ReadRequiredChannel<ReadRequiredChannel<ReadRequiredChannel<NoneMore, R>, G>, B>,
        (R, G, B), Pixels, Create, Set
    >
        where
            R: FromNativeSample, G: FromNativeSample, B: FromNativeSample,
            Create: Fn(Vec2<usize>, &RgbChannels) -> Pixels,
            Set: Fn(&mut Pixels, Vec2<usize>, (R,G,B)),
    {
        self.specific_channels()
            .required("R").required("G").required("B")
            .collect_pixels(create_pixels, set_pixel)
    }

    /// Read only layers that contain the specified channels, skipping any other channels in the layer.
    /// Further specify which channels should be included by calling `.required("ChannelName")`
    /// or `.optional("ChannelName", default_value)` on the result of this function.
    /// Call `collect_pixels` afterwards to define the pixel container for your set of channels.
    ///
    /// Throws an error for images with deep data or subsampling.
    pub fn specific_channels(self) -> ReadZeroChannels {
        ReadZeroChannels { }
    }
}

/// Specify to read all contained resolution levels from the image, if any.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ReadAllLevels<DeepOrFlatSamples> {

    /// The sample reading specification
    pub read_samples: DeepOrFlatSamples
}

impl<ReadDeepOrFlatSamples> ReadAllLevels<ReadDeepOrFlatSamples> {

    /// Read all arbitrary channels in each layer.
    pub fn all_channels(self) -> ReadAnyChannels<Self> { ReadAnyChannels { read_samples: self } }

    // TODO specific channels for multiple resolution levels

}

/*pub struct ReadLevels<S> {
    read_samples: S,
}*/

/// Processes pixel blocks from a file and accumulates them into multiple levels per channel.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AllLevelsReader<SamplesReader> {
    levels: Levels<SamplesReader>,
}

/// A template that creates a [`SamplesReader`] once for each resolution level.
pub trait ReadSamplesLevel {

    /// The type of the temporary level reader
    type Reader: SamplesReader;

    /// Create a single reader for a single resolution level
    fn create_samples_level_reader(&self, header: &Header, channel: &ChannelDescription, level: Vec2<usize>, resolution: Vec2<usize>) -> Result<Self::Reader>;
}


impl<S: ReadSamplesLevel> ReadSamples for ReadAllLevels<S> {
    type Reader = AllLevelsReader<S::Reader>;

    fn create_sample_reader(&self, header: &Header, channel: &ChannelDescription) -> Result<Self::Reader> {
        let data_size = header.layer_size / channel.sampling;

        let levels = {
            if let crate::meta::BlockDescription::Tiles(tiles) = &header.blocks {
                match tiles.level_mode {
                    LevelMode::Singular => Levels::Singular(self.read_samples.create_samples_level_reader(header, channel, Vec2(0,0), header.layer_size)?),

                    LevelMode::MipMap => Levels::Mip {
                        rounding_mode: tiles.rounding_mode,
                        level_data: {
                            let round = tiles.rounding_mode;
                            let maps: Result<LevelMaps<S::Reader>> = mip_map_levels(round, data_size)
                                .map(|(index, level_size)| self.read_samples.create_samples_level_reader(header, channel, Vec2(index, index), level_size))
                                .collect();

                            maps?
                        },
                    },

                    // TODO put this into Levels::new(..) ?
                    LevelMode::RipMap => Levels::Rip {
                        rounding_mode: tiles.rounding_mode,
                        level_data: {
                            let round = tiles.rounding_mode;
                            let level_count_x = compute_level_count(round, data_size.width());
                            let level_count_y = compute_level_count(round, data_size.height());
                            let maps: Result<LevelMaps<S::Reader>> = rip_map_levels(round, data_size)
                                .map(|(index, level_size)| self.read_samples.create_samples_level_reader(header, channel, index, level_size))
                                .collect();

                            RipMaps {
                                map_data: maps?,
                                level_count: Vec2(level_count_x, level_count_y)
                            }
                        },
                    },
                }
            }

            // scan line blocks never have mip maps
            else {
                Levels::Singular(self.read_samples.create_samples_level_reader(header, channel, Vec2(0, 0), data_size)?)
            }
        };

        Ok(AllLevelsReader { levels })
    }
}


impl<S: SamplesReader> SamplesReader for AllLevelsReader<S> {
    type Samples = Levels<S::Samples>;

    fn filter_block(&self, _: TileCoordinates) -> bool {
        true
    }

    fn read_line(&mut self, line: LineRef<'_>) -> UnitResult {
        self.levels.get_level_mut(line.location.level)?.read_line(line)
    }

    fn into_samples(self) -> Self::Samples {
        match self.levels {
            Levels::Singular(level) => Levels::Singular(level.into_samples()),
            Levels::Mip { rounding_mode, level_data } => Levels::Mip {
                rounding_mode, level_data: level_data.into_iter().map(|s| s.into_samples()).collect(),
            },

            Levels::Rip { rounding_mode, level_data } => Levels::Rip {
                rounding_mode,
                level_data: RipMaps {
                    level_count: level_data.level_count,
                    map_data: level_data.map_data.into_iter().map(|s| s.into_samples()).collect(),
                }
            },
        }
    }
}

//! How to write samples (a grid of `f32`, `f16` or `u32` values).

use crate::meta::attribute::{LevelMode, SampleType, TileDescription};
use crate::meta::header::Header;
use crate::block::lines::LineRefMut;
use crate::image::{FlatSamples, Levels, RipMaps};
use crate::math::{Vec2, RoundingMode};
use crate::meta::{rip_map_levels, mip_map_levels, rip_map_indices, mip_map_indices, BlockDescription};

/// Enable an image with this sample grid to be written to a file.
/// Also can contain multiple resolution levels.
/// Usually contained within `Channels`.
pub trait WritableSamples<'slf> {
    // fn is_deep(&self) -> bool;

    /// Generate the file meta data regarding the number type of this storage
    fn sample_type(&self) -> SampleType;

    /// Generate the file meta data regarding resolution levels
    fn infer_level_modes(&self) -> (LevelMode, RoundingMode);

    /// The type of the temporary writer for this sample storage
    type Writer: SamplesWriter;

    /// Create a temporary writer for this sample storage
    fn create_samples_writer(&'slf self, header: &Header) -> Self::Writer;
}

/// Enable an image with this single level sample grid to be written to a file.
/// Only contained within `Levels`.
pub trait WritableLevel<'slf> {

    /// Generate the file meta data regarding the number type of these samples
    fn sample_type(&self) -> SampleType;

    /// The type of the temporary writer for this single level of samples
    type Writer: SamplesWriter;

    /// Create a temporary writer for this single level of samples
    fn create_level_writer(&'slf self, size: Vec2<usize>) -> Self::Writer;
}

/// A temporary writer for one or more resolution levels containing samples
pub trait SamplesWriter: Sync {

    /// Deliver a single short horizontal list of samples for a specific channel.
    fn extract_line(&self, line: LineRefMut<'_>);
}

/// A temporary writer for a predefined non-deep sample storage
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FlatSamplesWriter<'samples> {
    resolution: Vec2<usize>, // respects resolution level
    samples: &'samples FlatSamples
}



// used if no layers are used and the flat samples are directly inside the channels
impl<'samples> WritableSamples<'samples> for FlatSamples {
    fn sample_type(&self) -> SampleType {
        match self {
            FlatSamples::F16(_) => SampleType::F16,
            FlatSamples::F32(_) => SampleType::F32,
            FlatSamples::U32(_) => SampleType::U32,
        }
    }

    fn infer_level_modes(&self) -> (LevelMode, RoundingMode) { (LevelMode::Singular, RoundingMode::Down) }

    type Writer = FlatSamplesWriter<'samples>; //&'s FlatSamples;
    fn create_samples_writer(&'samples self, header: &Header) -> Self::Writer {
        FlatSamplesWriter {
            resolution: header.layer_size,
            samples: self
        }
    }
}

// used if layers are used and the flat samples are inside the levels
impl<'samples> WritableLevel<'samples> for FlatSamples {
    fn sample_type(&self) -> SampleType {
        match self {
            FlatSamples::F16(_) => SampleType::F16,
            FlatSamples::F32(_) => SampleType::F32,
            FlatSamples::U32(_) => SampleType::U32,
        }
    }

    type Writer = FlatSamplesWriter<'samples>;
    fn create_level_writer(&'samples self, size: Vec2<usize>) -> Self::Writer {
        FlatSamplesWriter {
            resolution: size,
            samples: self
        }
    }
}

impl<'samples> SamplesWriter for FlatSamplesWriter<'samples> {
    fn extract_line(&self, line: LineRefMut<'_>) {
        let image_width = self.resolution.width(); // header.layer_size.width();
        debug_assert_ne!(image_width, 0, "image width calculation bug");

        let start_index = line.location.position.y() * image_width + line.location.position.x();
        let end_index = start_index + line.location.sample_count;

        debug_assert!(
            start_index < end_index && end_index <= self.samples.len(),
            "for resolution {:?}, this is an invalid line: {:?}",
            self.resolution, line.location
        );

        match self.samples {
            FlatSamples::F16(samples) => line.write_samples_from_slice(&samples[start_index .. end_index]),
            FlatSamples::F32(samples) => line.write_samples_from_slice(&samples[start_index .. end_index]),
            FlatSamples::U32(samples) => line.write_samples_from_slice(&samples[start_index .. end_index]),
        }.expect("writing line bytes failed");
    }
}


impl<'samples, LevelSamples> WritableSamples<'samples> for Levels<LevelSamples>
    where LevelSamples: WritableLevel<'samples>
{
    fn sample_type(&self) -> SampleType {
        let sample_type = self.levels_as_slice().first().expect("no levels found").sample_type();

        debug_assert!(
            self.levels_as_slice().iter().skip(1).all(|ty| ty.sample_type() == sample_type),
            "sample types must be the same across all levels"
        );

        sample_type
    }

    fn infer_level_modes(&self) -> (LevelMode, RoundingMode) {
        match self {
            Levels::Singular(_) => (LevelMode::Singular, RoundingMode::Down),
            Levels::Mip { rounding_mode, .. } => (LevelMode::MipMap, *rounding_mode),
            Levels::Rip { rounding_mode, .. } => (LevelMode::RipMap, *rounding_mode),
        }
    }

    type Writer = LevelsWriter<LevelSamples::Writer>;
    fn create_samples_writer(&'samples self, header: &Header) -> Self::Writer {
        let rounding = match header.blocks {
            BlockDescription::Tiles(TileDescription { rounding_mode, .. }) => Some(rounding_mode),
            BlockDescription::ScanLines => None,
        };

        LevelsWriter {
            levels: match self {
                Levels::Singular(level) => Levels::Singular(level.create_level_writer(header.layer_size)),
                Levels::Mip { level_data, rounding_mode } => {
                    debug_assert_eq!(
                        level_data.len(),
                        mip_map_indices(rounding.expect("mip maps only with tiles"), header.layer_size).count(),
                        "invalid mip map count"
                    );

                    Levels::Mip { // TODO store level size in image??
                        rounding_mode: *rounding_mode,
                        level_data: level_data.iter()
                            .zip(mip_map_levels(rounding.expect("mip maps only with tiles"), header.layer_size))
                            // .map(|level| level.create_samples_writer(header))
                            .map(|(level, (_level_index, level_size))| level.create_level_writer(level_size))
                            .collect()
                    }
                },
                Levels::Rip { level_data, rounding_mode } => {
                    debug_assert_eq!(level_data.map_data.len(), level_data.level_count.area(), "invalid rip level count");
                    debug_assert_eq!(
                        level_data.map_data.len(),
                        rip_map_indices(rounding.expect("rip maps only with tiles"), header.layer_size).count(),
                        "invalid rip map count"
                    );

                    Levels::Rip {
                        rounding_mode: *rounding_mode,
                        level_data: RipMaps {
                            level_count: level_data.level_count,
                            map_data: level_data.map_data.iter()
                                .zip(rip_map_levels(rounding.expect("rip maps only with tiles"), header.layer_size))
                                .map(|(level, (_level_index, level_size))| level.create_level_writer(level_size))
                                .collect(),
                        }
                    }
                }
            }
        }
    }
}

/// A temporary writer for multiple resolution levels
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LevelsWriter<SamplesWriter> {
    levels: Levels<SamplesWriter>,
}

impl<Samples> SamplesWriter for LevelsWriter<Samples> where Samples: SamplesWriter {
    fn extract_line(&self, line: LineRefMut<'_>) {
        self.levels.get_level(line.location.level).expect("invalid level index") // TODO compute level size from line index??
            .extract_line(line)
    }
}

// Copyright (c) 2020-2023, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use num_derive::*;

use crate::partition::BlockSize;
use crate::serialize::{Deserialize, Serialize};

use std::fmt;

// NOTE: Add Structures at the end.
/// Contains the speed settings.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SpeedSettings {
  /// Enables inter-frames to have multiple reference frames.
  ///
  /// Enabled is slower.
  pub multiref: bool,

  /// Enables fast deblocking filter.
  pub fast_deblock: bool,

  /// The number of lookahead frames to be used for temporal RDO.
  ///
  /// Higher is slower.
  pub rdo_lookahead_frames: usize,

  /// Which scene detection mode to use. Standard is slower, but best.
  pub scene_detection_mode: SceneDetectionSpeed,

  /// Enables CDEF.
  pub cdef: bool,

  /// Enables LRF.
  pub lrf: bool,

  /// Enable searching loop restoration units when no transforms have been coded
  /// restoration unit.
  pub lru_on_skip: bool,

  /// The amount of search done for self guided restoration.
  pub sgr_complexity: SGRComplexityLevel,

  /// Search level for segmentation.
  ///
  /// Full search is at least twice as slow.
  pub segmentation: SegmentationLevel,

  // NOTE: put enums and basic type fields above
  /// Speed settings related to partition decision
  pub partition: PartitionSpeedSettings,

  /// Speed settings related to transform size and type decision
  pub transform: TransformSpeedSettings,

  /// Speed settings related to intra prediction mode selection
  pub prediction: PredictionSpeedSettings,

  /// Speed settings related to motion estimation and motion vector selection
  pub motion: MotionSpeedSettings,
}

impl Default for SpeedSettings {
  /// The default settings are equivalent to speed 0
  fn default() -> Self {
    SpeedSettings {
      multiref: true,
      fast_deblock: false,
      rdo_lookahead_frames: 40,
      scene_detection_mode: SceneDetectionSpeed::Standard,
      cdef: true,
      lrf: true,
      lru_on_skip: true,
      sgr_complexity: SGRComplexityLevel::Full,
      segmentation: SegmentationLevel::Complex,
      partition: PartitionSpeedSettings {
        encode_bottomup: true,
        non_square_partition_max_threshold: BlockSize::BLOCK_64X64,
        partition_range: PartitionRange::new(
          BlockSize::BLOCK_4X4,
          BlockSize::BLOCK_64X64,
        ),
      },
      transform: TransformSpeedSettings {
        reduced_tx_set: false,
        // TX domain distortion is always faster, with no significant quality change,
        // although it will be ignored when Tune == Psychovisual.
        tx_domain_distortion: true,
        tx_domain_rate: false,
        rdo_tx_decision: true,
        enable_inter_tx_split: false,
      },
      prediction: PredictionSpeedSettings {
        prediction_modes: PredictionModesSetting::ComplexAll,
        fine_directional_intra: true,
      },
      motion: MotionSpeedSettings {
        include_near_mvs: true,
        use_satd_subpel: true,
        me_allow_full_search: true,
      },
    }
  }
}

impl SpeedSettings {
  /// Set the speed setting according to a numeric speed preset.
  pub fn from_preset(speed: u8) -> Self {
    // The default settings are equivalent to speed 0
    let mut settings = SpeedSettings::default();

    if speed >= 1 {
      settings.lru_on_skip = false;
      settings.segmentation = SegmentationLevel::Simple;
    }

    if speed >= 2 {
      settings.partition.non_square_partition_max_threshold =
        BlockSize::BLOCK_8X8;

      settings.prediction.prediction_modes =
        PredictionModesSetting::ComplexKeyframes;
    }

    if speed >= 3 {
      settings.rdo_lookahead_frames = 30;

      settings.partition.partition_range =
        PartitionRange::new(BlockSize::BLOCK_8X8, BlockSize::BLOCK_64X64);
    }

    if speed >= 4 {
      settings.partition.encode_bottomup = false;
    }

    if speed >= 5 {
      settings.sgr_complexity = SGRComplexityLevel::Reduced;
      settings.motion.include_near_mvs = false;
    }

    if speed >= 6 {
      settings.rdo_lookahead_frames = 20;

      settings.transform.rdo_tx_decision = false;
      settings.transform.reduced_tx_set = true;

      settings.motion.me_allow_full_search = false;
    }

    if speed >= 7 {
      settings.prediction.prediction_modes = PredictionModesSetting::Simple;
      // Multiref is enabled automatically if low_latency is false.
      //
      // If low_latency is true, enabling multiref allows using multiple
      // backwards references. low_latency false enables both forward and
      // backwards references.
      settings.multiref = false;
      settings.fast_deblock = true;
    }

    if speed >= 8 {
      settings.rdo_lookahead_frames = 10;
      settings.lrf = false;
    }

    if speed >= 9 {
      // 8x8 is fast enough to use until very high speed levels,
      // because 8x8 with reduced TX set is faster but with equivalent
      // or better quality compared to 16x16 (to which reduced TX set does not apply).
      settings.partition.partition_range =
        PartitionRange::new(BlockSize::BLOCK_16X16, BlockSize::BLOCK_32X32);

      // FIXME: With unknown reasons, inter_tx_split does not work if reduced_tx_set is false
      settings.transform.enable_inter_tx_split = true;
    }

    if speed >= 10 {
      settings.scene_detection_mode = SceneDetectionSpeed::Fast;

      settings.partition.partition_range =
        PartitionRange::new(BlockSize::BLOCK_32X32, BlockSize::BLOCK_32X32);

      settings.motion.use_satd_subpel = false;
    }

    settings
  }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
/// Speed settings related to transform size and type decision
pub struct TransformSpeedSettings {
  /// Enables reduced transform set.
  ///
  /// Enabled is faster.
  pub reduced_tx_set: bool,

  /// Enables using transform-domain distortion instead of pixel-domain.
  ///
  /// Enabled is faster.
  pub tx_domain_distortion: bool,

  /// Enables using transform-domain rate estimation.
  ///
  /// Enabled is faster.
  pub tx_domain_rate: bool,

  /// Enables searching transform size and type with RDO.
  ///
  /// Enabled is slower.
  pub rdo_tx_decision: bool,

  /// Enable tx split for inter mode block.
  pub enable_inter_tx_split: bool,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
/// Speed settings related to partition decision
pub struct PartitionSpeedSettings {
  /// Enables bottom-up encoding, rather than top-down.
  ///
  /// Enabled is slower.
  pub encode_bottomup: bool,

  /// Allow non-square partition type outside of frame borders
  /// on any blocks at or below this size.
  pub non_square_partition_max_threshold: BlockSize,

  /// Range of partition sizes that can be used. Larger ranges are slower.
  ///
  /// Must be based on square block sizes, so e.g. 8×4 isn't allowed here.
  pub partition_range: PartitionRange,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
/// Speed settings related to motion estimation and motion vector selection
pub struct MotionSpeedSettings {
  /// Use SATD instead of SAD for subpixel search.
  ///
  /// Enabled is slower.
  pub use_satd_subpel: bool,

  /// Enables searching near motion vectors during RDO.
  ///
  /// Enabled is slower.
  pub include_near_mvs: bool,

  /// Enable full search in some parts of motion estimation. Allowing full
  /// search is slower.
  pub me_allow_full_search: bool,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
/// Speed settings related to intra prediction mode selection
pub struct PredictionSpeedSettings {
  /// Prediction modes to search.
  ///
  /// Complex settings are slower.
  pub prediction_modes: PredictionModesSetting,

  /// Use fine directional intra prediction
  pub fine_directional_intra: bool,
}

/// Range of block sizes to use.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PartitionRange {
  pub(crate) min: BlockSize,
  pub(crate) max: BlockSize,
}

impl PartitionRange {
  /// Creates a new partition range with min and max partition sizes.
  ///
  /// # Panics
  ///
  /// - Panics if `max` is larger than `min`.
  /// - Panics if either `min` or `max` are not square.
  pub fn new(min: BlockSize, max: BlockSize) -> Self {
    assert!(max >= min);
    // Topdown search checks the min block size for PARTITION_SPLIT only, so
    // the min block size must be square.
    assert!(min.is_sqr());
    // Rectangular max partition sizes have not been tested.
    assert!(max.is_sqr());

    Self { min, max }
  }
}

#[cfg(test)]
impl Default for PartitionRange {
  fn default() -> Self {
    PartitionRange::new(BlockSize::BLOCK_4X4, BlockSize::BLOCK_64X64)
  }
}

/// Prediction modes to search.
#[derive(
  Clone,
  Copy,
  Debug,
  PartialOrd,
  PartialEq,
  Eq,
  FromPrimitive,
  Serialize,
  Deserialize,
)]
pub enum SceneDetectionSpeed {
  /// Fastest scene detection using pixel-wise comparison
  Fast,
  /// Scene detection using motion vectors and cost estimates
  Standard,
  /// Completely disable scene detection and only place keyframes
  /// at fixed intervals.
  None,
}

impl fmt::Display for SceneDetectionSpeed {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(
      f,
      "{}",
      match self {
        SceneDetectionSpeed::Fast => "Fast",
        SceneDetectionSpeed::Standard => "Standard",
        SceneDetectionSpeed::None => "None",
      }
    )
  }
}

/// Prediction modes to search.
#[derive(
  Clone,
  Copy,
  Debug,
  PartialOrd,
  PartialEq,
  Eq,
  FromPrimitive,
  Serialize,
  Deserialize,
)]
#[cfg_attr(test, derive(Default))]
pub enum PredictionModesSetting {
  /// Only simple prediction modes.
  #[cfg_attr(test, default)]
  Simple,
  /// Search all prediction modes on key frames and simple modes on other
  /// frames.
  ComplexKeyframes,
  /// Search all prediction modes on all frames.
  ComplexAll,
}

impl fmt::Display for PredictionModesSetting {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(
      f,
      "{}",
      match self {
        PredictionModesSetting::Simple => "Simple",
        PredictionModesSetting::ComplexKeyframes => "Complex-KFs",
        PredictionModesSetting::ComplexAll => "Complex-All",
      }
    )
  }
}

/// Search level for self guided restoration
#[derive(
  Clone,
  Copy,
  Debug,
  PartialOrd,
  PartialEq,
  Eq,
  FromPrimitive,
  Serialize,
  Deserialize,
)]
pub enum SGRComplexityLevel {
  /// Search all sgr parameters
  Full,
  /// Search a reduced set of sgr parameters
  Reduced,
}

impl fmt::Display for SGRComplexityLevel {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(
      f,
      "{}",
      match self {
        SGRComplexityLevel::Full => "Full",
        SGRComplexityLevel::Reduced => "Reduced",
      }
    )
  }
}

/// Search level for segmentation
#[derive(
  Clone,
  Copy,
  Debug,
  PartialOrd,
  PartialEq,
  Eq,
  FromPrimitive,
  Serialize,
  Deserialize,
)]
pub enum SegmentationLevel {
  /// No segmentation is signalled.
  Disabled,
  /// Segmentation index is derived from source statistics.
  Simple,
  /// Segmentation index range is derived from source statistics.
  Complex,
  /// Search all segmentation indices.
  Full,
}

impl fmt::Display for SegmentationLevel {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(
      f,
      "{}",
      match self {
        SegmentationLevel::Disabled => "Disabled",
        SegmentationLevel::Simple => "Simple",
        SegmentationLevel::Complex => "Complex",
        SegmentationLevel::Full => "Full",
      }
    )
  }
}

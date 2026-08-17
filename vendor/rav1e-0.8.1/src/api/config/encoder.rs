// Copyright (c) 2020-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use itertools::*;

use crate::api::color::*;
use crate::api::config::GrainTableSegment;
use crate::api::{Rational, SpeedSettings};
use crate::encoder::Tune;
use crate::serialize::{Deserialize, Serialize};

use std::fmt;

// We add 1 to rdo_lookahead_frames in a bunch of places.
pub(crate) const MAX_RDO_LOOKAHEAD_FRAMES: usize = usize::MAX - 1;
// Due to the math in RCState::new() regarding the reservoir frame delay.
pub(crate) const MAX_MAX_KEY_FRAME_INTERVAL: u64 = i32::MAX as u64 / 3;

/// Encoder settings which impact the produced bitstream.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EncoderConfig {
  // output size
  /// Width of the frames in pixels.
  pub width: usize,
  /// Height of the frames in pixels.
  pub height: usize,
  /// Sample aspect ratio (for anamorphic video).
  pub sample_aspect_ratio: Rational,
  /// Video time base.
  pub time_base: Rational,

  // data format and ancillary color information
  /// Bit depth.
  pub bit_depth: usize,
  /// Chroma subsampling.
  pub chroma_sampling: ChromaSampling,
  /// Chroma sample position.
  pub chroma_sample_position: ChromaSamplePosition,
  /// Pixel value range.
  pub pixel_range: PixelRange,
  /// Content color description (primaries, transfer characteristics, matrix).
  pub color_description: Option<ColorDescription>,
  /// HDR mastering display parameters.
  pub mastering_display: Option<MasteringDisplay>,
  /// HDR content light parameters.
  pub content_light: Option<ContentLight>,

  /// AV1 level index to target (0-31).
  /// If None, allow the encoder to decide.
  /// Currently, rav1e is unable to guarantee that the output bitstream
  /// meets the rate limitations of the specified level.
  pub level_idx: Option<u8>,

  /// Enable signaling timing info in the bitstream.
  pub enable_timing_info: bool,

  /// Still picture mode flag.
  pub still_picture: bool,

  /// Flag to force all frames to be error resilient.
  pub error_resilient: bool,

  /// Interval between switch frames (0 to disable)
  pub switch_frame_interval: u64,

  // encoder configuration
  /// The *minimum* interval between two keyframes
  pub min_key_frame_interval: u64,
  /// The *maximum* interval between two keyframes
  pub max_key_frame_interval: u64,
  /// The number of temporal units over which to distribute the reservoir
  /// usage.
  pub reservoir_frame_delay: Option<i32>,
  /// Flag to enable low latency mode.
  ///
  /// In this mode the frame reordering is disabled.
  pub low_latency: bool,
  /// The base quantizer to use.
  pub quantizer: usize,
  /// The minimum allowed base quantizer to use in bitrate mode.
  pub min_quantizer: u8,
  /// The target bitrate for the bitrate mode.
  pub bitrate: i32,
  /// Metric to tune the quality for.
  pub tune: Tune,
  /// Parameters for grain synthesis.
  pub film_grain_params: Option<Vec<GrainTableSegment>>,
  /// Number of tiles horizontally. Must be a power of two.
  ///
  /// Overridden by [`tiles`], if present.
  ///
  /// [`tiles`]: #structfield.tiles
  pub tile_cols: usize,
  /// Number of tiles vertically. Must be a power of two.
  ///
  /// Overridden by [`tiles`], if present.
  ///
  /// [`tiles`]: #structfield.tiles
  pub tile_rows: usize,
  /// Total number of tiles desired.
  ///
  /// Encoder will try to optimally split to reach this number of tiles,
  /// rounded up. Overrides [`tile_cols`] and [`tile_rows`].
  ///
  /// [`tile_cols`]: #structfield.tile_cols
  /// [`tile_rows`]: #structfield.tile_rows
  pub tiles: usize,

  /// Settings which affect the encoding speed vs. quality trade-off.
  pub speed_settings: SpeedSettings,
}

/// Default preset for `EncoderConfig`: it is a balance between quality and
/// speed. See [`with_speed_preset()`].
///
/// [`with_speed_preset()`]: struct.EncoderConfig.html#method.with_speed_preset
impl Default for EncoderConfig {
  fn default() -> Self {
    const DEFAULT_SPEED: u8 = 6;
    Self::with_speed_preset(DEFAULT_SPEED)
  }
}

impl EncoderConfig {
  /// This is a preset which provides default settings according to a speed
  /// value in the specific range 0–10. Each speed value corresponds to a
  /// different preset. See [`from_preset()`]. If the input value is greater
  /// than 10, it will result in the same settings as 10.
  ///
  /// [`from_preset()`]: struct.SpeedSettings.html#method.from_preset
  pub fn with_speed_preset(speed: u8) -> Self {
    EncoderConfig {
      width: 640,
      height: 480,
      sample_aspect_ratio: Rational { num: 1, den: 1 },
      time_base: Rational { num: 1, den: 30 },

      bit_depth: 8,
      chroma_sampling: ChromaSampling::Cs420,
      chroma_sample_position: ChromaSamplePosition::Unknown,
      pixel_range: Default::default(),
      color_description: None,
      mastering_display: None,
      content_light: None,

      level_idx: None,

      enable_timing_info: false,

      still_picture: false,

      error_resilient: false,
      switch_frame_interval: 0,

      min_key_frame_interval: 12,
      max_key_frame_interval: 240,
      min_quantizer: 0,
      reservoir_frame_delay: None,
      low_latency: false,
      quantizer: 100,
      bitrate: 0,
      tune: Tune::default(),
      film_grain_params: None,
      tile_cols: 0,
      tile_rows: 0,
      tiles: 0,
      speed_settings: SpeedSettings::from_preset(speed),
    }
  }

  /// Sets the minimum and maximum keyframe interval, handling special cases as needed.
  pub fn set_key_frame_interval(
    &mut self, min_interval: u64, max_interval: u64,
  ) {
    self.min_key_frame_interval = min_interval;

    // Map an input value of 0 to an infinite interval
    self.max_key_frame_interval = if max_interval == 0 {
      MAX_MAX_KEY_FRAME_INTERVAL
    } else {
      max_interval
    };
  }

  /// Returns the video frame rate computed from [`time_base`].
  ///
  /// [`time_base`]: #structfield.time_base
  pub fn frame_rate(&self) -> f64 {
    Rational::from_reciprocal(self.time_base).as_f64()
  }

  /// Computes the render width and height of the stream based
  /// on [`width`], [`height`], and [`sample_aspect_ratio`].
  ///
  /// [`width`]: #structfield.width
  /// [`height`]: #structfield.height
  /// [`sample_aspect_ratio`]: #structfield.sample_aspect_ratio
  pub fn render_size(&self) -> (usize, usize) {
    let sar = self.sample_aspect_ratio.as_f64();

    if sar > 1.0 {
      ((self.width as f64 * sar).round() as usize, self.height)
    } else {
      (self.width, (self.height as f64 / sar).round() as usize)
    }
  }

  /// Is temporal RDO enabled ?
  #[inline]
  pub const fn temporal_rdo(&self) -> bool {
    // Note: This function is called frequently, unlike most other functions here.

    // `compute_distortion_scale` computes a scaling factor for the distortion
    // of an 8x8 block (4x4 blocks simply use the scaling of the enclosing 8x8
    // block). As long as distortion is always computed on <= 8x8 blocks, this
    // has the property that the scaled distortion of a 2Nx2N block is always
    // equal to the sum of the scaled distortions of the NxN sub-blocks it's
    // made of, this is a necessary property to be able to do RDO between
    // multiple partition sizes properly. Unfortunately, when tx domain
    // distortion is used, distortion is only known at the tx block level which
    // might be bigger than 8x8. So temporal RDO is always disabled in that case.
    !self.speed_settings.transform.tx_domain_distortion
  }

  /// Describes whether the output is targeted as HDR
  pub fn is_hdr(&self) -> bool {
    self
      .color_description
      .map(|colors| {
        colors.transfer_characteristics == TransferCharacteristics::SMPTE2084
      })
      .unwrap_or(false)
  }

  pub(crate) fn get_film_grain_at(
    &self, timestamp: u64,
  ) -> Option<&GrainTableSegment> {
    self.film_grain_params.as_ref().and_then(|entries| {
      entries.iter().find(|entry| {
        timestamp >= entry.start_time && timestamp < entry.end_time
      })
    })
  }

  pub(crate) fn get_film_grain_mut_at(
    &mut self, timestamp: u64,
  ) -> Option<&mut GrainTableSegment> {
    self.film_grain_params.as_mut().and_then(|entries| {
      entries.iter_mut().find(|entry| {
        timestamp >= entry.start_time && timestamp < entry.end_time
      })
    })
  }
}

impl fmt::Display for EncoderConfig {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    let pairs = [
      ("keyint_min", self.min_key_frame_interval.to_string()),
      ("keyint_max", self.max_key_frame_interval.to_string()),
      ("quantizer", self.quantizer.to_string()),
      ("bitrate", self.bitrate.to_string()),
      ("min_quantizer", self.min_quantizer.to_string()),
      ("low_latency", self.low_latency.to_string()),
      ("tune", self.tune.to_string()),
      (
        "rdo_lookahead_frames",
        self.speed_settings.rdo_lookahead_frames.to_string(),
      ),
      (
        "multiref",
        (!self.low_latency || self.speed_settings.multiref).to_string(),
      ),
      ("fast_deblock", self.speed_settings.fast_deblock.to_string()),
      (
        "scene_detection_mode",
        self.speed_settings.scene_detection_mode.to_string(),
      ),
      ("cdef", self.speed_settings.cdef.to_string()),
      ("lrf", self.speed_settings.lrf.to_string()),
      ("enable_timing_info", self.enable_timing_info.to_string()),
      (
        "min_block_size",
        self.speed_settings.partition.partition_range.min.to_string(),
      ),
      (
        "max_block_size",
        self.speed_settings.partition.partition_range.max.to_string(),
      ),
      (
        "encode_bottomup",
        self.speed_settings.partition.encode_bottomup.to_string(),
      ),
      (
        "non_square_partition_max_threshold",
        self
          .speed_settings
          .partition
          .non_square_partition_max_threshold
          .to_string(),
      ),
      (
        "reduced_tx_set",
        self.speed_settings.transform.reduced_tx_set.to_string(),
      ),
      (
        "tx_domain_distortion",
        self.speed_settings.transform.tx_domain_distortion.to_string(),
      ),
      (
        "tx_domain_rate",
        self.speed_settings.transform.tx_domain_rate.to_string(),
      ),
      (
        "rdo_tx_decision",
        self.speed_settings.transform.rdo_tx_decision.to_string(),
      ),
      (
        "prediction_modes",
        self.speed_settings.prediction.prediction_modes.to_string(),
      ),
      (
        "fine_directional_intra",
        self.speed_settings.prediction.fine_directional_intra.to_string(),
      ),
      (
        "include_near_mvs",
        self.speed_settings.motion.include_near_mvs.to_string(),
      ),
      (
        "use_satd_subpel",
        self.speed_settings.motion.use_satd_subpel.to_string(),
      ),
    ];
    write!(
      f,
      "{}",
      pairs.iter().map(|pair| format!("{}={}", pair.0, pair.1)).join(" ")
    )
  }
}

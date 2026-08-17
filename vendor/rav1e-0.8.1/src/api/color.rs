// Copyright (c) 2019-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use crate::serialize::*;
use crate::wasm_bindgen::*;

use arg_enum_proc_macro::ArgEnum;
use num_derive::FromPrimitive;

/// Sample position for subsampled chroma
#[wasm_bindgen]
#[derive(
  Copy,
  Clone,
  Debug,
  PartialEq,
  Eq,
  FromPrimitive,
  Serialize,
  Deserialize,
  Default,
)]
#[repr(C)]
pub enum ChromaSamplePosition {
  /// The source video transfer function must be signaled
  /// outside the AV1 bitstream.
  #[default]
  Unknown,
  /// Horizontally co-located with (0, 0) luma sample, vertically positioned
  /// in the middle between two luma samples.
  Vertical,
  /// Co-located with (0, 0) luma sample.
  Colocated,
}

pub use v_frame::pixel::ChromaSampling;

/// Supported Color Primaries
///
/// As defined by “Color primaries” section of ISO/IEC 23091-4/ITU-T H.273
#[derive(
  ArgEnum,
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  FromPrimitive,
  Serialize,
  Deserialize,
  Default,
)]
#[repr(C)]
pub enum ColorPrimaries {
  /// BT.709
  BT709 = 1,
  /// Unspecified, must be signaled or inferred outside of the bitstream
  #[default]
  Unspecified,
  /// BT.470 System M (historical)
  BT470M = 4,
  /// BT.470 System B, G (historical)
  BT470BG,
  /// BT.601-7 525 (SMPTE 170 M)
  BT601,
  /// SMPTE 240M (historical)
  SMPTE240,
  /// Generic film
  GenericFilm,
  /// BT.2020, BT.2100
  BT2020,
  /// SMPTE 248 (CIE 1921 XYZ)
  XYZ,
  /// SMPTE RP 431-2
  SMPTE431,
  /// SMPTE EG 432-1
  SMPTE432,
  /// EBU Tech. 3213-E
  EBU3213 = 22,
}

/// Supported Transfer Characteristics
///
/// As defined by “Transfer characteristics” section of ISO/IEC 23091-4/ITU-TH.273.
#[derive(
  ArgEnum,
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  FromPrimitive,
  Serialize,
  Deserialize,
  Default,
)]
#[repr(C)]
pub enum TransferCharacteristics {
  /// BT.709
  BT709 = 1,
  /// Unspecified, must be signaled or inferred outside of the bitstream
  #[default]
  Unspecified,
  /// BT.470 System M (historical)
  BT470M = 4,
  /// BT.470 System B, G (historical)
  BT470BG,
  /// BT.601-7 525 (SMPTE 170 M)
  BT601,
  /// SMPTE 240 M
  SMPTE240,
  /// Linear
  Linear,
  /// Logarithmic (100:1 range)
  Log100,
  /// Logarithmic ((100 * √10):1 range)
  Log100Sqrt10,
  /// IEC 61966-2-4
  IEC61966,
  /// BT.1361 extended color gamut system (historical)
  BT1361,
  /// sRGB or sYCC
  SRGB,
  /// BT.2020 10-bit systems
  BT2020_10Bit,
  /// BT.2020 12-bit systems
  BT2020_12Bit,
  /// SMPTE ST 2084, ITU BT.2100 PQ
  SMPTE2084,
  /// SMPTE ST 428
  SMPTE428,
  /// BT.2100 HLG (Hybrid Log Gamma), ARIB STD-B67
  HLG,
}

/// Matrix coefficients
///
/// As defined by the “Matrix coefficients” section of ISO/IEC 23091-4/ITU-TH.273.
#[derive(
  ArgEnum,
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  FromPrimitive,
  Serialize,
  Deserialize,
  Default,
)]
#[repr(C)]
pub enum MatrixCoefficients {
  /// Identity matrix
  Identity = 0,
  /// BT.709
  BT709,
  /// Unspecified, must be signaled or inferred outside of the bitstream.
  #[default]
  Unspecified,
  /// US FCC 73.628
  FCC = 4,
  /// BT.470 System B, G (historical)
  BT470BG,
  /// BT.601-7 525 (SMPTE 170 M)
  BT601,
  /// SMPTE 240 M
  SMPTE240,
  /// `YCgCo`
  YCgCo,
  /// BT.2020 non-constant luminance, BT.2100 `YCbCr`
  BT2020NCL,
  /// BT.2020 constant luminance
  BT2020CL,
  /// SMPTE ST 2085 `YDzDx`
  SMPTE2085,
  /// Chromaticity-derived non-constant luminance
  ChromatNCL,
  /// Chromaticity-derived constant luminance
  ChromatCL,
  /// BT.2020 `ICtCp`
  ICtCp,
}

/// Signal the content color description
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ColorDescription {
  /// Color primaries.
  pub color_primaries: ColorPrimaries,
  /// Transfer charasteristics.
  pub transfer_characteristics: TransferCharacteristics,
  /// Matrix coefficients.
  pub matrix_coefficients: MatrixCoefficients,
}

impl ColorDescription {
  pub(crate) fn is_srgb_triple(self) -> bool {
    self.color_primaries == ColorPrimaries::BT709
      && self.transfer_characteristics == TransferCharacteristics::SRGB
      && self.matrix_coefficients == MatrixCoefficients::Identity
  }
}

/// Allowed pixel value range
///
/// C.f. `VideoFullRangeFlag` variable specified in ISO/IEC 23091-4/ITU-T H.273
#[wasm_bindgen]
#[derive(
  ArgEnum,
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  FromPrimitive,
  Serialize,
  Deserialize,
  Default,
)]
#[repr(C)]
pub enum PixelRange {
  /// Studio swing representation
  #[default]
  Limited,
  /// Full swing representation
  Full,
}

/// High dynamic range content light level
///
/// As defined by CEA-861.3, Appendix A.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ContentLight {
  /// Maximum content light level
  pub max_content_light_level: u16,
  /// Maximum frame-average light level
  pub max_frame_average_light_level: u16,
}

/// Chromaticity coordinates as defined by CIE 1931, expressed as 0.16
/// fixed-point values.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[repr(C)]
pub struct ChromaticityPoint {
  /// The X coordinate.
  pub x: u16,
  /// The Y coordinate.
  pub y: u16,
}

/// High dynamic range mastering display color volume
///
/// As defined by CIE 1931
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct MasteringDisplay {
  /// Chromaticity coordinates in Red, Green, Blue order
  /// expressed as 0.16 fixed-point
  pub primaries: [ChromaticityPoint; 3],
  /// Chromaticity coordinates expressed as 0.16 fixed-point
  pub white_point: ChromaticityPoint,
  /// 24.8 fixed-point maximum luminance in candelas per square meter
  pub max_luminance: u32,
  /// 18.14 fixed-point minimum luminance in candelas per square meter
  pub min_luminance: u32,
}

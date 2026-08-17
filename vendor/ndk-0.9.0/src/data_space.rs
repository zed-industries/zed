//! Bindings for [`ADataSpace`]
//!
//! [`ADataSpace`]: https://developer.android.com/ndk/reference/group/a-data-space#group___a_data_space_1ga2759ad19cae46646cc5f7002758c4a1c
#![cfg(feature = "api-level-28")]

use std::fmt;

use num_enum::{FromPrimitive, IntoPrimitive};

/// Describes how to interpret colors.
///
/// <https://developer.android.com/ndk/reference/group/a-data-space#group___a_data_space_1ga2759ad19cae46646cc5f7002758c4a1c>
#[repr(i32)]
#[derive(Clone, Copy, Hash, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[doc(alias = "ADataSpace")]
#[non_exhaustive]
pub enum DataSpace {
    /// Default-assumption data space, when not explicitly specified.
    ///
    /// It is safest to assume the buffer is an image with `sRGB` primaries and encoding ranges,
    /// but the consumer and/or the producer of the data may simply be using defaults. No automatic
    /// gamma transform should be expected, except for a possible display gamma transform when drawn
    /// to a screen.
    #[doc(alias = "ADATASPACE_UNKNOWN")]
    Unknown = ffi::ADataSpace::ADATASPACE_UNKNOWN.0,

    /// Adobe RGB.
    ///
    /// Uses [full range], [gamma `2.2` transfer] and [Adobe RGB standard].
    ///
    /// Note: Application is responsible for gamma encoding the data as a `2.2` gamma encoding is
    /// not supported in HW.
    ///
    /// [full range]: DataSpaceRange::Full
    /// [gamma `2.2` transfer]: DataSpaceTransfer::Gamma2_2
    /// [Adobe RGB standard]: DataSpaceStandard::AdobeRgb
    #[doc(alias = "ADATASPACE_ADOBE_RGB")]
    AdobeRgb = ffi::ADataSpace::ADATASPACE_ADOBE_RGB.0,
    /// ITU-R Recommendation 2020 (`BT.2020`).
    ///
    /// Ultra High-definition television.
    ///
    /// Uses [full range], [`SMPTE 170M` transfer] and [`BT2020` standard].
    ///
    /// [full range]: DataSpaceRange::Full
    /// [`SMPTE 170M` transfer]: DataSpaceTransfer::Smpte170M
    /// [`BT2020` standard]: DataSpaceStandard::Bt2020
    #[doc(alias = "ADATASPACE_BT2020")]
    Bt2020 = ffi::ADataSpace::ADATASPACE_BT2020.0,
    /// Hybrid Log Gamma encoding.
    ///
    /// Uses [full range], [hybrid log gamma transfer] and [`BT2020` standard].
    ///
    /// [full range]: DataSpaceRange::Full
    /// [hybrid log gamma transfer]: DataSpaceTransfer::HLG
    /// [`BT2020` standard]: DataSpaceStandard::Bt2020
    #[doc(alias = "ADATASPACE_BT2020_HLG")]
    Bt2020Hlg = ffi::ADataSpace::ADATASPACE_BT2020_HLG.0,
    /// ITU Hybrid Log Gamma encoding.
    ///
    /// Uses [limited range], [hybrid log gamma transfer] and [`BT2020` standard].
    ///
    /// [limited range]: DataSpaceRange::Limited
    /// [hybrid log gamma transfer]: DataSpaceTransfer::HLG
    /// [`BT2020` standard]: DataSpaceStandard::Bt2020
    #[doc(alias = "ADATASPACE_BT2020_ITU_HLG")]
    Bt2020ItuHlg = ffi::ADataSpace::ADATASPACE_BT2020_ITU_HLG.0,
    /// ITU-R Recommendation 2020 (`BT.2020`).
    ///
    /// Ultra High-definition television.
    ///
    /// Uses [limited range], [`SMPTE 2084 (PQ)` transfer] and [`BT2020` standard].
    ///
    /// [limited range]: DataSpaceRange::Limited
    /// [`SMPTE 2084 (PQ)` transfer]: DataSpaceTransfer::St2084
    /// [`BT2020` standard]: DataSpaceStandard::Bt2020
    #[doc(alias = "ADATASPACE_BT2020_ITU_PQ")]
    Bt2020ItuPq = ffi::ADataSpace::ADATASPACE_BT2020_ITU_PQ.0,
    /// ITU-R Recommendation 2020 (`BT.2020`).
    ///
    /// Ultra High-definition television.
    ///
    /// Uses [full range], [`SMPTE 2084 (PQ)` transfer] and [`BT2020` standard].
    ///
    /// [full range]: DataSpaceRange::Full
    /// [`SMPTE 2084 (PQ)` transfer]: DataSpaceTransfer::St2084
    /// [`BT2020` standard]: DataSpaceStandard::Bt2020
    #[doc(alias = "ADATASPACE_BT2020_PQ")]
    Bt2020Pq = ffi::ADataSpace::ADATASPACE_BT2020_PQ.0,
    /// ITU-R Recommendation 601 (`BT.601`) - 525-line.
    ///
    /// Standard-definition television, 525 Lines (NTSC).
    ///
    /// Uses [limited range], [`SMPTE 170M` transfer] and [`BT.601_525` standard].
    ///
    /// [limited range]: DataSpaceRange::Limited
    /// [`SMPTE 170M` transfer]: DataSpaceTransfer::Smpte170M
    /// [`BT.601_525` standard]: DataSpaceStandard::Bt601_525
    #[doc(alias = "ADATASPACE_BT601_525")]
    Bt601_525 = ffi::ADataSpace::ADATASPACE_BT601_525.0,
    /// ITU-R Recommendation 601 (`BT.601`) - 625-line.
    ///
    /// Standard-definition television, 625 Lines (PAL).
    ///
    /// Uses [limited range], [`SMPTE 170M` transfer] and [`BT.601_625` standard].
    ///
    /// [limited range]: DataSpaceRange::Limited
    /// [`SMPTE 170M` transfer]: DataSpaceTransfer::Smpte170M
    /// [`BT.601_625` standard]: DataSpaceStandard::Bt601_625
    #[doc(alias = "ADATASPACE_BT601_625")]
    Bt601_625 = ffi::ADataSpace::ADATASPACE_BT601_625.0,
    /// ITU-R Recommendation 709 (`BT.709`).
    ///
    /// High-definition television.
    ///
    /// Uses [limited range], [`SMPTE 170M` transfer] and [`BT.709` standard].
    ///
    /// [limited range]: DataSpaceRange::Limited
    /// [`SMPTE 170M` transfer]: DataSpaceTransfer::Smpte170M
    /// [`BT.709` standard]: DataSpaceStandard::Bt709
    #[doc(alias = "ADATASPACE_BT709")]
    Bt709 = ffi::ADataSpace::ADATASPACE_BT709.0,
    /// `SMPTE EG 432-1` and `SMPTE RP 431-2`.
    ///
    /// Digital Cinema `DCI-P3`.
    ///
    /// Uses [full range], [gamma `2.6` transfer] and [`D65` `DCI-P3` standard].
    ///
    /// Note: Application is responsible for gamma encoding the data as a `2.6` gamma encoding is
    /// not supported in HW.
    ///
    /// [full range]: DataSpaceRange::Full
    /// [gamma `2.6` transfer]: DataSpaceTransfer::Gamma2_6
    /// [`D65` `DCI-P3` standard]: DataSpaceStandard::DciP3
    #[doc(alias = "ADATASPACE_DCI_P3")]
    DciP3 = ffi::ADataSpace::ADATASPACE_DCI_P3.0,
    /// Display P3.
    ///
    /// Uses [full range], [`sRGB` transfer] and [`D65` `DCI-P3` standard].
    ///
    /// [full range]: DataSpaceRange::Full
    /// [`sRGB` transfer]: DataSpaceTransfer::Srgb
    /// [`D65` `DCI-P3` standard]: DataSpaceStandard::DciP3
    #[doc(alias = "ADATASPACE_DISPLAY_P3")]
    DisplayP3 = ffi::ADataSpace::ADATASPACE_DISPLAY_P3.0,
    /// JPEG File Interchange Format (`JFIF`).
    ///
    /// Same model as `BT.601-625`, but all values (`Y`, `Cb`, `Cr`) range from `0` to `255`.
    ///
    /// Uses [full range], [`SMPTE 170M` transfer] and [`BT.601_625` standard].
    ///
    /// [full range]: DataSpaceRange::Full
    /// [`SMPTE 170M` transfer]: DataSpaceTransfer::Smpte170M
    /// [`BT.601_625` standard]: DataSpaceStandard::Bt601_625
    #[doc(alias = "ADATASPACE_JFIF")]
    Jfif = ffi::ADataSpace::ADATASPACE_JFIF.0,
    /// `scRGB`.
    ///
    /// The `red`, `green`, and `blue` components are stored in [extended][extended range] `sRGB`
    /// space, and gamma- encoded using the [`sRGB` transfer] function.
    ///
    /// The values are floating point. A pixel value of `1.0`, `1.0`, `1.0` corresponds to `sRGB`
    /// white (`D65`) at `80` nits. Values beyond the range `[0.0 - 1.0]` would correspond to other
    /// colors spaces and/or HDR content.
    ///
    /// Uses [extended range], [`sRGB` transfer] and [`BT.709` standard].
    ///
    /// [extended range]: DataSpaceRange::Extended
    /// [`sRGB` transfer]: DataSpaceTransfer::Srgb
    /// [`BT.709` standard]: DataSpaceStandard::Bt709
    #[doc(alias = "ADATASPACE_SCRGB")]
    Scrgb = ffi::ADataSpace::ADATASPACE_SCRGB.0,
    /// `scRGB` linear encoding
    ///
    /// The `red`, `green`, and `blue` components are stored in [extended][extended range] `sRGB`
    /// space, but are linear, not gamma-encoded.
    ///
    /// The values are floating point. A pixel value of `1.0`, `1.0`, `1.0` corresponds to `sRGB`
    /// white (`D65`) at `80` nits. Values beyond the range `[0.0 - 1.0]` would correspond to other
    /// colors spaces and/or HDR content.
    ///
    /// Uses [extended range], [linear transfer] and [`BT.709` standard].
    ///
    /// [extended range]: DataSpaceRange::Extended
    /// [linear transfer]: DataSpaceTransfer::Linear
    /// [`BT.709` standard]: DataSpaceStandard::Bt709
    #[doc(alias = "ADATASPACE_SCRGB_LINEAR")]
    ScrgbLinear = ffi::ADataSpace::ADATASPACE_SCRGB_LINEAR.0,
    /// `sRGB` gamma encoding.
    ///
    /// The `red`, `green` and `blue` components are stored in `sRGB` space, and converted to linear
    /// space when read, using the [`sRGB` transfer] function for each of the `R`, `G` and `B`
    /// components. When written, the inverse transformation is performed.
    ///
    /// The `alpha` component, if present, is always stored in linear space and is left unmodified
    /// when read or written.
    ///
    /// Uses [full range], [`sRGB` transfer] and [`BT.709` standard].
    ///
    /// [full range]: DataSpaceRange::Full
    /// [`sRGB` transfer]: DataSpaceTransfer::Srgb
    /// [`BT.709` standard]: DataSpaceStandard::Bt709
    #[doc(alias = "ADATASPACE_SRGB")]
    Srgb = ffi::ADataSpace::ADATASPACE_SRGB.0,
    /// `sRGB` linear encoding.
    ///
    /// The `red`, `green`, and `blue` components are stored in `sRGB` space, but are linear, not
    /// gamma-encoded. The `RGB` primaries and the white point are the same as [`BT.709]`.
    ///
    /// The values are encoded using the [full range] (`[0, 255]` for 8-bit) for all components.
    ///
    /// Uses [full range], [linear transfer] and [`BT.709` standard].
    ///
    /// [full range]: DataSpaceRange::Full
    /// [linear transfer]: DataSpaceTransfer::Linear
    /// [`BT.709` standard]: DataSpaceStandard::Bt709
    #[doc(alias = "ADATASPACE_SRGB_LINEAR")]
    SrgbLinear = ffi::ADataSpace::ADATASPACE_SRGB_LINEAR.0,

    /// Depth.
    ///
    /// This value is valid with formats [`HAL_PIXEL_FORMAT_Y16`] and [`HAL_PIXEL_FORMAT_BLOB`].
    ///
    /// [`HAL_PIXEL_FORMAT_Y16`]: https://cs.android.com/android/platform/superproject/main/+/main:frameworks/native/libs/nativewindow/include/vndk/hardware_buffer.h;l=74-75;drc=45317f5c7c966fc816843217adc96a2ddea8bf29
    /// [`HAL_PIXEL_FORMAT_BLOB`]: super::hardware_buffer_format::HardwareBufferFormat::BLOB
    #[doc(alias = "ADATASPACE_DEPTH")]
    Depth = ffi::ADataSpace::ADATASPACE_DEPTH.0,
    /// ISO `16684-1:2011(E)` Dynamic Depth.
    ///
    /// Embedded depth metadata following the dynamic depth specification.
    #[doc(alias = "ADATASPACE_DYNAMIC_DEPTH")]
    DynamicDepth = ffi::ADataSpace::ADATASPACE_DYNAMIC_DEPTH.0,

    #[doc(hidden)]
    #[num_enum(catch_all)]
    __Unknown(i32),
}

impl fmt::Display for DataSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Unknown => "Unknown",
            Self::AdobeRgb => "AdobeRgb",
            Self::Bt2020 => "Bt2020",
            Self::Bt2020Hlg => "Bt2020Hlg",
            Self::Bt2020ItuHlg => "Bt2020ItuHlg",
            Self::Bt2020ItuPq => "Bt2020ItuPq",
            Self::Bt2020Pq => "Bt2020Pq",
            Self::Bt601_525 => "Bt601_525",
            Self::Bt601_625 => "Bt601_625",
            Self::Bt709 => "Bt709",
            Self::DciP3 => "DciP3",
            Self::DisplayP3 => "DisplayP3",
            Self::Jfif => "Jfif",
            Self::Scrgb => "Scrgb",
            Self::ScrgbLinear => "ScrgbLinear",
            Self::Srgb => "Srgb",
            Self::SrgbLinear => "SrgbLinear",
            Self::Depth => "Depth",
            Self::DynamicDepth => "DynamicDepth",
            Self::__Unknown(u) => {
                return write!(
                    f,
                    "Unknown DataSpace({u:x?}, standard: {:?}, transfer: {:?}, range: {:?})",
                    self.standard(),
                    self.transfer(),
                    self.range()
                )
            }
        })
    }
}

impl fmt::Debug for DataSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DataSpace({self}, standard: {:?}, transfer: {:?}, range: {:?})",
            self.standard(),
            self.transfer(),
            self.range(),
        )
    }
}

impl DataSpace {
    /// Construct a [`DataSpace`] from individual `standard`, `transfer` and `range` components.
    ///
    /// Together these should correspond to a single format.
    pub fn from_parts(
        standard: DataSpaceStandard,
        transfer: DataSpaceTransfer,
        range: DataSpaceRange,
    ) -> Self {
        Self::from(i32::from(standard) | i32::from(transfer) | i32::from(range))
    }

    /// Extracts and returns the color-description aspect from this [`DataSpace`].
    #[doc(alias = "STANDARD_MASK")]
    pub fn standard(self) -> DataSpaceStandard {
        let standard = i32::from(self) & ffi::ADataSpace::STANDARD_MASK.0;
        standard.into()
    }

    /// Extracts and returns the transfer aspect from this [`DataSpace`].
    #[doc(alias = "TRANSFER_MASK")]
    pub fn transfer(self) -> DataSpaceTransfer {
        let transfer = i32::from(self) & ffi::ADataSpace::TRANSFER_MASK.0;
        transfer.into()
    }

    /// Extracts and returns the range aspect from this [`DataSpace`].
    #[doc(alias = "RANGE_MASK")]
    pub fn range(self) -> DataSpaceRange {
        let range = i32::from(self) & ffi::ADataSpace::RANGE_MASK.0;
        range.into()
    }
}

/// Color-description aspects.
///
/// The following aspects define various characteristics of the color specification. These represent
/// bitfields, so that a data space value can specify each of them independently. Standard aspect
/// defines the chromaticity coordinates of the source primaries in terms of the CIE 1931 definition
/// of `x` and `y` specified in ISO 11664-1.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[doc(alias = "STANDARD_MASK")]
#[non_exhaustive]
pub enum DataSpaceStandard {
    /// Chromacity coordinates are unknown or are determined by the application. Implementations
    /// shall use the following suggested standards:
    ///
    /// All `YCbCr` formats: [`BT.709`] if size is `720p` or larger (since most video content is
    ///                      letterboxed this corresponds to width is `1280` or greater, or height
    ///                      is 720 or greater). [`BT.601_625`] if size is smaller than `720p` or
    ///                      is `JPEG`.
    /// All `RGB` formats:   [`BT.709`].
    ///
    /// For all other formats the standard is undefined, and implementations should use an
    /// appropriate standard for the data represented.
    ///
    /// [`BT.709`]: Self::Bt709
    /// [`BT.601_625`]: Self::Bt601_625
    #[doc(alias = "STANDARD_UNSPECIFIED")]
    Unspecified = ffi::ADataSpace::STANDARD_UNSPECIFIED.0,
    /// | Primaries   | x      | y      |
    /// | ----------- | ------ | ------ |
    /// | green       | 0.300  | 0.600  |
    /// | blue        | 0.150  | 0.060  |
    /// | red         | 0.640  | 0.330  |
    /// | white (D65) | 0.3127 | 0.3290 |
    ///
    /// Use the unadjusted `KR = 0.2126`, `KB = 0.0722` luminance interpretation for `RGB`
    /// conversion.
    #[doc(alias = "STANDARD_BT709")]
    Bt709 = ffi::ADataSpace::STANDARD_BT709.0,
    /// | Primaries   | x      | y      |
    /// | ----------- | ------ | ------ |
    /// | green       | 0.290  | 0.600  |
    /// | blue        | 0.150  | 0.060  |
    /// | red         | 0.640  | 0.330  |
    /// | white (D65) | 0.3127 | 0.3290 |
    ///
    /// `KR = 0.299`, `KB = 0.114`. This adjusts the luminance interpretation for `RGB` conversion
    /// from the one purely determined by the primaries to minimize the color shift into `RGB`
    /// space that uses [`BT.709`] primaries.
    ///
    /// [`BT.709`]: Self::Bt709
    #[doc(alias = "STANDARD_BT601_625")]
    Bt601_625 = ffi::ADataSpace::STANDARD_BT601_625.0,
    /// | Primaries   | x      | y      |
    /// | ----------- | ------ | ------ |
    /// | green       | 0.290  | 0.600  |
    /// | blue        | 0.150  | 0.060  |
    /// | red         | 0.640  | 0.330  |
    /// | white (D65) | 0.3127 | 0.3290 |
    ///
    /// Use the unadjusted `KR = 0.222`, `KB = 0.071` luminance interpretation for `RGB` conversion.
    #[doc(alias = "STANDARD_BT601_625_UNADJUSTED")]
    Bt601_625Unadjusted = ffi::ADataSpace::STANDARD_BT601_625_UNADJUSTED.0,
    /// | Primaries   | x      | y      |
    /// | ----------- | ------ | ------ |
    /// | green       | 0.310  | 0.595  |
    /// | blue        | 0.155  | 0.070  |
    /// | red         | 0.630  | 0.340  |
    /// | white (D65) | 0.3127 | 0.3290 |
    ///
    /// `KR = 0.299`, `KB = 0.114`. This adjusts the luminance interpretation for `RGB` conversion
    /// from the one purely determined by the primaries to minimize the color shift into `RGB` space
    /// that uses [`BT.709`] primaries.
    ///
    /// [`BT.709`]: Self::Bt709
    #[doc(alias = "STANDARD_BT601_525")]
    Bt601_525 = ffi::ADataSpace::STANDARD_BT601_525.0,
    /// | Primaries   | x      | y      |
    /// | ----------- | ------ | ------ |
    /// | green       | 0.310  | 0.595  |
    /// | blue        | 0.155  | 0.070  |
    /// | red         | 0.630  | 0.340  |
    /// | white (D65) | 0.3127 | 0.3290 |
    ///
    /// Use the unadjusted `KR = 0.212`, `KB = 0.087` luminance interpretation
    /// for `RGB` conversion (as in `SMPTE 240M`).
    #[doc(alias = "STANDARD_BT601_525_UNADJUSTED")]
    Bt601_525Unadjusted = ffi::ADataSpace::STANDARD_BT601_525_UNADJUSTED.0,
    /// | Primaries   | x      | y      |
    /// | ----------- | ------ | ------ |
    /// | green       | 0.170  | 0.797  |
    /// | blue        | 0.131  | 0.046  |
    /// | red         | 0.708  | 0.292  |
    /// | white (D65) | 0.3127 | 0.3290 |
    ///
    /// Use the unadjusted `KR = 0.2627`, `KB = 0.0593` luminance interpretation for `RGB`
    /// conversion.
    #[doc(alias = "STANDARD_BT2020")]
    Bt2020 = ffi::ADataSpace::STANDARD_BT2020.0,
    /// | Primaries   | x      | y      |
    /// | ----------- | ------ | ------ |
    /// | green       | 0.170  | 0.797  |
    /// | blue        | 0.131  | 0.046  |
    /// | red         | 0.708  | 0.292  |
    /// | white (D65) | 0.3127 | 0.3290 |
    ///
    /// Use the unadjusted `KR = 0.2627`, `KB = 0.0593` luminance interpretation for `RGB`
    /// conversion using the linear domain.
    #[doc(alias = "STANDARD_BT2020_CONSTANT_LUMINANCE")]
    Bt2020ConstantLuminance = ffi::ADataSpace::STANDARD_BT2020_CONSTANT_LUMINANCE.0,
    /// | Primaries | x     | y    |
    /// | --------- | ----- | ---- |
    /// | green     | 0.21  |0.71  |
    /// | blue      | 0.14  |0.08  |
    /// | red       | 0.67  |0.33  |
    /// | white (C) | 0.310 |0.316 |
    ///
    /// Use the unadjusted `KR = 0.30`, `KB = 0.11` luminance interpretation for `RGB` conversion.
    #[doc(alias = "STANDARD_BT470M")]
    Bt470M = ffi::ADataSpace::STANDARD_BT470M.0,
    /// | Primaries | x     | y     |
    /// | --------- | ----- | ----- |
    /// | green     | 0.243 | 0.692 |
    /// | blue      | 0.145 | 0.049 |
    /// | red       | 0.681 | 0.319 |
    /// | white (C) | 0.310 | 0.316 |
    ///
    /// Use the unadjusted `KR = 0.254`, `KB = 0.068` luminance interpretation for `RGB` conversion.
    #[doc(alias = "STANDARD_FILM")]
    Film = ffi::ADataSpace::STANDARD_FILM.0,
    /// `SMPTE EG 432-1` and `SMPTE RP 431-2`. (`DCI-P3`)
    ///
    /// | Primaries   | x      | y      |
    /// | ----------- | ------ | ------ |
    /// | green       | 0.265  | 0.690  |
    /// | blue        | 0.150  | 0.060  |
    /// | red         | 0.680  | 0.320  |
    /// | white (D65) | 0.3127 | 0.3290 |
    #[doc(alias = "STANDARD_DCI_P3")]
    DciP3 = ffi::ADataSpace::STANDARD_DCI_P3.0,
    /// Adobe RGB
    ///
    /// | Primaries   | x      | y      |
    /// | ----------- | ------ | ------ |
    /// | green       | 0.210  | 0.710  |
    /// | blue        | 0.150  | 0.060  |
    /// | red         | 0.640  | 0.330  |
    /// | white (D65) | 0.3127 | 0.3290 |
    #[doc(alias = "STANDARD_ADOBE_RGB")]
    AdobeRgb = ffi::ADataSpace::STANDARD_ADOBE_RGB.0,

    #[doc(hidden)]
    #[num_enum(catch_all)]
    __Unknown(i32),
}

/// Transfer aspect.
///
/// Transfer characteristics are the opto-electronic transfer characteristic at the source as a
///function of linear optical intensity (luminance).
///
/// For digital signals, `E` corresponds to the recorded value. Normally, the transfer function is
/// applied in `RGB` space to each of the `R`, `G` and `B` components independently. This may result
/// in color shift that can be minimized by applying the transfer function in `Lab` space only for
/// the `L` component. Implementation may apply the transfer function in `RGB` space for all pixel
/// formats if desired.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[doc(alias = "TRANSFER_MASK")]
#[non_exhaustive]
pub enum DataSpaceTransfer {
    /// Transfer characteristics are unknown or are determined by the application.
    ///
    /// Implementations should use the following transfer functions:
    ///
    /// - For `YCbCr` formats: use [`DataSpaceTransfer::Smpte170M`]
    /// - For `RGB` formats: use [`DataSpaceTransfer::Srgb`]
    ///
    /// For all other formats the transfer function is undefined, and implementations should use an
    /// appropriate standard for the data represented.
    #[doc(alias = "TRANSFER_UNSPECIFIED")]
    Unspecified = ffi::ADataSpace::TRANSFER_UNSPECIFIED.0,

    /// Linear transfer.
    ///
    /// Transfer characteristic curve:
    /// ```ignore
    /// E = L
    /// ```
    /// - `L`: luminance of image `0 <= L <= 1` for conventional colorimetry
    /// - `E`: corresponding electrical signal
    #[doc(alias = "TRANSFER_LINEAR")]
    Linear = ffi::ADataSpace::TRANSFER_LINEAR.0,
    /// `sRGB` transfer.
    ///
    /// Transfer characteristic curve:
    ///
    /// ```ignore
    /// E = 1.055 * L^(1/2.4) - 0.055 for 0.0031308 <= L <= 1
    ///   = 12.92 * L                 for 0 <= L < 0.0031308
    /// ```
    /// - `L`: luminance of image `0 <= L <= 1` for conventional colorimetry
    /// - `E`: corresponding electrical signal
    #[doc(alias = "TRANSFER_SRGB")]
    Srgb = ffi::ADataSpace::TRANSFER_SRGB.0,
    /// SMPTE 170M transfer.
    ///
    /// Transfer characteristic curve:
    /// ```ignore
    /// E = 1.099 * L ^ 0.45 - 0.099 for 0.018 <= L <= 1
    ///   = 4.500 * L                for 0 <= L < 0.018
    /// ```
    /// - `L`: luminance of image `0 <= L <= 1` for conventional colorimetry
    /// - `E`: corresponding electrical signal
    #[doc(alias = "TRANSFER_SMPTE_170M")]
    Smpte170M = ffi::ADataSpace::TRANSFER_SMPTE_170M.0,
    /// Display gamma `2.2`.
    ///
    /// Transfer characteristic curve:
    /// ```ignore
    /// E = L ^ (1/2.2)
    /// ```
    /// - `L`: luminance of image `0 <= L <= 1` for conventional colorimetry
    /// - `E`: corresponding electrical signal
    #[doc(alias = "TRANSFER_GAMMA2_2")]
    Gamma2_2 = ffi::ADataSpace::TRANSFER_GAMMA2_2.0,
    /// Display gamma `2.6`.
    ///
    /// Transfer characteristic curve:
    /// ```ignore
    /// E = L ^ (1/2.6)
    /// ```
    /// - `L`: luminance of image `0 <= L <= 1` for conventional colorimetry
    /// - `E`: corresponding electrical signal
    #[doc(alias = "TRANSFER_GAMMA2_6")]
    Gamma2_6 = ffi::ADataSpace::TRANSFER_GAMMA2_6.0,
    /// Display gamma `2.8`.
    ///
    /// Transfer characteristic curve:
    /// ```ignore
    /// E = L ^ (1/2.8)
    /// ```
    /// - `L`: luminance of image `0 <= L <= 1` for conventional colorimetry
    /// - `E`: corresponding electrical signal
    #[doc(alias = "TRANSFER_GAMMA2_8")]
    Gamma2_8 = ffi::ADataSpace::TRANSFER_GAMMA2_8.0,
    /// SMPTE ST 2084 (Dolby Perceptual Quantizer).
    ///
    /// Transfer characteristic curve:
    /// ```ignore
    /// E = ((c1 + c2 * L^n) / (1 + c3 * L^n)) ^ m
    /// c1 = c3 - c2 + 1 = 3424 / 4096 = 0.8359375
    /// c2 = 32 * 2413 / 4096 = 18.8515625
    /// c3 = 32 * 2392 / 4096 = 18.6875
    /// m = 128 * 2523 / 4096 = 78.84375
    /// n = 0.25 * 2610 / 4096 = 0.1593017578125
    /// ```
    /// - `L`: luminance of image 0 <= L <= 1 for HDR colorimetry.
    ///        `L = 1` corresponds to `10000 cd/m2`
    #[doc(alias = "TRANSFER_ST2084")]
    St2084 = ffi::ADataSpace::TRANSFER_ST2084.0,
    /// ARIB STD-B67 Hybrid Log Gamma.
    ///
    /// Transfer characteristic curve:
    /// ```ignore
    /// E = r * L^0.5                 for 0 <= L <= 1
    ///   = a * ln(L - b) + c         for 1 < L
    /// a = 0.17883277
    /// b = 0.28466892
    /// c = 0.55991073
    /// r = 0.5
    /// ```
    /// - `L`: luminance of image `0 <= L` for HDR colorimetry.
    ///        `L = 1` corresponds to reference white level of `100 cd/m2`
    /// - `E`: corresponding electrical signal
    #[doc(alias = "TRANSFER_HLG")]
    HLG = ffi::ADataSpace::TRANSFER_HLG.0,

    #[doc(hidden)]
    #[num_enum(catch_all)]
    __Unknown(i32),
}

/// Range aspect.
///
/// Defines the range of values corresponding to the unit range of `0-1`. This is defined for
/// `YCbCr` only, but can be expanded to `RGB` space.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[doc(alias = "RANGE_MASK")]
#[non_exhaustive]
pub enum DataSpaceRange {
    /// Range is unknown or are determined by the application.  Implementations shall use the
    /// following suggested ranges:
    ///
    /// - All YCbCr formats: limited range.
    /// - All RGB or RGBA formats (including RAW and Bayer): full range.
    /// - All Y formats: full range
    ///
    /// For all other formats range is undefined, and implementations should use an appropriate
    /// range for the data represented.
    #[doc(alias = "RANGE_UNSPECIFIED")]
    Unspecified = ffi::ADataSpace::RANGE_UNSPECIFIED.0,
    /// Full range uses all values for `Y`, `Cb` and `Cr` from `0` to `2^b-1`, where `b` is the bit
    /// depth of the color format.
    #[doc(alias = "RANGE_FULL")]
    Full = ffi::ADataSpace::RANGE_FULL.0,
    /// Limited range uses values `16/256*2^b` to `235/256*2^b` for `Y`, and `1/16*2^b` to
    /// `15/16*2^b` for `Cb`, `Cr`, `R`, `G` and `B`, where `b` is the bit depth of the color
    /// format.
    ///
    /// E.g. For 8-bit-depth formats: Luma (`Y`) samples should range from `16` to `235`, inclusive
    /// Chroma `(Cb, Cr)` samples should range from `16` to `240`, inclusive.
    ///
    /// For 10-bit-depth formats: Luma (`Y`) samples should range from `64` to `940`, inclusive
    /// Chroma `(Cb, Cr)` samples should range from `64` to `960`, inclusive.
    #[doc(alias = "RANGE_LIMITED")]
    Limited = ffi::ADataSpace::RANGE_LIMITED.0,
    /// Extended range is used for `scRGB`.
    ///
    /// Intended for use with floating point pixel formats. `[0.0 - 1.0]` is the standard `sRGB`
    /// space. Values outside the range `0.0 - 1.0` can encode color outside the `sRGB` gamut. Used
    /// to blend / merge multiple dataspaces on a single display.
    #[doc(alias = "RANGE_EXTENDED")]
    Extended = ffi::ADataSpace::RANGE_EXTENDED.0,

    #[doc(hidden)]
    #[num_enum(catch_all)]
    __Unknown(i32),
}

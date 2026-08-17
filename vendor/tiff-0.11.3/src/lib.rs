//! Decoding and Encoding of TIFF Images
//!
//! TIFF (Tagged Image File Format) is a versatile image format that supports
//! lossless and lossy compression.
//!
//! # Related Links
//! * <https://web.archive.org/web/20210108073850/https://www.adobe.io/open/standards/TIFF.html> - The TIFF specification

mod bytecast;
pub mod decoder;
mod directory;
pub mod encoder;
mod error;
pub mod tags;

pub use self::directory::Directory;
pub use self::error::{TiffError, TiffFormatError, TiffResult, TiffUnsupportedError, UsageError};

/// An enumeration over supported color types and their bit depths
#[derive(Copy, PartialEq, Eq, Debug, Clone, Hash)]
#[non_exhaustive]
pub enum ColorType {
    /// Pixel is grayscale
    Gray(u8),

    /// Pixel contains R, G and B channels
    RGB(u8),

    /// Pixel is an index into a color palette
    Palette(u8),

    /// Pixel is grayscale with an alpha channel
    GrayA(u8),

    /// Pixel is RGB with an alpha channel
    RGBA(u8),

    /// Pixel is CMYK
    CMYK(u8),

    /// Pixel is CMYK with an alpha channel
    CMYKA(u8),

    /// Pixel is YCbCr
    YCbCr(u8),

    /// Pixel is CIE L*a*b* (ICC LAb)
    Lab(u8),

    /// Pixel has multiple bands/channels
    Multiband { bit_depth: u8, num_samples: u16 },
}
impl ColorType {
    /// The number of bits used to store each sample.
    pub fn bit_depth(&self) -> u8 {
        match *self {
            ColorType::Gray(b)
            | ColorType::RGB(b)
            | ColorType::Palette(b)
            | ColorType::GrayA(b)
            | ColorType::RGBA(b)
            | ColorType::CMYK(b)
            | ColorType::CMYKA(b)
            | ColorType::YCbCr(b)
            | ColorType::Lab(b)
            | ColorType::Multiband { bit_depth: b, .. } => b,
        }
    }

    /// The number of samples in each representation of a single color.
    pub fn num_samples(&self) -> u16 {
        match *self {
            ColorType::Gray(_) => 1,
            ColorType::RGB(_) => 3,
            ColorType::Palette(_) => 1,
            ColorType::GrayA(_) => 2,
            ColorType::RGBA(_) => 4,
            ColorType::CMYK(_) => 4,
            ColorType::CMYKA(_) => 5,
            ColorType::YCbCr(_) => 3,
            ColorType::Lab(_) => 3,
            ColorType::Multiband { num_samples, .. } => num_samples,
        }
    }
}

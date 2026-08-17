/*
 * Copyright (c) 2023.
 *
 * This software is free software;
 *
 * You can redistribute it or modify it under terms of the MIT, Apache License or Zlib license
 */

//! Image Colorspace information and manipulation utilities.

/// All possible image colorspaces
/// Some of them aren't yet supported exist here.
#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ColorSpace {
    /// Red, Green , Blue
    RGB,
    /// Red, Green, Blue, Alpha
    RGBA,
    /// YUV colorspace
    YCbCr,
    /// Grayscale colorspace
    Luma,
    /// Grayscale with alpha colorspace
    LumaA,
    YCCK,
    /// Cyan , Magenta, Yellow, Black
    CMYK,
    /// Blue, Green, Red
    BGR,
    /// Blue, Green, Red, Alpha
    BGRA,
    /// The colorspace is unknown
    Unknown,
    /// Alpha Red Green Blue
    ARGB,
    /// Hue,Saturation,Lightness
    /// Conversion from RGB to HSL and back matches that of Python [colorsys](https://docs.python.org/3/library/colorsys.html) module
    /// Color type is expected to be in floating point
    HSL,
    /// Hue, Saturation,Value
    ///
    /// Conversion from RGB to HSV and back matches that of Python [colorsys](https://docs.python.org/3/library/colorsys.html) module
    /// Color type is expected to be in floating point
    HSV,
    /// Multiple arbitrary image channels.
    ///
    /// This introduces **limited** support for multi-band/multichannel images
    /// that can have n channels.
    ///
    /// The library contains optimized and well-supported routines for up to 4 image channels as is
    /// more common out there, but some pipelines may require multi-band image support.
    ///
    /// For operations, multi-band images are assumed to be n-channel images with no alpha
    /// to allow for generic processing, without necessarily caring for the underlying interpretation
    MultiBand(core::num::NonZeroU32)
}

impl ColorSpace {
    /// Number of color channels present for a certain colorspace
    ///
    /// E.g. RGB returns 3 since it contains R,G and B colors to make up a pixel
    pub const fn num_components(&self) -> usize {
        match self {
            Self::RGB | Self::YCbCr | Self::BGR | Self::HSV | Self::HSL => 3,
            Self::RGBA | Self::YCCK | Self::CMYK | Self::BGRA | Self::ARGB => 4,
            Self::Luma => 1,
            Self::LumaA => 2,
            Self::Unknown => 0,
            Self::MultiBand(n) => n.get() as usize
        }
    }

    pub const fn has_alpha(&self) -> bool {
        matches!(self, Self::RGBA | Self::LumaA | Self::BGRA | Self::ARGB)
    }

    pub const fn is_grayscale(&self) -> bool {
        matches!(self, Self::LumaA | Self::Luma)
    }

    /// Returns the position of the alpha pixel in a pixel
    ///
    ///
    /// That is for an array of color components say `[0,1,2,3]` if the image has an alpha channel
    /// and is in RGBA format, this will return `Some(3)`, indicating alpha is found in the third index
    /// but if the image is in `ARGB` format, it will return `Some(0)` indicating alpha is found in  
    /// index 0
    ///
    /// If an image doesn't have an alpha channel returns `None`
    ///
    pub const fn alpha_position(&self) -> Option<usize> {
        match self {
            ColorSpace::RGBA => Some(3),
            ColorSpace::LumaA => Some(1),
            ColorSpace::BGRA => Some(3),
            ColorSpace::ARGB => Some(0),
            _ => None
        }
    }
}

/// Encapsulates all colorspaces supported by
/// the library
///
/// This explicitly leaves out multi-band images
pub static ALL_COLORSPACES: [ColorSpace; 12] = [
    ColorSpace::RGB,
    ColorSpace::RGBA,
    ColorSpace::LumaA,
    ColorSpace::Luma,
    ColorSpace::CMYK,
    ColorSpace::BGRA,
    ColorSpace::BGR,
    ColorSpace::YCCK,
    ColorSpace::YCbCr,
    ColorSpace::ARGB,
    ColorSpace::HSL,
    ColorSpace::HSV
];

/// Color characteristics
///
/// Gives more information about values in a certain
/// colorspace
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ColorCharacteristics {
    /// sRGB Transfer function
    sRGB,
    /// Rec.709 Transfer function
    Rec709,
    /// Pure gamma 2.2 Transfer function, ITU-R 470M
    Gamma2p2,
    /// Pure gamma 2.8 Transfer function, ITU-R 470BG
    Gamma2p8,
    /// Smpte 428 Transfer function
    Smpte428,
    /// Log100 Transfer function
    Log100,
    /// Log100Sqrt10 Transfer function
    Log100Sqrt10,
    /// Bt1361 Transfer function
    Bt1361,
    /// Smpte 240 Transfer function
    Smpte240,
    /// IEC 61966 Transfer function
    Iec61966,
    /// Linear transfer function
    Linear
}
/// Represents a single channel color primary.
///
/// This can be viewed as a 3D coordinate of the color primary
/// for a given colorspace
#[derive(Default, Debug, Copy, Clone)]
pub struct SingleColorPrimary {
    pub x: f64,
    pub y: f64,
    pub z: f64
}
/// A collection of red,green and blue color primaries placed
/// in one struct for easy manipulation
#[derive(Default, Debug, Copy, Clone)]
pub struct ColorPrimaries {
    /// Red color primaries
    pub red:   SingleColorPrimary,
    /// Green color primaries
    pub green: SingleColorPrimary,
    /// Blue color primaries
    pub blue:  SingleColorPrimary
}

/// Rendering intents indicate what one may want to do with colors outside of it's gamut
///
///
/// Further reading
///  - [IBM Rendering Intent](https://www.ibm.com/docs/en/i/7.5?topic=management-rendering-intents)
///  - [ColorGate Blog](https://blog.colorgate.com/en/rendering-intent-explained)   
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum RenderingIntent {
    AbsoluteColorimetric,
    Saturation,
    RelativeColorimetric,
    Perceptual
}

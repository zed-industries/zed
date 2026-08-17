//! Types for the CIE CAM16 color appearance model.
//!
//! CIE CAM16 is a color appearance model that tries to predict the appearance
//! of a color under certain viewing conditions, as specified via the
//! [`Parameters`] type. The [`Cam16`] type has descriptions for the CAM16
//! attributes. The [Color appearance model page on Wikipedia][wikipedia_cam]
//! has some history and background as well.
//!
//! # Converting Between XYZ and CAM16
//!
//! The CIE CAM16 implementation in Palette has the [`Cam16`] type and its
//! partial variants on one side of the boundary, and [`Xyz`](crate::Xyz) on the
//! other. Going between `Xyz` and `Cam16` requires the viewing conditions to be
//! specified as [`Parameters`].
//!
//! ```
//! use palette::{
//!     Srgb, FromColor, IntoColor,
//!     cam16::{Cam16, Parameters},
//! };
//!
//! // Customize these according to the viewing conditions:
//! let mut example_parameters = Parameters::default_static_wp(40.0);
//!
//! // CAM16 from sRGB, or most other color spaces:
//! let rgb = Srgb::new(0.3f32, 0.8, 0.1);
//! let cam16_from_rgb = Cam16::from_xyz(rgb.into_color(), example_parameters);
//!
//! // sRGB from CAM16, using lightness, chroma and hue by default:
//! let rgb_from_cam16 = Srgb::from_color(cam16_from_rgb.into_xyz(example_parameters));
//! ```
//!
//! For more control over the attributes to use when converting from CAM16, one
//! of the partial CAM16 types can be used:
//!
//! * [`Cam16Jch`]: lightness and chroma.
//! * [`Cam16Jmh`]: lightness and colorfulness.
//! * [`Cam16Jsh`]: lightness and saturation.
//! * [`Cam16Qch`]: brightness and chroma.
//! * [`Cam16Qmh`]: brightness and colorfulness.
//! * [`Cam16Qsh`]: brightness and saturation.
//!
//! Generic traits and functions can make use of the [`IntoCam16Unclamped`],
//! [`FromCam16Unclamped`], [`Cam16IntoUnclamped`], and [`Cam16FromUnclamped`]
//! traits. They are similar to the traits from the [`convert`][crate::convert]
//! module and help abstracting away most of the implementation details.
//!
//! # The CAM16-UCS Color Space
//!
//! CIE CAM16 specifies a visually uniform color space that can be used for
//! color manipulation. It's represented by the [`Cam16UcsJmh`] and
//! [`Cam16UcsJab`] types, similar to [`Lch`][crate::Lch] and
//! [`Lab`][crate::Lab].
//!
//! ```
//! use palette::{
//!     Srgb, FromColor, IntoColor,
//!     cam16::{Cam16Jmh, Parameters, Cam16UcsJmh},
//! };
//!
//! // Customize these according to the viewing conditions:
//! let mut example_parameters = Parameters::default_static_wp(40.0);
//!
//! // CAM16-UCS from sRGB, or most other color spaces:
//! let rgb = Srgb::new(0.3f32, 0.8, 0.1);
//! let cam16 = Cam16Jmh::from_xyz(rgb.into_color(), example_parameters);
//! let mut ucs_from_rgb = Cam16UcsJmh::from_color(cam16);
//!
//! // Shift the hue by 120 degrees in CAM16-UCS:
//! ucs_from_rgb.hue += 120.0;
//!
//! // Convert back to sRGB under the same viewing conditions:
//! let rgb = Srgb::from_color(Cam16Jmh::from_color(ucs_from_rgb).into_xyz(example_parameters));
//! ```
//!
//! [wikipedia_cam]: https://en.wikipedia.org/wiki/Color_appearance_model

pub use full::*;
pub use parameters::*;
pub use partial::*;
pub use ucs_jab::{Cam16UcsJab, Cam16UcsJaba, Iter as Cam16UcsJabIter};
pub use ucs_jmh::{Cam16UcsJmh, Cam16UcsJmha, Iter as Cam16UcsJmhIter};

#[cfg(feature = "random")]
pub use ucs_jab::UniformCam16UcsJab;
#[cfg(feature = "random")]
pub use ucs_jmh::UniformCam16UcsJmh;

mod full;
pub(crate) mod math;
mod parameters;
mod partial;
mod ucs_jab;
mod ucs_jmh;

/// A trait for converting into a CAM16 color type from `C` without clamping.
pub trait Cam16FromUnclamped<WpParam, C> {
    /// The number type that's used in `parameters` when converting.
    type Scalar;

    /// Converts `color` into `Self`, using the provided parameters.
    fn cam16_from_unclamped(color: C, parameters: BakedParameters<WpParam, Self::Scalar>) -> Self;
}

/// A trait for converting into a CAM16 color type `C` without clamping.
pub trait IntoCam16Unclamped<WpParam, C> {
    /// The number type that's used in `parameters` when converting.
    type Scalar;

    /// Converts `self` into `C`, using the provided parameters.
    fn into_cam16_unclamped(self, parameters: BakedParameters<WpParam, Self::Scalar>) -> C;
}

impl<WpParam, T, U> IntoCam16Unclamped<WpParam, T> for U
where
    T: Cam16FromUnclamped<WpParam, U>,
{
    type Scalar = T::Scalar;

    fn into_cam16_unclamped(self, parameters: BakedParameters<WpParam, Self::Scalar>) -> T {
        T::cam16_from_unclamped(self, parameters)
    }
}

/// A trait for converting from a CAM16 color type `C` without clamping.
pub trait FromCam16Unclamped<WpParam, C> {
    /// The number type that's used in `parameters` when converting.
    type Scalar;

    /// Converts `cam16` into `Self`, using the provided parameters.
    fn from_cam16_unclamped(cam16: C, parameters: BakedParameters<WpParam, Self::Scalar>) -> Self;
}

/// A trait for converting from a CAM16 color type into `C` without clamping.
pub trait Cam16IntoUnclamped<WpParam, C> {
    /// The number type that's used in `parameters` when converting.
    type Scalar;

    /// Converts `self` into `C`, using the provided parameters.
    fn cam16_into_unclamped(self, parameters: BakedParameters<WpParam, Self::Scalar>) -> C;
}

impl<WpParam, T, U> Cam16IntoUnclamped<WpParam, T> for U
where
    T: FromCam16Unclamped<WpParam, U>,
{
    type Scalar = T::Scalar;

    fn cam16_into_unclamped(self, parameters: BakedParameters<WpParam, Self::Scalar>) -> T {
        T::from_cam16_unclamped(self, parameters)
    }
}

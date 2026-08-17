//! Types for the RGB color space, including spaces and standards.
//!
//! # Linear And Non-linear RGB
//!
//! Colors in images are often "gamma corrected", or converted using some
//! non-linear transfer function into a format like sRGB before being stored or
//! displayed. This is done as a compression method and to prevent banding; it's
//! also a bit of a legacy from the ages of the CRT monitors, where the output
//! from the electron gun was non-linear. The problem is that these formats are
//! *non-linear color spaces*, which means that many operations that you may
//! want to perform on colors (addition, subtraction, multiplication, linear
//! interpolation, etc.) will work unexpectedly when performed in such a
//! non-linear color space. Thus, the compression has to be reverted to restore
//! linearity and ensure that many operations on the colors behave as expected.
//!
//! But, even when colors *are* 'linear', there is yet more to explore.
//!
//! The most common way that colors are defined, especially for computer
//! storage, is in terms of so-called *tristimulus values*, meaning that all
//! colors can be represented as a vector of three values.
//! The reason colors can generally be stored as only a three-dimensional
//! vector, and not an *N*-dimensional one, where *N* is some number of possible
//! wavelengths of light, is because our eyes contain only three types of cones.
//! Each of these cones has its own sensitivity curve in response to the
//! wavelengths of visible light, giving us three "dimensions" of sensitivity to color.
//! These cones are often called the L, M, and S (for long, medium, and short)
//! cones, and their sensitivity curves *roughly* position them as most
//! sensitive to "red", "green", and "blue" parts of the spectrum. As such, we
//! can choose only three values to represent any possible color that a human is
//! able to see. An interesting consequence of this is that humans can see two
//! different objects which are emitting *completely different actual light
//! spectra* as the *exact same perceptual color* so long as those wavelengths,
//! when transformed by the sensitivity curves of our cones, end up resulting in
//! the same L, M, and S values sent to our brains.
//!
//! A **color space** (which simply refers to a set of standards by which we map
//! a set of arbitrary values to real-world colors) which uses tristimulus
//! values is often defined in terms of
//!
//!  1. Its **primaries**
//!  2. Its **reference white** or **white point**
//!
//! The **primaries** together represent the total *gamut* (i.e. displayable
//! range of colors) of that color space. The **white point** defines a
//! concrete tristimulus value that corresponds to a real, physical white
//! reflecting object being lit by a known light source and observed by the
//! 'standard observer' (i.e. a standardized model of human color perception).
//!
//! The informal "RGB" color space is such a tristimulus color space, since it
//! is defined by three values, but it is underspecified since we don't know
//! which primaries are being used (i.e. how exactly are the canonical "red",
//! "green", and "blue" defined?), nor its white point. In most cases, when
//! people talk about "RGB" or "Linear RGB" colors, what they are *actually*
//! talking about is the "Linear sRGB" color space, which uses the primaries and
//! white point defined in the sRGB standard, but which *does not* have the
//! (non-linear) sRGB *transfer function* applied.
//!
//! Palette takes these details into account and encodes them as type
//! parameters, with sRGB as the default. The goal is to make it easy to use
//! colors correctly and still allow advanced users a high degree of
//! flexibility.

use crate::{
    encoding::{self, FromLinear, Gamma, IntoLinear, Linear},
    stimulus::{FromStimulus, Stimulus},
    white_point::Any,
    Mat3, Yxy,
};

pub use self::rgb::{FromHexError, Iter, Rgb, Rgba};

pub mod channels;
#[allow(clippy::module_inception)]
mod rgb;

/// Non-linear sRGB, the most common RGB input/output format.
///
/// If you are looking for "just RGB", this is probably it. This type alias
/// helps by locking the more generic [`Rgb`] type to the sRGB format.
///
/// See [`Rgb`] for more details on how to create a value and use it.
pub type Srgb<T = f32> = Rgb<encoding::Srgb, T>;

/// Non-linear sRGB with an alpha component.
///
/// This is a transparent version of [`Srgb`], which is commonly used as the
/// input or output format. If you are looking for "just RGBA", this is probably
/// it.
///
/// See [`Rgb`], [`Rgba`] and [`Alpha`](crate::Alpha) for more details on how to
/// create a value and use it.
pub type Srgba<T = f32> = Rgba<encoding::Srgb, T>;

/// Linear sRGB.
///
/// You probably want [`Srgb`] if you are looking for an input or output format
/// (or "just RGB"). This is the linear version of sRGB, which is what you would
/// usually convert to before working with the color.
///
/// See [`Rgb`] for more details on how to create a value and use it.
#[doc(alias = "linear")]
pub type LinSrgb<T = f32> = Rgb<Linear<encoding::Srgb>, T>;

/// Linear sRGB with an alpha component.
///
/// You probably want [`Srgba`] if you are looking for an input or output format
/// (or "just RGB"). This is the linear version of sRGBA, which is what you
/// would usually convert to before working with the color.
///
/// See [`Rgb`], [`Rgba`] and [`Alpha`](crate::Alpha) for more details on how to
/// create a value and use it.
#[doc(alias = "linear")]
pub type LinSrgba<T = f32> = Rgba<Linear<encoding::Srgb>, T>;

/// Gamma 2.2 encoded sRGB.
///
/// This is similar to [`Srgb`], but uses the exponent function as an
/// approximation. It's a common trick to speed up conversion when accuracy can
/// be sacrificed. It's still faster to use `Srgb` when also converting to and
/// from `u8` at the same time.
///
/// See [`Rgb`] for more details on how to create a value and use it.
pub type GammaSrgb<T = f32> = Rgb<Gamma<encoding::Srgb>, T>;

/// Gamma 2.2 encoded sRGB with an alpha component.
///
/// This is similar to [`Srgba`], but uses the exponent function as an
/// approximation. It's a common trick to speed up conversion when accuracy can
/// be sacrificed. It's still faster to use `Srgba` when also converting to and
/// from `u8` at the same time.
///
/// See [`Rgb`], [`Rgba`] and [`Alpha`](crate::Alpha) for more details on how to
/// create a value and use it.
pub type GammaSrgba<T = f32> = Rgba<Gamma<encoding::Srgb>, T>;

/// An RGB space and a transfer function.
pub trait RgbStandard {
    /// The RGB color space.
    type Space: RgbSpace;

    /// The transfer function for the color components.
    type TransferFn;
}

impl<Sp, Tf> RgbStandard for (Sp, Tf)
where
    Sp: RgbSpace,
{
    type Space = Sp;
    type TransferFn = Tf;
}

impl<Pr, Wp, Tf> RgbStandard for (Pr, Wp, Tf)
where
    (Pr, Wp): RgbSpace,
{
    type Space = (Pr, Wp);
    type TransferFn = Tf;
}

/// A set of primaries and a white point.
pub trait RgbSpace {
    /// The primaries of the RGB color space.
    type Primaries;

    /// The white point of the RGB color space.
    type WhitePoint;

    /// Get a pre-defined matrix for converting an RGB value with this standard
    /// into an XYZ value.
    ///
    /// Returning `None` (as in the default implementation) means that the
    /// matrix will be computed dynamically, which is significantly slower.
    #[inline(always)]
    fn rgb_to_xyz_matrix() -> Option<Mat3<f64>> {
        None
    }

    /// Get a pre-defined matrix for converting an XYZ value into an RGB value
    /// with this standard.
    ///
    /// Returning `None` (as in the default implementation) means that the
    /// matrix will be computed dynamically, which is significantly slower.
    #[inline(always)]
    fn xyz_to_rgb_matrix() -> Option<Mat3<f64>> {
        None
    }
}

impl<P, W> RgbSpace for (P, W) {
    type Primaries = P;
    type WhitePoint = W;
}

/// Represents the red, green and blue primaries of an RGB space.
pub trait Primaries<T> {
    /// Primary red.
    fn red() -> Yxy<Any, T>;
    /// Primary green.
    fn green() -> Yxy<Any, T>;
    /// Primary blue.
    fn blue() -> Yxy<Any, T>;
}

impl<T, U> From<LinSrgb<T>> for Srgb<U>
where
    crate::encoding::Srgb: RgbStandard<Space = crate::encoding::Srgb> + FromLinear<T, U>,
{
    #[inline]
    fn from(lin_srgb: LinSrgb<T>) -> Self {
        lin_srgb.into_encoding()
    }
}

impl<T, U> From<Srgb<T>> for LinSrgb<U>
where
    crate::encoding::Srgb: RgbStandard<Space = crate::encoding::Srgb> + IntoLinear<U, T>,
{
    #[inline]
    fn from(srgb: Srgb<T>) -> Self {
        srgb.into_linear()
    }
}

impl<T, U> From<LinSrgb<T>> for Srgba<U>
where
    U: Stimulus,
    crate::encoding::Srgb: RgbStandard<Space = crate::encoding::Srgb> + FromLinear<T, U>,
{
    #[inline]
    fn from(lin_srgb: LinSrgb<T>) -> Self {
        let non_lin = Srgb::from_linear(lin_srgb);
        non_lin.into()
    }
}

impl<T, U> From<LinSrgba<T>> for Srgba<U>
where
    U: FromStimulus<T>,
    crate::encoding::Srgb: RgbStandard<Space = crate::encoding::Srgb> + FromLinear<T, U>,
{
    #[inline]
    fn from(lin_srgba: LinSrgba<T>) -> Self {
        Srgba::from_linear(lin_srgba)
    }
}

impl<T, U> From<Srgb<T>> for LinSrgba<U>
where
    U: Stimulus,
    crate::encoding::Srgb: RgbStandard<Space = crate::encoding::Srgb> + IntoLinear<U, T>,
{
    #[inline]
    fn from(srgb: Srgb<T>) -> Self {
        srgb.into_linear().into()
    }
}

impl<T, U> From<Srgba<T>> for LinSrgba<U>
where
    U: FromStimulus<T>,
    crate::encoding::Srgb: RgbStandard<Space = crate::encoding::Srgb> + IntoLinear<U, T>,
{
    #[inline]
    fn from(srgba: Srgba<T>) -> Self {
        srgba.into_linear()
    }
}

/// A packed representation of RGBA in RGBA order.
pub type PackedRgba<P = u32> = crate::cast::Packed<channels::Rgba, P>;

/// A packed representation of RGBA in ARGB order.
pub type PackedArgb<P = u32> = crate::cast::Packed<channels::Argb, P>;

/// A packed representation of RGBA in BGRA order.
pub type PackedBgra<P = u32> = crate::cast::Packed<channels::Bgra, P>;

/// A packed representation of RGBA in ABGR order.
pub type PackedAbgr<P = u32> = crate::cast::Packed<channels::Abgr, P>;

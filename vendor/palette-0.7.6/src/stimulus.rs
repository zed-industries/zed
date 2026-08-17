//! Traits for working with stimulus colors and values, such as RGB and XYZ.

use crate::{
    clamp,
    num::{One, Real, Round, Zero},
};

/// Color components that represent a stimulus intensity.
///
/// The term "stimulus" comes from "tristimulus", literally a set of three
/// stimuli, which is a term for color spaces that measure the intensity of
/// three primary color values. Classic examples of tristimulus color space are
/// XYZ and RGB.
///
/// Stimulus values are expected to have these properties:
///  * Has a typical range from `0` to some finite maximum, the "max intensity".
///    This represents a range from `0%` to `100%`. For example `0u8` to
///    `255u8`, `0.0f32` to `1.0f32`.
///  * Values below `0` are considered invalid for display purposes, but may
///    still be used in calculations.
///  * Values above the "max intensity" are sometimes supported, depending on
///    the application. For example in 3D rendering, where high values represent
///    intense light.
///  * Unsigned integer values (`u8`, `u16`, `u32`, etc.) have a range from `0`
///    to their largest representable value. For example `0u8` to `255u8` or
///    `0u16` to `65535u16`.
///  * Real values (`f32`, `f64`, fixed point types, etc.) have a range from
///    `0.0` to `1.0`.
pub trait Stimulus: Zero {
    /// The highest displayable value this component type can reach. Integers
    /// types are expected to return their maximum value, while real numbers
    /// (like floats) return 1.0. Higher values are allowed, but they may be
    /// lowered to this before converting to another format.
    #[must_use]
    fn max_intensity() -> Self;
}

impl<T> Stimulus for T
where
    T: Real + One + Zero,
{
    #[inline]
    fn max_intensity() -> Self {
        Self::one()
    }
}

macro_rules! impl_uint_components {
    ($($ty: ident),+) => {
        $(
            impl Stimulus for $ty {
                #[inline]
                fn max_intensity() -> Self {
                    $ty::MAX
                }
            }
        )*
    };
}

impl_uint_components!(u8, u16, u32, u64, u128);

/// A marker trait for colors where all components are stimuli.
///
/// Typical stimulus colors are RGB and XYZ.
pub trait StimulusColor {}

/// Converts from a stimulus color component type, while performing the
/// appropriate scaling, rounding and clamping.
///
/// ```
/// use palette::stimulus::FromStimulus;
///
/// // Scales the value up to u8::MAX while converting.
/// let u8_component = u8::from_stimulus(1.0f32);
/// assert_eq!(u8_component, 255);
/// ```
pub trait FromStimulus<T> {
    /// Converts `other` into `Self`, while performing the appropriate scaling,
    /// rounding and clamping.
    #[must_use]
    fn from_stimulus(other: T) -> Self;
}

impl<T, U: IntoStimulus<T>> FromStimulus<U> for T {
    #[inline]
    fn from_stimulus(other: U) -> T {
        other.into_stimulus()
    }
}

/// Converts into a stimulus color component type, while performing the
/// appropriate scaling, rounding and clamping.
///
/// ```
/// use palette::stimulus::IntoStimulus;
///
/// // Scales the value up to u8::MAX while converting.
/// let u8_component: u8 = 1.0f32.into_stimulus();
/// assert_eq!(u8_component, 255);
/// ```
pub trait IntoStimulus<T> {
    /// Converts `self` into `T`, while performing the appropriate scaling,
    /// rounding and clamping.
    #[must_use]
    fn into_stimulus(self) -> T;
}

impl<T> IntoStimulus<T> for T {
    #[inline]
    fn into_stimulus(self) -> T {
        self
    }
}

// C23 = 2^23, in f32
// C52 = 2^52, in f64
const C23: u32 = 0x4b00_0000;
const C52: u64 = 0x4330_0000_0000_0000;

// Float to uint conversion with rounding to nearest even number. Formula
// follows the form (x_f32 + C23_f32) - C23_u32, where x is the component. From
// Hacker's Delight, p. 378-380.
// Works on the range of [-0.25, 2^23] for f32, [-0.25, 2^52] for f64.
//
// Special cases:
// NaN -> uint::MAX
// inf -> uint::MAX
// -inf -> 0
// Greater than 2^23 for f64, 2^52 for f64 -> uint::MAX
macro_rules! convert_float_to_uint {
    ($float: ident; direct ($($direct_target: ident),+); $(via $temporary: ident ($($target: ident),+);)*) => {
        $(
            impl IntoStimulus<$direct_target> for $float {
                #[inline]
                fn into_stimulus(self) -> $direct_target {
                    let max = $direct_target::max_intensity() as $float;
                    let scaled = (self * max).min(max);
                    let f = scaled + f32::from_bits(C23);
                    (f.to_bits().saturating_sub(C23)) as $direct_target
                }
            }
        )+

        $(
            $(
                impl IntoStimulus<$target> for $float {
                    #[inline]
                    fn into_stimulus(self) -> $target {
                        let max = $target::max_intensity() as $temporary;
                        let scaled = (self as $temporary * max).min(max);
                        let f = scaled + f64::from_bits(C52);
                        (f.to_bits().saturating_sub(C52)) as  $target
                    }
                }
            )+
        )*
    };
}

// Double to uint conversion with rounding to nearest even number. Formula
// follows the form (x_f64 + C52_f64) - C52_u64, where x is the component.
macro_rules! convert_double_to_uint {
    ($double: ident; direct ($($direct_target: ident),+);) => {
        $(
            impl IntoStimulus<$direct_target> for $double {
                #[inline]
                fn into_stimulus(self) -> $direct_target {
                    let max = $direct_target::max_intensity() as $double;
                    let scaled = (self * max).min(max);
                    let f = scaled + f64::from_bits(C52);
                    (f.to_bits().saturating_sub(C52)) as $direct_target
                }
            }
        )+
    };
}

// Uint to float conversion with the formula (x_u32 + C23_u32) - C23_f32, where
// x is the component. We convert the component to f32 then multiply it by the
// reciprocal of the float representation max value for u8.
// Works on the range of [0, 2^23] for f32, [0, 2^52 - 1] for f64.
impl IntoStimulus<f32> for u8 {
    #[inline]
    fn into_stimulus(self) -> f32 {
        let comp_u = u32::from(self) + C23;
        let comp_f = f32::from_bits(comp_u) - f32::from_bits(C23);
        let max_u = u32::from(u8::MAX) + C23;
        let max_f = (f32::from_bits(max_u) - f32::from_bits(C23)).recip();
        comp_f * max_f
    }
}

// Uint to f64 conversion with the formula (x_u64 + C23_u64) - C23_f64.
impl IntoStimulus<f64> for u8 {
    #[inline]
    fn into_stimulus(self) -> f64 {
        let comp_u = u64::from(self) + C52;
        let comp_f = f64::from_bits(comp_u) - f64::from_bits(C52);
        let max_u = u64::from(u8::MAX) + C52;
        let max_f = (f64::from_bits(max_u) - f64::from_bits(C52)).recip();
        comp_f * max_f
    }
}

macro_rules! convert_uint_to_float {
    ($uint: ident; $(via $temporary: ident ($($target: ident),+);)*) => {
        $(
            $(
                impl IntoStimulus<$target> for $uint {
                    #[inline]
                    fn into_stimulus(self) -> $target {
                        let max = $uint::max_intensity() as $temporary;
                        let scaled = self as $temporary / max;
                        scaled as $target
                    }
                }
            )+
        )*
    };
}

macro_rules! convert_uint_to_uint {
    ($uint: ident; $(via $temporary: ident ($($target: ident),+);)*) => {
        $(
            $(
                impl IntoStimulus<$target> for $uint {
                    #[inline]
                    fn into_stimulus(self) -> $target {
                        let target_max = $target::max_intensity() as $temporary;
                        let own_max = $uint::max_intensity() as $temporary;
                        let scaled = (self as $temporary / own_max) * target_max;
                        clamp(Round::round(scaled), 0.0, target_max) as $target
                    }
                }
            )+
        )*
    };
}

impl IntoStimulus<f64> for f32 {
    #[inline]
    fn into_stimulus(self) -> f64 {
        f64::from(self)
    }
}
convert_float_to_uint!(f32; direct (u8, u16); via f64 (u32, u64, u128););

impl IntoStimulus<f32> for f64 {
    #[inline]
    fn into_stimulus(self) -> f32 {
        self as f32
    }
}
convert_double_to_uint!(f64; direct (u8, u16, u32, u64, u128););

convert_uint_to_uint!(u8; via f32 (u16); via f64 (u32, u64, u128););

convert_uint_to_float!(u16; via f32 (f32); via f64 (f64););
convert_uint_to_uint!(u16; via f32 (u8); via f64 (u32, u64, u128););

convert_uint_to_float!(u32; via f64 (f32, f64););
convert_uint_to_uint!(u32; via f64 (u8, u16, u64, u128););

convert_uint_to_float!(u64; via f64 (f32, f64););
convert_uint_to_uint!(u64; via f64 (u8, u16, u32, u128););

convert_uint_to_float!(u128; via f64 (f32, f64););
convert_uint_to_uint!(u128; via f64 (u8, u16, u32, u64););

#[cfg(test)]
mod test {
    use crate::stimulus::IntoStimulus;

    #[test]
    fn float_to_uint() {
        let data = vec![
            -800.0,
            -0.3,
            0.0,
            0.005,
            0.024983,
            0.01,
            0.15,
            0.3,
            0.5,
            0.6,
            0.7,
            0.8,
            0.8444,
            0.9,
            0.955,
            0.999,
            1.0,
            1.4,
            f32::from_bits(0x4b44_0000),
            core::f32::MAX,
            core::f32::MIN,
            core::f32::NAN,
            core::f32::INFINITY,
            core::f32::NEG_INFINITY,
        ];

        let expected = vec![
            0u8, 0, 0, 1, 6, 3, 38, 76, 128, 153, 178, 204, 215, 230, 244, 255, 255, 255, 255, 255,
            0, 255, 255, 0,
        ];

        for (d, e) in data.into_iter().zip(expected) {
            assert_eq!(IntoStimulus::<u8>::into_stimulus(d), e);
        }
    }

    #[test]
    fn double_to_uint() {
        let data = vec![
            -800.0,
            -0.3,
            0.0,
            0.005,
            0.024983,
            0.01,
            0.15,
            0.3,
            0.5,
            0.6,
            0.7,
            0.8,
            0.8444,
            0.9,
            0.955,
            0.999,
            1.0,
            1.4,
            f64::from_bits(0x4334_0000_0000_0000),
            core::f64::MAX,
            core::f64::MIN,
            core::f64::NAN,
            core::f64::INFINITY,
            core::f64::NEG_INFINITY,
        ];

        let expected = vec![
            0u8, 0, 0, 1, 6, 3, 38, 76, 128, 153, 178, 204, 215, 230, 244, 255, 255, 255, 255, 255,
            0, 255, 255, 0,
        ];

        for (d, e) in data.into_iter().zip(expected) {
            assert_eq!(IntoStimulus::<u8>::into_stimulus(d), e);
        }
    }

    #[cfg(feature = "approx")]
    #[test]
    fn uint_to_float() {
        fn into_stimulus_old(n: u8) -> f32 {
            let max = u8::MAX as f32;
            n as f32 / max
        }

        for n in (0..=255).step_by(5) {
            assert_relative_eq!(IntoStimulus::<f32>::into_stimulus(n), into_stimulus_old(n))
        }
    }

    #[cfg(feature = "approx")]
    #[test]
    fn uint_to_double() {
        fn into_stimulus_old(n: u8) -> f64 {
            let max = u8::MAX as f64;
            n as f64 / max
        }

        for n in (0..=255).step_by(5) {
            assert_relative_eq!(IntoStimulus::<f64>::into_stimulus(n), into_stimulus_old(n))
        }
    }
}

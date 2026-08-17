//! Traits related to traditional color theory.
//!
//! Traditional color theory is sometimes used as a guide when selecting colors
//! for artistic purposes. While it's not the same as modern color science, and
//! much more subjective, it may still be a helpful set of principles.
//!
//! This module is primarily based on the 12 color wheel, meaning that they use
//! colors that are separated by 30° around the hue circle. There are however
//! some concepts, such as [`Complementary`] colors, that are generally
//! independent from the 12 color wheel concept.
//!
//! Most of the traits in this module require the color space to have a hue
//! component. You will often see people use [`Hsv`][crate::Hsv] or
//! [`Hsl`][crate::Hsl] when demonstrating some of these techniques, but Palette
//! lets you use any hue based color space. Some traits are also implemented for
//! other color spaces, when it's possible to avoid converting them to their hue
//! based counterparts.

use crate::{angle::HalfRotation, num::Real, ShiftHue};

/// Represents the complementary color scheme.
///
/// A complementary color scheme consists of two colors on the opposite sides of
/// the color wheel.
pub trait Complementary: Sized {
    /// Return the complementary color of `self`.
    ///
    /// This is the same as if the hue of `self` would be rotated by 180°.
    ///
    /// The following example makes a complementary color pair:
    ///
    /// <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: hsl(120deg, 80%, 50%);"></div>
    /// <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: hsl(300deg, 80%, 50%);"></div>
    ///
    /// ```
    /// use palette::{Hsl, color_theory::Complementary};
    ///
    /// let primary = Hsl::new_srgb(120.0f32, 8.0, 0.5);
    /// let complementary = primary.complementary();
    ///
    /// let hues = (
    ///     primary.hue.into_positive_degrees(),
    ///     complementary.hue.into_positive_degrees(),
    /// );
    ///
    /// assert_eq!(hues, (120.0, 300.0));
    /// ```
    fn complementary(self) -> Self;
}

impl<T> Complementary for T
where
    T: ShiftHue,
    T::Scalar: HalfRotation,
{
    fn complementary(self) -> Self {
        self.shift_hue(T::Scalar::half_rotation())
    }
}

/// Represents the split complementary color scheme.
///
/// A split complementary color scheme consists of three colors, where the
/// second and third are adjacent to (30° away from) the complementary color of
/// the first.
pub trait SplitComplementary: Sized {
    /// Return the two split complementary colors of `self`.
    ///
    /// The colors are ordered by ascending hue, or `(hue+150°, hue+210°)`.
    /// Combined with the input color, these make up 3 adjacent colors.
    ///
    /// The following example makes a split complementary color scheme:
    ///
    /// <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: hsl(120deg, 80%, 50%);"></div>
    /// <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: hsl(270deg, 80%, 50%);"></div>
    /// <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: hsl(330deg, 80%, 50%);"></div>
    ///
    /// ```
    /// use palette::{Hsl, color_theory::SplitComplementary};
    ///
    /// let primary = Hsl::new_srgb(120.0f32, 8.0, 0.5);
    /// let (complementary1, complementary2) = primary.split_complementary();
    ///
    /// let hues = (
    ///     primary.hue.into_positive_degrees(),
    ///     complementary1.hue.into_positive_degrees(),
    ///     complementary2.hue.into_positive_degrees(),
    /// );
    ///
    /// assert_eq!(hues, (120.0, 270.0, 330.0));
    /// ```
    fn split_complementary(self) -> (Self, Self);
}

impl<T> SplitComplementary for T
where
    T: ShiftHue + Clone,
    T::Scalar: Real,
{
    fn split_complementary(self) -> (Self, Self) {
        let first = self.clone().shift_hue(T::Scalar::from_f64(150.0));
        let second = self.shift_hue(T::Scalar::from_f64(210.0));

        (first, second)
    }
}

/// Represents the analogous color scheme on a 12 color wheel.
///
/// An analogous color scheme consists of three colors next to each other (30°
/// apart) on the color wheel.
pub trait Analogous: Sized {
    /// Return the two additional colors of an analogous color scheme.
    ///
    /// The colors are ordered by ascending hue difference, or `(hue-30°,
    /// hue+30°)`. Combined with the input color, these make up 3 adjacent
    /// colors.
    ///
    /// The following example makes a 3 color analogous scheme:
    ///
    /// <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: hsl(90deg, 80%, 50%);"></div>
    /// <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: hsl(120deg, 80%, 50%);"></div>
    /// <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: hsl(150deg, 80%, 50%);"></div>
    ///
    /// ```
    /// use palette::{Hsl, color_theory::Analogous};
    ///
    /// let primary = Hsl::new_srgb(120.0f32, 0.8, 0.5);
    /// let (analog_down, analog_up) = primary.analogous();
    ///
    /// let hues = (
    ///     analog_down.hue.into_positive_degrees(),
    ///     primary.hue.into_positive_degrees(),
    ///     analog_up.hue.into_positive_degrees(),
    /// );
    ///
    /// assert_eq!(hues, (90.0, 120.0, 150.0));
    /// ```
    fn analogous(self) -> (Self, Self);

    /// Return the next two analogous colors, after the colors `analogous` returns.
    ///
    /// The colors are ordered by ascending hue difference, or `(hue-60°,
    /// hue+60°)`. Combined with the input color and the colors from
    /// `analogous`, these make up 5 adjacent colors.
    ///
    /// The following example makes a 5 color analogous scheme:
    ///
    /// <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: hsl(60deg, 80%, 50%);"></div>
    /// <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: hsl(90deg, 80%, 50%);"></div>
    /// <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: hsl(120deg, 80%, 50%);"></div>
    /// <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: hsl(150deg, 80%, 50%);"></div>
    /// <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: hsl(180deg, 80%, 50%);"></div>
    ///
    /// ```
    /// use palette::{Hsl, color_theory::Analogous};
    ///
    /// let primary = Hsl::new_srgb(120.0f32, 0.8, 0.5);
    /// let (analog_down1, analog_up1) = primary.analogous();
    /// let (analog_down2, analog_up2) = primary.analogous_secondary();
    ///
    /// let hues = (
    ///     analog_down2.hue.into_positive_degrees(),
    ///     analog_down1.hue.into_positive_degrees(),
    ///     primary.hue.into_positive_degrees(),
    ///     analog_up1.hue.into_positive_degrees(),
    ///     analog_up2.hue.into_positive_degrees(),
    /// );
    ///
    /// assert_eq!(hues, (60.0, 90.0, 120.0, 150.0, 180.0));
    /// ```
    fn analogous_secondary(self) -> (Self, Self);
}

impl<T> Analogous for T
where
    T: ShiftHue + Clone,
    T::Scalar: Real,
{
    fn analogous(self) -> (Self, Self) {
        let first = self.clone().shift_hue(T::Scalar::from_f64(330.0));
        let second = self.shift_hue(T::Scalar::from_f64(30.0));

        (first, second)
    }

    fn analogous_secondary(self) -> (Self, Self) {
        let first = self.clone().shift_hue(T::Scalar::from_f64(300.0));
        let second = self.shift_hue(T::Scalar::from_f64(60.0));

        (first, second)
    }
}

/// Represents the triadic color scheme.
///
/// A triadic color scheme consists of thee colors at a 120° distance from each
/// other.
pub trait Triadic: Sized {
    /// Return the two additional colors of a triadic color scheme.
    ///
    /// The colors are ordered by ascending relative hues, or `(hue+120°,
    /// hue+240°)`.
    ///
    /// The following example makes a triadic scheme:
    ///
    /// <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: hsl(120deg, 80%, 50%);"></div>
    /// <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: hsl(240deg, 80%, 50%);"></div>
    /// <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: hsl(0deg, 80%, 50%);"></div>
    ///
    /// ```
    /// use palette::{Hsl, color_theory::Triadic};
    ///
    /// let primary = Hsl::new_srgb(120.0f32, 0.8, 0.5);
    /// let (triadic1, triadic2) = primary.triadic();
    ///
    /// let hues = (
    ///     primary.hue.into_positive_degrees(),
    ///     triadic1.hue.into_positive_degrees(),
    ///     triadic2.hue.into_positive_degrees(),
    /// );
    ///
    /// assert_eq!(hues, (120.0, 240.0, 0.0));
    /// ```
    fn triadic(self) -> (Self, Self);
}

impl<T> Triadic for T
where
    T: ShiftHue + Clone,
    T::Scalar: Real,
{
    fn triadic(self) -> (Self, Self) {
        let first = self.clone().shift_hue(T::Scalar::from_f64(120.0));
        let second = self.shift_hue(T::Scalar::from_f64(240.0));

        (first, second)
    }
}

/// Represents the tetradic, or square, color scheme.
///
/// A tetradic color scheme consists of four colors at a 90° distance from each
/// other. These form two pairs of complementary colors.
#[doc(alias = "Square")]
pub trait Tetradic: Sized {
    /// Return the three additional colors of a tetradic color scheme.
    ///
    /// The colors are ordered by ascending relative hues, or `(hue+90°,
    /// hue+180°, hue+270°)`.
    ///
    /// The following example makes a tetradic scheme:
    ///
    /// <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: hsl(120deg, 80%, 50%);"></div>
    /// <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: hsl(210deg, 80%, 50%);"></div>
    /// <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: hsl(300deg, 80%, 50%);"></div>
    /// <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: hsl(30deg, 80%, 50%);"></div>
    ///
    /// ```
    /// use palette::{Hsl, color_theory::Tetradic};
    ///
    /// let primary = Hsl::new_srgb(120.0f32, 0.8, 0.5);
    /// let (tetradic1, tetradic2, tetradic3) = primary.tetradic();
    ///
    /// let hues = (
    ///     primary.hue.into_positive_degrees(),
    ///     tetradic1.hue.into_positive_degrees(),
    ///     tetradic2.hue.into_positive_degrees(),
    ///     tetradic3.hue.into_positive_degrees(),
    /// );
    ///
    /// assert_eq!(hues, (120.0, 210.0, 300.0, 30.0));
    /// ```
    fn tetradic(self) -> (Self, Self, Self);
}

impl<T> Tetradic for T
where
    T: ShiftHue + Clone,
    T::Scalar: Real,
{
    fn tetradic(self) -> (Self, Self, Self) {
        let first = self.clone().shift_hue(T::Scalar::from_f64(90.0));
        let second = self.clone().shift_hue(T::Scalar::from_f64(180.0));
        let third = self.shift_hue(T::Scalar::from_f64(270.0));

        (first, second, third)
    }
}

//! Algorithms for calculating the difference between colors.
//!
//! ## Selecting an algorithm
//!
//! Different distance/difference algorithms and formulas are good for different
//! situations. Some are faster but less accurate and some may only be suitable
//! for certain color spaces. This table may help navigating the options a bit
//! by summarizing the difference between the traits in this module.
//!
//! **Disclaimer:** _This is not an actual benchmark! It's always best to test
//! and evaluate the differences in an actual application, when possible._
//!
//! Property explanations:
//! - **Complexity:** Low complexity options are generally faster than high
//!   complexity options.
//! - **Accuracy:** How the numerical difference compares to the perceived
//!   difference. May differ with the color space.
//!
//! | Trait | Complexity | Accuracy | Notes |
//! |-------|------------|----------|-------|
//! | [`Ciede2000`] | High | High for small differences, lower for large differences | The de-facto standard, but requires complex calculations to compensate for increased errors in certain areas of the CIE L\*a\*b\* (CIELAB) space.
//! | [`ImprovedCiede2000`] | High | High for small differences, lower for large differences | A general improvement of [`Ciede2000`], using a formula by Huang et al.
//! | [`DeltaE`] | Usually low | Medium to high | The formula differs between color spaces and may not always be the best. Other formulas, such as [`Ciede2000`], may be preferred for some spaces.
//! | [`ImprovedDeltaE`] | Usually low to medium | Medium to high | A general improvement of [`DeltaE`], using a formula by Huang et al.
//! | [`EuclideanDistance`] | Low | Medium to high for perceptually uniform spaces, otherwise low | Can be good enough for perceptually uniform spaces or as a "quick and dirty" check.
//! | [`HyAb`] | Low | High accuracy for medium to large differences. Less accurate than CIEDE2000 for small differences, but still performs well and is much less computationally expensive. | Similar to Euclidean distance, but separates lightness and chroma more. Limited to Cartesian spaces with a lightness axis and a chroma plane.
//! | [`Wcag21RelativeContrast`] | Low | Low and only compares lightness | Meant for checking contrasts in computer graphics (such as between text and background colors), assuming sRGB. Mostly useful as a hint or for checking WCAG 2.1 compliance, considering the criticism it has received.

use core::ops::{Add, BitAnd, BitOr, Div, Mul};

use crate::{
    angle::RealAngle,
    bool_mask::{HasBoolMask, LazySelect},
    convert::IntoColorUnclamped,
    num::{
        Abs, Arithmetics, Exp, Hypot, MinMax, One, PartialCmp, Powf, Powi, Real, Sqrt,
        Trigonometry, Zero,
    },
    white_point::D65,
    Lab, Lch, LinLuma,
};

/// A trait for calculating the color difference between two colors.
#[deprecated(
    since = "0.7.2",
    note = "replaced by `palette::color_difference::Ciede2000`"
)]
pub trait ColorDifference {
    /// The type of the calculated color difference.
    type Scalar;

    /// Return the difference or distance between two colors.
    #[must_use]
    fn get_color_difference(self, other: Self) -> Self::Scalar;
}

/// Calculate the CIEDE2000 Δ*E\** (Delta E) color difference between two
/// colors.
///
/// CIEDE2000 is a formula by the CIE that calculates a distance metric, Δ*E\**
/// (also known as Delta E), as an estimate of perceived color distance or
/// difference. CIEDE2000 is an improvement over Δ*E* (see [`DeltaE`]) for CIE
/// L\*a\*b\* and CIE L\*C\*h° (see [`Lab`] and [`Lch`]).
///
/// There is a "just noticeable difference" between two colors when the Δ*E\**
/// is roughly greater than 1. Thus, the color difference is more suited for
/// calculating small distances between colors as opposed to large differences.
#[doc(alias = "ColorDifference")]
pub trait Ciede2000 {
    /// The type for the Δ*E\** (Delta E).
    type Scalar;

    /// Calculate the CIEDE2000 Δ*E\** (Delta E) color difference between `self` and `other`.
    #[must_use]
    fn difference(self, other: Self) -> Self::Scalar;
}

/// Calculate the CIEDE2000 Δ*E'* (improved IEDE2000 Δ*E\**) color difference.
///
/// The "improved CIEDE2000" uses the output of [`Ciede2000`] and enhances it
/// according to *Power functions improving the performance of color-difference
/// formulas* by Huang et al.
pub trait ImprovedCiede2000: Ciede2000 {
    /// Calculate the CIEDE2000 Δ*E'* (improved IEDE2000 Δ*E\**) color
    /// difference between `self` and `other`.
    #[must_use]
    fn improved_difference(self, other: Self) -> Self::Scalar;
}

impl<C> ImprovedCiede2000 for C
where
    C: Ciede2000,
    C::Scalar: Real + Mul<C::Scalar, Output = C::Scalar> + Powf,
{
    #[inline]
    fn improved_difference(self, other: Self) -> Self::Scalar {
        // Coefficients from "Power functions improving the performance of
        // color-difference formulas" by Huang et al.
        // https://opg.optica.org/oe/fulltext.cfm?uri=oe-23-1-597&id=307643
        C::Scalar::from_f64(1.43) * self.difference(other).powf(C::Scalar::from_f64(0.7))
    }
}

/// Container of components necessary to calculate CIEDE color difference
pub(crate) struct LabColorDiff<T> {
    /// Lab color lightness
    pub l: T,
    /// Lab color a* value
    pub a: T,
    /// Lab color b* value
    pub b: T,
    /// Lab color chroma value
    pub chroma: T,
}

impl<Wp, T> From<Lab<Wp, T>> for LabColorDiff<T>
where
    T: Hypot + Clone,
{
    #[inline]
    fn from(color: Lab<Wp, T>) -> Self {
        // Color difference calculation requires Lab and chroma components. This
        // function handles the conversion into those components which are then
        // passed to `get_ciede_difference()` where calculation is completed.
        LabColorDiff {
            l: color.l,
            a: color.a.clone(),
            b: color.b.clone(),
            chroma: color.a.hypot(color.b),
        }
    }
}

impl<Wp, T> From<Lch<Wp, T>> for LabColorDiff<T>
where
    T: Clone,
    Lch<Wp, T>: IntoColorUnclamped<Lab<Wp, T>>,
{
    #[inline]
    fn from(color: Lch<Wp, T>) -> Self {
        let chroma = color.chroma.clone();
        let Lab { l, a, b, .. } = color.into_color_unclamped();

        LabColorDiff { l, a, b, chroma }
    }
}

/// Calculate the CIEDE2000 color difference for two colors in Lab color space.
/// There is a "just noticeable difference" between two colors when the delta E
/// is roughly greater than 1. Thus, the color difference is more suited for
/// calculating small distances between colors as opposed to large differences.
#[rustfmt::skip]
pub(crate) fn get_ciede2000_difference<T>(this: LabColorDiff<T>, other: LabColorDiff<T>) -> T
where
    T: Real
        + RealAngle
        + One
        + Zero
        + Trigonometry
        + Abs
        + Sqrt
        + Powi
        + Exp
        + Arithmetics
        + PartialCmp
        + Clone,
    T::Mask: LazySelect<T> + BitAnd<Output = T::Mask> + BitOr<Output = T::Mask>
{
    let c_bar = (this.chroma + other.chroma) / T::from_f64(2.0);
    let c_bar_pow_seven = c_bar.powi(7);
    let twenty_five_pow_seven = T::from_f64(6103515625.0);
    let pi_over_180 = T::from_f64(core::f64::consts::PI / 180.0);

    let g = T::from_f64(0.5)
        * (T::one() - (c_bar_pow_seven.clone() / (c_bar_pow_seven + &twenty_five_pow_seven)).sqrt());
    let a_one_prime = this.a * (T::one() + &g);
    let a_two_prime = other.a * (T::one() + g);
    let c_one_prime = (a_one_prime.clone() * &a_one_prime + this.b.clone() * &this.b).sqrt();
    let c_two_prime = (a_two_prime.clone() * &a_two_prime + other.b.clone() * &other.b).sqrt();

    let calc_h_prime = |b: T, a_prime: T| -> T {
        lazy_select! {
            if b.eq(&T::zero()) & a_prime.eq(&T::zero()) => T::zero(),
            else => {
                let result = T::radians_to_degrees(b.atan2(a_prime));
                lazy_select! {
                    if result.lt(&T::zero()) => result.clone() + T::from_f64(360.0),
                    else => result.clone(),
                }
            },
        }
    };
    let h_one_prime = calc_h_prime(this.b, a_one_prime);
    let h_two_prime = calc_h_prime(other.b, a_two_prime);

    let h_prime_diff = h_two_prime.clone() - &h_one_prime;
    let h_prime_abs_diff = h_prime_diff.clone().abs();

    let delta_h_prime: T = lazy_select! {
        if c_one_prime.eq(&T::zero()) | c_two_prime.eq(&T::zero()) => T::zero(),
        if h_prime_abs_diff.lt_eq(&T::from_f64(180.0)) => h_prime_diff.clone(),
        if h_two_prime.lt_eq(&h_one_prime) => h_prime_diff.clone() + T::from_f64(360.0),
        else => h_prime_diff.clone() - T::from_f64(360.0),
    };

    let delta_big_h_prime = T::from_f64(2.0)
        * (c_one_prime.clone() * &c_two_prime).sqrt()
        * (delta_h_prime / T::from_f64(2.0) * &pi_over_180).sin();
    let h_prime_sum = h_one_prime + h_two_prime;
    let h_bar_prime = lazy_select! {
        if c_one_prime.eq(&T::zero()) | c_two_prime.eq(&T::zero()) => h_prime_sum.clone(),
        if h_prime_abs_diff.gt(&T::from_f64(180.0)) => {
            (h_prime_sum.clone() + T::from_f64(360.0)) / T::from_f64(2.0)
        },
        else => h_prime_sum.clone() / T::from_f64(2.0),
    };

    let l_bar = (this.l.clone() + &other.l) / T::from_f64(2.0);
    let c_bar_prime = (c_one_prime.clone() + &c_two_prime) / T::from_f64(2.0);

    let t: T = T::one()
        - T::from_f64(0.17) * ((h_bar_prime.clone() - T::from_f64(30.0)) * &pi_over_180).cos()
        + T::from_f64(0.24) * ((h_bar_prime.clone() * T::from_f64(2.0)) * &pi_over_180).cos()
        + T::from_f64(0.32) * ((h_bar_prime.clone() * T::from_f64(3.0) + T::from_f64(6.0)) * &pi_over_180).cos()
        - T::from_f64(0.20) * ((h_bar_prime.clone() * T::from_f64(4.0) - T::from_f64(63.0)) * &pi_over_180).cos();
    let s_l = T::one()
        + ((T::from_f64(0.015) * (l_bar.clone() - T::from_f64(50.0)) * (l_bar.clone() - T::from_f64(50.0)))
            / ((l_bar.clone() - T::from_f64(50.0)) * (l_bar - T::from_f64(50.0)) + T::from_f64(20.0)).sqrt());
    let s_c = T::one() + T::from_f64(0.045) * &c_bar_prime;
    let s_h = T::one() + T::from_f64(0.015) * &c_bar_prime * t;

    let delta_theta = T::from_f64(30.0)
        * (-(((h_bar_prime.clone() - T::from_f64(275.0)) / T::from_f64(25.0))
            * ((h_bar_prime - T::from_f64(275.0)) / T::from_f64(25.0))))
        .exp();
    let c_bar_prime_pow_seven = c_bar_prime.powi(7);
    let r_c: T = T::from_f64(2.0)
        * (c_bar_prime_pow_seven.clone() / (c_bar_prime_pow_seven + twenty_five_pow_seven)).sqrt();
    let r_t = -r_c * (T::from_f64(2.0) * delta_theta * pi_over_180).sin();

    let k_l = T::one();
    let k_c = T::one();
    let k_h = T::one();
    let delta_l_prime = other.l - this.l;
    let delta_c_prime = c_two_prime - c_one_prime;

    ((delta_l_prime.clone() / (k_l.clone() * &s_l)) * (delta_l_prime / (k_l * s_l))
        + (delta_c_prime.clone() / (k_c.clone() * &s_c)) * (delta_c_prime.clone() / (k_c.clone() * &s_c))
        + (delta_big_h_prime.clone() / (k_h.clone() * &s_h)) * (delta_big_h_prime.clone() / (k_h.clone() * &s_h))
        + (r_t * delta_c_prime * delta_big_h_prime) / (k_c * s_c * k_h * s_h))
        .sqrt()
}

/// Calculate the distance between two colors as if they were coordinates in
/// Euclidean space.
///
/// Euclidean distance is not always a good measurement of visual color
/// difference, depending on the color space. Some spaces, like [`Lab`] and
/// [`Oklab`][crate::Oklab], will give a fairly uniform result, while other
/// spaces, such as [`Rgb`][crate::rgb::Rgb], will give much less uniform
/// results. Despite that, it's still appropriate for some applications.
pub trait EuclideanDistance: Sized {
    /// The type for the distance value.
    type Scalar;

    /// Calculate the Euclidean distance from `self` to `other`.
    #[must_use]
    fn distance(self, other: Self) -> Self::Scalar
    where
        Self::Scalar: Sqrt,
    {
        self.distance_squared(other).sqrt()
    }

    /// Calculate the squared Euclidean distance from `self` to `other`.
    ///
    /// This is typically a faster option than [`Self::distance`] for some
    /// cases, such as when comparing two distances.
    #[must_use]
    fn distance_squared(self, other: Self) -> Self::Scalar;
}

/// Calculate and check the WCAG 2.1 relative contrast and relative luminance.
///
/// W3C's Web Content Accessibility Guidelines (WCAG) 2.1 suggest a method to
/// calculate accessible contrast ratios of text and background colors for those
/// with low vision or color vision deficiencies, and for contrast of colors
/// used in user interface graphics objects.
///
/// These criteria come with a couple of caveats:
/// * sRGB is assumed as the presentation color space, which is why it's only
///   implemented for a limited set of [`Rgb`][crate::rgb::Rgb] and
///   [`Luma`][crate::Luma] spaces.
/// * The contrast ratio is not considered entirely consistent with the
///   perceived contrast. WCAG 3.x is supposed to provide a better measurement.
///
/// Because of the inconsistency with perceived contrast, these methods are more
/// suitable as hints and for mechanical verification of standards compliance,
/// than for accurate analysis. Remember to not only rely on the numbers, but to
/// also test your interfaces with actual people in actual situations for the
/// best results.
///
/// The following example checks the contrast ratio of two colors in sRGB
/// format:
///
/// ```rust
/// use std::str::FromStr;
/// use palette::{Srgb, color_difference::Wcag21RelativeContrast};
/// # fn main() -> Result<(), palette::rgb::FromHexError> {
///
/// // the rustdoc "DARK" theme background and text colors
/// let background: Srgb<f32> = Srgb::from(0x353535).into_format();
/// let foreground = Srgb::from_str("#ddd")?.into_format();
///
/// assert!(background.has_enhanced_contrast_text(foreground));
/// # Ok(())
/// # }
/// ```
pub trait Wcag21RelativeContrast: Sized {
    /// The scalar type used for luminance and contrast.
    type Scalar: Real
        + Add<Self::Scalar, Output = Self::Scalar>
        + Div<Self::Scalar, Output = Self::Scalar>
        + PartialCmp
        + MinMax;

    /// Returns the WCAG 2.1 [relative
    /// luminance](https://www.w3.org/TR/WCAG21/#dfn-relative-luminance) of
    /// `self`.
    ///
    /// The relative luminance is a value between 0 and 1, where 0 is the
    /// darkest black and 1 is the lightest white. This is the same as clamped
    /// [`LinLuma`], meaning that the typical implementation of this method
    /// would be `self.into_color()`.
    #[must_use]
    fn relative_luminance(self) -> LinLuma<D65, Self::Scalar>;

    /// Returns the WCAG 2.1 relative luminance contrast between `self` and
    /// `other`.
    ///
    /// A return value of, for example, 4 represents a contrast ratio of 4:1
    /// between the lightest and darkest of the two colors. The range is from
    /// 1:1 to 21:1, and a higher contrast ratio is generally desirable.
    ///
    /// This method is independent of the order of the colors, so
    /// `a.relative_contrast(b)` and `b.relative_contrast(a)` would return the
    /// same value.
    #[must_use]
    #[inline]
    fn relative_contrast(self, other: Self) -> Self::Scalar {
        let (min_luma, max_luma) = self
            .relative_luminance()
            .luma
            .min_max(other.relative_luminance().luma);

        (Self::Scalar::from_f64(0.05) + max_luma) / (Self::Scalar::from_f64(0.05) + min_luma)
    }

    /// Verify the contrast between two colors satisfies SC 1.4.3. Contrast is
    /// at least 4.5:1 (Level AA).
    ///
    /// This applies for meaningful text, such as body text. Font sizes of 18
    /// points or lager, or 14 points when bold, are considered large and can be
    /// checked with
    /// [`has_min_contrast_large_text`][Wcag21RelativeContrast::has_min_contrast_large_text]
    /// instead.
    ///
    /// [Success Criterion 1.4.3 Contrast (Minimum) (Level
    /// AA)](https://www.w3.org/WAI/WCAG21/Understanding/contrast-minimum)
    #[must_use]
    #[inline]
    fn has_min_contrast_text(self, other: Self) -> <Self::Scalar as HasBoolMask>::Mask {
        self.relative_contrast(other)
            .gt_eq(&Self::Scalar::from_f64(4.5))
    }

    /// Verify the contrast between two colors satisfies SC 1.4.3 for large
    /// text. Contrast is at least 3:1 (Level AA).
    ///
    /// This applies for meaningful large text, such as headings. Font sizes of
    /// 18 points or lager, or 14 points when bold, are considered large.
    ///
    /// [Success Criterion 1.4.3 Contrast (Minimum) (Level
    /// AA)](https://www.w3.org/WAI/WCAG21/Understanding/contrast-minimum)
    #[must_use]
    #[inline]
    fn has_min_contrast_large_text(self, other: Self) -> <Self::Scalar as HasBoolMask>::Mask {
        self.relative_contrast(other)
            .gt_eq(&Self::Scalar::from_f64(3.0))
    }

    /// Verify the contrast between two colors satisfies SC 1.4.6. Contrast is
    /// at least 7:1 (Level AAA).
    ///
    /// This applies for meaningful text, such as body text. Font sizes of 18
    /// points or lager, or 14 points when bold, are considered large and can be
    /// checked with
    /// [`has_enhanced_contrast_large_text`][Wcag21RelativeContrast::has_enhanced_contrast_large_text]
    /// instead.
    ///
    /// [Success Criterion 1.4.6 Contrast (Enhanced) (Level
    /// AAA)](https://www.w3.org/WAI/WCAG21/Understanding/contrast-enhanced)
    #[must_use]
    #[inline]
    fn has_enhanced_contrast_text(self, other: Self) -> <Self::Scalar as HasBoolMask>::Mask {
        self.relative_contrast(other)
            .gt_eq(&Self::Scalar::from_f64(7.0))
    }

    /// Verify the contrast between two colors satisfies SC 1.4.6 for large
    /// text. Contrast is at least 4.5:1 (Level AAA).
    ///
    /// This applies for meaningful large text, such as headings. Font sizes of
    /// 18 points or lager, or 14 points when bold, are considered large.
    ///
    /// [Success Criterion 1.4.6 Contrast (Enhanced) (Level
    /// AAA)](https://www.w3.org/WAI/WCAG21/Understanding/contrast-enhanced)
    #[must_use]
    #[inline]
    fn has_enhanced_contrast_large_text(self, other: Self) -> <Self::Scalar as HasBoolMask>::Mask {
        self.relative_contrast(other)
            .gt_eq(&Self::Scalar::from_f64(4.5))
    }

    /// Verify the contrast between two colors satisfies SC 1.4.11 for graphical
    /// objects. Contrast is at least 3:1 (Level AA).
    ///
    /// This applies for any graphical object that aren't text, such as
    /// meaningful images and interactive user interface elements.
    ///
    /// [Success Criterion 1.4.11 Non-text Contrast (Level
    /// AA)](https://www.w3.org/WAI/WCAG21/Understanding/non-text-contrast.html)
    #[must_use]
    #[inline]
    fn has_min_contrast_graphics(self, other: Self) -> <Self::Scalar as HasBoolMask>::Mask {
        self.relative_contrast(other)
            .gt_eq(&Self::Scalar::from_f64(3.0))
    }
}

/// Calculate a combination of Euclidean and Manhattan/City-block distance
/// between two colors.
///
/// The HyAB distance was suggested as an alternative to CIEDE2000 for large
/// color differences in [Distance metrics for very large color
/// differences](http://markfairchild.org/PDFs/PAP40.pdf) (in [Color Res Appl.
/// 2019;1–16](https://doi.org/10.1002/col.22451)) by Saeedeh Abasi, Mohammad
/// Amani Tehran and Mark D. Fairchild. It's originally meant for [CIE L\*a\*b\*
/// (CIELAB)][crate::Lab], but this trait is also implemented for other color
/// spaces that have similar semantics, although **without the same quality
/// guarantees**.
///
/// The hybrid distance is the sum of the absolute lightness difference and the
/// distance on the chroma plane. This makes the lightness and chroma
/// differences more independent from each other, which is meant to correspond
/// more to how humans perceive the two qualities.
pub trait HyAb {
    /// The type for the distance value.
    type Scalar;

    /// Calculate the hybrid distance between `self` and `other`.
    ///
    /// This returns the sum of the absolute lightness difference and the
    /// distance on the chroma plane.
    #[must_use]
    fn hybrid_distance(self, other: Self) -> Self::Scalar;
}

/// Calculate the Δ*E* color difference between two colors.
///
/// This represents the original Δ*E* formula for a color space. It's often a
/// Euclidean distance for perceptually uniform color spaces and may not always
/// be the best option. See the [`color_difference`](self) module for more
/// details and options.
pub trait DeltaE {
    /// The type for the distance value.
    type Scalar;

    /// Calculate the Δ*E* color difference metric for `self` and `other`,
    /// according to the color space's specification.
    #[must_use]
    fn delta_e(self, other: Self) -> Self::Scalar;
}

/// Calculate the Δ*E'* (improved Δ*E*) color difference between two colors.
///
/// The Δ*E'* uses the output of [`DeltaE`] and enhances it according to *Power
/// functions improving the performance of color-difference formulas* by Huang
/// et al. Only spaces with specified coefficients implement this trait.
pub trait ImprovedDeltaE: DeltaE {
    /// Calculate the Δ*E'* (improved Δ*E*) color difference metric for `self`
    /// and `other`, according to the color space's specification and later
    /// improvements by Huang et al.
    #[must_use]
    fn improved_delta_e(self, other: Self) -> Self::Scalar;
}

#[cfg(feature = "approx")]
#[cfg(test)]
mod test {
    use core::str::FromStr;

    use super::{HyAb, Wcag21RelativeContrast};
    use crate::{FromColor, Lab, Srgb};

    #[test]
    fn relative_contrast() {
        let white = Srgb::new(1.0f32, 1.0, 1.0);
        let black = Srgb::new(0.0, 0.0, 0.0);

        assert_relative_eq!(white.relative_contrast(white), 1.0);
        assert_relative_eq!(white.relative_contrast(black), 21.0);
        assert_relative_eq!(
            white.relative_contrast(black),
            black.relative_contrast(white)
        );

        let c1 = Srgb::from_str("#600").unwrap().into_format();

        assert_relative_eq!(c1.relative_contrast(white), 13.41, epsilon = 0.01);
        assert_relative_eq!(c1.relative_contrast(black), 1.56, epsilon = 0.01);

        assert!(c1.has_min_contrast_text(white));
        assert!(c1.has_min_contrast_large_text(white));
        assert!(c1.has_enhanced_contrast_text(white));
        assert!(c1.has_enhanced_contrast_large_text(white));
        assert!(c1.has_min_contrast_graphics(white));

        assert!(!c1.has_min_contrast_text(black));
        assert!(!c1.has_min_contrast_large_text(black));
        assert!(!c1.has_enhanced_contrast_text(black));
        assert!(!c1.has_enhanced_contrast_large_text(black));
        assert!(!c1.has_min_contrast_graphics(black));

        let c1 = Srgb::from_str("#066").unwrap().into_format();

        assert_relative_eq!(c1.relative_contrast(white), 6.79, epsilon = 0.01);
        assert_relative_eq!(c1.relative_contrast(black), 3.09, epsilon = 0.01);

        let c1 = Srgb::from_str("#9f9").unwrap().into_format();

        assert_relative_eq!(c1.relative_contrast(white), 1.22, epsilon = 0.01);
        assert_relative_eq!(c1.relative_contrast(black), 17.11, epsilon = 0.01);
    }

    #[test]
    fn hyab() {
        // From https://github.com/Evercoder/culori/blob/cd1fe08a12fa9ddfcf6b2e82914733d23ac117d0/test/difference.test.js#L186
        let red = Lab::<_, f64>::from_color(Srgb::from(0xff0000).into_linear());
        let green = Lab::<_, f64>::from_color(Srgb::from(0x008000).into_linear());
        assert_relative_eq!(
            red.hybrid_distance(green),
            139.93576718451553,
            epsilon = 0.000001
        );
    }
}

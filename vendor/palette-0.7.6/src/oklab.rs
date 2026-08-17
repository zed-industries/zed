//! Types for the Oklab color space.

use core::{any::TypeId, fmt::Debug, ops::Mul};

pub use alpha::Oklaba;

use crate::{
    angle::RealAngle,
    bool_mask::HasBoolMask,
    convert::{FromColorUnclamped, IntoColorUnclamped},
    encoding::{IntoLinear, Srgb},
    matrix::multiply_xyz,
    num::{Arithmetics, Cbrt, Hypot, MinMax, One, Powi, Real, Sqrt, Trigonometry, Zero},
    ok_utils::{toe_inv, ChromaValues, LC, ST},
    rgb::{Rgb, RgbSpace, RgbStandard},
    white_point::D65,
    LinSrgb, Mat3, Okhsl, Okhsv, Oklch, Xyz,
};

pub use self::properties::Iter;

#[cfg(feature = "random")]
pub use self::random::UniformOklab;

mod alpha;
mod properties;
#[cfg(feature = "random")]
mod random;
#[cfg(test)]
#[cfg(feature = "approx")]
mod visual_eq;

// Using recalculated matrix values from
// https://github.com/LeaVerou/color.js/blob/master/src/spaces/oklab.js
//
// see https://github.com/w3c/csswg-drafts/issues/6642#issuecomment-943521484
// and the following https://github.com/w3c/csswg-drafts/issues/6642#issuecomment-945714988

/// XYZ to LSM transformation matrix
#[rustfmt::skip]
fn m1<T: Real>() -> Mat3<T> {
    [
        T::from_f64(0.8190224432164319), T::from_f64(0.3619062562801221), T::from_f64(-0.12887378261216414),
        T::from_f64(0.0329836671980271), T::from_f64(0.9292868468965546), T::from_f64(0.03614466816999844),
        T::from_f64(0.048177199566046255), T::from_f64(0.26423952494422764), T::from_f64(0.6335478258136937),
    ]
}

/// LMS to XYZ transformation matrix
#[rustfmt::skip]
pub(crate) fn m1_inv<T: Real>() -> Mat3<T> {
    [
        T::from_f64(1.2268798733741557), T::from_f64(-0.5578149965554813), T::from_f64(0.28139105017721583),
        T::from_f64(-0.04057576262431372), T::from_f64(1.1122868293970594), T::from_f64(-0.07171106666151701),
        T::from_f64(-0.07637294974672142), T::from_f64(-0.4214933239627914), T::from_f64(1.5869240244272418),
    ]
}

/// LMS to Oklab transformation matrix
#[rustfmt::skip]
fn m2<T: Real>() -> Mat3<T> {
    [
        T::from_f64(0.2104542553), T::from_f64(0.7936177850), T::from_f64(-0.0040720468),
        T::from_f64(1.9779984951), T::from_f64(-2.4285922050), T::from_f64(0.4505937099),
        T::from_f64(0.0259040371), T::from_f64(0.7827717662), T::from_f64(-0.8086757660),
    ]
}

/// Oklab to LMS transformation matrix
#[rustfmt::skip]
#[allow(clippy::excessive_precision)]
pub(crate) fn m2_inv<T: Real>() -> Mat3<T> {
    [
        T::from_f64(0.99999999845051981432), T::from_f64(0.39633779217376785678), T::from_f64(0.21580375806075880339),
        T::from_f64(1.0000000088817607767), T::from_f64(-0.1055613423236563494), T::from_f64(-0.063854174771705903402),
        T::from_f64(1.0000000546724109177), T::from_f64(-0.089484182094965759684), T::from_f64(-1.2914855378640917399),
    ]
}

/// The [Oklab color space](https://bottosson.github.io/posts/oklab/).
///
/// # Characteristics
/// `Oklab` is a *perceptual* color space. It does not relate to an output
/// device (a monitor or printer) but instead relates to the [CIE standard
/// observer](https://en.wikipedia.org/wiki/CIE_1931_color_space#CIE_standard_observer)
/// -- an averaging of the results of color matching experiments under
/// laboratory conditions.
///
/// `Oklab` is a uniform color space ([Compare to the HSV color
/// space](https://bottosson.github.io/posts/oklab/#comparing-oklab-to-hsv)). It
/// is useful for things like:
/// * Turning an image grayscale, while keeping the perceived lightness the same
/// * Increasing the saturation of colors, while maintaining perceived hue and
///   lightness
/// * Creating smooth and uniform looking transitions between colors
///
/// `Oklab`'s structure is similar to [L\*a\*b\*](crate::Lab). It is based on
/// the [opponent color model of human
/// vision](https://en.wikipedia.org/wiki/Opponent_process), where red and green
/// form an opponent pair, and blue and yellow form an opponent pair.
///
/// `Oklab` uses [D65](https://en.wikipedia.org/wiki/Illuminant_D65)'s
/// whitepoint -- daylight illumination, which is also used by sRGB, rec2020 and
/// Display P3 color spaces -- and assumes normal well-lit viewing conditions,
/// to which the eye is adapted. Thus `Oklab`s lightness `l` technically is a
/// measure of relative brightness -- a subjective measure -- not relative
/// luminance. The lightness is scale/exposure-independend, i.e. independent of
/// the actual luminance of the color, as displayed by some medium, and even for
/// blindingly bright colors or very bright or dark viewing conditions assumes,
/// that the eye is adapted to the color's luminance and the hue and chroma are
/// perceived linearly.
///
///
/// `Oklab`'s chroma is unlimited. Thus it can represent colors of any color
/// space (including HDR). `l` is in the range `0.0 .. 1.0` and `a` and `b` are
/// unbounded.
///
/// # Conversions
/// [`Oklch`] is a cylindrical form of `Oklab`.
///
/// `Oklab` colors converted from valid (i.e. clamped) `sRGB` will be in the
/// `sRGB` gamut.
///
/// [`Okhsv`], [`Okhwb`][crate::Okhsv] and [`Okhsl`] reference the `sRGB` gamut.
/// The transformation from `Oklab` to one of them is based on the assumption,
/// that the transformed `Oklab` value is within `sRGB`.
///
/// `Okhsv`, `Okhwb` and `Okhsl` are not applicable to HDR, which also come with
/// color spaces with wider gamuts. They require [additional
/// research](https://bottosson.github.io/posts/colorpicker/#ideas-for-future-work).
///
/// When a `Oklab` color is converted from [`Srgb`](crate::rgb::Srgb) or a
/// equivalent color space, e.g. [`Hsv`][crate::Hsv], [`Okhsv`],
/// [`Hsl`][crate::Hsl], [`Okhsl`], [`Hwb`][crate::Hwb],
/// [`Okhwb`][crate::Okhwb], it's lightness will be relative to the (user
/// controlled) maximum contrast and luminance of the display device, to which
/// the eye is assumed to be adapted.
///
/// # Clamping
/// [`Clamp`][crate::Clamp]ing will only clamp `l`. Clamping does not guarantee
/// the color to be inside the perceptible or any display-dependent color space
/// (like *sRGB*).
///
/// To ensure a color is within the *sRGB* gamut, it can first be converted to
/// `Okhsv`, clamped there and converted it back to `Oklab`.
///
/// ```
/// # use approx::assert_abs_diff_eq;
/// # use palette::{convert::FromColorUnclamped,IsWithinBounds, LinSrgb, Okhsv, Oklab};
/// # use palette::Clamp;
/// // Display P3 yellow according to https://colorjs.io/apps/convert/?color=color(display-p3%201%201%200)&precision=17
/// let oklab = Oklab::from_color_unclamped(LinSrgb::new(1.0, 1.0, -0.098273600140966));
/// let okhsv: Okhsv<f64> = Okhsv::from_color_unclamped(oklab);
/// assert!(!okhsv.is_within_bounds());
/// let clamped_okhsv = okhsv.clamp();
/// assert!(clamped_okhsv.is_within_bounds());
/// let linsrgb = LinSrgb::from_color_unclamped(clamped_okhsv);
/// let  expected = LinSrgb::new(1.0, 0.9876530763223166, 0.0);
/// assert_abs_diff_eq!(expected, linsrgb, epsilon = 0.02);
/// ```
/// Since the conversion contains a gamut mapping, it will map the color to one
/// of the perceptually closest locations in the `sRGB` gamut. Gamut mapping --
/// unlike clamping -- is an expensive operation. To get computationally cheaper
/// (and perceptually much worse) results, convert directly to [`Srgb`] and
/// clamp there.
///
/// # Lightening / Darkening
/// [`Lighten`](crate::Lighten)ing and [`Darken`](crate::Darken)ing will change
/// `l`, as expected. However, either operation may leave an implicit color
/// space (the percetible or a display dependent color space like *sRGB*).
///
/// To ensure a color is within the *sRGB* gamut, first convert it to `Okhsl`,
/// lighten/darken it there and convert it back to `Oklab`.

#[derive(Debug, Copy, Clone, ArrayCast, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    white_point = "D65",
    component = "T",
    skip_derives(Oklab, Oklch, Okhsv, Okhsl, Xyz, Rgb)
)]
#[repr(C)]
pub struct Oklab<T = f32> {
    /// `l` is the lightness of the color. `0` gives absolute black and `1` gives the
    /// full white point luminance of the display medium.
    ///
    /// [`D65` (normalized with Y=1, i.e. white according to the adaption of the
    /// eye) transforms to
    /// L=1,a=0,b=0](https://bottosson.github.io/posts/oklab/#how-oklab-was-derived).
    /// However intermediate values differ from those of CIELab non-linearly.
    pub l: T,

    /// `a` changes the hue from reddish to greenish, when moving from positive
    /// to negative values and becomes more intense with larger absolute values.
    ///
    /// The exact orientation is determined by `b`
    pub a: T,

    /// `b` changes the hue from yellowish to blueish, when moving from positive
    /// to negative values and becomes more intense with larger absolute values.
    ///
    /// [Positive b is oriented to the same yellow color as
    /// CAM16](https://bottosson.github.io/posts/oklab/#how-oklab-was-derived)
    pub b: T,
}

impl<T> Oklab<T> {
    /// Create an Oklab color.
    pub const fn new(l: T, a: T, b: T) -> Self {
        Self { l, a, b }
    }

    /// Convert to a `(L, a, b)` tuple.
    pub fn into_components(self) -> (T, T, T) {
        (self.l, self.a, self.b)
    }

    /// Convert from a `(L, a, b)` tuple.
    pub fn from_components((l, a, b): (T, T, T)) -> Self {
        Self::new(l, a, b)
    }
}

// component bounds
// For `Oklab` in general a and b are unbounded.
// In the sRGB gamut `Oklab`s chroma (and thus a and b) are bounded.
impl<T> Oklab<T>
where
    T: Zero + One,
{
    /// Return the `l` value minimum.
    pub fn min_l() -> T {
        T::zero()
    }

    /// Return the `l` value maximum.
    pub fn max_l() -> T {
        T::one()
    }
}

impl_reference_component_methods!(Oklab, [l, a, b]);
impl_struct_of_arrays_methods!(Oklab, [l, a, b]);

impl<T> Oklab<T>
where
    T: Hypot + Clone,
{
    /// Returns the chroma.
    pub(crate) fn get_chroma(&self) -> T {
        T::hypot(self.a.clone(), self.b.clone())
    }
}

impl<T> FromColorUnclamped<Oklab<T>> for Oklab<T> {
    fn from_color_unclamped(color: Self) -> Self {
        color
    }
}

impl<T> FromColorUnclamped<Xyz<D65, T>> for Oklab<T>
where
    T: Real + Cbrt + Arithmetics,
{
    fn from_color_unclamped(color: Xyz<D65, T>) -> Self {
        let m1 = m1();
        let m2 = m2();

        let Xyz {
            x: l, y: m, z: s, ..
        } = multiply_xyz(m1, color.with_white_point());

        let l_m_s_ = Xyz::new(l.cbrt(), m.cbrt(), s.cbrt());

        let Xyz {
            x: l, y: a, z: b, ..
        } = multiply_xyz(m2, l_m_s_);

        Self::new(l, a, b)
    }
}

fn linear_srgb_to_oklab<T>(c: LinSrgb<T>) -> Oklab<T>
where
    T: Real + Arithmetics + Cbrt + Copy,
{
    let l = T::from_f64(0.4122214708) * c.red
        + T::from_f64(0.5363325363) * c.green
        + T::from_f64(0.0514459929) * c.blue;
    let m = T::from_f64(0.2119034982) * c.red
        + T::from_f64(0.6806995451) * c.green
        + T::from_f64(0.1073969566) * c.blue;
    let s = T::from_f64(0.0883024619) * c.red
        + T::from_f64(0.2817188376) * c.green
        + T::from_f64(0.6299787005) * c.blue;

    let l_ = l.cbrt();
    let m_ = m.cbrt();
    let s_ = s.cbrt();

    Oklab::new(
        T::from_f64(0.2104542553) * l_ + T::from_f64(0.7936177850) * m_
            - T::from_f64(0.0040720468) * s_,
        T::from_f64(1.9779984951) * l_ - T::from_f64(2.4285922050) * m_
            + T::from_f64(0.4505937099) * s_,
        T::from_f64(0.0259040371) * l_ + T::from_f64(0.7827717662) * m_
            - T::from_f64(0.8086757660) * s_,
    )
}

pub(crate) fn oklab_to_linear_srgb<T>(c: Oklab<T>) -> LinSrgb<T>
where
    T: Real + Arithmetics + Copy,
{
    let l_ = c.l + T::from_f64(0.3963377774) * c.a + T::from_f64(0.2158037573) * c.b;
    let m_ = c.l - T::from_f64(0.1055613458) * c.a - T::from_f64(0.0638541728) * c.b;
    let s_ = c.l - T::from_f64(0.0894841775) * c.a - T::from_f64(1.2914855480) * c.b;

    let l = l_ * l_ * l_;
    let m = m_ * m_ * m_;
    let s = s_ * s_ * s_;

    LinSrgb::new(
        T::from_f64(4.0767416621) * l - T::from_f64(3.3077115913) * m
            + T::from_f64(0.2309699292) * s,
        T::from_f64(-1.2684380046) * l + T::from_f64(2.6097574011) * m
            - T::from_f64(0.3413193965) * s,
        T::from_f64(-0.0041960863) * l - T::from_f64(0.7034186147) * m
            + T::from_f64(1.7076147010) * s,
    )
}

impl<S, T> FromColorUnclamped<Rgb<S, T>> for Oklab<T>
where
    T: Real + Cbrt + Arithmetics + Copy,
    S: RgbStandard,
    S::TransferFn: IntoLinear<T, T>,
    S::Space: RgbSpace<WhitePoint = D65> + 'static,
    Xyz<D65, T>: FromColorUnclamped<Rgb<S, T>>,
{
    fn from_color_unclamped(rgb: Rgb<S, T>) -> Self {
        if TypeId::of::<<S as RgbStandard>::Space>() == TypeId::of::<Srgb>() {
            // Use direct sRGB to Oklab conversion
            // Rounding errors are likely a contributing factor to differences.
            // Also the conversion via XYZ doesn't use pre-defined matrices (yet)
            linear_srgb_to_oklab(rgb.into_linear().reinterpret_as())
        } else {
            // Convert via XYZ
            Xyz::from_color_unclamped(rgb).into_color_unclamped()
        }
    }
}

impl<T> FromColorUnclamped<Oklch<T>> for Oklab<T>
where
    T: RealAngle + Zero + MinMax + Trigonometry + Mul<Output = T> + Clone,
{
    fn from_color_unclamped(color: Oklch<T>) -> Self {
        let (a, b) = color.hue.into_cartesian();
        let chroma = color.chroma.max(T::zero());

        Oklab {
            l: color.l,
            a: a * chroma.clone(),
            b: b * chroma,
        }
    }
}

/// # See
/// See [`okhsl_to_srgb`](https://bottosson.github.io/posts/colorpicker/#hsl-2)
impl<T> FromColorUnclamped<Okhsl<T>> for Oklab<T>
where
    T: RealAngle
        + One
        + Zero
        + Arithmetics
        + Sqrt
        + MinMax
        + PartialOrd
        + HasBoolMask<Mask = bool>
        + Powi
        + Cbrt
        + Trigonometry
        + Clone,
    Oklab<T>: IntoColorUnclamped<LinSrgb<T>>,
{
    fn from_color_unclamped(hsl: Okhsl<T>) -> Self {
        let h = hsl.hue;
        let s = hsl.saturation;
        let l = hsl.lightness;

        if l == T::one() {
            return Oklab::new(T::one(), T::zero(), T::zero());
        } else if l == T::zero() {
            return Oklab::new(T::zero(), T::zero(), T::zero());
        }

        let (a_, b_) = h.into_cartesian();
        let oklab_lightness = toe_inv(l);

        let cs = ChromaValues::from_normalized(oklab_lightness.clone(), a_.clone(), b_.clone());

        // Interpolate the three values for C so that:
        // At s=0: dC/ds = cs.zero, C = 0
        // At s=0.8: C = cs.mid
        // At s=1.0: C = cs.max

        let mid = T::from_f64(0.8);
        let mid_inv = T::from_f64(1.25);

        let chroma = if s < mid {
            let t = mid_inv * s;

            let k_1 = mid * cs.zero;
            let k_2 = T::one() - k_1.clone() / cs.mid;

            t.clone() * k_1 / (T::one() - k_2 * t)
        } else {
            let t = (s - &mid) / (T::one() - &mid);

            let k_0 = cs.mid.clone();
            let k_1 = (T::one() - mid) * &cs.mid * &cs.mid * &mid_inv * mid_inv / cs.zero;
            let k_2 = T::one() - k_1.clone() / (cs.max - cs.mid);

            k_0 + t.clone() * k_1 / (T::one() - k_2 * t)
        };

        Oklab::new(oklab_lightness, chroma.clone() * a_, chroma * b_)
    }
}

impl<T> FromColorUnclamped<Okhsv<T>> for Oklab<T>
where
    T: RealAngle
        + PartialOrd
        + HasBoolMask<Mask = bool>
        + MinMax
        + Powi
        + Arithmetics
        + Clone
        + One
        + Zero
        + Cbrt
        + Trigonometry,
    Oklab<T>: IntoColorUnclamped<LinSrgb<T>>,
{
    fn from_color_unclamped(hsv: Okhsv<T>) -> Self {
        if hsv.value == T::zero() {
            // pure black
            return Self {
                l: T::zero(),
                a: T::zero(),
                b: T::zero(),
            };
        }

        if hsv.saturation == T::zero() {
            // totally desaturated color -- the triangle is just the 0-chroma-line
            let l = toe_inv(hsv.value);
            return Self {
                l,
                a: T::zero(),
                b: T::zero(),
            };
        }

        let h_radians = hsv.hue.into_raw_radians();
        let a_ = T::cos(h_radians.clone());
        let b_ = T::sin(h_radians);

        let cusp = LC::find_cusp(a_.clone(), b_.clone());
        let cusp: ST<T> = cusp.into();
        let s_0 = T::from_f64(0.5);
        let k = T::one() - s_0.clone() / cusp.s;

        // first we compute L and V as if the gamut is a perfect triangle

        // L, C, when v == 1:
        let l_v = T::one()
            - hsv.saturation.clone() * s_0.clone()
                / (s_0.clone() + &cusp.t - cusp.t.clone() * &k * &hsv.saturation);
        let c_v =
            hsv.saturation.clone() * &cusp.t * &s_0 / (s_0 + &cusp.t - cusp.t * k * hsv.saturation);

        // then we compensate for both toe and the curved top part of the triangle:
        let l_vt = toe_inv(l_v.clone());
        let c_vt = c_v.clone() * &l_vt / &l_v;

        let mut lightness = hsv.value.clone() * l_v;
        let mut chroma = hsv.value * c_v;
        let lightness_new = toe_inv(lightness.clone());
        chroma = chroma * &lightness_new / lightness;
        // the values may be outside the normal range
        let rgb_scale: LinSrgb<T> =
            Oklab::new(l_vt, a_.clone() * &c_vt, b_.clone() * c_vt).into_color_unclamped();
        let lightness_scale_factor = T::cbrt(
            T::one()
                / T::max(
                    T::max(rgb_scale.red, rgb_scale.green),
                    T::max(rgb_scale.blue, T::zero()),
                ),
        );

        lightness = lightness_new * &lightness_scale_factor;
        chroma = chroma * lightness_scale_factor;

        Oklab::new(lightness, chroma.clone() * a_, chroma * b_)
    }
}

impl_tuple_conversion!(Oklab as (T, T, T));

impl<T> HasBoolMask for Oklab<T>
where
    T: HasBoolMask,
{
    type Mask = T::Mask;
}

impl<T> Default for Oklab<T>
where
    T: Zero,
{
    fn default() -> Self {
        Self::new(T::zero(), T::zero(), T::zero())
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Zeroable for Oklab<T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Pod for Oklab<T> where T: bytemuck::Pod {}

#[cfg(test)]
mod test {
    use crate::Oklab;

    test_convert_into_from_xyz!(Oklab);

    #[cfg(feature = "approx")]
    mod conversion {
        use core::str::FromStr;

        use crate::{
            convert::FromColorUnclamped, rgb::Rgb, visual::VisuallyEqual, white_point::D65,
            FromColor, Lab, LinSrgb, Oklab, Srgb,
        };

        /// Asserts that, for any color space, the lightness of pure white is converted to `l == 1.0`
        #[test]
        fn lightness_of_white_is_one() {
            let rgb: Srgb<f64> = Rgb::from_str("#ffffff").unwrap().into_format();
            let lin_rgb = LinSrgb::from_color_unclamped(rgb);
            let oklab = Oklab::from_color_unclamped(lin_rgb);
            println!("white {rgb:?} == {oklab:?}");
            assert_abs_diff_eq!(oklab.l, 1.0, epsilon = 1e-7);
            assert_abs_diff_eq!(oklab.a, 0.0, epsilon = 1e-7);
            assert_abs_diff_eq!(oklab.b, 0.0, epsilon = 1e-7);

            let lab: Lab<D65, f64> = Lab::from_components((100.0, 0.0, 0.0));
            let rgb: Srgb<f64> = Srgb::from_color_unclamped(lab);
            let oklab = Oklab::from_color_unclamped(lab);
            println!("white {lab:?} == {rgb:?} == {oklab:?}");
            assert_abs_diff_eq!(oklab.l, 1.0, epsilon = 1e-4);
            assert_abs_diff_eq!(oklab.a, 0.0, epsilon = 1e-4);
            assert_abs_diff_eq!(oklab.b, 0.0, epsilon = 1e-4);
        }

        #[test]
        fn blue_srgb() {
            // use f64 to be comparable to javascript
            let rgb: Srgb<f64> = Rgb::from_str("#0000ff").unwrap().into_format();
            let lin_rgb = LinSrgb::from_color_unclamped(rgb);
            let oklab = Oklab::from_color_unclamped(lin_rgb);

            // values from Ok Color Picker, which seems to use  Björn Ottosson's original
            // algorithm (from the direct srgb2oklab conversion, not via the XYZ color space)
            assert_abs_diff_eq!(oklab.l, 0.4520137183853429, epsilon = 1e-9);
            assert_abs_diff_eq!(oklab.a, -0.03245698416876397, epsilon = 1e-9);
            assert_abs_diff_eq!(oklab.b, -0.3115281476783751, epsilon = 1e-9);
        }

        #[test]
        fn red() {
            let a = Oklab::from_color(LinSrgb::new(1.0, 0.0, 0.0));
            // from https://github.com/bottosson/bottosson.github.io/blob/master/misc/ok_color.h
            let b = Oklab::new(0.6279553606145516, 0.22486306106597395, 0.1258462985307351);
            assert!(Oklab::visually_eq(a, b, 1e-8));
        }

        #[test]
        fn green() {
            let a = Oklab::from_color(LinSrgb::new(0.0, 1.0, 0.0));
            // from https://github.com/bottosson/bottosson.github.io/blob/master/misc/ok_color.h
            let b = Oklab::new(
                0.8664396115356694,
                -0.23388757418790812,
                0.17949847989672985,
            );
            assert!(Oklab::visually_eq(a, b, 1e-8));
        }

        #[test]
        fn blue() {
            let a = Oklab::from_color(LinSrgb::new(0.0, 0.0, 1.0));
            println!("Oklab blue: {:?}", a);
            // from https://github.com/bottosson/bottosson.github.io/blob/master/misc/ok_color.h
            let b = Oklab::new(0.4520137183853429, -0.0324569841687640, -0.3115281476783751);
            assert!(Oklab::visually_eq(a, b, 1e-8));
        }
    }

    #[cfg(feature = "approx")]
    mod visually_eq {
        use crate::{visual::VisuallyEqual, Oklab};

        #[test]
        fn black_eq_different_black() {
            assert!(Oklab::visually_eq(
                Oklab::new(0.0, 1.0, 0.0),
                Oklab::new(0.0, 0.0, 1.0),
                1e-8
            ));
        }

        #[test]
        fn white_eq_different_white() {
            assert!(Oklab::visually_eq(
                Oklab::new(1.0, 1.0, 0.0),
                Oklab::new(1.0, 0.0, 1.0),
                1e-8
            ));
        }

        #[test]
        fn white_ne_black() {
            assert!(!Oklab::visually_eq(
                Oklab::new(1.0, 1.0, 0.0),
                Oklab::new(0.0, 0.0, 1.0),
                1e-8
            ));
            assert!(!Oklab::visually_eq(
                Oklab::new(1.0, 1.0, 0.0),
                Oklab::new(0.0, 1.0, 0.0),
                1e-8
            ));
        }

        #[test]
        fn non_bw_neq_different_non_bw() {
            assert!(!Oklab::visually_eq(
                Oklab::new(0.3, 1.0, 0.0),
                Oklab::new(0.3, 0.0, 1.0),
                1e-8
            ));
        }
    }

    #[test]
    fn ranges() {
        assert_ranges! {
            Oklab<f64>;
            clamped {
                l: 0.0 => 1.0
                // a and b are unbounded --> not part of test
            }
            clamped_min {}
            unclamped {}
        };
    }

    #[test]
    fn check_min_max_components() {
        assert_eq!(Oklab::<f32>::min_l(), 0.0);
        assert_eq!(Oklab::<f32>::max_l(), 1.0);
    }

    struct_of_arrays_tests!(
        Oklab[l, a, b],
        super::Oklaba::new(0.1f32, 0.2, 0.3, 0.4),
        super::Oklaba::new(0.2, 0.3, 0.4, 0.5),
        super::Oklaba::new(0.3, 0.4, 0.5, 0.6)
    );

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Oklab::new(0.3, 0.8, 0.1)).unwrap();

        assert_eq!(serialized, r#"{"l":0.3,"a":0.8,"b":0.1}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Oklab = ::serde_json::from_str(r#"{"l":0.3,"a":0.8,"b":0.1}"#).unwrap();

        assert_eq!(deserialized, Oklab::new(0.3, 0.8, 0.1));
    }

    test_uniform_distribution! {
        Oklab {
            l: (0.0, 1.0),
            a: (-1.0, 1.0),
            b: (-1.0, 1.0)
        },
        min: Oklab::new(0.0, -1.0, -1.0),
        max: Oklab::new(1.0, 1.0, 1.0)
    }
}

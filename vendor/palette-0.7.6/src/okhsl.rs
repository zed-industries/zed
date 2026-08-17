//! Types for the Okhsl color space.

pub use alpha::Okhsla;

use crate::{
    angle::FromAngle,
    convert::{FromColorUnclamped, IntoColorUnclamped},
    num::{Arithmetics, Cbrt, Hypot, IsValidDivisor, MinMax, One, Powi, Real, Sqrt, Zero},
    ok_utils::{toe, ChromaValues},
    stimulus::{FromStimulus, Stimulus},
    white_point::D65,
    GetHue, HasBoolMask, LinSrgb, Oklab, OklabHue,
};

pub use self::properties::Iter;

#[cfg(feature = "random")]
pub use self::random::UniformOkhsl;

mod alpha;
mod properties;
#[cfg(feature = "random")]
mod random;
#[cfg(test)]
#[cfg(feature = "approx")]
mod visual_eq;

/// A Hue/Saturation/Lightness representation of [`Oklab`] in the `sRGB` color space.
///
/// Allows
/// * changing hue/chroma/saturation, while keeping perceived lightness constant (like HSLuv)
/// * changing lightness/chroma/saturation, while keeping perceived hue constant
/// * changing the perceived saturation (more or less) proportionally with the numerical
/// amount of change (unlike HSLuv)
#[derive(Debug, Copy, Clone, ArrayCast, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    white_point = "D65",
    component = "T",
    skip_derives(Oklab)
)]
#[repr(C)]
pub struct Okhsl<T = f32> {
    /// The hue of the color, in degrees of a circle.
    ///
    /// For fully saturated, bright colors
    /// * 0° corresponds to a kind of magenta-pink (RBG #ff0188),
    /// * 90° to a kind of yellow (RBG RGB #ffcb00)
    /// * 180° to a kind of cyan (RBG #00ffe1) and
    /// * 240° to a kind of blue (RBG #00aefe).
    ///
    /// For s == 0 or v == 0, the hue is irrelevant.
    #[palette(unsafe_same_layout_as = "T")]
    pub hue: OklabHue<T>,

    /// The saturation (freedom of black or white) of the color.
    ///
    /// * `0.0` corresponds to pure mixture of black and white without any color.
    /// The black to white relation depends on v.
    /// * `1.0` to a fully saturated color without any white.
    ///
    /// For v == 0 the saturation is irrelevant.
    pub saturation: T,

    /// The relative luminance of the color, where
    /// * `0.0` corresponds to pure black
    /// * `1.0` corresponds to white
    ///
    /// This luminance is visually similar to [Cielab](crate::Lab)'s luminance for a
    /// `D65` reference white point.
    ///
    /// `Okhsv`'s `value` component goes from black to non-black
    /// -- a maximally bright color in the `sRGB` gamut.
    ///
    /// `Okhsl`'s `lightness` component goes from black to white in the `sRGB` color space.
    pub lightness: T,
}

impl<T> Okhsl<T> {
    /// Create an Okhsl color.
    pub fn new<H: Into<OklabHue<T>>>(hue: H, saturation: T, lightness: T) -> Self {
        Self {
            hue: hue.into(),
            saturation,
            lightness,
        }
    }

    /// Create an `Okhsl` color. This is the same as `Okhsl::new` without the
    /// generic hue type. It's temporary until `const fn` supports traits.
    pub const fn new_const(hue: OklabHue<T>, saturation: T, lightness: T) -> Self {
        Self {
            hue,
            saturation,
            lightness,
        }
    }

    /// Convert into another component type.
    pub fn into_format<U>(self) -> Okhsl<U>
    where
        U: FromStimulus<T> + FromAngle<T>,
    {
        Okhsl {
            hue: self.hue.into_format(),
            saturation: U::from_stimulus(self.saturation),
            lightness: U::from_stimulus(self.lightness),
        }
    }

    /// Convert from another component type.
    pub fn from_format<U>(color: Okhsl<U>) -> Self
    where
        T: FromStimulus<U> + FromAngle<U>,
    {
        color.into_format()
    }

    /// Convert to a `(h, s, l)` tuple.
    pub fn into_components(self) -> (OklabHue<T>, T, T) {
        (self.hue, self.saturation, self.lightness)
    }

    /// Convert from a `(h, s, l)` tuple.
    pub fn from_components<H: Into<OklabHue<T>>>((hue, saturation, lightness): (H, T, T)) -> Self {
        Self::new(hue, saturation, lightness)
    }
}

impl<T> Okhsl<T>
where
    T: Stimulus,
{
    /// Return the `saturation` value minimum.
    pub fn min_saturation() -> T {
        T::zero()
    }

    /// Return the `saturation` value maximum.
    pub fn max_saturation() -> T {
        T::max_intensity()
    }

    /// Return the `lightness` value minimum.
    pub fn min_lightness() -> T {
        T::zero()
    }

    /// Return the `lightness` value maximum.
    pub fn max_lightness() -> T {
        T::max_intensity()
    }
}

impl_reference_component_methods_hue!(Okhsl, [saturation, lightness]);
impl_struct_of_arrays_methods_hue!(Okhsl, [saturation, lightness]);

/// # See
/// See [`srgb_to_okhsl`](https://bottosson.github.io/posts/colorpicker/#hsl-2)
impl<T> FromColorUnclamped<Oklab<T>> for Okhsl<T>
where
    T: Real
        + One
        + Zero
        + Arithmetics
        + Powi
        + Sqrt
        + Hypot
        + MinMax
        + Cbrt
        + IsValidDivisor<Mask = bool>
        + HasBoolMask<Mask = bool>
        + PartialOrd
        + Clone,
    Oklab<T>: GetHue<Hue = OklabHue<T>> + IntoColorUnclamped<LinSrgb<T>>,
{
    fn from_color_unclamped(lab: Oklab<T>) -> Self {
        // refer to the SRGB reference-white-based lightness L_r as l for consistency with HSL
        let l = toe(lab.l.clone());
        let chroma = lab.get_chroma();

        // Not part of the reference implementation. Added to prevent
        // https://github.com/Ogeon/palette/issues/368 and other cases of NaN.
        if !chroma.is_valid_divisor() || lab.l == T::one() || !lab.l.is_valid_divisor() {
            return Self::new(T::zero(), T::zero(), l);
        }

        let hue = lab.get_hue();
        let cs = ChromaValues::from_normalized(lab.l, lab.a / &chroma, lab.b / &chroma);

        // Inverse of the interpolation in okhsl_to_srgb:

        let mid = T::from_f64(0.8);
        let mid_inv = T::from_f64(1.25);

        let s = if chroma < cs.mid {
            let k_1 = mid.clone() * cs.zero;
            let k_2 = T::one() - k_1.clone() / cs.mid;

            let t = chroma.clone() / (k_1 + k_2 * chroma);
            t * mid
        } else {
            let k_0 = cs.mid.clone();
            let k_1 = (T::one() - &mid) * (cs.mid.clone() * mid_inv).powi(2) / cs.zero;
            let k_2 = T::one() - k_1.clone() / (cs.max - cs.mid);

            let t = (chroma.clone() - &k_0) / (k_1 + k_2 * (chroma - k_0));
            mid.clone() + (T::one() - mid) * t
        };

        Self::new(hue, s, l)
    }
}

impl<T> HasBoolMask for Okhsl<T>
where
    T: HasBoolMask,
{
    type Mask = T::Mask;
}

impl<T> Default for Okhsl<T>
where
    T: Stimulus,
    OklabHue<T>: Default,
{
    fn default() -> Okhsl<T> {
        Okhsl::new(
            OklabHue::default(),
            Self::min_saturation(),
            Self::min_lightness(),
        )
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Zeroable for Okhsl<T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Pod for Okhsl<T> where T: bytemuck::Pod {}

#[cfg(test)]
mod tests {
    use crate::{
        convert::{FromColorUnclamped, IntoColorUnclamped},
        encoding,
        rgb::Rgb,
        Okhsl, Oklab, Srgb,
    };

    test_convert_into_from_xyz!(Okhsl);

    #[cfg(feature = "approx")]
    mod conversion {
        use core::str::FromStr;

        use crate::{
            convert::FromColorUnclamped,
            visual::{VisualColor, VisuallyEqual},
            LinSrgb, Okhsl, Oklab, Srgb,
        };

        #[cfg_attr(miri, ignore)]
        #[test]
        fn test_roundtrip_okhsl_oklab_is_original() {
            let colors = [
                (
                    "red",
                    Oklab::from_color_unclamped(LinSrgb::new(1.0, 0.0, 0.0)),
                ),
                (
                    "green",
                    Oklab::from_color_unclamped(LinSrgb::new(0.0, 1.0, 0.0)),
                ),
                (
                    "cyan",
                    Oklab::from_color_unclamped(LinSrgb::new(0.0, 1.0, 1.0)),
                ),
                (
                    "magenta",
                    Oklab::from_color_unclamped(LinSrgb::new(1.0, 0.0, 1.0)),
                ),
                (
                    "black",
                    Oklab::from_color_unclamped(LinSrgb::new(0.0, 0.0, 0.0)),
                ),
                (
                    "grey",
                    Oklab::from_color_unclamped(LinSrgb::new(0.5, 0.5, 0.5)),
                ),
                (
                    "yellow",
                    Oklab::from_color_unclamped(LinSrgb::new(1.0, 1.0, 0.0)),
                ),
                (
                    "blue",
                    Oklab::from_color_unclamped(LinSrgb::new(0.0, 0.0, 1.0)),
                ),
                (
                    "white",
                    Oklab::from_color_unclamped(LinSrgb::new(1.0, 1.0, 1.0)),
                ),
            ];

            // unlike in okhwb we are using f64 here, which actually works.
            // So we can afford a small tolerance.
            // For some reason the roundtrip of Okhsl seems to produce a greater
            // divergence than the round trip of Okhsv (1e-8 vs 1e-10)
            const EPSILON: f64 = 1e-8;

            for (name, color) in colors {
                let rgb: Srgb<u8> = Srgb::<f64>::from_color_unclamped(color).into_format();
                println!(
                    "\n\
                    roundtrip of {} (#{:x} / {:?})\n\
                    =================================================",
                    name, rgb, color
                );

                println!("Color is white: {}", color.is_white(EPSILON));

                let okhsl = Okhsl::from_color_unclamped(color);
                println!("Okhsl: {:?}", okhsl);
                let roundtrip_color = Oklab::from_color_unclamped(okhsl);
                assert!(
                    Oklab::visually_eq(roundtrip_color, color, EPSILON),
                    "'{}' failed.\n{:?}\n!=\n{:?}",
                    name,
                    roundtrip_color,
                    color
                );
            }
        }

        #[test]
        fn test_blue() {
            let lab = Oklab::new(
                0.45201371519623734_f64,
                -0.03245697990291002,
                -0.3115281336419824,
            );
            let okhsl = Okhsl::<f64>::from_color_unclamped(lab);
            assert!(
                abs_diff_eq!(
                    okhsl.hue.into_raw_degrees(),
                    360.0 * 0.7334778365225699,
                    epsilon = 1e-10
                ),
                "{}\n!=\n{}",
                okhsl.hue.into_raw_degrees(),
                360.0 * 0.7334778365225699
            );
            assert!(
                abs_diff_eq!(okhsl.saturation, 0.9999999897262261, epsilon = 1e-8),
                "{}\n!=\n{}",
                okhsl.saturation,
                0.9999999897262261
            );
            assert!(
                abs_diff_eq!(okhsl.lightness, 0.366565335813274, epsilon = 1e-10),
                "{}\n!=\n{}",
                okhsl.lightness,
                0.366565335813274
            );
        }

        #[test]
        fn test_srgb_to_okhsl() {
            let red_hex = "#834941";
            let rgb: Srgb<f64> = Srgb::from_str(red_hex).unwrap().into_format();
            let lin_rgb = LinSrgb::<f64>::from_color_unclamped(rgb);
            let oklab = Oklab::from_color_unclamped(lin_rgb);
            println!(
                "RGB: {:?}\n\
            LinRgb: {:?}\n\
            Oklab: {:?}",
                rgb, lin_rgb, oklab
            );
            let okhsl = Okhsl::from_color_unclamped(oklab);

            // test data from Ok Color picker
            assert_relative_eq!(
                okhsl.hue.into_raw_degrees(),
                360.0 * 0.07992730371382328,
                epsilon = 1e-10,
                max_relative = 1e-13
            );
            assert_relative_eq!(okhsl.saturation, 0.4629217183454986, epsilon = 1e-10);
            assert_relative_eq!(okhsl.lightness, 0.3900998146147427, epsilon = 1e-10);
        }
    }

    #[test]
    fn test_okhsl_to_srgb() {
        let okhsl = Okhsl::new(0.0_f32, 0.5, 0.5);
        let rgb = Srgb::from_color_unclamped(okhsl);
        let rgb8: Rgb<encoding::Srgb, u8> = rgb.into_format();
        let hex_str = format!("{:x}", rgb8);
        assert_eq!(hex_str, "aa5a74");
    }

    #[test]
    fn test_okhsl_to_srgb_saturated_black() {
        let okhsl = Okhsl::new(0.0_f32, 1.0, 0.0);
        let rgb = Srgb::from_color_unclamped(okhsl);
        assert_eq!(rgb, Srgb::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_oklab_to_okhsl_saturated_white() {
        // Minimized check for the case in
        // https://github.com/Ogeon/palette/issues/368. It ended up resulting in
        // an Oklab value where a or b was larger than 0, which bypassed the
        // chroma check.
        let oklab = Oklab::new(1.0, 1.0, 0.0);
        let okhsl: Okhsl = oklab.into_color_unclamped();
        assert_eq!(okhsl, Okhsl::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_oklab_to_okhsl_saturated_black() {
        // Minimized check for the case in
        // https://github.com/Ogeon/palette/issues/368. This wasn't the reported
        // case, but another variant of it.
        let oklab = Oklab::new(0.0, 1.0, 0.0);
        let okhsl: Okhsl = oklab.into_color_unclamped();
        assert_eq!(okhsl, Okhsl::new(0.0, 0.0, 0.0));
    }

    struct_of_arrays_tests!(
        Okhsl[hue, saturation, lightness],
        super::Okhsla::new(0.1f32, 0.2, 0.3, 0.4),
        super::Okhsla::new(0.2, 0.3, 0.4, 0.5),
        super::Okhsla::new(0.3, 0.4, 0.5, 0.6)
    );
}

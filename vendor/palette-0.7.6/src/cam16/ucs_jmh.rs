use crate::{
    angle::RealAngle,
    bool_mask::HasBoolMask,
    color_difference::{DeltaE, ImprovedDeltaE},
    convert::{FromColorUnclamped, IntoColorUnclamped},
    hues::{Cam16Hue, Cam16HueIter},
    num::{Arithmetics, Hypot, Ln, One, Real, Trigonometry, Zero},
    Alpha,
};

use super::{Cam16Jmh, Cam16UcsJab};

/// Polar CAM16-UCS with an alpha component.
///
/// See the [`Cam16UcsJmha` implementation in
/// `Alpha`](crate::Alpha#Cam16UcsJmha).
pub type Cam16UcsJmha<T> = Alpha<Cam16UcsJmh<T>, T>;

/// The polar form of CAM16-UCS, or J'M'h'.
///
/// CAM16-UCS is a perceptually uniform color space, based on CAM16 lightness
/// and colorfulness. Its cartesian counterpart is [`Cam16UcsJab`].
///
/// # Creating a Value
///
/// ```
/// use palette::{
///     Srgb, FromColor, IntoColor, hues::Cam16Hue,
///     cam16::{Cam16, Parameters, Cam16UcsJmh},
/// };
///
/// let ucs = Cam16UcsJmh::new(50.0f32, 80.0, 120.0);
///
/// // There's also `new_const`:
/// const UCS: Cam16UcsJmh<f32> = Cam16UcsJmh::new_const(50.0, 80.0, Cam16Hue::new(120.0));
///
/// // Customize these according to the viewing conditions:
/// let mut example_parameters = Parameters::default_static_wp(40.0);
///
/// // CAM16-UCS from sRGB, or most other color spaces:
/// let rgb = Srgb::new(0.3f32, 0.8, 0.1);
/// let cam16 = Cam16::from_xyz(rgb.into_color(), example_parameters);
/// let ucs_from_rgb = Cam16UcsJmh::from_color(cam16);
///
/// // It's also possible to convert from (and to) arrays and tuples:
/// let ucs_from_array = Cam16UcsJmh::from([50.0f32, 80.0, 120.0]);
/// let ucs_from_tuple = Cam16UcsJmh::from((50.0f32, 80.0, 120.0));
/// ```
#[derive(Clone, Copy, Debug, Default, WithAlpha, ArrayCast, FromColorUnclamped)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    component = "T",
    skip_derives(Cam16Jmh, Cam16UcsJmh, Cam16UcsJab)
)]
#[repr(C)]
pub struct Cam16UcsJmh<T> {
    /// The lightness (J') of the color.
    ///
    /// It's derived from [`Cam16::lightness`][crate::cam16::Cam16::lightness]
    /// and ranges from `0.0` to `100.0`.
    pub lightness: T,

    /// The colorfulness (M') of the color.
    ///
    /// It's derived from [`Cam16::colorfulness`][crate::cam16::Cam16::colorfulness].
    pub colorfulness: T,

    /// The hue (h') of the color.
    ///
    /// It's the same as [`Cam16::hue`][crate::cam16::Cam16::hue], despite the
    /// h' notation.
    #[palette(unsafe_same_layout_as = "T")]
    pub hue: Cam16Hue<T>,
}

impl<T> Cam16UcsJmh<T> {
    /// Create a CAM16-UCS J' M' h' color.
    pub fn new<H: Into<Cam16Hue<T>>>(lightness: T, colorfulness: T, hue: H) -> Self {
        Self::new_const(lightness, colorfulness, hue.into())
    }

    /// Create a CAM16-UCS J' M' h' color. This is the same as
    /// `Cam16UcsJmh::new` without the generic hue type. It's temporary until
    /// `const fn` supports traits.
    pub const fn new_const(lightness: T, colorfulness: T, hue: Cam16Hue<T>) -> Self {
        Self {
            lightness,
            colorfulness,
            hue,
        }
    }

    /// Convert to a `(J', M', h')` tuple.
    pub fn into_components(self) -> (T, T, Cam16Hue<T>) {
        (self.lightness, self.colorfulness, self.hue)
    }

    /// Convert from a `(J', M', h')` tuple.
    pub fn from_components<H: Into<Cam16Hue<T>>>(
        (lightness, colorfulness, hue): (T, T, H),
    ) -> Self {
        Self::new(lightness, colorfulness, hue)
    }
}

impl<T> Cam16UcsJmh<T>
where
    T: Zero + Real,
{
    /// Return the `lightness` value minimum.
    pub fn min_lightness() -> T {
        T::zero()
    }

    /// Return the `lightness` value maximum.
    pub fn max_lightness() -> T {
        T::from_f64(100.0)
    }

    /// Return the `colorfulness` value minimum.
    pub fn min_colorfulness() -> T {
        T::zero()
    }

    /// Return a `colorfulness` value maximum that includes the sRGB gamut.
    ///
    /// <p class="warning">
    /// This is entirely arbitrary and only for use in `Lighten`, `Darken` and
    /// random generation. Colorfulness doesn't have a well defined upper
    /// bound.
    /// </p>
    pub fn max_srgb_colorfulness() -> T {
        // Based on a plot from https://facelessuser.github.io/coloraide/colors/cam16_ucs/
        T::from_f64(50.0)
    }
}

///<span id="Cam16UcsJmha"></span>[`Cam16UcsJmha`](crate::cam16::Cam16UcsJmha) implementations.
impl<T, A> Alpha<Cam16UcsJmh<T>, A> {
    /// Create a CAM16-UCS J' M' h' color with transparency.
    pub fn new<H: Into<Cam16Hue<T>>>(lightness: T, colorfulness: T, hue: H, alpha: A) -> Self {
        Self::new_const(lightness, colorfulness, hue.into(), alpha)
    }

    /// Create a CAM16-UCS J' M' h' color with transparency. This is the same as
    /// `Cam16UcsJmha::new` without the generic hue type. It's temporary until
    /// `const fn` supports traits.
    pub const fn new_const(lightness: T, colorfulness: T, hue: Cam16Hue<T>, alpha: A) -> Self {
        Self {
            color: Cam16UcsJmh::new_const(lightness, colorfulness, hue),
            alpha,
        }
    }

    /// Convert to a `(J', M', h', a)` tuple.
    pub fn into_components(self) -> (T, T, Cam16Hue<T>, A) {
        (
            self.color.lightness,
            self.color.colorfulness,
            self.color.hue,
            self.alpha,
        )
    }

    /// Convert from a `(J', M', h', a)` tuple.
    pub fn from_components<H: Into<Cam16Hue<T>>>(
        (lightness, colorfulness, hue, alpha): (T, T, H, A),
    ) -> Self {
        Self::new(lightness, colorfulness, hue, alpha)
    }
}

impl<T> FromColorUnclamped<Cam16UcsJmh<T>> for Cam16UcsJmh<T> {
    fn from_color_unclamped(val: Cam16UcsJmh<T>) -> Self {
        val
    }
}

impl<T> FromColorUnclamped<Cam16Jmh<T>> for Cam16UcsJmh<T>
where
    T: Real + One + Ln + Arithmetics,
{
    fn from_color_unclamped(val: Cam16Jmh<T>) -> Self {
        let colorfulness =
            (T::one() + T::from_f64(0.0228) * val.colorfulness).ln() / T::from_f64(0.0228);
        let lightness =
            T::from_f64(1.7) * &val.lightness / (T::one() + T::from_f64(0.007) * val.lightness);

        Cam16UcsJmh {
            lightness,
            colorfulness,
            hue: val.hue,
        }
    }
}

impl<T> FromColorUnclamped<Cam16UcsJab<T>> for Cam16UcsJmh<T>
where
    T: RealAngle + Hypot + Trigonometry + Arithmetics + Clone,
{
    fn from_color_unclamped(val: Cam16UcsJab<T>) -> Self {
        Self {
            lightness: val.lightness,
            colorfulness: val.a.clone().hypot(val.b.clone()),
            hue: Cam16Hue::from_cartesian(val.a, val.b),
        }
    }
}

impl<T> DeltaE for Cam16UcsJmh<T>
where
    Cam16UcsJab<T>: DeltaE<Scalar = T> + FromColorUnclamped<Self>,
{
    type Scalar = T;

    #[inline]
    fn delta_e(self, other: Self) -> Self::Scalar {
        // Jab and Jmh have the same delta E.
        Cam16UcsJab::from_color_unclamped(self).delta_e(other.into_color_unclamped())
    }
}

impl<T> ImprovedDeltaE for Cam16UcsJmh<T>
where
    Cam16UcsJab<T>: ImprovedDeltaE<Scalar = T> + FromColorUnclamped<Self>,
{
    #[inline]
    fn improved_delta_e(self, other: Self) -> Self::Scalar {
        // Jab and Jmh have the same delta E.
        Cam16UcsJab::from_color_unclamped(self).improved_delta_e(other.into_color_unclamped())
    }
}

impl<T> HasBoolMask for Cam16UcsJmh<T>
where
    T: HasBoolMask,
{
    type Mask = T::Mask;
}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Zeroable for Cam16UcsJmh<T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Pod for Cam16UcsJmh<T> where T: bytemuck::Pod {}

// Macro implementations

impl_reference_component_methods_hue!(Cam16UcsJmh, [lightness, colorfulness]);
impl_struct_of_arrays_methods_hue!(Cam16UcsJmh, [lightness, colorfulness]);
impl_tuple_conversion_hue!(Cam16UcsJmh as (T, T, H), Cam16Hue);

impl_is_within_bounds! {
    Cam16UcsJmh {
        lightness => [Self::min_lightness(), Self::max_lightness()],
        colorfulness => [Self::min_colorfulness(), None]
    }
    where T: Zero + Real
}
impl_clamp! {
    Cam16UcsJmh {
        lightness => [Self::min_lightness(), Self::max_lightness()],
        colorfulness => [Self::min_colorfulness()]
    }
    other {hue}
    where T: Zero + Real
}

impl_mix_hue!(Cam16UcsJmh {
    lightness,
    colorfulness
});
impl_lighten!(Cam16UcsJmh increase {lightness => [Self::min_lightness(), Self::max_lightness()]} other {hue, colorfulness});
impl_saturate!(Cam16UcsJmh increase {colorfulness => [Self::min_colorfulness(), Self::max_srgb_colorfulness()]} other {hue, lightness});
impl_hue_ops!(Cam16UcsJmh, Cam16Hue);

impl_color_add!(Cam16UcsJmh, [lightness, colorfulness, hue]);
impl_color_sub!(Cam16UcsJmh, [lightness, colorfulness, hue]);

impl_array_casts!(Cam16UcsJmh<T>, [T; 3]);
impl_simd_array_conversion_hue!(Cam16UcsJmh, [lightness, colorfulness]);
impl_struct_of_array_traits_hue!(Cam16UcsJmh, Cam16HueIter, [lightness, colorfulness]);

impl_eq_hue!(Cam16UcsJmh, Cam16Hue, [lightness, colorfulness, hue]);

impl_rand_traits_cylinder!(
    UniformCam16UcsJmh,
    Cam16UcsJmh {
        hue: UniformCam16Hue => Cam16Hue,
        height: lightness => [|l: T| l * Cam16UcsJmh::<T>::max_lightness()],
        radius: colorfulness => [|c| c *  Cam16UcsJmh::<T>::max_srgb_colorfulness()]
    }
    where T: Real + Zero + core::ops::Mul<Output = T>,
);

// Unit tests

#[cfg(test)]
mod test {
    use crate::{
        cam16::{Cam16Jmh, Cam16UcsJmh},
        convert::FromColorUnclamped,
    };

    #[cfg(feature = "approx")]
    use crate::color_difference::DeltaE;

    #[cfg(all(feature = "approx", feature = "alloc"))]
    use crate::{
        cam16::Cam16UcsJab, color_difference::ImprovedDeltaE, convert::IntoColorUnclamped,
    };

    #[test]
    fn ranges() {
        assert_ranges! {
            Cam16UcsJmh<f64>;
            clamped {
                lightness: 0.0 => 100.0
            }
            clamped_min {
                colorfulness: 0.0 => 200.0
            }
            unclamped {
                hue: -360.0 => 360.0
            }
        }
    }

    #[test]
    fn cam16_roundtrip() {
        let ucs = Cam16UcsJmh::new(50.0f64, 80.0, 120.0);
        let cam16 = Cam16Jmh::from_color_unclamped(ucs);
        assert_eq!(Cam16UcsJmh::from_color_unclamped(cam16), ucs);
    }

    raw_pixel_conversion_tests!(Cam16UcsJmh<>: lightness, colorfulness, hue);
    raw_pixel_conversion_fail_tests!(Cam16UcsJmh<>: lightness, colorfulness, hue);

    #[test]
    #[cfg(feature = "approx")]
    fn delta_e_large_hue_diff() {
        let lhs1 = Cam16UcsJmh::<f64>::new(50.0, 64.0, -730.0);
        let rhs1 = Cam16UcsJmh::new(50.0, 64.0, 730.0);

        let lhs2 = Cam16UcsJmh::<f64>::new(50.0, 64.0, -10.0);
        let rhs2 = Cam16UcsJmh::new(50.0, 64.0, 10.0);

        assert_relative_eq!(
            lhs1.delta_e(rhs1),
            lhs2.delta_e(rhs2),
            epsilon = 0.0000000000001
        );
    }

    // Jab and Jmh have the same delta E.
    #[test]
    #[cfg(all(feature = "approx", feature = "alloc"))]
    fn jab_delta_e_equality() {
        let mut jab_colors: Vec<Cam16UcsJab<f64>> = alloc::vec::Vec::new();

        for j_step in 0i8..5 {
            for a_step in -2i8..3 {
                for b_step in -2i8..3 {
                    jab_colors.push(Cam16UcsJab::new(
                        j_step as f64 * 25.0,
                        a_step as f64 * 60.0,
                        b_step as f64 * 60.0,
                    ))
                }
            }
        }

        let jmh_colors: alloc::vec::Vec<Cam16UcsJmh<_>> = jab_colors.clone().into_color_unclamped();

        for (&lhs_jab, &lhs_jmh) in jab_colors.iter().zip(&jmh_colors) {
            for (&rhs_jab, &rhs_jmh) in jab_colors.iter().zip(&jmh_colors) {
                let delta_e_jab = lhs_jab.delta_e(rhs_jab);
                let delta_e_jmh = lhs_jmh.delta_e(rhs_jmh);
                assert_relative_eq!(delta_e_jab, delta_e_jmh, epsilon = 0.0000000000001);
            }
        }
    }

    // Jab and Jmh have the same delta E, so should also have the same improved
    // delta E.
    #[test]
    #[cfg(all(feature = "approx", feature = "alloc"))]
    fn jab_improved_delta_e_equality() {
        let mut jab_colors: Vec<Cam16UcsJab<f64>> = alloc::vec::Vec::new();

        for j_step in 0i8..5 {
            for a_step in -2i8..3 {
                for b_step in -2i8..3 {
                    jab_colors.push(Cam16UcsJab::new(
                        j_step as f64 * 25.0,
                        a_step as f64 * 60.0,
                        b_step as f64 * 60.0,
                    ))
                }
            }
        }

        let jmh_colors: alloc::vec::Vec<Cam16UcsJmh<_>> = jab_colors.clone().into_color_unclamped();

        for (&lhs_jab, &lhs_jmh) in jab_colors.iter().zip(&jmh_colors) {
            for (&rhs_jab, &rhs_jmh) in jab_colors.iter().zip(&jmh_colors) {
                let delta_e_jab = lhs_jab.improved_delta_e(rhs_jab);
                let delta_e_jmh = lhs_jmh.improved_delta_e(rhs_jmh);
                assert_relative_eq!(delta_e_jab, delta_e_jmh, epsilon = 0.0000000000001);
            }
        }
    }

    struct_of_arrays_tests!(
        Cam16UcsJmh[lightness, colorfulness, hue],
        super::Cam16UcsJmha::new(0.1f32, 0.2, 0.3, 0.4),
        super::Cam16UcsJmha::new(0.2, 0.3, 0.4, 0.5),
        super::Cam16UcsJmha::new(0.3, 0.4, 0.5, 0.6)
    );

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Cam16UcsJmh::new(0.3, 0.8, 0.1)).unwrap();

        assert_eq!(
            serialized,
            r#"{"lightness":0.3,"colorfulness":0.8,"hue":0.1}"#
        );
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Cam16UcsJmh<f32> =
            ::serde_json::from_str(r#"{"lightness":0.3,"colorfulness":0.8,"hue":0.1}"#).unwrap();

        assert_eq!(deserialized, Cam16UcsJmh::new(0.3, 0.8, 0.1));
    }

    test_uniform_distribution! {
        Cam16UcsJmh<f32> as crate::cam16::Cam16UcsJab<f32> {
            lightness: (0.0, 100.0),
            a: (-30.0, 30.0),
            b: (-30.0, 30.0),
        },
        min: Cam16UcsJmh::new(0.0f32, 0.0, 0.0),
        max: Cam16UcsJmh::new(100.0, 50.0, 360.0)
    }
}

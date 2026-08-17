use core::ops::Mul;

use crate::{
    angle::RealAngle,
    bool_mask::HasBoolMask,
    color_difference::{DeltaE, EuclideanDistance, ImprovedDeltaE},
    convert::FromColorUnclamped,
    num::{MinMax, Powf, Real, Sqrt, Trigonometry, Zero},
    Alpha,
};

use super::Cam16UcsJmh;

/// Cartesian CAM16-UCS with an alpha component.
///
/// See the [`Cam16UcsJaba` implementation in
/// `Alpha`](crate::Alpha#Cam16UcsJaba).
pub type Cam16UcsJaba<T> = Alpha<Cam16UcsJab<T>, T>;

/// The Cartesian form of CAM16-UCS, or J' a' b'.
///
/// CAM16-UCS is a perceptually uniform color space, based on CAM16 lightness
/// and colorfulness. Its polar counterpart is [`Cam16UcsJmh`].
///
/// # Creating a Value
///
/// ```
/// use palette::{
///     Srgb, FromColor, IntoColor,
///     cam16::{Cam16, Parameters, Cam16UcsJab},
/// };
///
/// let ucs = Cam16UcsJab::new(50.0f32, 80.0, -30.0);
///
/// // `new` is also `const`:
/// const UCS: Cam16UcsJab<f32> = Cam16UcsJab::new(50.0, 80.0, -30.0);
///
/// // Customize these according to the viewing conditions:
/// let mut example_parameters = Parameters::default_static_wp(40.0);
///
/// // CAM16-UCS from sRGB, or most other color spaces:
/// let rgb = Srgb::new(0.3f32, 0.8, 0.1);
/// let cam16 = Cam16::from_xyz(rgb.into_color(), example_parameters);
/// let ucs_from_rgb = Cam16UcsJab::from_color(cam16);
///
/// // It's also possible to convert from (and to) arrays and tuples:
/// let ucs_from_array = Cam16UcsJab::from([50.0f32, 80.0, -30.0]);
/// let ucs_from_tuple = Cam16UcsJab::from((50.0f32, 80.0, -30.0));
/// ```
#[derive(Clone, Copy, Debug, Default, WithAlpha, ArrayCast, FromColorUnclamped)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    component = "T",
    skip_derives(Cam16UcsJmh, Cam16UcsJab)
)]
#[repr(C)]
pub struct Cam16UcsJab<T> {
    /// The lightness (J') of the color.
    ///
    /// It's derived from [`Cam16::lightness`][crate::cam16::Cam16::lightness]
    /// and ranges from `0.0` to `100.0`.
    pub lightness: T,

    /// The redness/greenness (a') of the color.
    ///
    /// It's derived from [`Cam16::hue`][crate::cam16::Cam16::hue] and
    /// [`Cam16::colorfulness`][crate::cam16::Cam16::colorfulness].
    pub a: T,

    /// The yellowness/blueness (b') of the color.
    ///
    /// It's derived from [`Cam16::hue`][crate::cam16::Cam16::hue] and
    /// [`Cam16::colorfulness`][crate::cam16::Cam16::colorfulness].
    pub b: T,
}

impl<T> Cam16UcsJab<T> {
    /// Create a CAM16-UCS J' a' b' color.
    pub const fn new(lightness: T, a: T, b: T) -> Self {
        Self { lightness, a, b }
    }

    /// Convert to a `(J', a', b')` tuple.
    pub fn into_components(self) -> (T, T, T) {
        (self.lightness, self.a, self.b)
    }

    /// Convert from a `(J', a', b')` tuple.
    pub fn from_components((lightness, a, b): (T, T, T)) -> Self {
        Self::new(lightness, a, b)
    }
}

impl<T> Cam16UcsJab<T>
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

    /// Return an `a` value minimum that includes the sRGB gamut.
    ///
    /// <p class="warning">
    /// This is entirely arbitrary and only for use in random generation.
    /// Colorfulness doesn't have a well defined upper bound, which makes
    /// a' unbounded.
    /// </p>
    pub fn min_srgb_a() -> T {
        // Based on a plot from https://facelessuser.github.io/coloraide/colors/cam16_ucs/
        T::from_f64(-50.0)
    }

    /// Return an `a` value maximum that includes the sRGB gamut.
    ///
    /// <p class="warning">
    /// This is entirely arbitrary and only for use in random generation.
    /// Colorfulness doesn't have a well defined upper bound, which makes
    /// a' unbounded.
    /// </p>
    pub fn max_srgb_a() -> T {
        // Based on a plot from https://facelessuser.github.io/coloraide/colors/cam16_ucs/
        T::from_f64(50.0)
    }

    /// Return a `b` value minimum that includes the sRGB gamut.
    ///
    /// <p class="warning">
    /// This is entirely arbitrary and only for use in random generation.
    /// Colorfulness doesn't have a well defined upper bound, which makes
    /// b' unbounded.
    /// </p>
    pub fn min_srgb_b() -> T {
        // Based on a plot from https://facelessuser.github.io/coloraide/colors/cam16_ucs/
        T::from_f64(-50.0)
    }

    /// Return a `b` value maximum that includes the sRGB gamut.
    ///
    /// <p class="warning">
    /// This is entirely arbitrary and only for use in random generation.
    /// Colorfulness doesn't have a well defined upper bound, which makes
    /// b' unbounded.
    /// </p>
    pub fn max_srgb_b() -> T {
        // Based on a plot from https://facelessuser.github.io/coloraide/colors/cam16_ucs/
        T::from_f64(50.0)
    }
}

///<span id="Cam16UcsJaba"></span>[`Cam16UcsJaba`](crate::cam16::Cam16UcsJaba) implementations.
impl<T, A> Alpha<Cam16UcsJab<T>, A> {
    /// Create a CAM16-UCS J' a' b' color with transparency.
    pub const fn new(lightness: T, a: T, b: T, alpha: A) -> Self {
        Self {
            color: Cam16UcsJab::new(lightness, a, b),
            alpha,
        }
    }

    /// Convert to a `(J', a', b', a)` tuple.
    pub fn into_components(self) -> (T, T, T, A) {
        (self.color.lightness, self.color.a, self.color.b, self.alpha)
    }

    /// Convert from a `(J', a', b', a)` tuple.
    pub fn from_components((lightness, a, b, alpha): (T, T, T, A)) -> Self {
        Self::new(lightness, a, b, alpha)
    }
}

impl<T> FromColorUnclamped<Cam16UcsJab<T>> for Cam16UcsJab<T> {
    fn from_color_unclamped(val: Cam16UcsJab<T>) -> Self {
        val
    }
}

impl<T> FromColorUnclamped<Cam16UcsJmh<T>> for Cam16UcsJab<T>
where
    T: RealAngle + Zero + Mul<Output = T> + Trigonometry + MinMax + Clone,
{
    fn from_color_unclamped(val: Cam16UcsJmh<T>) -> Self {
        let (a, b) = val.hue.into_cartesian();
        let colorfulness = val.colorfulness.max(T::zero());

        Self {
            lightness: val.lightness,
            a: a * colorfulness.clone(),
            b: b * colorfulness,
        }
    }
}

impl<T> DeltaE for Cam16UcsJab<T>
where
    Self: EuclideanDistance<Scalar = T>,
    T: Sqrt,
{
    type Scalar = T;

    #[inline]
    fn delta_e(self, other: Self) -> Self::Scalar {
        self.distance(other)
    }
}

impl<T> ImprovedDeltaE for Cam16UcsJab<T>
where
    Self: DeltaE<Scalar = T> + EuclideanDistance<Scalar = T>,
    T: Real + Mul<T, Output = T> + Powf,
{
    #[inline]
    fn improved_delta_e(self, other: Self) -> Self::Scalar {
        // Coefficients from "Power functions improving the performance of
        // color-difference formulas" by Huang et al.
        // https://opg.optica.org/oe/fulltext.cfm?uri=oe-23-1-597&id=307643
        //
        // The multiplication of 0.5 in the exponent makes it square root the
        // squared distance.
        T::from_f64(1.41) * self.distance_squared(other).powf(T::from_f64(0.63 * 0.5))
    }
}

impl<T> HasBoolMask for Cam16UcsJab<T>
where
    T: HasBoolMask,
{
    type Mask = T::Mask;
}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Zeroable for Cam16UcsJab<T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Pod for Cam16UcsJab<T> where T: bytemuck::Pod {}

// Macro implementations

impl_reference_component_methods!(Cam16UcsJab, [lightness, a, b]);
impl_struct_of_arrays_methods!(Cam16UcsJab, [lightness, a, b]);

impl_tuple_conversion!(Cam16UcsJab as (T, T, T));

impl_is_within_bounds! {
    Cam16UcsJab {
        lightness => [Self::min_lightness(), Self::max_lightness()]
    }
    where T: Real + Zero
}
impl_clamp! {
    Cam16UcsJab {
        lightness => [Self::min_lightness(), Self::max_lightness()]
    }
    other {a, b}
    where T: Real + Zero
}

impl_mix!(Cam16UcsJab);
impl_lighten!(Cam16UcsJab increase {lightness => [Self::min_lightness(), Self::max_lightness()]} other {a, b});
impl_premultiply!(Cam16UcsJab { lightness, a, b });
impl_euclidean_distance!(Cam16UcsJab { lightness, a, b });
impl_hyab!(Cam16UcsJab {
    lightness: lightness,
    chroma1: a,
    chroma2: b
});
impl_lab_color_schemes!(Cam16UcsJab[lightness]);

impl_color_add!(Cam16UcsJab, [lightness, a, b]);
impl_color_sub!(Cam16UcsJab, [lightness, a, b]);
impl_color_mul!(Cam16UcsJab, [lightness, a, b]);
impl_color_div!(Cam16UcsJab, [lightness, a, b]);

impl_array_casts!(Cam16UcsJab<T>, [T; 3]);
impl_simd_array_conversion!(Cam16UcsJab, [lightness, a, b]);
impl_struct_of_array_traits!(Cam16UcsJab, [lightness, a, b]);

impl_eq!(Cam16UcsJab, [lightness, a, b]);

impl_rand_traits_cartesian!(
    UniformCam16UcsJab,
    Cam16UcsJab {
        lightness => [|x| x * Cam16UcsJab::<T>::max_lightness()],
        a => [|x| Cam16UcsJab::<T>::min_srgb_a() + x * (Cam16UcsJab::<T>::max_srgb_a() - Cam16UcsJab::<T>::min_srgb_a())],
        b => [|x| Cam16UcsJab::<T>::min_srgb_b() + x * (Cam16UcsJab::<T>::max_srgb_b() - Cam16UcsJab::<T>::min_srgb_b())]
    }
    where T: Real + Zero + core::ops::Add<Output = T> + core::ops::Sub<Output = T> + core::ops::Mul<Output = T>
);

// Unit test

#[cfg(test)]
mod test {
    #[cfg(feature = "approx")]
    use crate::{cam16::Cam16Jmh, convert::FromColorUnclamped};

    use super::Cam16UcsJab;

    #[test]
    fn ranges() {
        assert_ranges! {
            Cam16UcsJab<f64>;
            clamped {
                lightness: 0.0 => 100.0
            }
            clamped_min {}
            unclamped {
                a: -100.0 => 100.0,
                b: -100.0 => 100.0
            }
        }
    }

    #[cfg(feature = "approx")]
    #[test]
    fn cam16_roundtrip() {
        let ucs = Cam16UcsJab::new(50.0f64, 80.0, -30.0);
        let cam16 = Cam16Jmh::from_color_unclamped(ucs);
        assert_relative_eq!(
            Cam16UcsJab::from_color_unclamped(cam16),
            ucs,
            epsilon = 0.0000000000001
        );
    }

    raw_pixel_conversion_tests!(Cam16UcsJab<>: lightness, a, b);
    raw_pixel_conversion_fail_tests!(Cam16UcsJab<>: lightness, a, b);

    struct_of_arrays_tests!(
        Cam16UcsJab[lightness, a, b],
        super::Cam16UcsJaba::new(0.1f32, 0.2, 0.3, 0.4),
        super::Cam16UcsJaba::new(0.2, 0.3, 0.4, 0.5),
        super::Cam16UcsJaba::new(0.3, 0.4, 0.5, 0.6)
    );

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Cam16UcsJab::<f32>::new(0.3, 0.8, 0.1)).unwrap();

        assert_eq!(serialized, r#"{"lightness":0.3,"a":0.8,"b":0.1}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Cam16UcsJab<f32> =
            ::serde_json::from_str(r#"{"lightness":0.3,"a":0.8,"b":0.1}"#).unwrap();

        assert_eq!(deserialized, Cam16UcsJab::new(0.3, 0.8, 0.1));
    }

    test_uniform_distribution! {
        Cam16UcsJab<f32> {
            lightness: (0.0, 100.0),
            a: (-50.0, 50.0),
            b: (-50.0, 50.0)
        },
        min: Cam16UcsJab::new(0.0f32, -50.0, -50.0),
        max: Cam16UcsJab::new(100.0, 50.0, 50.0)
    }
}

//! Types for the CIE 1931 XYZ color space.

use core::{marker::PhantomData, ops::Mul};

use crate::{
    angle::{RealAngle, SignedAngle},
    bool_mask::{HasBoolMask, LazySelect},
    cam16::{
        Cam16, Cam16IntoUnclamped, Cam16Jch, Cam16Jmh, Cam16Jsh, Cam16Qch, Cam16Qmh, Cam16Qsh,
        FromCam16Unclamped, WhitePointParameter,
    },
    convert::{FromColorUnclamped, IntoColorUnclamped},
    encoding::IntoLinear,
    luma::LumaStandard,
    matrix::{matrix_map, multiply_rgb_to_xyz, multiply_xyz, rgb_to_xyz_matrix},
    num::{
        Abs, Arithmetics, FromScalar, IsValidDivisor, One, PartialCmp, Powf, Powi, Real, Recip,
        Signum, Sqrt, Trigonometry, Zero,
    },
    oklab,
    rgb::{Primaries, Rgb, RgbSpace, RgbStandard},
    stimulus::{Stimulus, StimulusColor},
    white_point::{Any, WhitePoint, D65},
    Alpha, Lab, Luma, Luv, Oklab, Yxy,
};

/// CIE 1931 XYZ with an alpha component. See the [`Xyza` implementation in
/// `Alpha`](crate::Alpha#Xyza).
pub type Xyza<Wp = D65, T = f32> = Alpha<Xyz<Wp, T>, T>;

/// The CIE 1931 XYZ color space.
///
/// XYZ links the perceived colors to their wavelengths and simply makes it
/// possible to describe the way we see colors as numbers. It's often used when
/// converting from one color space to an other, and requires a standard
/// illuminant and a standard observer to be defined.
///
/// Conversions and operations on this color space depend on the defined white
/// point
#[derive(Debug, ArrayCast, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    white_point = "Wp",
    component = "T",
    skip_derives(Xyz, Yxy, Luv, Rgb, Lab, Oklab, Luma)
)]
#[repr(C)]
pub struct Xyz<Wp = D65, T = f32> {
    /// X is the scale of what can be seen as a response curve for the cone
    /// cells in the human eye. Its range depends
    /// on the white point and goes from 0.0 to 0.95047 for the default D65.
    pub x: T,

    /// Y is the luminance of the color, where 0.0 is black and 1.0 is white.
    pub y: T,

    /// Z is the scale of what can be seen as the blue stimulation. Its range
    /// depends on the white point and goes from 0.0 to 1.08883 for the
    /// default D65.
    pub z: T,

    /// The white point associated with the color's illuminant and observer.
    /// D65 for 2 degree observer is used by default.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette(unsafe_zero_sized)]
    pub white_point: PhantomData<Wp>,
}

impl<Wp, T> Xyz<Wp, T> {
    /// Create a CIE XYZ color.
    pub const fn new(x: T, y: T, z: T) -> Xyz<Wp, T> {
        Xyz {
            x,
            y,
            z,
            white_point: PhantomData,
        }
    }

    /// Convert to a `(X, Y, Z)` tuple.
    pub fn into_components(self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }

    /// Convert from a `(X, Y, Z)` tuple.
    pub fn from_components((x, y, z): (T, T, T)) -> Self {
        Self::new(x, y, z)
    }

    /// Changes the reference white point without changing the color value.
    ///
    /// This function doesn't change the numerical values, and thus the color it
    /// represents in an absolute sense. However, the appearance of the color
    /// may not be the same when observed with the new white point. The effect
    /// would be similar to taking a photo with an incorrect white balance.
    ///
    /// See [chromatic_adaptation](crate::chromatic_adaptation) for operations
    /// that can change the white point while preserving the color's appearance.
    #[inline]
    pub fn with_white_point<NewWp>(self) -> Xyz<NewWp, T> {
        Xyz::new(self.x, self.y, self.z)
    }
}

impl<Wp, T> Xyz<Wp, T>
where
    T: Zero,
    Wp: WhitePoint<T>,
{
    /// Return the `x` value minimum.
    pub fn min_x() -> T {
        T::zero()
    }

    /// Return the `x` value maximum.
    pub fn max_x() -> T {
        Wp::get_xyz().x
    }

    /// Return the `y` value minimum.
    pub fn min_y() -> T {
        T::zero()
    }

    /// Return the `y` value maximum.
    pub fn max_y() -> T {
        Wp::get_xyz().y
    }

    /// Return the `z` value minimum.
    pub fn min_z() -> T {
        T::zero()
    }

    /// Return the `z` value maximum.
    pub fn max_z() -> T {
        Wp::get_xyz().z
    }
}

///<span id="Xyza"></span>[`Xyza`](crate::Xyza) implementations.
impl<Wp, T, A> Alpha<Xyz<Wp, T>, A> {
    /// Create a CIE XYZ color with transparency.
    pub const fn new(x: T, y: T, z: T, alpha: A) -> Self {
        Alpha {
            color: Xyz::new(x, y, z),
            alpha,
        }
    }

    /// Convert to a `(X, Y, Z, alpha)` tuple.
    pub fn into_components(self) -> (T, T, T, A) {
        (self.color.x, self.color.y, self.color.z, self.alpha)
    }

    /// Convert from a `(X, Y, Z, alpha)` tuple.
    pub fn from_components((x, y, z, alpha): (T, T, T, A)) -> Self {
        Self::new(x, y, z, alpha)
    }

    /// Changes the reference white point without changing the color value.
    ///
    /// This function doesn't change the numerical values, and thus the color it
    /// represents in an absolute sense. However, the appearance of the color
    /// may not be the same when observed with the new white point. The effect
    /// would be similar to taking a photo with an incorrect white balance.
    ///
    /// See [chromatic_adaptation](crate::chromatic_adaptation) for operations
    /// that can change the white point while preserving the color's appearance.
    #[inline]
    pub fn with_white_point<NewWp>(self) -> Alpha<Xyz<NewWp, T>, A> {
        Alpha::<Xyz<NewWp, T>, A>::new(self.color.x, self.color.y, self.color.z, self.alpha)
    }
}

impl_reference_component_methods!(Xyz<Wp>, [x, y, z], white_point);
impl_struct_of_arrays_methods!(Xyz<Wp>, [x, y, z], white_point);

impl<Wp, T> FromColorUnclamped<Xyz<Wp, T>> for Xyz<Wp, T> {
    fn from_color_unclamped(color: Xyz<Wp, T>) -> Self {
        color
    }
}

impl<Wp, T, S> FromColorUnclamped<Rgb<S, T>> for Xyz<Wp, T>
where
    T: Arithmetics + FromScalar,
    T::Scalar: Real
        + Recip
        + IsValidDivisor<Mask = bool>
        + Arithmetics
        + FromScalar<Scalar = T::Scalar>
        + Clone,
    Wp: WhitePoint<T::Scalar>,
    S: RgbStandard,
    S::TransferFn: IntoLinear<T, T>,
    S::Space: RgbSpace<WhitePoint = Wp>,
    <S::Space as RgbSpace>::Primaries: Primaries<T::Scalar>,
    Yxy<Any, T::Scalar>: IntoColorUnclamped<Xyz<Any, T::Scalar>>,
{
    fn from_color_unclamped(color: Rgb<S, T>) -> Self {
        let transform_matrix = S::Space::rgb_to_xyz_matrix()
            .map_or_else(rgb_to_xyz_matrix::<S::Space, T::Scalar>, |matrix| {
                matrix_map(matrix, T::Scalar::from_f64)
            });
        multiply_rgb_to_xyz(transform_matrix, color.into_linear())
    }
}

impl<Wp, T> FromColorUnclamped<Yxy<Wp, T>> for Xyz<Wp, T>
where
    T: Zero + One + IsValidDivisor + Arithmetics + Clone,
    T::Mask: LazySelect<T> + Clone,
{
    fn from_color_unclamped(color: Yxy<Wp, T>) -> Self {
        let Yxy { x, y, luma, .. } = color;

        // If denominator is zero, NAN or INFINITE leave x and z at the default 0
        let mask = y.is_valid_divisor();
        let xyz = Xyz {
            z: lazy_select! {
                if mask.clone() => (T::one() - &x - &y) / &y,
                else => T::zero(),
            },
            x: lazy_select! {
                if mask => x / y,
                else => T::zero(),
            },
            y: T::one(),
            white_point: PhantomData,
        };

        xyz * luma
    }
}

impl<Wp, T> FromColorUnclamped<Lab<Wp, T>> for Xyz<Wp, T>
where
    T: Real + Recip + Powi + Arithmetics + PartialCmp + Clone,
    T::Mask: LazySelect<T>,
    Wp: WhitePoint<T>,
{
    fn from_color_unclamped(color: Lab<Wp, T>) -> Self {
        // Recip call shows performance benefits in benchmarks for this function
        let y = (color.l + T::from_f64(16.0)) * T::from_f64(116.0).recip();
        let x = y.clone() + (color.a * T::from_f64(500.0).recip());
        let z = y.clone() - (color.b * T::from_f64(200.0).recip());

        let epsilon: T = T::from_f64(6.0 / 29.0);
        let kappa: T = T::from_f64(108.0 / 841.0);
        let delta: T = T::from_f64(4.0 / 29.0);

        let convert = |c: T| {
            lazy_select! {
                if c.gt(&epsilon) => c.clone().powi(3),
                else => (c.clone() - &delta) * &kappa
            }
        };

        Xyz::new(convert(x), convert(y), convert(z)) * Wp::get_xyz().with_white_point()
    }
}

impl<Wp, T> FromColorUnclamped<Luv<Wp, T>> for Xyz<Wp, T>
where
    T: Real + Zero + Recip + Powi + Arithmetics + PartialOrd + Clone + HasBoolMask<Mask = bool>,
    Wp: WhitePoint<T>,
{
    fn from_color_unclamped(color: Luv<Wp, T>) -> Self {
        let kappa = T::from_f64(29.0 / 3.0).powi(3);

        let w = Wp::get_xyz();
        let ref_denom_recip =
            (w.x.clone() + T::from_f64(15.0) * &w.y + T::from_f64(3.0) * w.z).recip();
        let u_ref = T::from_f64(4.0) * w.x * &ref_denom_recip;
        let v_ref = T::from_f64(9.0) * &w.y * ref_denom_recip;

        if color.l < T::from_f64(1e-5) {
            return Xyz::new(T::zero(), T::zero(), T::zero());
        }

        let y = if color.l > T::from_f64(8.0) {
            ((color.l.clone() + T::from_f64(16.0)) * T::from_f64(116.0).recip()).powi(3)
        } else {
            color.l.clone() * kappa.recip()
        } * w.y;

        let u_prime = color.u / (T::from_f64(13.0) * &color.l) + u_ref;
        let v_prime = color.v / (T::from_f64(13.0) * color.l) + v_ref;

        let x = y.clone() * T::from_f64(2.25) * &u_prime / &v_prime;
        let z = y.clone()
            * (T::from_f64(3.0) - T::from_f64(0.75) * u_prime - T::from_f64(5.0) * &v_prime)
            / v_prime;
        Xyz::new(x, y, z)
    }
}

impl<T> FromColorUnclamped<Oklab<T>> for Xyz<D65, T>
where
    T: Real + Powi + Arithmetics,
{
    fn from_color_unclamped(color: Oklab<T>) -> Self {
        let m1_inv = oklab::m1_inv();
        let m2_inv = oklab::m2_inv();

        let Xyz {
            x: l, y: m, z: s, ..
        } = multiply_xyz(m2_inv, Xyz::new(color.l, color.a, color.b));

        let lms = Xyz::new(l.powi(3), m.powi(3), s.powi(3));
        multiply_xyz(m1_inv, lms).with_white_point()
    }
}

impl<Wp, T, S> FromColorUnclamped<Luma<S, T>> for Xyz<Wp, T>
where
    Self: Mul<T, Output = Self>,
    Wp: WhitePoint<T>,
    S: LumaStandard<WhitePoint = Wp>,
    S::TransferFn: IntoLinear<T, T>,
{
    fn from_color_unclamped(color: Luma<S, T>) -> Self {
        Wp::get_xyz().with_white_point::<Wp>() * color.into_linear().luma
    }
}

impl<WpParam, T> FromCam16Unclamped<WpParam, Cam16<T>> for Xyz<WpParam::StaticWp, T>
where
    WpParam: WhitePointParameter<T>,
    T: FromScalar,
    Cam16Jch<T>: Cam16IntoUnclamped<WpParam, Self, Scalar = T::Scalar>,
{
    type Scalar = T::Scalar;

    fn from_cam16_unclamped(
        cam16: Cam16<T>,
        parameters: crate::cam16::BakedParameters<WpParam, Self::Scalar>,
    ) -> Self {
        Cam16Jch::from(cam16).cam16_into_unclamped(parameters)
    }
}

macro_rules! impl_from_cam16_partial {
    ($($name: ident),+) => {
        $(
            impl<WpParam, T> FromCam16Unclamped<WpParam, $name<T>> for Xyz<WpParam::StaticWp, T>
            where
                WpParam: WhitePointParameter<T>,
                T: Real
                    + FromScalar
                    + One
                    + Zero
                    + Sqrt
                    + Powf
                    + Abs
                    + Signum
                    + Arithmetics
                    + Trigonometry
                    + RealAngle
                    + SignedAngle
                    + PartialCmp
                    + Clone,
                T::Mask: LazySelect<T> + Clone,
                T::Scalar: Clone,
            {
                type Scalar = T::Scalar;

                fn from_cam16_unclamped(
                    cam16: $name<T>,
                    parameters: crate::cam16::BakedParameters<WpParam, Self::Scalar>,
                ) -> Self {
                    crate::cam16::math::cam16_to_xyz(cam16.into_dynamic(), parameters.inner)
                        .with_white_point()
                }
            }
        )+
    };
}

impl_from_cam16_partial!(Cam16Jmh, Cam16Jch, Cam16Jsh, Cam16Qmh, Cam16Qch, Cam16Qsh);

impl_tuple_conversion!(Xyz<Wp> as (T, T, T));

impl_is_within_bounds! {
    Xyz<Wp> {
        x => [Self::min_x(), Self::max_x()],
        y => [Self::min_y(), Self::max_y()],
        z => [Self::min_z(), Self::max_z()]
    }
    where
        T: Zero,
        Wp: WhitePoint<T>
}
impl_clamp! {
    Xyz<Wp> {
        x => [Self::min_x(), Self::max_x()],
        y => [Self::min_y(), Self::max_y()],
        z => [Self::min_z(), Self::max_z()]
    }
    other {white_point}
    where
        T: Zero,
        Wp: WhitePoint<T>
}

impl_mix!(Xyz<Wp>);
impl_lighten! {
    Xyz<Wp>
    increase {
        x => [Self::min_x(), Self::max_x()],
        y => [Self::min_y(), Self::max_y()],
        z => [Self::min_z(), Self::max_z()]
    }
    other {}
    phantom: white_point
    where Wp: WhitePoint<T>
}
impl_premultiply!(Xyz<Wp> {x, y, z} phantom: white_point);
impl_euclidean_distance!(Xyz<Wp> {x, y, z});

impl<Wp, T> StimulusColor for Xyz<Wp, T> where T: Stimulus {}

impl<Wp, T> HasBoolMask for Xyz<Wp, T>
where
    T: HasBoolMask,
{
    type Mask = T::Mask;
}

impl<Wp, T> Default for Xyz<Wp, T>
where
    T: Zero,
{
    fn default() -> Xyz<Wp, T> {
        Xyz::new(T::zero(), T::zero(), T::zero())
    }
}

impl_color_add!(Xyz<Wp>, [x, y, z], white_point);
impl_color_sub!(Xyz<Wp>, [x, y, z], white_point);
impl_color_mul!(Xyz<Wp>, [x, y, z], white_point);
impl_color_div!(Xyz<Wp>, [x, y, z], white_point);

impl_array_casts!(Xyz<Wp, T>, [T; 3]);
impl_simd_array_conversion!(Xyz<Wp>, [x, y, z], white_point);
impl_struct_of_array_traits!(Xyz<Wp>, [x, y, z], white_point);

impl_copy_clone!(Xyz<Wp>, [x, y, z], white_point);
impl_eq!(Xyz<Wp>, [x, y, z]);

#[allow(deprecated)]
impl<Wp, T> crate::RelativeContrast for Xyz<Wp, T>
where
    T: Real + Arithmetics + PartialCmp,
    T::Mask: LazySelect<T>,
{
    type Scalar = T;

    #[inline]
    fn get_contrast_ratio(self, other: Self) -> T {
        crate::contrast_ratio(self.y, other.y)
    }
}

impl_rand_traits_cartesian!(
    UniformXyz,
    Xyz<Wp> {
        x => [|x| x * Wp::get_xyz().x],
        y => [|y| y * Wp::get_xyz().y],
        z => [|z| z * Wp::get_xyz().z]
    }
    phantom: white_point: PhantomData<Wp>
    where T: core::ops::Mul<Output = T>, Wp: WhitePoint<T>
);

#[cfg(feature = "bytemuck")]
unsafe impl<Wp, T> bytemuck::Zeroable for Xyz<Wp, T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<Wp: 'static, T> bytemuck::Pod for Xyz<Wp, T> where T: bytemuck::Pod {}

#[cfg(test)]
mod test {
    use super::Xyz;
    use crate::white_point::D65;

    #[cfg(feature = "random")]
    use crate::white_point::WhitePoint;

    const X_N: f64 = 0.95047;
    const Y_N: f64 = 1.0;
    const Z_N: f64 = 1.08883;

    test_convert_into_from_xyz!(Xyz);

    #[cfg(feature = "approx")]
    mod conversion {
        use crate::{white_point::D65, FromColor, LinLuma, LinSrgb, Xyz};

        #[test]
        fn luma() {
            let a = Xyz::<D65>::from_color(LinLuma::new(0.5));
            let b = Xyz::new(0.475235, 0.5, 0.544415);
            assert_relative_eq!(a, b, epsilon = 0.0001);
        }

        #[test]
        fn red() {
            let a = Xyz::from_color(LinSrgb::new(1.0, 0.0, 0.0));
            let b = Xyz::new(0.41240, 0.21260, 0.01930);
            assert_relative_eq!(a, b, epsilon = 0.0001);
        }

        #[test]
        fn green() {
            let a = Xyz::from_color(LinSrgb::new(0.0, 1.0, 0.0));
            let b = Xyz::new(0.35760, 0.71520, 0.11920);
            assert_relative_eq!(a, b, epsilon = 0.0001);
        }

        #[test]
        fn blue() {
            let a = Xyz::from_color(LinSrgb::new(0.0, 0.0, 1.0));
            let b = Xyz::new(0.18050, 0.07220, 0.95030);
            assert_relative_eq!(a, b, epsilon = 0.0001);
        }
    }

    #[test]
    fn ranges() {
        assert_ranges! {
            Xyz<D65, f64>;
            clamped {
                x: 0.0 => X_N,
                y: 0.0 => Y_N,
                z: 0.0 => Z_N
            }
            clamped_min {}
            unclamped {}
        }
    }

    raw_pixel_conversion_tests!(Xyz<D65>: x, y, z);
    raw_pixel_conversion_fail_tests!(Xyz<D65>: x, y, z);

    #[test]
    fn check_min_max_components() {
        assert_eq!(Xyz::<D65>::min_x(), 0.0);
        assert_eq!(Xyz::<D65>::min_y(), 0.0);
        assert_eq!(Xyz::<D65>::min_z(), 0.0);
        assert_eq!(Xyz::<D65, f64>::max_x(), X_N);
        assert_eq!(Xyz::<D65, f64>::max_y(), Y_N);
        assert_eq!(Xyz::<D65, f64>::max_z(), Z_N);
    }

    struct_of_arrays_tests!(
        Xyz<D65>[x, y, z] phantom: white_point,
        super::Xyza::new(0.1f32, 0.2, 0.3, 0.4),
        super::Xyza::new(0.2, 0.3, 0.4, 0.5),
        super::Xyza::new(0.3, 0.4, 0.5, 0.6)
    );

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Xyz::<D65>::new(0.3, 0.8, 0.1)).unwrap();

        assert_eq!(serialized, r#"{"x":0.3,"y":0.8,"z":0.1}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Xyz = ::serde_json::from_str(r#"{"x":0.3,"y":0.8,"z":0.1}"#).unwrap();

        assert_eq!(deserialized, Xyz::new(0.3, 0.8, 0.1));
    }

    test_uniform_distribution! {
        Xyz<D65, f32> {
            x: (0.0, D65::get_xyz().x),
            y: (0.0, D65::get_xyz().y),
            z: (0.0, D65::get_xyz().z)
        },
        min: Xyz::new(0.0f32, 0.0, 0.0),
        max: D65::get_xyz().with_white_point()
    }
}

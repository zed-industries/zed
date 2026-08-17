//! Types for the CIE L\*a\*b\* (CIELAB) color space.

use core::{
    marker::PhantomData,
    ops::{Add, BitAnd, BitOr, Mul, Neg},
};

use crate::{
    angle::RealAngle,
    bool_mask::{HasBoolMask, LazySelect},
    color_difference::{
        get_ciede2000_difference, Ciede2000, DeltaE, EuclideanDistance, ImprovedDeltaE,
        LabColorDiff,
    },
    convert::FromColorUnclamped,
    num::{
        Abs, Arithmetics, Cbrt, Exp, Hypot, MinMax, One, PartialCmp, Powf, Powi, Real, Sqrt,
        Trigonometry, Zero,
    },
    white_point::{WhitePoint, D65},
    Alpha, FromColor, GetHue, LabHue, Lch, Xyz,
};

/// CIE L\*a\*b\* (CIELAB) with an alpha component. See the [`Laba`
/// implementation in `Alpha`](crate::Alpha#Laba).
pub type Laba<Wp = D65, T = f32> = Alpha<Lab<Wp, T>, T>;

/// The CIE L\*a\*b\* (CIELAB) color space.
///
/// CIE L\*a\*b\* is a device independent color space which includes all
/// perceivable colors. It's sometimes used to convert between other color
/// spaces, because of its ability to represent all of their colors, and
/// sometimes in color manipulation, because of its perceptual uniformity. This
/// means that the perceptual difference between two colors is equal to their
/// numerical difference. It was, however, [never designed for the perceptual
/// qualities required for gamut mapping](http://www.brucelindbloom.com/UPLab.html).
/// For perceptually uniform color manipulation the newer color spaces based on
/// [`Oklab`](crate::Oklab) are preferable:
/// [`Oklch`](crate::Oklch), [`Okhsv`](crate::Okhsv), [`Okhsl`](crate::Okhsl),
/// [`Okhwb`](crate::Okhwb) (Note that the latter three are tied to the sRGB gamut
/// and reference white).
///
/// The parameters of L\*a\*b\* are quite different, compared to many other
/// color spaces, so manipulating them manually may be unintuitive.
#[derive(Debug, ArrayCast, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    white_point = "Wp",
    component = "T",
    skip_derives(Xyz, Lab, Lch)
)]
#[repr(C)]
pub struct Lab<Wp = D65, T = f32> {
    /// L\* is the lightness of the color. 0.0 gives absolute black and 100
    /// give the brightest white.
    pub l: T,

    /// a\* goes from red at -128 to green at 127.
    pub a: T,

    /// b\* goes from yellow at -128 to blue at 127.
    pub b: T,

    /// The white point associated with the color's illuminant and observer.
    /// D65 for 2 degree observer is used by default.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette(unsafe_zero_sized)]
    pub white_point: PhantomData<Wp>,
}

impl<Wp, T> Lab<Wp, T> {
    /// Create a CIE L\*a\*b\* color.
    pub const fn new(l: T, a: T, b: T) -> Lab<Wp, T> {
        Lab {
            l,
            a,
            b,
            white_point: PhantomData,
        }
    }

    /// Convert to a `(L\*, a\*, b\*)` tuple.
    pub fn into_components(self) -> (T, T, T) {
        (self.l, self.a, self.b)
    }

    /// Convert from a `(L\*, a\*, b\*)` tuple.
    pub fn from_components((l, a, b): (T, T, T)) -> Self {
        Self::new(l, a, b)
    }
}

impl<Wp, T> Lab<Wp, T>
where
    T: Zero + Real,
{
    /// Return the `l` value minimum.
    pub fn min_l() -> T {
        T::zero()
    }

    /// Return the `l` value maximum.
    pub fn max_l() -> T {
        T::from_f64(100.0)
    }

    /// Return the `a` value minimum.
    pub fn min_a() -> T {
        T::from_f64(-128.0)
    }

    /// Return the `a` value maximum.
    pub fn max_a() -> T {
        T::from_f64(127.0)
    }

    /// Return the `b` value minimum.
    pub fn min_b() -> T {
        T::from_f64(-128.0)
    }

    /// Return the `b` value maximum.
    pub fn max_b() -> T {
        T::from_f64(127.0)
    }
}

///<span id="Laba"></span>[`Laba`](crate::Laba) implementations.
impl<Wp, T, A> Alpha<Lab<Wp, T>, A> {
    /// Create a CIE L\*a\*b\* with transparency.
    pub const fn new(l: T, a: T, b: T, alpha: A) -> Self {
        Alpha {
            color: Lab::new(l, a, b),
            alpha,
        }
    }

    /// Convert to a `(L\*, a\*, b\*, alpha)` tuple.
    pub fn into_components(self) -> (T, T, T, A) {
        (self.color.l, self.color.a, self.color.b, self.alpha)
    }

    /// Convert from a `(L\*, a\*, b\*, alpha)` tuple.
    pub fn from_components((l, a, b, alpha): (T, T, T, A)) -> Self {
        Self::new(l, a, b, alpha)
    }
}

impl_reference_component_methods!(Lab<Wp>, [l, a, b], white_point);
impl_struct_of_arrays_methods!(Lab<Wp>, [l, a, b], white_point);

impl<Wp, T> FromColorUnclamped<Lab<Wp, T>> for Lab<Wp, T> {
    fn from_color_unclamped(color: Lab<Wp, T>) -> Self {
        color
    }
}

impl<Wp, T> FromColorUnclamped<Xyz<Wp, T>> for Lab<Wp, T>
where
    Wp: WhitePoint<T>,
    T: Real + Powi + Cbrt + Arithmetics + PartialCmp + Clone,
    T::Mask: LazySelect<T>,
{
    fn from_color_unclamped(color: Xyz<Wp, T>) -> Self {
        let Xyz { x, y, z, .. } = color / Wp::get_xyz().with_white_point();

        let epsilon = T::from_f64(6.0 / 29.0).powi(3);
        let kappa: T = T::from_f64(841.0 / 108.0);
        let delta: T = T::from_f64(4.0 / 29.0);

        let convert = |c: T| {
            lazy_select! {
                if c.gt(&epsilon) => c.clone().cbrt(),
                else => (kappa.clone() * &c) + &delta,
            }
        };

        let x = convert(x);
        let y = convert(y);
        let z = convert(z);

        Lab {
            l: ((y.clone() * T::from_f64(116.0)) - T::from_f64(16.0)),
            a: ((x - &y) * T::from_f64(500.0)),
            b: ((y - z) * T::from_f64(200.0)),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> FromColorUnclamped<Lch<Wp, T>> for Lab<Wp, T>
where
    T: RealAngle + Zero + MinMax + Trigonometry + Mul<Output = T> + Clone,
{
    fn from_color_unclamped(color: Lch<Wp, T>) -> Self {
        let (a, b) = color.hue.into_cartesian();
        let chroma = color.chroma.max(T::zero());

        Lab {
            l: color.l,
            a: a * chroma.clone(),
            b: b * chroma,
            white_point: PhantomData,
        }
    }
}

impl_tuple_conversion!(Lab<Wp> as (T, T, T));

impl_is_within_bounds! {
    Lab<Wp> {
        l => [Self::min_l(), Self::max_l()],
        a => [Self::min_a(), Self::max_a()],
        b => [Self::min_b(), Self::max_b()]
    }
    where T: Real + Zero
}
impl_clamp! {
    Lab<Wp> {
        l => [Self::min_l(), Self::max_l()],
        a => [Self::min_a(), Self::max_a()],
        b => [Self::min_b(), Self::max_b()]
    }
    other {white_point}
    where T: Real + Zero
}

impl_mix!(Lab<Wp>);
impl_lighten!(Lab<Wp> increase {l => [Self::min_l(), Self::max_l()]} other {a, b} phantom: white_point);
impl_premultiply!(Lab<Wp> {l, a, b} phantom: white_point);
impl_euclidean_distance!(Lab<Wp> {l, a, b});
impl_hyab!(Lab<Wp> {lightness: l, chroma1: a, chroma2: b});
impl_lab_color_schemes!(Lab<Wp>[l, white_point]);

impl<Wp, T> GetHue for Lab<Wp, T>
where
    T: RealAngle + Trigonometry + Add<T, Output = T> + Neg<Output = T> + Clone,
{
    type Hue = LabHue<T>;

    fn get_hue(&self) -> LabHue<T> {
        LabHue::from_cartesian(self.a.clone(), self.b.clone())
    }
}

impl<Wp, T> DeltaE for Lab<Wp, T>
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

impl<Wp, T> ImprovedDeltaE for Lab<Wp, T>
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
        T::from_f64(1.26) * self.distance_squared(other).powf(T::from_f64(0.55 * 0.5))
    }
}

#[allow(deprecated)]
impl<Wp, T> crate::ColorDifference for Lab<Wp, T>
where
    T: Real
        + RealAngle
        + One
        + Zero
        + Powi
        + Exp
        + Trigonometry
        + Abs
        + Sqrt
        + Arithmetics
        + PartialCmp
        + Clone,
    T::Mask: LazySelect<T> + BitAnd<Output = T::Mask> + BitOr<Output = T::Mask>,
    Self: Into<LabColorDiff<T>>,
{
    type Scalar = T;

    #[inline]
    fn get_color_difference(self, other: Lab<Wp, T>) -> Self::Scalar {
        get_ciede2000_difference(self.into(), other.into())
    }
}

impl<Wp, T> Ciede2000 for Lab<Wp, T>
where
    T: Real
        + RealAngle
        + One
        + Zero
        + Powi
        + Exp
        + Trigonometry
        + Abs
        + Sqrt
        + Arithmetics
        + PartialCmp
        + Hypot
        + Clone,
    T::Mask: LazySelect<T> + BitAnd<Output = T::Mask> + BitOr<Output = T::Mask>,
{
    type Scalar = T;

    #[inline]
    fn difference(self, other: Self) -> Self::Scalar {
        get_ciede2000_difference(self.into(), other.into())
    }
}

impl<Wp, T> HasBoolMask for Lab<Wp, T>
where
    T: HasBoolMask,
{
    type Mask = T::Mask;
}

impl<Wp, T> Default for Lab<Wp, T>
where
    T: Zero,
{
    fn default() -> Lab<Wp, T> {
        Lab::new(T::zero(), T::zero(), T::zero())
    }
}

impl_color_add!(Lab<Wp>, [l, a, b], white_point);
impl_color_sub!(Lab<Wp>, [l, a, b], white_point);
impl_color_mul!(Lab<Wp>, [l, a, b], white_point);
impl_color_div!(Lab<Wp>, [l, a, b], white_point);

impl_array_casts!(Lab<Wp, T>, [T; 3]);
impl_simd_array_conversion!(Lab<Wp>, [l, a, b], white_point);
impl_struct_of_array_traits!(Lab<Wp>, [l, a, b], white_point);

impl_eq!(Lab<Wp>, [l, a, b]);
impl_copy_clone!(Lab<Wp>, [l, a, b], white_point);

#[allow(deprecated)]
impl<Wp, T> crate::RelativeContrast for Lab<Wp, T>
where
    T: Real + Arithmetics + PartialCmp,
    T::Mask: LazySelect<T>,
    Xyz<Wp, T>: FromColor<Self>,
{
    type Scalar = T;

    #[inline]
    fn get_contrast_ratio(self, other: Self) -> T {
        let xyz1 = Xyz::from_color(self);
        let xyz2 = Xyz::from_color(other);

        crate::contrast_ratio(xyz1.y, xyz2.y)
    }
}

impl_rand_traits_cartesian!(
    UniformLab,
    Lab<Wp> {
        l => [|x| x * T::from_f64(100.0)],
        a => [|x| x * T::from_f64(255.0) - T::from_f64(128.0)],
        b => [|x| x * T::from_f64(255.0) - T::from_f64(128.0)]
    }
    phantom: white_point: PhantomData<Wp>
    where T: Real + core::ops::Sub<Output = T> + core::ops::Mul<Output = T>
);

#[cfg(feature = "bytemuck")]
unsafe impl<Wp, T> bytemuck::Zeroable for Lab<Wp, T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<Wp: 'static, T> bytemuck::Pod for Lab<Wp, T> where T: bytemuck::Pod {}

#[cfg(test)]
mod test {
    use super::Lab;
    use crate::white_point::D65;

    #[cfg(feature = "approx")]
    use crate::Lch;

    test_convert_into_from_xyz!(Lab);

    #[cfg(feature = "approx")]
    mod conversion {
        use crate::{FromColor, Lab, LinSrgb};

        #[test]
        fn red() {
            let a = Lab::from_color(LinSrgb::new(1.0, 0.0, 0.0));
            let b = Lab::new(53.23288, 80.09246, 67.2031);
            assert_relative_eq!(a, b, epsilon = 0.01);
        }

        #[test]
        fn green() {
            let a = Lab::from_color(LinSrgb::new(0.0, 1.0, 0.0));
            let b = Lab::new(87.73704, -86.184654, 83.18117);
            assert_relative_eq!(a, b, epsilon = 0.01);
        }

        #[test]
        fn blue() {
            let a = Lab::from_color(LinSrgb::new(0.0, 0.0, 1.0));
            let b = Lab::new(32.302586, 79.19668, -107.863686);
            assert_relative_eq!(a, b, epsilon = 0.01);
        }
    }

    #[test]
    fn ranges() {
        assert_ranges! {
            Lab<D65, f64>;
            clamped {
                l: 0.0 => 100.0,
                a: -128.0 => 127.0,
                b: -128.0 => 127.0
            }
            clamped_min {}
            unclamped {}
        }
    }

    raw_pixel_conversion_tests!(Lab<D65>: l, a, b);
    raw_pixel_conversion_fail_tests!(Lab<D65>: l, a, b);

    #[test]
    fn check_min_max_components() {
        assert_eq!(Lab::<D65, f32>::min_l(), 0.0);
        assert_eq!(Lab::<D65, f32>::min_a(), -128.0);
        assert_eq!(Lab::<D65, f32>::min_b(), -128.0);
        assert_eq!(Lab::<D65, f32>::max_l(), 100.0);
        assert_eq!(Lab::<D65, f32>::max_a(), 127.0);
        assert_eq!(Lab::<D65, f32>::max_b(), 127.0);
    }

    struct_of_arrays_tests!(
        Lab<D65>[l, a, b] phantom: white_point,
        super::Laba::new(0.1f32, 0.2, 0.3, 0.4),
        super::Laba::new(0.2, 0.3, 0.4, 0.5),
        super::Laba::new(0.3, 0.4, 0.5, 0.6)
    );

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Lab::<D65>::new(0.3, 0.8, 0.1)).unwrap();

        assert_eq!(serialized, r#"{"l":0.3,"a":0.8,"b":0.1}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Lab = ::serde_json::from_str(r#"{"l":0.3,"a":0.8,"b":0.1}"#).unwrap();

        assert_eq!(deserialized, Lab::new(0.3, 0.8, 0.1));
    }

    test_uniform_distribution! {
        Lab<D65, f32> {
            l: (0.0, 100.0),
            a: (-128.0, 127.0),
            b: (-128.0, 127.0)
        },
        min: Lab::new(0.0f32, -128.0, -128.0),
        max: Lab::new(100.0, 127.0, 127.0)
    }

    test_lab_color_schemes!(Lab/Lch [l, white_point]);
}

//! Types for the CIE 1931 Yxy (xyY) color space.

use core::marker::PhantomData;

use crate::{
    bool_mask::{HasBoolMask, LazySelect},
    convert::{FromColorUnclamped, IntoColorUnclamped},
    encoding::IntoLinear,
    luma::LumaStandard,
    num::{Arithmetics, IsValidDivisor, One, PartialCmp, Real, Zero},
    white_point::{WhitePoint, D65},
    Alpha, Luma, Xyz,
};

/// CIE 1931 Yxy (xyY) with an alpha component. See the [`Yxya` implementation
/// in `Alpha`](crate::Alpha#Yxya).
pub type Yxya<Wp = D65, T = f32> = Alpha<Yxy<Wp, T>, T>;

/// The CIE 1931 Yxy (xyY) color space.
///
/// Yxy is a luminance-chromaticity color space derived from the CIE XYZ
/// color space. It is widely used to define colors. The chromaticity diagrams
/// for the color spaces are a plot of this color space's x and y coordinates.
///
/// Conversions and operations on this color space depend on the white point.
#[derive(Debug, ArrayCast, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    white_point = "Wp",
    component = "T",
    skip_derives(Xyz, Yxy, Luma)
)]
#[repr(C)]
#[doc(alias = "xyY")]
pub struct Yxy<Wp = D65, T = f32> {
    /// x chromaticity co-ordinate derived from XYZ color space as X/(X+Y+Z).
    /// Typical range is between 0 and 1
    pub x: T,

    /// y chromaticity co-ordinate derived from XYZ color space as Y/(X+Y+Z).
    /// Typical range is between 0 and 1
    pub y: T,

    /// luma (Y) was a measure of the brightness or luminance of a color.
    /// It is the same as the Y from the XYZ color space. Its range is from
    ///0 to 1, where 0 is black and 1 is white.
    pub luma: T,

    /// The white point associated with the color's illuminant and observer.
    /// D65 for 2 degree observer is used by default.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette(unsafe_zero_sized)]
    pub white_point: PhantomData<Wp>,
}

impl<Wp, T> Yxy<Wp, T> {
    /// Create a CIE Yxy color.
    pub const fn new(x: T, y: T, luma: T) -> Yxy<Wp, T> {
        Yxy {
            x,
            y,
            luma,
            white_point: PhantomData,
        }
    }

    /// Convert to a `(x, y, luma)`, a.k.a. `(x, y, Y)` tuple.
    pub fn into_components(self) -> (T, T, T) {
        (self.x, self.y, self.luma)
    }

    /// Convert from a `(x, y, luma)`, a.k.a. `(x, y, Y)` tuple.
    pub fn from_components((x, y, luma): (T, T, T)) -> Self {
        Self::new(x, y, luma)
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
    pub fn with_white_point<NewWp>(self) -> Yxy<NewWp, T> {
        Yxy::new(self.x, self.y, self.luma)
    }
}

impl<Wp, T> Yxy<Wp, T>
where
    T: Zero + One,
{
    /// Return the `x` value minimum.
    pub fn min_x() -> T {
        T::zero()
    }

    /// Return the `x` value maximum.
    pub fn max_x() -> T {
        T::one()
    }

    /// Return the `y` value minimum.
    pub fn min_y() -> T {
        T::zero()
    }

    /// Return the `y` value maximum.
    pub fn max_y() -> T {
        T::one()
    }

    /// Return the `luma` value minimum.
    pub fn min_luma() -> T {
        T::zero()
    }

    /// Return the `luma` value maximum.
    pub fn max_luma() -> T {
        T::one()
    }
}

///<span id="Yxya"></span>[`Yxya`](crate::Yxya) implementations.
impl<Wp, T, A> Alpha<Yxy<Wp, T>, A> {
    /// Create a CIE Yxy color with transparency.
    pub const fn new(x: T, y: T, luma: T, alpha: A) -> Self {
        Alpha {
            color: Yxy::new(x, y, luma),
            alpha,
        }
    }

    /// Convert to a `(x, y, luma)`, a.k.a. `(x, y, Y)` tuple.
    pub fn into_components(self) -> (T, T, T, A) {
        (self.color.x, self.color.y, self.color.luma, self.alpha)
    }

    /// Convert from a `(x, y, luma)`, a.k.a. `(x, y, Y)` tuple.
    pub fn from_components((x, y, luma, alpha): (T, T, T, A)) -> Self {
        Self::new(x, y, luma, alpha)
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
    pub fn with_white_point<NewWp>(self) -> Alpha<Yxy<NewWp, T>, A> {
        Alpha::<Yxy<NewWp, T>, A>::new(self.color.x, self.color.y, self.color.luma, self.alpha)
    }
}

impl_reference_component_methods!(Yxy<Wp>, [x, y, luma], white_point);
impl_struct_of_arrays_methods!(Yxy<Wp>, [x, y, luma], white_point);

impl_tuple_conversion!(Yxy<Wp> as (T, T, T));

impl<Wp, T> FromColorUnclamped<Yxy<Wp, T>> for Yxy<Wp, T> {
    fn from_color_unclamped(color: Yxy<Wp, T>) -> Self {
        color
    }
}

impl<Wp, T> FromColorUnclamped<Xyz<Wp, T>> for Yxy<Wp, T>
where
    T: Zero + IsValidDivisor + Arithmetics + Clone,
    T::Mask: LazySelect<T> + Clone,
{
    fn from_color_unclamped(xyz: Xyz<Wp, T>) -> Self {
        let Xyz { x, y, z, .. } = xyz;

        let sum = x.clone() + &y + z;

        // If denominator is zero, NAN or INFINITE leave x and y at the default 0
        let mask = sum.is_valid_divisor();
        Yxy {
            x: lazy_select! {
                if mask.clone() => x / &sum,
                else => T::zero(),
            },
            y: lazy_select! {
                if mask => y.clone() / sum,
                else => T::zero()
            },
            luma: y,
            white_point: PhantomData,
        }
    }
}

impl<T, S> FromColorUnclamped<Luma<S, T>> for Yxy<S::WhitePoint, T>
where
    S: LumaStandard,
    S::TransferFn: IntoLinear<T, T>,
    Self: Default,
{
    fn from_color_unclamped(luma: Luma<S, T>) -> Self {
        Yxy {
            luma: luma.into_linear().luma,
            ..Default::default()
        }
    }
}

impl_is_within_bounds! {
    Yxy<Wp> {
        x => [Self::min_x(), Self::max_x()],
        y => [Self::min_y(), Self::max_y()],
        luma => [Self::min_luma(), Self::max_luma()]
    }
    where T: Zero + One
}
impl_clamp! {
    Yxy<Wp> {
        x => [Self::min_x(), Self::max_x()],
        y => [Self::min_y(), Self::max_y()],
        luma => [Self::min_luma(), Self::max_luma()]
    }
    other {white_point}
    where T: Zero + One
}

impl_mix!(Yxy<Wp>);
impl_lighten!(Yxy<Wp> increase {luma => [Self::min_luma(), Self::max_luma()]} other {x, y} phantom: white_point where T: One);
impl_premultiply!(Yxy<Wp> {x, y, luma} phantom: white_point);
impl_euclidean_distance!(Yxy<Wp> {x, y, luma});

impl<Wp, T> HasBoolMask for Yxy<Wp, T>
where
    T: HasBoolMask,
{
    type Mask = T::Mask;
}

impl<Wp, T> Default for Yxy<Wp, T>
where
    T: Zero,
    Wp: WhitePoint<T>,
    Xyz<Wp, T>: IntoColorUnclamped<Self>,
{
    fn default() -> Yxy<Wp, T> {
        // The default for x and y are the white point x and y ( from the default D65).
        // Since Y (luma) is 0.0, this makes the default color black just like for
        // other colors. The reason for not using 0 for x and y is that this
        // outside the usual color gamut and might cause scaling issues.
        Yxy {
            luma: T::zero(),
            ..Wp::get_xyz().with_white_point().into_color_unclamped()
        }
    }
}

impl_color_add!(Yxy<Wp>, [x, y, luma], white_point);
impl_color_sub!(Yxy<Wp>, [x, y, luma], white_point);
impl_color_mul!(Yxy<Wp>, [x, y, luma], white_point);
impl_color_div!(Yxy<Wp>, [x, y, luma], white_point);

impl_array_casts!(Yxy<Wp, T>, [T; 3]);
impl_simd_array_conversion!(Yxy<Wp>, [x, y, luma], white_point);
impl_struct_of_array_traits!(Yxy<Wp>, [x, y, luma], white_point);

impl_eq!(Yxy<Wp>, [x, y, luma]);
impl_copy_clone!(Yxy<Wp>, [x, y, luma], white_point);

#[allow(deprecated)]
impl<Wp, T> crate::RelativeContrast for Yxy<Wp, T>
where
    T: Real + Arithmetics + PartialCmp,
    T::Mask: LazySelect<T>,
{
    type Scalar = T;

    #[inline]
    fn get_contrast_ratio(self, other: Self) -> T {
        crate::contrast_ratio(self.luma, other.luma)
    }
}

impl_rand_traits_cartesian!(UniformYxy, Yxy<Wp> {x, y, luma} phantom: white_point: PhantomData<Wp>);

#[cfg(feature = "bytemuck")]
unsafe impl<Wp, T> bytemuck::Zeroable for Yxy<Wp, T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<Wp: 'static, T> bytemuck::Pod for Yxy<Wp, T> where T: bytemuck::Pod {}

#[cfg(test)]
mod test {
    use super::Yxy;
    use crate::white_point::D65;

    test_convert_into_from_xyz!(Yxy);

    #[cfg(feature = "approx")]
    mod conversion {
        use crate::{white_point::D65, FromColor, LinLuma, LinSrgb, Yxy};

        #[test]
        fn luma() {
            let a = Yxy::<D65>::from_color(LinLuma::new(0.5));
            let b = Yxy::new(0.312727, 0.329023, 0.5);
            assert_relative_eq!(a, b, epsilon = 0.000001);
        }

        #[test]
        fn red() {
            let a = Yxy::from_color(LinSrgb::new(1.0, 0.0, 0.0));
            let b = Yxy::new(0.64, 0.33, 0.212673);
            assert_relative_eq!(a, b, epsilon = 0.000001);
        }

        #[test]
        fn green() {
            let a = Yxy::from_color(LinSrgb::new(0.0, 1.0, 0.0));
            let b = Yxy::new(0.3, 0.6, 0.715152);
            assert_relative_eq!(a, b, epsilon = 0.000001);
        }

        #[test]
        fn blue() {
            let a = Yxy::from_color(LinSrgb::new(0.0, 0.0, 1.0));
            let b = Yxy::new(0.15, 0.06, 0.072175);
            assert_relative_eq!(a, b, epsilon = 0.000001);
        }
    }

    #[test]
    fn ranges() {
        assert_ranges! {
            Yxy<D65, f64>;
            clamped {
                x: 0.0 => 1.0,
                y: 0.0 => 1.0,
                luma: 0.0 => 1.0
            }
            clamped_min {}
            unclamped {}
        }
    }

    raw_pixel_conversion_tests!(Yxy<D65>: x, y, luma);
    raw_pixel_conversion_fail_tests!(Yxy<D65>: x, y, luma);

    #[test]
    fn check_min_max_components() {
        assert_eq!(Yxy::<D65>::min_x(), 0.0);
        assert_eq!(Yxy::<D65>::min_y(), 0.0);
        assert_eq!(Yxy::<D65>::min_luma(), 0.0);
        assert_eq!(Yxy::<D65>::max_x(), 1.0);
        assert_eq!(Yxy::<D65>::max_y(), 1.0);
        assert_eq!(Yxy::<D65>::max_luma(), 1.0);
    }

    struct_of_arrays_tests!(
        Yxy<D65>[x, y, luma] phantom: white_point,
        super::Yxya::new(0.1f32, 0.2, 0.3, 0.4),
        super::Yxya::new(0.2, 0.3, 0.4, 0.5),
        super::Yxya::new(0.3, 0.4, 0.5, 0.6)
    );

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Yxy::<D65>::new(0.3, 0.8, 0.1)).unwrap();

        assert_eq!(serialized, r#"{"x":0.3,"y":0.8,"luma":0.1}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Yxy = ::serde_json::from_str(r#"{"x":0.3,"y":0.8,"luma":0.1}"#).unwrap();

        assert_eq!(deserialized, Yxy::new(0.3, 0.8, 0.1));
    }

    test_uniform_distribution! {
        Yxy<D65, f32> {
            x: (0.0, 1.0),
            y: (0.0, 1.0),
            luma: (0.0, 1.0)
        },
        min: Yxy::new(0.0f32, 0.0, 0.0),
        max: Yxy::new(1.0, 1.0, 1.0),
    }
}

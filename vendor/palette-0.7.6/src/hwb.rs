//! Types for the HWB color space.

use core::{any::TypeId, marker::PhantomData};

#[cfg(feature = "random")]
use crate::hsv::UniformHsv;

use crate::{
    angle::FromAngle,
    bool_mask::{HasBoolMask, LazySelect, Select},
    convert::FromColorUnclamped,
    encoding::Srgb,
    hues::RgbHueIter,
    num::{Arithmetics, One, PartialCmp, Real},
    rgb::{RgbSpace, RgbStandard},
    stimulus::{FromStimulus, Stimulus},
    Alpha, FromColor, Hsv, RgbHue, Xyz,
};

/// Linear HWB with an alpha component. See the [`Hwba` implementation in
/// `Alpha`](crate::Alpha#Hwba).
pub type Hwba<S = Srgb, T = f32> = Alpha<Hwb<S, T>, T>;

/// HWB color space.
///
/// HWB is a cylindrical version of [RGB](crate::rgb::Rgb) and it's very
/// closely related to [HSV](crate::Hsv). It describes colors with a
/// starting hue, then a degree of whiteness and blackness to mix into that
/// base hue.
///
/// HWB component values are typically real numbers (such as floats), but may
/// also be converted to and from `u8` for storage and interoperability
/// purposes. The hue is then within the range `[0, 255]`.
///
/// ```
/// use approx::assert_relative_eq;
/// use palette::Hwb;
///
/// let hwb_u8 = Hwb::new_srgb(128u8, 85, 51);
/// let hwb_f32 = hwb_u8.into_format::<f32>();
///
/// assert_relative_eq!(hwb_f32, Hwb::new(180.0, 1.0 / 3.0, 0.2));
/// ```
///
/// It is very intuitive for humans to use and many color-pickers are based on
/// the HWB color system
#[derive(Debug, ArrayCast, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    rgb_standard = "S",
    component = "T",
    skip_derives(Hsv, Hwb)
)]
#[repr(C)]
pub struct Hwb<S = Srgb, T = f32> {
    /// The hue of the color, in degrees. Decides if it's red, blue, purple,
    /// etc. Same as the hue for HSL and HSV.
    #[palette(unsafe_same_layout_as = "T")]
    pub hue: RgbHue<T>,

    /// The whiteness of the color. It specifies the amount white to mix into
    /// the hue. It varies from 0 to 1, with 1 being always full white and 0
    /// always being the color shade (a mixture of a pure hue with black)
    /// chosen with the other two controls.
    pub whiteness: T,

    /// The blackness of the color. It specifies the amount black to mix into
    /// the hue. It varies from 0 to 1, with 1 being always full black and
    /// 0 always being the color tint (a mixture of a pure hue with white)
    /// chosen with the other two
    //controls.
    pub blackness: T,

    /// The white point and RGB primaries this color is adapted to. The default
    /// is the sRGB standard.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette(unsafe_zero_sized)]
    pub standard: PhantomData<S>,
}

impl<T> Hwb<Srgb, T> {
    /// Create an sRGB HWB color. This method can be used instead of `Hwb::new`
    /// to help type inference.
    pub fn new_srgb<H: Into<RgbHue<T>>>(hue: H, whiteness: T, blackness: T) -> Self {
        Self::new_const(hue.into(), whiteness, blackness)
    }

    /// Create an sRGB HWB color. This is the same as `Hwb::new_srgb` without the
    /// generic hue type. It's temporary until `const fn` supports traits.
    pub const fn new_srgb_const(hue: RgbHue<T>, whiteness: T, blackness: T) -> Self {
        Self::new_const(hue, whiteness, blackness)
    }
}

impl<S, T> Hwb<S, T> {
    /// Create an HWB color.
    pub fn new<H: Into<RgbHue<T>>>(hue: H, whiteness: T, blackness: T) -> Self {
        Self::new_const(hue.into(), whiteness, blackness)
    }

    /// Create an HWB color. This is the same as `Hwb::new` without the generic
    /// hue type. It's temporary until `const fn` supports traits.
    pub const fn new_const(hue: RgbHue<T>, whiteness: T, blackness: T) -> Self {
        Hwb {
            hue,
            whiteness,
            blackness,
            standard: PhantomData,
        }
    }

    /// Convert into another component type.
    pub fn into_format<U>(self) -> Hwb<S, U>
    where
        U: FromStimulus<T> + FromAngle<T>,
    {
        Hwb {
            hue: self.hue.into_format(),
            whiteness: U::from_stimulus(self.whiteness),
            blackness: U::from_stimulus(self.blackness),
            standard: PhantomData,
        }
    }

    /// Convert from another component type.
    pub fn from_format<U>(color: Hwb<S, U>) -> Self
    where
        T: FromStimulus<U> + FromAngle<U>,
    {
        color.into_format()
    }

    /// Convert to a `(hue, whiteness, blackness)` tuple.
    pub fn into_components(self) -> (RgbHue<T>, T, T) {
        (self.hue, self.whiteness, self.blackness)
    }

    /// Convert from a `(hue, whiteness, blackness)` tuple.
    pub fn from_components<H: Into<RgbHue<T>>>((hue, whiteness, blackness): (H, T, T)) -> Self {
        Self::new(hue, whiteness, blackness)
    }

    #[inline]
    fn reinterpret_as<St>(self) -> Hwb<St, T> {
        Hwb {
            hue: self.hue,
            whiteness: self.whiteness,
            blackness: self.blackness,
            standard: PhantomData,
        }
    }
}

impl<S, T> Hwb<S, T>
where
    T: Stimulus,
{
    /// Return the `whiteness` value minimum.
    pub fn min_whiteness() -> T {
        T::zero()
    }

    /// Return the `whiteness` value maximum.
    pub fn max_whiteness() -> T {
        T::max_intensity()
    }

    /// Return the `blackness` value minimum.
    pub fn min_blackness() -> T {
        T::zero()
    }

    /// Return the `blackness` value maximum.
    pub fn max_blackness() -> T {
        T::max_intensity()
    }
}

///<span id="Hwba"></span>[`Hwba`](crate::Hwba) implementations.
impl<T, A> Alpha<Hwb<Srgb, T>, A> {
    /// Create an sRGB HWB color with transparency. This method can be used
    /// instead of `Hwba::new` to help type inference.
    pub fn new_srgb<H: Into<RgbHue<T>>>(hue: H, whiteness: T, blackness: T, alpha: A) -> Self {
        Self::new_const(hue.into(), whiteness, blackness, alpha)
    }

    /// Create an sRGB HWB color with transparency. This is the same as
    /// `Hwba::new_srgb` without the generic hue type. It's temporary until `const
    /// fn` supports traits.
    pub const fn new_srgb_const(hue: RgbHue<T>, whiteness: T, blackness: T, alpha: A) -> Self {
        Self::new_const(hue, whiteness, blackness, alpha)
    }
}

///<span id="Hwba"></span>[`Hwba`](crate::Hwba) implementations.
impl<S, T, A> Alpha<Hwb<S, T>, A> {
    /// Create an HWB color with transparency.
    pub fn new<H: Into<RgbHue<T>>>(hue: H, whiteness: T, blackness: T, alpha: A) -> Self {
        Self::new_const(hue.into(), whiteness, blackness, alpha)
    }

    /// Create an HWB color with transparency. This is the same as `Hwba::new` without the
    /// generic hue type. It's temporary until `const fn` supports traits.
    pub const fn new_const(hue: RgbHue<T>, whiteness: T, blackness: T, alpha: A) -> Self {
        Alpha {
            color: Hwb::new_const(hue, whiteness, blackness),
            alpha,
        }
    }

    /// Convert into another component type.
    pub fn into_format<U, B>(self) -> Alpha<Hwb<S, U>, B>
    where
        U: FromStimulus<T> + FromAngle<T>,
        B: FromStimulus<A>,
    {
        Alpha {
            color: self.color.into_format(),
            alpha: B::from_stimulus(self.alpha),
        }
    }

    /// Convert from another component type.
    pub fn from_format<U, B>(color: Alpha<Hwb<S, U>, B>) -> Self
    where
        T: FromStimulus<U> + FromAngle<U>,
        A: FromStimulus<B>,
    {
        color.into_format()
    }

    /// Convert to a `(hue, whiteness, blackness, alpha)` tuple.
    pub fn into_components(self) -> (RgbHue<T>, T, T, A) {
        (
            self.color.hue,
            self.color.whiteness,
            self.color.blackness,
            self.alpha,
        )
    }

    /// Convert from a `(hue, whiteness, blackness, alpha)` tuple.
    pub fn from_components<H: Into<RgbHue<T>>>(
        (hue, whiteness, blackness, alpha): (H, T, T, A),
    ) -> Self {
        Self::new(hue, whiteness, blackness, alpha)
    }
}

impl_reference_component_methods_hue!(Hwb<S>, [whiteness, blackness], standard);
impl_struct_of_arrays_methods_hue!(Hwb<S>, [whiteness, blackness], standard);

impl<S1, S2, T> FromColorUnclamped<Hwb<S1, T>> for Hwb<S2, T>
where
    S1: RgbStandard + 'static,
    S2: RgbStandard + 'static,
    S1::Space: RgbSpace<WhitePoint = <S2::Space as RgbSpace>::WhitePoint>,
    Hsv<S1, T>: FromColorUnclamped<Hwb<S1, T>>,
    Hsv<S2, T>: FromColorUnclamped<Hsv<S1, T>>,
    Self: FromColorUnclamped<Hsv<S2, T>>,
{
    #[inline]
    fn from_color_unclamped(hwb: Hwb<S1, T>) -> Self {
        if TypeId::of::<S1>() == TypeId::of::<S2>() {
            hwb.reinterpret_as()
        } else {
            let hsv = Hsv::<S1, T>::from_color_unclamped(hwb);
            let converted_hsv = Hsv::<S2, T>::from_color_unclamped(hsv);
            Self::from_color_unclamped(converted_hsv)
        }
    }
}

impl<S, T> FromColorUnclamped<Hsv<S, T>> for Hwb<S, T>
where
    T: One + Arithmetics,
{
    #[inline]
    fn from_color_unclamped(color: Hsv<S, T>) -> Self {
        Hwb {
            hue: color.hue,
            whiteness: (T::one() - color.saturation) * &color.value,
            blackness: (T::one() - color.value),
            standard: PhantomData,
        }
    }
}

impl_tuple_conversion_hue!(Hwb<S> as (H, T, T), RgbHue);
impl_is_within_bounds_hwb!(Hwb<S> where T: Stimulus);
impl_clamp_hwb!(Hwb<S> phantom: standard where T: Stimulus);

impl_mix_hue!(Hwb<S> {whiteness, blackness} phantom: standard);
impl_lighten_hwb!(Hwb<S> phantom: standard where T: Stimulus);
impl_hue_ops!(Hwb<S>, RgbHue);

impl<S, T> HasBoolMask for Hwb<S, T>
where
    T: HasBoolMask,
{
    type Mask = T::Mask;
}

impl<S, T> Default for Hwb<S, T>
where
    T: Stimulus,
    RgbHue<T>: Default,
{
    fn default() -> Hwb<S, T> {
        Hwb::new(
            RgbHue::default(),
            Self::min_whiteness(),
            Self::max_blackness(),
        )
    }
}

impl_color_add!(Hwb<S>, [hue, whiteness, blackness], standard);
impl_color_sub!(Hwb<S>, [hue, whiteness, blackness], standard);

impl_array_casts!(Hwb<S, T>, [T; 3]);
impl_simd_array_conversion_hue!(Hwb<S>, [whiteness, blackness], standard);
impl_struct_of_array_traits_hue!(Hwb<S>, RgbHueIter, [whiteness, blackness], standard);

impl_copy_clone!(Hwb<S>, [hue, whiteness, blackness], standard);
impl_eq_hue!(Hwb<S>, RgbHue, [hue, whiteness, blackness]);

#[allow(deprecated)]
impl<S, T> crate::RelativeContrast for Hwb<S, T>
where
    T: Real + Arithmetics + PartialCmp,
    T::Mask: LazySelect<T>,
    S: RgbStandard,
    Xyz<<S::Space as RgbSpace>::WhitePoint, T>: FromColor<Self>,
{
    type Scalar = T;

    #[inline]
    fn get_contrast_ratio(self, other: Self) -> T {
        let xyz1 = Xyz::from_color(self);
        let xyz2 = Xyz::from_color(other);

        crate::contrast_ratio(xyz1.y, xyz2.y)
    }
}

impl_rand_traits_hwb_cone!(
    UniformHwb,
    Hwb<S>,
    UniformHsv,
    Hsv {
        height: value,
        radius: saturation
    }
    phantom: standard: PhantomData<S>
);

#[cfg(feature = "bytemuck")]
unsafe impl<S, T> bytemuck::Zeroable for Hwb<S, T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<S: 'static, T> bytemuck::Pod for Hwb<S, T> where T: bytemuck::Pod {}

#[cfg(test)]
mod test {
    use super::Hwb;

    test_convert_into_from_xyz!(Hwb);

    #[cfg(feature = "approx")]
    mod conversion {
        use crate::{FromColor, Hwb, Srgb};

        #[test]
        fn red() {
            let a = Hwb::from_color(Srgb::new(1.0, 0.0, 0.0));
            let b = Hwb::new_srgb(0.0, 0.0, 0.0);
            assert_relative_eq!(a, b);
        }

        #[test]
        fn orange() {
            let a = Hwb::from_color(Srgb::new(1.0, 0.5, 0.0));
            let b = Hwb::new_srgb(30.0, 0.0, 0.0);
            assert_relative_eq!(a, b);
        }

        #[test]
        fn green() {
            let a = Hwb::from_color(Srgb::new(0.0, 1.0, 0.0));
            let b = Hwb::new_srgb(120.0, 0.0, 0.0);
            assert_relative_eq!(a, b);
        }

        #[test]
        fn blue() {
            let a = Hwb::from_color(Srgb::new(0.0, 0.0, 1.0));
            let b = Hwb::new_srgb(240.0, 0.0, 0.0);
            assert_relative_eq!(a, b);
        }

        #[test]
        fn purple() {
            let a = Hwb::from_color(Srgb::new(0.5, 0.0, 1.0));
            let b = Hwb::new_srgb(270.0, 0.0, 0.0);
            assert_relative_eq!(a, b);
        }
    }

    #[cfg(feature = "approx")]
    mod clamp {
        use crate::{Clamp, Hwb};

        #[test]
        fn clamp_invalid() {
            let expected = Hwb::new_srgb(240.0, 0.0, 0.0);
            let clamped = Hwb::new_srgb(240.0, -3.0, -4.0).clamp();
            assert_relative_eq!(expected, clamped);
        }

        #[test]
        fn clamp_none() {
            let expected = Hwb::new_srgb(240.0, 0.3, 0.7);
            let clamped = Hwb::new_srgb(240.0, 0.3, 0.7).clamp();
            assert_relative_eq!(expected, clamped);
        }
        #[test]
        fn clamp_over_one() {
            let expected = Hwb::new_srgb(240.0, 0.2, 0.8);
            let clamped = Hwb::new_srgb(240.0, 5.0, 20.0).clamp();
            assert_relative_eq!(expected, clamped);
        }
        #[test]
        fn clamp_under_one() {
            let expected = Hwb::new_srgb(240.0, 0.3, 0.1);
            let clamped = Hwb::new_srgb(240.0, 0.3, 0.1).clamp();
            assert_relative_eq!(expected, clamped);
        }
    }

    raw_pixel_conversion_tests!(Hwb<crate::encoding::Srgb>: hue, whiteness, blackness);
    raw_pixel_conversion_fail_tests!(Hwb<crate::encoding::Srgb>: hue, whiteness, blackness);

    #[test]
    fn check_min_max_components() {
        use crate::encoding::Srgb;

        assert_eq!(Hwb::<Srgb>::min_whiteness(), 0.0,);
        assert_eq!(Hwb::<Srgb>::min_blackness(), 0.0,);
        assert_eq!(Hwb::<Srgb>::max_whiteness(), 1.0,);
        assert_eq!(Hwb::<Srgb>::max_blackness(), 1.0,);
    }

    struct_of_arrays_tests!(
        Hwb<crate::encoding::Srgb>[hue, whiteness, blackness] phantom: standard,
        super::Hwba::new(0.1f32, 0.2, 0.3, 0.4),
        super::Hwba::new(0.2, 0.3, 0.4, 0.5),
        super::Hwba::new(0.3, 0.4, 0.5, 0.6)
    );

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Hwb::new_srgb(0.3, 0.8, 0.1)).unwrap();

        assert_eq!(serialized, r#"{"hue":0.3,"whiteness":0.8,"blackness":0.1}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Hwb =
            ::serde_json::from_str(r#"{"hue":0.3,"whiteness":0.8,"blackness":0.1}"#).unwrap();

        assert_eq!(deserialized, Hwb::new(0.3, 0.8, 0.1));
    }

    test_uniform_distribution! {
        Hwb<crate::encoding::Srgb, f32> as crate::rgb::Rgb {
            red: (0.0, 1.0),
            green: (0.0, 1.0),
            blue: (0.0, 1.0)
        },
        min: Hwb::new(0.0f32, 0.0, 0.0),
        max: Hwb::new(360.0, 1.0, 1.0)
    }
}

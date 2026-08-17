//! Types for the HSV color space.

use core::{any::TypeId, marker::PhantomData};

use crate::{
    angle::{FromAngle, RealAngle},
    bool_mask::{BitOps, BoolMask, HasBoolMask, LazySelect, Select},
    convert::FromColorUnclamped,
    encoding::Srgb,
    hues::RgbHueIter,
    num::{Arithmetics, IsValidDivisor, MinMax, One, PartialCmp, Real, Zero},
    rgb::{Rgb, RgbSpace, RgbStandard},
    stimulus::{FromStimulus, Stimulus},
    Alpha, FromColor, Hsl, Hwb, RgbHue, Xyz,
};

/// Linear HSV with an alpha component. See the [`Hsva` implementation in
/// `Alpha`](crate::Alpha#Hsva).
pub type Hsva<S = Srgb, T = f32> = Alpha<Hsv<S, T>, T>;

/// HSV color space.
///
/// HSV is a cylindrical version of [RGB](crate::rgb::Rgb) and it's very similar
/// to [HSL](crate::Hsl). The difference is that the `value` component in HSV
/// determines the _brightness_ of the color, and not the _lightness_. The
/// difference is that, for example, red (100% R, 0% G, 0% B) and white (100% R,
/// 100% G, 100% B) has the same brightness (or value), but not the same
/// lightness.
///
/// HSV component values are typically real numbers (such as floats), but may
/// also be converted to and from `u8` for storage and interoperability
/// purposes. The hue is then within the range `[0, 255]`.
///
/// ```
/// use approx::assert_relative_eq;
/// use palette::Hsv;
///
/// let hsv_u8 = Hsv::new_srgb(128u8, 85, 51);
/// let hsv_f32 = hsv_u8.into_format::<f32>();
///
/// assert_relative_eq!(hsv_f32, Hsv::new(180.0, 1.0 / 3.0, 0.2));
/// ```
#[derive(Debug, ArrayCast, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    rgb_standard = "S",
    component = "T",
    skip_derives(Rgb, Hsl, Hwb, Hsv)
)]
#[repr(C)]
#[doc(alias = "hsb")]
pub struct Hsv<S = Srgb, T = f32> {
    /// The hue of the color, in degrees. Decides if it's red, blue, purple,
    /// etc.
    #[palette(unsafe_same_layout_as = "T")]
    pub hue: RgbHue<T>,

    /// The colorfulness of the color. 0.0 gives gray scale colors and 1.0 will
    /// give absolutely clear colors.
    pub saturation: T,

    /// Decides how bright the color will look. 0.0 will be black, and 1.0 will
    /// give a bright an clear color that goes towards white when `saturation`
    /// goes towards 0.0.
    pub value: T,

    /// The white point and RGB primaries this color is adapted to. The default
    /// is the sRGB standard.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette(unsafe_zero_sized)]
    pub standard: PhantomData<S>,
}

impl<T> Hsv<Srgb, T> {
    /// Create an sRGB HSV color. This method can be used instead of `Hsv::new`
    /// to help type inference.
    pub fn new_srgb<H: Into<RgbHue<T>>>(hue: H, saturation: T, value: T) -> Self {
        Self::new_const(hue.into(), saturation, value)
    }

    /// Create an sRGB HSV color. This is the same as `Hsv::new_srgb` without
    /// the generic hue type. It's temporary until `const fn` supports traits.
    pub const fn new_srgb_const(hue: RgbHue<T>, saturation: T, value: T) -> Self {
        Self::new_const(hue, saturation, value)
    }
}

impl<S, T> Hsv<S, T> {
    /// Create an HSV color.
    pub fn new<H: Into<RgbHue<T>>>(hue: H, saturation: T, value: T) -> Self {
        Self::new_const(hue.into(), saturation, value)
    }

    /// Create an HSV color. This is the same as `Hsv::new` without the generic
    /// hue type. It's temporary until `const fn` supports traits.
    pub const fn new_const(hue: RgbHue<T>, saturation: T, value: T) -> Self {
        Hsv {
            hue,
            saturation,
            value,
            standard: PhantomData,
        }
    }

    /// Convert into another component type.
    pub fn into_format<U>(self) -> Hsv<S, U>
    where
        U: FromStimulus<T> + FromAngle<T>,
    {
        Hsv {
            hue: self.hue.into_format(),
            saturation: U::from_stimulus(self.saturation),
            value: U::from_stimulus(self.value),
            standard: PhantomData,
        }
    }

    /// Convert from another component type.
    pub fn from_format<U>(color: Hsv<S, U>) -> Self
    where
        T: FromStimulus<U> + FromAngle<U>,
    {
        color.into_format()
    }

    /// Convert to a `(hue, saturation, value)` tuple.
    pub fn into_components(self) -> (RgbHue<T>, T, T) {
        (self.hue, self.saturation, self.value)
    }

    /// Convert from a `(hue, saturation, value)` tuple.
    pub fn from_components<H: Into<RgbHue<T>>>((hue, saturation, value): (H, T, T)) -> Self {
        Self::new(hue, saturation, value)
    }

    #[inline]
    fn reinterpret_as<St>(self) -> Hsv<St, T> {
        Hsv {
            hue: self.hue,
            saturation: self.saturation,
            value: self.value,
            standard: PhantomData,
        }
    }
}

impl<S, T> Hsv<S, T>
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

    /// Return the `value` value minimum.
    pub fn min_value() -> T {
        T::zero()
    }

    /// Return the `value` value maximum.
    pub fn max_value() -> T {
        T::max_intensity()
    }
}

///<span id="Hsva"></span>[`Hsva`](crate::Hsva) implementations.
impl<T, A> Alpha<Hsv<Srgb, T>, A> {
    /// Create an sRGB HSV color with transparency. This method can be used
    /// instead of `Hsva::new` to help type inference.
    pub fn new_srgb<H: Into<RgbHue<T>>>(hue: H, saturation: T, value: T, alpha: A) -> Self {
        Self::new_const(hue.into(), saturation, value, alpha)
    }

    /// Create an sRGB HSV color with transparency. This is the same as
    /// `Hsva::new_srgb` without the generic hue type. It's temporary until
    /// `const fn` supports traits.
    pub const fn new_srgb_const(hue: RgbHue<T>, saturation: T, value: T, alpha: A) -> Self {
        Self::new_const(hue, saturation, value, alpha)
    }
}

///<span id="Hsva"></span>[`Hsva`](crate::Hsva) implementations.
impl<S, T, A> Alpha<Hsv<S, T>, A> {
    /// Create an HSV color with transparency.
    pub fn new<H: Into<RgbHue<T>>>(hue: H, saturation: T, value: T, alpha: A) -> Self {
        Self::new_const(hue.into(), saturation, value, alpha)
    }

    /// Create an HSV color with transparency. This is the same as `Hsva::new`
    /// without the generic hue type. It's temporary until `const fn` supports
    /// traits.
    pub const fn new_const(hue: RgbHue<T>, saturation: T, value: T, alpha: A) -> Self {
        Alpha {
            color: Hsv::new_const(hue, saturation, value),
            alpha,
        }
    }

    /// Convert into another component type.
    pub fn into_format<U, B>(self) -> Alpha<Hsv<S, U>, B>
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
    pub fn from_format<U, B>(color: Alpha<Hsv<S, U>, B>) -> Self
    where
        T: FromStimulus<U> + FromAngle<U>,
        A: FromStimulus<B>,
    {
        color.into_format()
    }

    /// Convert to a `(hue, saturation, value, alpha)` tuple.
    pub fn into_components(self) -> (RgbHue<T>, T, T, A) {
        (
            self.color.hue,
            self.color.saturation,
            self.color.value,
            self.alpha,
        )
    }

    /// Convert from a `(hue, saturation, value, alpha)` tuple.
    pub fn from_components<H: Into<RgbHue<T>>>(
        (hue, saturation, value, alpha): (H, T, T, A),
    ) -> Self {
        Self::new(hue, saturation, value, alpha)
    }
}

impl_reference_component_methods_hue!(Hsv<S>, [saturation, value], standard);
impl_struct_of_arrays_methods_hue!(Hsv<S>, [saturation, value], standard);

impl<S1, S2, T> FromColorUnclamped<Hsv<S1, T>> for Hsv<S2, T>
where
    S1: RgbStandard + 'static,
    S2: RgbStandard + 'static,
    S1::Space: RgbSpace<WhitePoint = <S2::Space as RgbSpace>::WhitePoint>,
    Rgb<S1, T>: FromColorUnclamped<Hsv<S1, T>>,
    Rgb<S2, T>: FromColorUnclamped<Rgb<S1, T>>,
    Self: FromColorUnclamped<Rgb<S2, T>>,
{
    #[inline]
    fn from_color_unclamped(hsv: Hsv<S1, T>) -> Self {
        if TypeId::of::<S1>() == TypeId::of::<S2>() {
            hsv.reinterpret_as()
        } else {
            let rgb = Rgb::<S1, T>::from_color_unclamped(hsv);
            let converted_rgb = Rgb::<S2, T>::from_color_unclamped(rgb);
            Self::from_color_unclamped(converted_rgb)
        }
    }
}

impl<S, T> FromColorUnclamped<Rgb<S, T>> for Hsv<S, T>
where
    T: RealAngle + One + Zero + MinMax + Arithmetics + PartialCmp + Clone,
    T::Mask: BoolMask + BitOps + LazySelect<T> + Clone + 'static,
{
    fn from_color_unclamped(rgb: Rgb<S, T>) -> Self {
        // Avoid negative numbers
        let red = rgb.red.max(T::zero());
        let green = rgb.green.max(T::zero());
        let blue = rgb.blue.max(T::zero());

        // The SIMD optimized version showed significant slowdown for regular floats.
        if TypeId::of::<T::Mask>() == TypeId::of::<bool>() {
            let (max, min, sep, coeff) = {
                let (max, min, sep, coeff) = if red.gt(&green).is_true() {
                    (red.clone(), green.clone(), green.clone() - &blue, T::zero())
                } else {
                    (
                        green.clone(),
                        red.clone(),
                        blue.clone() - &red,
                        T::from_f64(2.0),
                    )
                };
                if blue.gt(&max).is_true() {
                    (blue, min, red - green, T::from_f64(4.0))
                } else {
                    let min_val = if blue.lt(&min).is_true() { blue } else { min };
                    (max, min_val, sep, coeff)
                }
            };

            let (h, s) = if max.neq(&min).is_true() {
                let d = max.clone() - min;
                let h = ((sep / &d) + coeff) * T::from_f64(60.0);
                let s = d / &max;

                (h, s)
            } else {
                (T::zero(), T::zero())
            };
            let v = max;

            Hsv {
                hue: h.into(),
                saturation: s,
                value: v,
                standard: PhantomData,
            }
        } else {
            // Based on OPTIMIZED RGB TO HSV COLOR CONVERSION USING SSE TECHNOLOGY
            // by KOBALICEK, Petr & BLIZNAK, Michal
            //
            // This implementation assumes less about the underlying mask and number
            // representation. The hue is also multiplied by 6 to avoid rounding
            // errors when using degrees.

            let six = T::from_f64(6.0);

            let value = red.clone().max(green.clone()).max(blue.clone());
            let min = red.clone().min(green.clone()).min(blue.clone());

            let chroma = value.clone() - min;
            let saturation = chroma
                .eq(&T::zero())
                .lazy_select(|| T::zero(), || chroma.clone() / &value);

            // Each of these represents an RGB component. The maximum will be false
            // while the two other will be true. They are later used for determining
            // which branch in the hue equation we end up in.
            let x = value.neq(&red);
            let y = value.eq(&red) | value.neq(&green);
            let z = value.eq(&red) | value.eq(&green);

            // The hue base is the `1`, `2/6`, `4/6` or 0 part of the hue equation,
            // except it's multiplied by 6 here.
            let hue_base = x.clone().select(
                z.clone().select(T::from_f64(-4.0), T::from_f64(4.0)),
                T::zero(),
            ) + &six;

            // Each of these is a part of `G - B`, `B - R`, `R - G` or 0 from the
            // hue equation. They become positive, negative or 0, depending on which
            // branch we should be in. This makes the sum of all three combine as
            // expected.
            let red_m = lazy_select! {
               if x => y.clone().select(red.clone(), -red),
               else => T::zero(),
            };
            let green_m = lazy_select! {
               if y.clone() => z.clone().select(green.clone(), -green),
               else => T::zero(),
            };
            let blue_m = lazy_select! {
               if z => y.select(-blue.clone(), blue),
               else => T::zero(),
            };

            // This is the hue equation parts combined. The hue base is the constant
            // and the RGB components are masked so up to two of them are non-zero.
            // Once again, this is multiplied by 6, so the chroma isn't multiplied
            // before dividing.
            //
            // We also avoid dividing by 0 for non-SIMD values.
            let hue = lazy_select! {
                if chroma.eq(&T::zero()) => T::zero(),
                else => hue_base + (red_m + green_m + blue_m) / &chroma,
            };

            // hue will always be within [0, 12) (it's multiplied by 6, compared to
            // the paper), so we can subtract by 6 instead of using % to get it
            // within [0, 6).
            let hue_sub = hue.gt_eq(&six).select(six, T::zero());
            let hue = hue - hue_sub;

            Hsv {
                hue: RgbHue::from_degrees(hue * T::from_f64(60.0)),
                saturation,
                value,
                standard: PhantomData,
            }
        }
    }
}

impl<S, T> FromColorUnclamped<Hsl<S, T>> for Hsv<S, T>
where
    T: Real + Zero + One + IsValidDivisor + Arithmetics + PartialCmp + Clone,
    T::Mask: LazySelect<T>,
{
    #[inline]
    fn from_color_unclamped(hsl: Hsl<S, T>) -> Self {
        let x = lazy_select! {
            if hsl.lightness.lt(&T::from_f64(0.5)) => hsl.lightness.clone(),
            else => T::one() - &hsl.lightness,
        } * hsl.saturation;

        let value = hsl.lightness + &x;

        // avoid divide by zero
        let saturation = lazy_select! {
            if value.is_valid_divisor() => x * T::from_f64(2.0) / &value,
            else => T::zero(),
        };

        Hsv {
            hue: hsl.hue,
            saturation,
            value,
            standard: PhantomData,
        }
    }
}

impl<S, T> FromColorUnclamped<Hwb<S, T>> for Hsv<S, T>
where
    T: One + Zero + IsValidDivisor + Arithmetics,
    T::Mask: LazySelect<T>,
{
    #[inline]
    fn from_color_unclamped(hwb: Hwb<S, T>) -> Self {
        let Hwb {
            hue,
            whiteness,
            blackness,
            ..
        } = hwb;

        let value = T::one() - blackness;

        // avoid divide by zero
        let saturation = lazy_select! {
            if value.is_valid_divisor() => T::one() - (whiteness / &value),
            else => T::zero(),
        };

        Hsv {
            hue,
            saturation,
            value,
            standard: PhantomData,
        }
    }
}

impl_tuple_conversion_hue!(Hsv<S> as (H, T, T), RgbHue);

impl_is_within_bounds! {
    Hsv<S> {
        saturation => [Self::min_saturation(), Self::max_saturation()],
        value => [Self::min_value(), Self::max_value()]
    }
    where T: Stimulus
}
impl_clamp! {
    Hsv<S> {
        saturation => [Self::min_saturation(), Self::max_saturation()],
        value => [Self::min_value(), Self::max_value()]
    }
    other {hue, standard}
    where T: Stimulus
}

impl_mix_hue!(Hsv<S> {saturation, value} phantom: standard);
impl_lighten!(Hsv<S> increase {value => [Self::min_value(), Self::max_value()]} other {hue, saturation} phantom: standard where T: Stimulus);
impl_saturate!(Hsv<S> increase {saturation => [Self::min_saturation(), Self::max_saturation()]} other {hue, value} phantom: standard where T: Stimulus);
impl_hue_ops!(Hsv<S>, RgbHue);

impl<S, T> HasBoolMask for Hsv<S, T>
where
    T: HasBoolMask,
{
    type Mask = T::Mask;
}

impl<S, T> Default for Hsv<S, T>
where
    T: Stimulus,
    RgbHue<T>: Default,
{
    fn default() -> Hsv<S, T> {
        Hsv::new(RgbHue::default(), Self::min_saturation(), Self::min_value())
    }
}

impl_color_add!(Hsv<S>, [hue, saturation, value], standard);
impl_color_sub!(Hsv<S>, [hue, saturation, value], standard);

impl_array_casts!(Hsv<S, T>, [T; 3]);
impl_simd_array_conversion_hue!(Hsv<S>, [saturation, value], standard);
impl_struct_of_array_traits_hue!(Hsv<S>, RgbHueIter, [saturation, value], standard);

impl_eq_hue!(Hsv<S>, RgbHue, [hue, saturation, value]);
impl_copy_clone!(Hsv<S>, [hue, saturation, value], standard);

#[allow(deprecated)]
impl<S, T> crate::RelativeContrast for Hsv<S, T>
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

impl_rand_traits_hsv_cone!(
    UniformHsv,
    Hsv<S> {
        hue: UniformRgbHue => RgbHue,
        height: value,
        radius: saturation
    }
    phantom: standard: PhantomData<S>
);

#[cfg(feature = "bytemuck")]
unsafe impl<S, T> bytemuck::Zeroable for Hsv<S, T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<S: 'static, T> bytemuck::Pod for Hsv<S, T> where T: bytemuck::Pod {}

#[cfg(test)]
mod test {
    use super::Hsv;

    test_convert_into_from_xyz!(Hsv);

    #[cfg(feature = "approx")]
    mod conversion {
        use crate::{FromColor, Hsl, Hsv, Srgb};

        #[test]
        fn red() {
            let a = Hsv::from_color(Srgb::new(1.0, 0.0, 0.0));
            let b = Hsv::new_srgb(0.0, 1.0, 1.0);
            let c = Hsv::from_color(Hsl::new_srgb(0.0, 1.0, 0.5));

            assert_relative_eq!(a, b);
            assert_relative_eq!(a, c);
        }

        #[test]
        fn orange() {
            let a = Hsv::from_color(Srgb::new(1.0, 0.5, 0.0));
            let b = Hsv::new_srgb(30.0, 1.0, 1.0);
            let c = Hsv::from_color(Hsl::new_srgb(30.0, 1.0, 0.5));

            assert_relative_eq!(a, b);
            assert_relative_eq!(a, c);
        }

        #[test]
        fn green() {
            let a = Hsv::from_color(Srgb::new(0.0, 1.0, 0.0));
            let b = Hsv::new_srgb(120.0, 1.0, 1.0);
            let c = Hsv::from_color(Hsl::new_srgb(120.0, 1.0, 0.5));

            assert_relative_eq!(a, b);
            assert_relative_eq!(a, c);
        }

        #[test]
        fn blue() {
            let a = Hsv::from_color(Srgb::new(0.0, 0.0, 1.0));
            let b = Hsv::new_srgb(240.0, 1.0, 1.0);
            let c = Hsv::from_color(Hsl::new_srgb(240.0, 1.0, 0.5));

            assert_relative_eq!(a, b);
            assert_relative_eq!(a, c);
        }

        #[test]
        fn purple() {
            let a = Hsv::from_color(Srgb::new(0.5, 0.0, 1.0));
            let b = Hsv::new_srgb(270.0, 1.0, 1.0);
            let c = Hsv::from_color(Hsl::new_srgb(270.0, 1.0, 0.5));

            assert_relative_eq!(a, b);
            assert_relative_eq!(a, c);
        }
    }

    #[test]
    fn ranges() {
        assert_ranges! {
            Hsv<crate::encoding::Srgb, f64>;
            clamped {
                saturation: 0.0 => 1.0,
                value: 0.0 => 1.0
            }
            clamped_min {}
            unclamped {
                hue: -360.0 => 360.0
            }
        }
    }

    raw_pixel_conversion_tests!(Hsv<crate::encoding::Srgb>: hue, saturation, value);
    raw_pixel_conversion_fail_tests!(Hsv<crate::encoding::Srgb>: hue, saturation, value);

    #[test]
    fn check_min_max_components() {
        use crate::encoding::Srgb;

        assert_eq!(Hsv::<Srgb>::min_saturation(), 0.0,);
        assert_eq!(Hsv::<Srgb>::min_value(), 0.0,);
        assert_eq!(Hsv::<Srgb>::max_saturation(), 1.0,);
        assert_eq!(Hsv::<Srgb>::max_value(), 1.0,);
    }

    struct_of_arrays_tests!(
        Hsv<crate::encoding::Srgb>[hue, saturation, value] phantom: standard,
        super::Hsva::new(0.1f32, 0.2, 0.3, 0.4),
        super::Hsva::new(0.2, 0.3, 0.4, 0.5),
        super::Hsva::new(0.3, 0.4, 0.5, 0.6)
    );

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Hsv::new_srgb(0.3, 0.8, 0.1)).unwrap();

        assert_eq!(serialized, r#"{"hue":0.3,"saturation":0.8,"value":0.1}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Hsv =
            ::serde_json::from_str(r#"{"hue":0.3,"saturation":0.8,"value":0.1}"#).unwrap();

        assert_eq!(deserialized, Hsv::new(0.3, 0.8, 0.1));
    }

    test_uniform_distribution! {
        Hsv<crate::encoding::Srgb, f32> as crate::rgb::Rgb {
            red: (0.0, 1.0),
            green: (0.0, 1.0),
            blue: (0.0, 1.0)
        },
        min: Hsv::new(0.0f32, 0.0, 0.0),
        max: Hsv::new(360.0, 1.0, 1.0)
    }
}

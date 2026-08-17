//! Types for the HSL color space.

use core::{any::TypeId, marker::PhantomData, ops::Not};

use crate::{
    angle::{FromAngle, RealAngle},
    bool_mask::{BitOps, BoolMask, HasBoolMask, LazySelect, Select},
    convert::FromColorUnclamped,
    encoding::Srgb,
    hues::RgbHueIter,
    num::{Arithmetics, IsValidDivisor, MinMax, One, PartialCmp, Real, Zero},
    rgb::{Rgb, RgbSpace, RgbStandard},
    stimulus::{FromStimulus, Stimulus},
    Alpha, FromColor, Hsv, RgbHue, Xyz,
};

/// Linear HSL with an alpha component. See the [`Hsla` implementation in
/// `Alpha`](crate::Alpha#Hsla).
pub type Hsla<S = Srgb, T = f32> = Alpha<Hsl<S, T>, T>;

/// HSL color space.
///
/// The HSL color space can be seen as a cylindrical version of
/// [RGB](crate::rgb::Rgb), where the `hue` is the angle around the color
/// cylinder, the `saturation` is the distance from the center, and the
/// `lightness` is the height from the bottom. Its composition makes it
/// especially good for operations like changing green to red, making a color
/// more gray, or making it darker.
///
/// HSL component values are typically real numbers (such as floats), but may
/// also be converted to and from `u8` for storage and interoperability
/// purposes. The hue is then within the range `[0, 255]`.
///
/// ```
/// use approx::assert_relative_eq;
/// use palette::Hsl;
///
/// let hsl_u8 = Hsl::new_srgb(128u8, 85, 51);
/// let hsl_f32 = hsl_u8.into_format::<f32>();
///
/// assert_relative_eq!(hsl_f32, Hsl::new(180.0, 1.0 / 3.0, 0.2));
/// ```
///
/// See [HSV](crate::Hsv) for a very similar color space, with brightness
/// instead of lightness.
#[derive(Debug, ArrayCast, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    rgb_standard = "S",
    component = "T",
    skip_derives(Rgb, Hsv, Hsl)
)]
#[repr(C)]
pub struct Hsl<S = Srgb, T = f32> {
    /// The hue of the color, in degrees. Decides if it's red, blue, purple,
    /// etc.
    #[palette(unsafe_same_layout_as = "T")]
    pub hue: RgbHue<T>,

    /// The colorfulness of the color. 0.0 gives gray scale colors and 1.0 will
    /// give absolutely clear colors.
    pub saturation: T,

    /// Decides how light the color will look. 0.0 will be black, 0.5 will give
    /// a clear color, and 1.0 will give white.
    pub lightness: T,

    /// The white point and RGB primaries this color is adapted to. The default
    /// is the sRGB standard.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette(unsafe_zero_sized)]
    pub standard: PhantomData<S>,
}

impl<T> Hsl<Srgb, T> {
    /// Create an sRGB HSL color. This method can be used instead of `Hsl::new`
    /// to help type inference.
    pub fn new_srgb<H: Into<RgbHue<T>>>(hue: H, saturation: T, lightness: T) -> Self {
        Self::new_const(hue.into(), saturation, lightness)
    }

    /// Create an sRGB HSL color. This is the same as `Hsl::new_srgb` without
    /// the generic hue type. It's temporary until `const fn` supports traits.
    pub const fn new_srgb_const(hue: RgbHue<T>, saturation: T, lightness: T) -> Self {
        Self::new_const(hue, saturation, lightness)
    }
}

impl<S, T> Hsl<S, T> {
    /// Create an HSL color.
    pub fn new<H: Into<RgbHue<T>>>(hue: H, saturation: T, lightness: T) -> Self {
        Self::new_const(hue.into(), saturation, lightness)
    }

    /// Create an HSL color. This is the same as `Hsl::new` without the generic
    /// hue type. It's temporary until `const fn` supports traits.
    pub const fn new_const(hue: RgbHue<T>, saturation: T, lightness: T) -> Self {
        Hsl {
            hue,
            saturation,
            lightness,
            standard: PhantomData,
        }
    }

    /// Convert into another component type.
    pub fn into_format<U>(self) -> Hsl<S, U>
    where
        U: FromStimulus<T> + FromAngle<T>,
    {
        Hsl {
            hue: self.hue.into_format(),
            saturation: U::from_stimulus(self.saturation),
            lightness: U::from_stimulus(self.lightness),
            standard: PhantomData,
        }
    }

    /// Convert from another component type.
    pub fn from_format<U>(color: Hsl<S, U>) -> Self
    where
        T: FromStimulus<U> + FromAngle<U>,
    {
        color.into_format()
    }

    /// Convert to a `(hue, saturation, lightness)` tuple.
    pub fn into_components(self) -> (RgbHue<T>, T, T) {
        (self.hue, self.saturation, self.lightness)
    }

    /// Convert from a `(hue, saturation, lightness)` tuple.
    pub fn from_components<H: Into<RgbHue<T>>>((hue, saturation, lightness): (H, T, T)) -> Self {
        Self::new(hue, saturation, lightness)
    }

    #[inline]
    fn reinterpret_as<St>(self) -> Hsl<St, T> {
        Hsl {
            hue: self.hue,
            saturation: self.saturation,
            lightness: self.lightness,
            standard: PhantomData,
        }
    }
}

impl<S, T> Hsl<S, T>
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

///<span id="Hsla"></span>[`Hsla`](crate::Hsla) implementations.
impl<T, A> Alpha<Hsl<Srgb, T>, A> {
    /// Create an sRGB HSL color with transparency. This method can be used
    /// instead of `Hsla::new` to help type inference.
    pub fn new_srgb<H: Into<RgbHue<T>>>(hue: H, saturation: T, lightness: T, alpha: A) -> Self {
        Self::new_const(hue.into(), saturation, lightness, alpha)
    }

    /// Create an sRGB HSL color with transparency. This is the same as
    /// `Hsla::new_srgb` without the generic hue type. It's temporary until
    /// `const fn` supports traits.
    pub const fn new_srgb_const(hue: RgbHue<T>, saturation: T, lightness: T, alpha: A) -> Self {
        Self::new_const(hue, saturation, lightness, alpha)
    }
}

///<span id="Hsla"></span>[`Hsla`](crate::Hsla) implementations.
impl<S, T, A> Alpha<Hsl<S, T>, A> {
    /// Create an HSL color with transparency.
    pub fn new<H: Into<RgbHue<T>>>(hue: H, saturation: T, lightness: T, alpha: A) -> Self {
        Self::new_const(hue.into(), saturation, lightness, alpha)
    }

    /// Create an HSL color with transparency. This is the same as `Hsla::new`
    /// without the generic hue type. It's temporary until `const fn` supports
    /// traits.
    pub const fn new_const(hue: RgbHue<T>, saturation: T, lightness: T, alpha: A) -> Self {
        Alpha {
            color: Hsl::new_const(hue, saturation, lightness),
            alpha,
        }
    }
    /// Convert into another component type.
    pub fn into_format<U, B>(self) -> Alpha<Hsl<S, U>, B>
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
    pub fn from_format<U, B>(color: Alpha<Hsl<S, U>, B>) -> Self
    where
        T: FromStimulus<U> + FromAngle<U>,
        A: FromStimulus<B>,
    {
        color.into_format()
    }

    /// Convert to a `(hue, saturation, lightness, alpha)` tuple.
    pub fn into_components(self) -> (RgbHue<T>, T, T, A) {
        (
            self.color.hue,
            self.color.saturation,
            self.color.lightness,
            self.alpha,
        )
    }

    /// Convert from a `(hue, saturation, lightness, alpha)` tuple.
    pub fn from_components<H: Into<RgbHue<T>>>(
        (hue, saturation, lightness, alpha): (H, T, T, A),
    ) -> Self {
        Self::new(hue, saturation, lightness, alpha)
    }
}

impl_reference_component_methods_hue!(Hsl<S>, [saturation, lightness], standard);
impl_struct_of_arrays_methods_hue!(Hsl<S>, [saturation, lightness], standard);

impl<S1, S2, T> FromColorUnclamped<Hsl<S1, T>> for Hsl<S2, T>
where
    S1: RgbStandard + 'static,
    S2: RgbStandard + 'static,
    S1::Space: RgbSpace<WhitePoint = <S2::Space as RgbSpace>::WhitePoint>,
    Rgb<S1, T>: FromColorUnclamped<Hsl<S1, T>>,
    Rgb<S2, T>: FromColorUnclamped<Rgb<S1, T>>,
    Self: FromColorUnclamped<Rgb<S2, T>>,
{
    #[inline]
    fn from_color_unclamped(hsl: Hsl<S1, T>) -> Self {
        if TypeId::of::<S1>() == TypeId::of::<S2>() {
            hsl.reinterpret_as()
        } else {
            let rgb = Rgb::<S1, T>::from_color_unclamped(hsl);
            let converted_rgb = Rgb::<S2, T>::from_color_unclamped(rgb);
            Self::from_color_unclamped(converted_rgb)
        }
    }
}

impl<S, T> FromColorUnclamped<Rgb<S, T>> for Hsl<S, T>
where
    T: RealAngle + Zero + One + MinMax + Arithmetics + PartialCmp + Clone,
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

            let mut h = T::zero();
            let mut s = T::zero();

            let sum = max.clone() + &min;
            let l = sum.clone() / T::from_f64(2.0);
            if max.neq(&min).is_true() {
                let d = max - min;
                s = if sum.gt(&T::one()).is_true() {
                    d.clone() / (T::from_f64(2.0) - sum)
                } else {
                    d.clone() / sum
                };
                h = ((sep / d) + coeff) * T::from_f64(60.0);
            };

            Hsl {
                hue: h.into(),
                saturation: s,
                lightness: l,
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

            let max = red.clone().max(green.clone()).max(blue.clone());
            let min = red.clone().min(green.clone()).min(blue.clone());

            let sum = max.clone() + &min;
            let lightness = T::from_f64(0.5) * &sum;

            let chroma = max.clone() - &min;
            let saturation = lazy_select! {
                if min.eq(&max) => T::zero(),
                else => chroma.clone() /
                    sum.gt(&T::one()).select(T::from_f64(2.0) - &sum, sum.clone()),
            };

            // Each of these represents an RGB component. The maximum will be false
            // while the two other will be true. They are later used for determining
            // which branch in the hue equation we end up in.
            let x = max.neq(&red);
            let y = max.eq(&red) | max.neq(&green);
            let z = max.eq(&red) | max.eq(&green);

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

            Hsl {
                hue: RgbHue::from_degrees(hue * T::from_f64(60.0)),
                saturation,
                lightness,
                standard: PhantomData,
            }
        }
    }
}

impl<S, T> FromColorUnclamped<Hsv<S, T>> for Hsl<S, T>
where
    T: Real + Zero + One + IsValidDivisor + Arithmetics + PartialCmp + Clone,
    T::Mask: LazySelect<T> + Not<Output = T::Mask>,
{
    fn from_color_unclamped(hsv: Hsv<S, T>) -> Self {
        let Hsv {
            hue,
            saturation,
            value,
            ..
        } = hsv;

        let x = (T::from_f64(2.0) - &saturation) * &value;
        let saturation = lazy_select! {
            if !value.is_valid_divisor() => T::zero(),
            if x.lt(&T::one()) => {
                lazy_select!{
                    if x.is_valid_divisor() => saturation.clone() * &value / &x,
                    else => T::zero(),
                }
            },
            else => {
                let denom = T::from_f64(2.0) - &x;
                lazy_select! {
                    if denom.is_valid_divisor() => saturation.clone() * &value / denom,
                    else => T::zero(),
                }
            },
        };

        Hsl {
            hue,
            saturation,
            lightness: x / T::from_f64(2.0),
            standard: PhantomData,
        }
    }
}

impl_tuple_conversion_hue!(Hsl<S> as (H, T, T), RgbHue);

impl_is_within_bounds! {
    Hsl<S> {
        saturation => [Self::min_saturation(), Self::max_saturation()],
        lightness => [Self::min_lightness(), Self::max_lightness()]
    }
    where T: Stimulus
}
impl_clamp! {
    Hsl<S> {
        saturation => [Self::min_saturation(), Self::max_saturation()],
        lightness => [Self::min_lightness(), Self::max_lightness()]
    }
    other {hue, standard}
    where T: Stimulus
}

impl_mix_hue!(Hsl<S> {saturation, lightness} phantom: standard);
impl_lighten!(Hsl<S> increase {lightness => [Self::min_lightness(), Self::max_lightness()]} other {hue, saturation} phantom: standard where T: Stimulus);
impl_saturate!(Hsl<S> increase {saturation => [Self::min_saturation(), Self::max_saturation()]} other {hue, lightness} phantom: standard where T: Stimulus);
impl_hue_ops!(Hsl<S>, RgbHue);

impl<S, T> HasBoolMask for Hsl<S, T>
where
    T: HasBoolMask,
{
    type Mask = T::Mask;
}

impl<S, T> Default for Hsl<S, T>
where
    T: Stimulus,
    RgbHue<T>: Default,
{
    fn default() -> Hsl<S, T> {
        Hsl::new(
            RgbHue::default(),
            Self::min_saturation(),
            Self::min_lightness(),
        )
    }
}

impl_color_add!(Hsl<S>, [hue, saturation, lightness], standard);
impl_color_sub!(Hsl<S>, [hue, saturation, lightness], standard);

impl_array_casts!(Hsl<S, T>, [T; 3]);
impl_simd_array_conversion_hue!(Hsl<S>, [saturation, lightness], standard);
impl_struct_of_array_traits_hue!(Hsl<S>, RgbHueIter, [saturation, lightness], standard);

impl_eq_hue!(Hsl<S>, RgbHue, [hue, saturation, lightness]);
impl_copy_clone!(Hsl<S>, [hue, saturation, lightness], standard);

#[allow(deprecated)]
impl<S, T> crate::RelativeContrast for Hsl<S, T>
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

impl_rand_traits_hsl_bicone!(
    UniformHsl,
    Hsl<S> {
        hue: UniformRgbHue => RgbHue,
        height: lightness,
        radius: saturation
    }
    phantom: standard: PhantomData<S>
);

#[cfg(feature = "bytemuck")]
unsafe impl<S, T> bytemuck::Zeroable for Hsl<S, T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<S: 'static, T> bytemuck::Pod for Hsl<S, T> where T: bytemuck::Pod {}

#[cfg(test)]
mod test {
    use super::Hsl;

    test_convert_into_from_xyz!(Hsl);

    #[cfg(feature = "approx")]
    mod conversion {
        use crate::{FromColor, Hsl, Hsv, Srgb};

        #[test]
        fn red() {
            let a = Hsl::from_color(Srgb::new(1.0, 0.0, 0.0));
            let b = Hsl::new_srgb(0.0, 1.0, 0.5);
            let c = Hsl::from_color(Hsv::new_srgb(0.0, 1.0, 1.0));

            assert_relative_eq!(a, b);
            assert_relative_eq!(a, c);
        }

        #[test]
        fn orange() {
            let a = Hsl::from_color(Srgb::new(1.0, 0.5, 0.0));
            let b = Hsl::new_srgb(30.0, 1.0, 0.5);
            let c = Hsl::from_color(Hsv::new_srgb(30.0, 1.0, 1.0));

            assert_relative_eq!(a, b);
            assert_relative_eq!(a, c);
        }

        #[test]
        fn green() {
            let a = Hsl::from_color(Srgb::new(0.0, 1.0, 0.0));
            let b = Hsl::new_srgb(120.0, 1.0, 0.5);
            let c = Hsl::from_color(Hsv::new_srgb(120.0, 1.0, 1.0));

            assert_relative_eq!(a, b);
            assert_relative_eq!(a, c);
        }

        #[test]
        fn blue() {
            let a = Hsl::from_color(Srgb::new(0.0, 0.0, 1.0));
            let b = Hsl::new_srgb(240.0, 1.0, 0.5);
            let c = Hsl::from_color(Hsv::new_srgb(240.0, 1.0, 1.0));

            assert_relative_eq!(a, b);
            assert_relative_eq!(a, c);
        }

        #[test]
        fn purple() {
            let a = Hsl::from_color(Srgb::new(0.5, 0.0, 1.0));
            let b = Hsl::new_srgb(270.0, 1.0, 0.5);
            let c = Hsl::from_color(Hsv::new_srgb(270.0, 1.0, 1.0));

            assert_relative_eq!(a, b);
            assert_relative_eq!(a, c);
        }
    }

    #[test]
    fn ranges() {
        assert_ranges! {
            Hsl<crate::encoding::Srgb, f64>;
            clamped {
                saturation: 0.0 => 1.0,
                lightness: 0.0 => 1.0
            }
            clamped_min {}
            unclamped {
                hue: -360.0 => 360.0
            }
        }
    }

    raw_pixel_conversion_tests!(Hsl<crate::encoding::Srgb>: hue, saturation, lightness);
    raw_pixel_conversion_fail_tests!(Hsl<crate::encoding::Srgb>: hue, saturation, lightness);

    #[test]
    fn check_min_max_components() {
        use crate::encoding::Srgb;

        assert_eq!(Hsl::<Srgb>::min_saturation(), 0.0);
        assert_eq!(Hsl::<Srgb>::min_lightness(), 0.0);
        assert_eq!(Hsl::<Srgb>::max_saturation(), 1.0);
        assert_eq!(Hsl::<Srgb>::max_lightness(), 1.0);
    }

    struct_of_arrays_tests!(
        Hsl<crate::encoding::Srgb>[hue, saturation, lightness] phantom: standard,
        super::Hsla::new(0.1f32, 0.2, 0.3, 0.4),
        super::Hsla::new(0.2, 0.3, 0.4, 0.5),
        super::Hsla::new(0.3, 0.4, 0.5, 0.6)
    );

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Hsl::new_srgb(0.3, 0.8, 0.1)).unwrap();

        assert_eq!(
            serialized,
            r#"{"hue":0.3,"saturation":0.8,"lightness":0.1}"#
        );
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Hsl =
            ::serde_json::from_str(r#"{"hue":0.3,"saturation":0.8,"lightness":0.1}"#).unwrap();

        assert_eq!(deserialized, Hsl::new(0.3, 0.8, 0.1));
    }

    test_uniform_distribution! {
        Hsl<crate::encoding::Srgb, f32> as crate::rgb::Rgb {
            red: (0.0, 1.0),
            green: (0.0, 1.0),
            blue: (0.0, 1.0)
        },
        min: Hsl::new(0.0f32, 0.0, 0.0),
        max: Hsl::new(360.0, 1.0, 1.0)
    }

    /// Sanity check to make sure the test doesn't start accepting known
    /// non-uniform distributions.
    #[cfg(feature = "random")]
    #[test]
    #[should_panic(expected = "is not uniform enough")]
    fn uniform_distribution_fail() {
        use rand::Rng;

        const BINS: usize = crate::random_sampling::test_utils::BINS;
        const SAMPLES: usize = crate::random_sampling::test_utils::SAMPLES;

        let mut red = [0; BINS];
        let mut green = [0; BINS];
        let mut blue = [0; BINS];

        let mut rng = rand_mt::Mt::new(1234); // We want the same seed on every run to avoid random fails

        for _ in 0..SAMPLES {
            let color = Hsl::<crate::encoding::Srgb, f32>::new(
                rng.gen::<f32>() * 360.0,
                rng.gen(),
                rng.gen(),
            );
            let color: crate::rgb::Rgb = crate::IntoColor::into_color(color);
            red[((color.red * BINS as f32) as usize).min(9)] += 1;
            green[((color.green * BINS as f32) as usize).min(9)] += 1;
            blue[((color.blue * BINS as f32) as usize).min(9)] += 1;
        }

        assert_uniform_distribution!(red);
        assert_uniform_distribution!(green);
        assert_uniform_distribution!(blue);
    }
}

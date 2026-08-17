//! Types for the CIE L\*C\*uv h°uv color space.

use core::{marker::PhantomData, ops::Mul};

use crate::{
    angle::RealAngle,
    bool_mask::{HasBoolMask, LazySelect},
    convert::FromColorUnclamped,
    hues::LuvHueIter,
    luv_bounds::LuvBounds,
    num::{Arithmetics, Hypot, PartialCmp, Powi, Real, Zero},
    white_point::D65,
    Alpha, FromColor, GetHue, Hsluv, Luv, LuvHue, Xyz,
};

/// CIE L\*C\*uv h°uv with an alpha component. See the [`Lchuva` implementation in
/// `Alpha`](crate::Alpha#Lchuva).
pub type Lchuva<Wp = D65, T = f32> = Alpha<Lchuv<Wp, T>, T>;

/// CIE L\*C\*uv h°uv, a polar version of [CIE L\*u\*v\*](crate::Luv).
///
/// L\*C\*uv h°uv shares its range and perceptual uniformity with L\*u\*v\*, but
/// it's a cylindrical color space, like [HSL](crate::Hsl) and
/// [HSV](crate::Hsv). This gives it the same ability to directly change
/// the hue and colorfulness of a color, while preserving other visual aspects.
#[derive(Debug, ArrayCast, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    white_point = "Wp",
    component = "T",
    skip_derives(Luv, Lchuv, Hsluv)
)]
#[repr(C)]
pub struct Lchuv<Wp = D65, T = f32> {
    /// L\* is the lightness of the color. 0.0 gives absolute black and 100.0
    /// gives the brightest white.
    pub l: T,

    /// C\*uv is the colorfulness of the color. It's similar to
    /// saturation. 0.0 gives gray scale colors, and numbers around
    /// 130-180 gives fully saturated colors, depending on the
    /// hue. The upper limit of 180 should include the whole
    /// L\*u\*v\*.
    pub chroma: T,

    /// The hue of the color, in degrees. Decides if it's red, blue, purple,
    /// etc.
    #[palette(unsafe_same_layout_as = "T")]
    pub hue: LuvHue<T>,

    /// The white point associated with the color's illuminant and observer.
    /// D65 for 2 degree observer is used by default.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette(unsafe_zero_sized)]
    pub white_point: PhantomData<Wp>,
}

impl<Wp, T> Lchuv<Wp, T> {
    /// Create a CIE L\*C\*uv h°uv color.
    pub fn new<H: Into<LuvHue<T>>>(l: T, chroma: T, hue: H) -> Self {
        Self::new_const(l, chroma, hue.into())
    }

    /// Create a CIE L\*C\*uv h°uv color. This is the same as `Lchuv::new`
    /// without the generic hue type. It's temporary until `const fn` supports
    /// traits.
    pub const fn new_const(l: T, chroma: T, hue: LuvHue<T>) -> Self {
        Lchuv {
            l,
            chroma,
            hue,
            white_point: PhantomData,
        }
    }

    /// Convert to a `(L\*, C\*uv, h°uv)` tuple.
    pub fn into_components(self) -> (T, T, LuvHue<T>) {
        (self.l, self.chroma, self.hue)
    }

    /// Convert from a `(L\*, C\*uv, h°uv)` tuple.
    pub fn from_components<H: Into<LuvHue<T>>>((l, chroma, hue): (T, T, H)) -> Self {
        Self::new(l, chroma, hue)
    }
}

impl<Wp, T> Lchuv<Wp, T>
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

    /// Return the `chroma` value minimum.
    pub fn min_chroma() -> T {
        T::zero()
    }

    /// Return the `chroma` value maximum.
    pub fn max_chroma() -> T {
        T::from_f64(180.0)
    }
}

///<span id="Lchuva"></span>[`Lchuva`](crate::Lchuva) implementations.
impl<Wp, T, A> Alpha<Lchuv<Wp, T>, A> {
    /// Create a CIE L\*C\*uv h°uv color with transparency.
    pub fn new<H: Into<LuvHue<T>>>(l: T, chroma: T, hue: H, alpha: A) -> Self {
        Self::new_const(l, chroma, hue.into(), alpha)
    }

    /// Create a CIE L\*C\*uv h°uv color with transparency. This is the same as
    /// `Lchuva::new` without the generic hue type. It's temporary until `const
    /// fn` supports traits.
    pub const fn new_const(l: T, chroma: T, hue: LuvHue<T>, alpha: A) -> Self {
        Alpha {
            color: Lchuv::new_const(l, chroma, hue),
            alpha,
        }
    }

    /// Convert to a `(L\*, C\*uv, h°uv, alpha)` tuple.
    pub fn into_components(self) -> (T, T, LuvHue<T>, A) {
        (self.color.l, self.color.chroma, self.color.hue, self.alpha)
    }

    /// Convert from a `(L\*, C\*uv, h°uv, alpha)` tuple.
    pub fn from_components<H: Into<LuvHue<T>>>((l, chroma, hue, alpha): (T, T, H, A)) -> Self {
        Self::new(l, chroma, hue, alpha)
    }
}

impl_reference_component_methods_hue!(Lchuv<Wp>, [l, chroma], white_point);
impl_struct_of_arrays_methods_hue!(Lchuv<Wp>, [l, chroma], white_point);

impl<Wp, T> FromColorUnclamped<Lchuv<Wp, T>> for Lchuv<Wp, T> {
    fn from_color_unclamped(color: Lchuv<Wp, T>) -> Self {
        color
    }
}

impl<Wp, T> FromColorUnclamped<Luv<Wp, T>> for Lchuv<Wp, T>
where
    T: Zero + Hypot,
    Luv<Wp, T>: GetHue<Hue = LuvHue<T>>,
{
    fn from_color_unclamped(color: Luv<Wp, T>) -> Self {
        Lchuv {
            hue: color.get_hue(),
            l: color.l,
            chroma: color.u.hypot(color.v),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> FromColorUnclamped<Hsluv<Wp, T>> for Lchuv<Wp, T>
where
    T: Real + RealAngle + Into<f64> + Powi + Mul<Output = T> + Clone,
{
    fn from_color_unclamped(color: Hsluv<Wp, T>) -> Self {
        // Apply the given saturation as a percentage of the max
        // chroma for that hue.
        let max_chroma =
            LuvBounds::from_lightness(color.l.clone()).max_chroma_at_hue(color.hue.clone());

        Lchuv::new(
            color.l,
            color.saturation * max_chroma * T::from_f64(0.01),
            color.hue,
        )
    }
}

impl_tuple_conversion_hue!(Lchuv<Wp> as (T, T, H), LuvHue);

impl_is_within_bounds! {
    Lchuv<Wp> {
        l => [Self::min_l(), Self::max_l()],
        chroma => [Self::min_chroma(), Self::max_chroma()]
    }
    where T: Real + Zero
}
impl_clamp! {
    Lchuv<Wp> {
        l => [Self::min_l(), Self::max_l()],
        chroma => [Self::min_chroma(), Self::max_chroma()]
    }
    other {hue, white_point}
    where T: Real + Zero
}

impl_mix_hue!(Lchuv<Wp> {l, chroma} phantom: white_point);
impl_lighten!(Lchuv<Wp> increase {l => [Self::min_l(), Self::max_l()]} other {hue, chroma} phantom: white_point);
impl_saturate!(Lchuv<Wp> increase {chroma => [Self::min_chroma(), Self::max_chroma()]} other {hue, l} phantom: white_point);
impl_hue_ops!(Lchuv<Wp>, LuvHue);

impl<Wp, T> HasBoolMask for Lchuv<Wp, T>
where
    T: HasBoolMask,
{
    type Mask = T::Mask;
}

impl<Wp, T> Default for Lchuv<Wp, T>
where
    T: Zero + Real,
    LuvHue<T>: Default,
{
    fn default() -> Lchuv<Wp, T> {
        Lchuv::new(Self::min_l(), Self::min_chroma(), LuvHue::default())
    }
}

impl_color_add!(Lchuv<Wp>, [l, chroma, hue], white_point);
impl_color_sub!(Lchuv<Wp>, [l, chroma, hue], white_point);

impl_array_casts!(Lchuv<Wp, T>, [T; 3]);
impl_simd_array_conversion_hue!(Lchuv<Wp>, [l, chroma], white_point);
impl_struct_of_array_traits_hue!(Lchuv<Wp>, LuvHueIter, [l, chroma], white_point);

impl_eq_hue!(Lchuv<Wp>, LuvHue, [l, chroma, hue]);
impl_copy_clone!(Lchuv<Wp>, [l, chroma, hue], white_point);

#[allow(deprecated)]
impl<Wp, T> crate::RelativeContrast for Lchuv<Wp, T>
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

impl_rand_traits_cylinder!(
    UniformLchuv,
    Lchuv<Wp> {
        hue: UniformLuvHue => LuvHue,
        height: l => [|l: T| l * Lchuv::<Wp, T>::max_l()],
        radius: chroma => [|chroma| chroma *  Lchuv::<Wp, T>::max_chroma()]
    }
    phantom: white_point: PhantomData<Wp>
    where T: Real + Zero + core::ops::Mul<Output = T>,
);

#[cfg(feature = "bytemuck")]
unsafe impl<Wp, T> bytemuck::Zeroable for Lchuv<Wp, T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<Wp: 'static, T> bytemuck::Pod for Lchuv<Wp, T> where T: bytemuck::Pod {}

#[cfg(test)]
mod test {
    use crate::white_point::D65;
    use crate::Lchuv;

    test_convert_into_from_xyz!(Lchuv);

    #[test]
    fn ranges() {
        assert_ranges! {
            Lchuv<D65, f64>;
            clamped {
                l: 0.0 => 100.0,
                chroma: 0.0 => 180.0
            }
            clamped_min {
            }
            unclamped {
                hue: -360.0 => 360.0
            }
        }
    }

    /// Check that the arithmetic operations (add/sub) are all
    /// implemented.
    #[test]
    fn test_arithmetic() {
        let lchuv = Lchuv::<D65>::new(120.0, 40.0, 30.0);
        let lchuv2 = Lchuv::new(200.0, 30.0, 40.0);
        let mut _lchuv3 = lchuv + lchuv2;
        _lchuv3 += lchuv2;
        let mut _lchuv4 = lchuv2 + 0.3;
        _lchuv4 += 0.1;

        _lchuv3 = lchuv2 - lchuv;
        _lchuv3 = _lchuv4 - 0.1;
        _lchuv4 -= _lchuv3;
        _lchuv3 -= 0.1;
    }

    raw_pixel_conversion_tests!(Lchuv<D65>: l, chroma, hue);
    raw_pixel_conversion_fail_tests!(Lchuv<D65>: l, chroma, hue);

    #[test]
    fn check_min_max_components() {
        assert_eq!(Lchuv::<D65, f32>::min_l(), 0.0);
        assert_eq!(Lchuv::<D65, f32>::max_l(), 100.0);
        assert_eq!(Lchuv::<D65, f32>::min_chroma(), 0.0);
        assert_eq!(Lchuv::<D65, f32>::max_chroma(), 180.0);
    }

    struct_of_arrays_tests!(
        Lchuv<D65>[l, chroma, hue] phantom: white_point,
        super::Lchuva::new(0.1f32, 0.2, 0.3, 0.4),
        super::Lchuva::new(0.2, 0.3, 0.4, 0.5),
        super::Lchuva::new(0.3, 0.4, 0.5, 0.6)
    );

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Lchuv::<D65>::new(80.0, 70.0, 130.0)).unwrap();

        assert_eq!(serialized, r#"{"l":80.0,"chroma":70.0,"hue":130.0}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Lchuv =
            ::serde_json::from_str(r#"{"l":70.0,"chroma":80.0,"hue":130.0}"#).unwrap();

        assert_eq!(deserialized, Lchuv::new(70.0, 80.0, 130.0));
    }

    test_uniform_distribution! {
        Lchuv<D65, f32> as crate::Luv {
            l: (0.0, 100.0),
            u: (-80.0, 80.0),
            v: (-80.0, 80.0),
        },
        min: Lchuv::new(0.0f32, 0.0, 0.0),
        max: Lchuv::new(100.0, 180.0, 360.0)
    }
}

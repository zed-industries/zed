//! Types for the HSLuv color space.

use core::marker::PhantomData;

use crate::{
    angle::RealAngle,
    bool_mask::{HasBoolMask, LazySelect},
    convert::FromColorUnclamped,
    hues::LuvHueIter,
    luv_bounds::LuvBounds,
    num::{Arithmetics, PartialCmp, Powi, Real, Zero},
    white_point::D65,
    Alpha, FromColor, Lchuv, LuvHue, Xyz,
};

/// HSLuv with an alpha component. See the [`Hsluva` implementation in
/// `Alpha`](crate::Alpha#Hsluva).
pub type Hsluva<Wp = D65, T = f32> = Alpha<Hsluv<Wp, T>, T>;

/// HSLuv color space.
///
/// The HSLuv color space can be seen as a cylindrical version of
/// [CIELUV](crate::luv::Luv), similar to
/// [LCHuv](crate::lchuv::Lchuv), with the additional benefit of
/// stretching the chroma values to a uniform saturation range [0.0,
/// 100.0]. This makes HSLuv much more convenient for generating
/// colors than Lchuv, as the set of valid saturation values is
/// independent of lightness and hue.
#[derive(Debug, ArrayCast, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    white_point = "Wp",
    component = "T",
    skip_derives(Lchuv, Hsluv)
)]
#[repr(C)]
pub struct Hsluv<Wp = D65, T = f32> {
    /// The hue of the color, in degrees. Decides if it's red, blue, purple,
    /// etc.
    #[palette(unsafe_same_layout_as = "T")]
    pub hue: LuvHue<T>,

    /// The colorfulness of the color, as a percentage of the maximum
    /// available chroma. 0.0 gives gray scale colors and 100.0 will
    /// give absolutely clear colors.
    pub saturation: T,

    /// Decides how light the color will look. 0.0 will be black, 50.0 will give
    /// a clear color, and 100.0 will give white.
    pub l: T,

    /// The white point and RGB primaries this color is adapted to. The default
    /// is the sRGB standard.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette(unsafe_zero_sized)]
    pub white_point: PhantomData<Wp>,
}

impl<Wp, T> Hsluv<Wp, T> {
    /// Create an HSLuv color.
    pub fn new<H: Into<LuvHue<T>>>(hue: H, saturation: T, l: T) -> Self {
        Self::new_const(hue.into(), saturation, l)
    }

    /// Create an HSLuv color. This is the same as `Hsluv::new` without the
    /// generic hue type. It's temporary until `const fn` supports traits.
    pub const fn new_const(hue: LuvHue<T>, saturation: T, l: T) -> Self {
        Hsluv {
            hue,
            saturation,
            l,
            white_point: PhantomData,
        }
    }

    /// Convert to a `(hue, saturation, l)` tuple.
    pub fn into_components(self) -> (LuvHue<T>, T, T) {
        (self.hue, self.saturation, self.l)
    }

    /// Convert from a `(hue, saturation, l)` tuple.
    pub fn from_components<H: Into<LuvHue<T>>>((hue, saturation, l): (H, T, T)) -> Self {
        Self::new(hue, saturation, l)
    }
}

impl<Wp, T> Hsluv<Wp, T>
where
    T: Zero + Real,
{
    /// Return the `saturation` value minimum.
    pub fn min_saturation() -> T {
        T::zero()
    }

    /// Return the `saturation` value maximum.
    pub fn max_saturation() -> T {
        T::from_f64(100.0)
    }

    /// Return the `l` value minimum.
    pub fn min_l() -> T {
        T::zero()
    }

    /// Return the `l` value maximum.
    pub fn max_l() -> T {
        T::from_f64(100.0)
    }
}

///<span id="Hsluva"></span>[`Hsluva`](crate::Hsluva) implementations.
impl<Wp, T, A> Alpha<Hsluv<Wp, T>, A> {
    /// Create an HSLuv color with transparency.
    pub fn new<H: Into<LuvHue<T>>>(hue: H, saturation: T, l: T, alpha: A) -> Self {
        Self::new_const(hue.into(), saturation, l, alpha)
    }

    /// Create an HSLuv color with transparency. This is the same as
    /// `Hsluva::new` without the generic hue type. It's temporary until `const
    /// fn` supports traits.
    pub const fn new_const(hue: LuvHue<T>, saturation: T, l: T, alpha: A) -> Self {
        Alpha {
            color: Hsluv::new_const(hue, saturation, l),
            alpha,
        }
    }

    /// Convert to a `(hue, saturation, l, alpha)` tuple.
    pub fn into_components(self) -> (LuvHue<T>, T, T, A) {
        (
            self.color.hue,
            self.color.saturation,
            self.color.l,
            self.alpha,
        )
    }

    /// Convert from a `(hue, saturation, l, alpha)` tuple.
    pub fn from_components<H: Into<LuvHue<T>>>((hue, saturation, l, alpha): (H, T, T, A)) -> Self {
        Self::new(hue, saturation, l, alpha)
    }
}

impl_reference_component_methods_hue!(Hsluv<Wp>, [saturation, l], white_point);
impl_struct_of_arrays_methods_hue!(Hsluv<Wp>, [saturation, l], white_point);

impl<Wp, T> FromColorUnclamped<Hsluv<Wp, T>> for Hsluv<Wp, T> {
    fn from_color_unclamped(hsluv: Hsluv<Wp, T>) -> Self {
        hsluv
    }
}

impl<Wp, T> FromColorUnclamped<Lchuv<Wp, T>> for Hsluv<Wp, T>
where
    T: Real + RealAngle + Into<f64> + Powi + Arithmetics + Clone,
{
    fn from_color_unclamped(color: Lchuv<Wp, T>) -> Self {
        // convert the chroma to a saturation based on the max
        // saturation at a particular hue.
        let max_chroma =
            LuvBounds::from_lightness(color.l.clone()).max_chroma_at_hue(color.hue.clone());

        Hsluv::new(
            color.hue,
            color.chroma / max_chroma * T::from_f64(100.0),
            color.l,
        )
    }
}

impl_tuple_conversion_hue!(Hsluv<Wp> as (H, T, T), LuvHue);

impl_is_within_bounds! {
    Hsluv<Wp> {
        saturation => [Self::min_saturation(), Self::max_saturation()],
        l => [Self::min_l(), Self::max_l()]
    }
    where T: Real + Zero
}
impl_clamp! {
    Hsluv<Wp> {
        saturation => [Self::min_saturation(), Self::max_saturation()],
        l => [Self::min_l(), Self::max_l()]
    }
    other {hue, white_point}
    where T: Real + Zero
}

impl_mix_hue!(Hsluv<Wp> {saturation, l} phantom: white_point);
impl_lighten!(Hsluv<Wp> increase {l => [Self::min_l(), Self::max_l()]} other {hue, saturation} phantom: white_point);
impl_saturate!(Hsluv<Wp> increase {saturation => [Self::min_saturation(), Self::max_saturation()]} other {hue, l} phantom: white_point);
impl_hue_ops!(Hsluv<Wp>, LuvHue);

impl<Wp, T> HasBoolMask for Hsluv<Wp, T>
where
    T: HasBoolMask,
{
    type Mask = T::Mask;
}

impl<Wp, T> Default for Hsluv<Wp, T>
where
    T: Real + Zero,
    LuvHue<T>: Default,
{
    fn default() -> Hsluv<Wp, T> {
        Hsluv::new(LuvHue::default(), Self::min_saturation(), Self::min_l())
    }
}

impl_color_add!(Hsluv<Wp>, [hue, saturation, l], white_point);
impl_color_sub!(Hsluv<Wp>, [hue, saturation, l], white_point);

impl_array_casts!(Hsluv<Wp, T>, [T; 3]);
impl_simd_array_conversion_hue!(Hsluv<Wp>, [saturation, l], white_point);
impl_struct_of_array_traits_hue!(Hsluv<Wp>, LuvHueIter, [saturation, l], white_point);

impl_eq_hue!(Hsluv<Wp>, LuvHue, [hue, saturation, l]);
impl_copy_clone!(Hsluv<Wp>, [hue, saturation, l], white_point);

#[allow(deprecated)]
impl<Wp, T> crate::RelativeContrast for Hsluv<Wp, T>
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

impl_rand_traits_hsl_bicone!(
    UniformHsluv,
    Hsluv<Wp> {
        hue: UniformLuvHue => LuvHue,
        height: l => [|l: T| l * T::from_f64(100.0), |l: T| l / T::from_f64(100.0)],
        radius: saturation => [|s: T| s * T::from_f64(100.0), |s: T| s / T::from_f64(100.0)]
    }
    phantom: white_point: PhantomData<Wp>
);

#[cfg(feature = "bytemuck")]
unsafe impl<Wp, T> bytemuck::Zeroable for Hsluv<Wp, T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<Wp: 'static, T> bytemuck::Pod for Hsluv<Wp, T> where T: bytemuck::Pod {}

#[cfg(test)]
mod test {
    use super::Hsluv;
    use crate::white_point::D65;

    test_convert_into_from_xyz!(Hsluv);

    #[cfg(feature = "approx")]
    #[cfg_attr(miri, ignore)]
    #[test]
    fn lchuv_round_trip() {
        use crate::{FromColor, Lchuv, LuvHue};

        for hue in (0..=20).map(|x| x as f64 * 18.0) {
            for sat in (0..=20).map(|x| x as f64 * 5.0) {
                for l in (1..=20).map(|x| x as f64 * 5.0) {
                    let hsluv = Hsluv::<D65, _>::new(hue, sat, l);
                    let lchuv = Lchuv::from_color(hsluv);
                    let mut to_hsluv = Hsluv::from_color(lchuv);
                    if to_hsluv.l < 1e-8 {
                        to_hsluv.hue = LuvHue::from(0.0);
                    }
                    assert_relative_eq!(hsluv, to_hsluv, epsilon = 1e-5);
                }
            }
        }
    }

    #[test]
    fn ranges() {
        assert_ranges! {
            Hsluv<D65, f64>;
            clamped {
                saturation: 0.0 => 100.0,
                l: 0.0 => 100.0
            }
            clamped_min {}
            unclamped {
                hue: -360.0 => 360.0
            }
        }
    }

    /// Check that the arithmetic operations (add/sub) are all
    /// implemented.
    #[test]
    fn test_arithmetic() {
        let hsl = Hsluv::<D65>::new(120.0, 40.0, 30.0);
        let hsl2 = Hsluv::new(200.0, 30.0, 40.0);
        let mut _hsl3 = hsl + hsl2;
        _hsl3 += hsl2;
        let mut _hsl4 = hsl2 + 0.3;
        _hsl4 += 0.1;

        _hsl3 = hsl2 - hsl;
        _hsl3 = _hsl4 - 0.1;
        _hsl4 -= _hsl3;
        _hsl3 -= 0.1;
    }

    #[cfg(feature = "approx")]
    #[test]
    fn saturate() {
        use crate::Saturate;

        for sat in (0..=10).map(|s| s as f64 * 10.0) {
            for a in (0..=10).map(|l| l as f64 * 10.0) {
                let hsl = Hsluv::<D65, _>::new(150.0, sat, a);
                let hsl_sat_fixed = hsl.saturate_fixed(0.1);
                let expected_sat_fixed = Hsluv::new(150.0, (sat + 10.0).min(100.0), a);
                assert_relative_eq!(hsl_sat_fixed, expected_sat_fixed);

                let hsl_sat = hsl.saturate(0.1);
                let expected_sat = Hsluv::new(150.0, (sat + (100.0 - sat) * 0.1).min(100.0), a);
                assert_relative_eq!(hsl_sat, expected_sat);
            }
        }
    }

    raw_pixel_conversion_tests!(Hsluv<D65>: hue, saturation, lightness);
    raw_pixel_conversion_fail_tests!(Hsluv<D65>: hue, saturation, lightness);

    #[test]
    fn check_min_max_components() {
        assert_eq!(Hsluv::<D65>::min_saturation(), 0.0);
        assert_eq!(Hsluv::<D65>::min_l(), 0.0);
        assert_eq!(Hsluv::<D65>::max_saturation(), 100.0);
        assert_eq!(Hsluv::<D65>::max_l(), 100.0);
    }

    struct_of_arrays_tests!(
        Hsluv<D65>[hue, saturation, l] phantom: white_point,
        super::Hsluva::new(0.1f32, 0.2, 0.3, 0.4),
        super::Hsluva::new(0.2, 0.3, 0.4, 0.5),
        super::Hsluva::new(0.3, 0.4, 0.5, 0.6)
    );

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Hsluv::<D65>::new(120.0, 80.0, 60.0)).unwrap();

        assert_eq!(serialized, r#"{"hue":120.0,"saturation":80.0,"l":60.0}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Hsluv =
            ::serde_json::from_str(r#"{"hue":120.0,"saturation":80.0,"l":60.0}"#).unwrap();

        assert_eq!(deserialized, Hsluv::new(120.0, 80.0, 60.0));
    }
}

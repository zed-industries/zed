use crate::{hues::OklabHueIter, white_point::D65};

use crate::{
    bool_mask::LazySelect,
    num::{Arithmetics, PartialCmp, Real},
    stimulus::Stimulus,
    FromColor, OklabHue, Xyz,
};

use super::Okhsl;

impl_is_within_bounds! {
    Okhsl {
        saturation => [Self::min_saturation(), Self::max_saturation()],
        lightness => [Self::min_lightness(), Self::max_lightness()]
    }
    where T: Stimulus
}
impl_clamp! {
    Okhsl {
        saturation => [Self::min_saturation(), Self::max_saturation()],
        lightness => [Self::min_lightness(), Self::max_lightness()]
    }
    other {hue}
    where T: Stimulus
}

impl_mix_hue!(Okhsl {
    saturation,
    lightness
});
impl_lighten!(Okhsl increase {lightness => [Self::min_lightness(), Self::max_lightness()]} other {hue, saturation}  where T: Stimulus);
impl_saturate!(Okhsl increase {saturation => [Self::min_saturation(), Self::max_saturation()]} other {hue, lightness}  where T: Stimulus);
impl_hue_ops!(Okhsl, OklabHue);

impl_color_add!(Okhsl, [hue, saturation, lightness]);
impl_color_sub!(Okhsl, [hue, saturation, lightness]);

impl_array_casts!(Okhsl<T>, [T; 3]);
impl_simd_array_conversion_hue!(Okhsl, [saturation, lightness]);
impl_struct_of_array_traits_hue!(Okhsl, OklabHueIter, [saturation, lightness]);

impl_eq_hue!(Okhsl, OklabHue, [hue, saturation, lightness]);

#[allow(deprecated)]
impl<T> crate::RelativeContrast for Okhsl<T>
where
    T: Real + Arithmetics + PartialCmp,
    T::Mask: LazySelect<T>,
    Xyz<D65, T>: FromColor<Self>,
{
    type Scalar = T;

    #[inline]
    fn get_contrast_ratio(self, other: Self) -> T {
        let xyz1 = Xyz::from_color(self);
        let xyz2 = Xyz::from_color(other);

        crate::contrast_ratio(xyz1.y, xyz2.y)
    }
}

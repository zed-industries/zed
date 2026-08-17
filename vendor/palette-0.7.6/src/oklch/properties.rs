use crate::{
    bool_mask::LazySelect,
    hues::OklabHueIter,
    num::{Arithmetics, One, PartialCmp, Real, Zero},
    white_point::D65,
    FromColor, OklabHue, Xyz,
};

use super::Oklch;

impl_is_within_bounds! {
    Oklch {
        l => [Self::min_l(), Self::max_l()],
        chroma => [Self::min_chroma(), None]
    }
    where T: Zero + One
}
impl_clamp! {
    Oklch {
        l => [Self::min_l(), Self::max_l()],
        chroma => [Self::min_chroma()]
    }
    other {hue}
    where T: Zero + One
}

impl_mix_hue!(Oklch { l, chroma });
impl_lighten!(Oklch increase {l => [Self::min_l(), Self::max_l()]} other {hue, chroma} where T: Zero + One);
impl_hue_ops!(Oklch, OklabHue);

impl_color_add!(Oklch, [l, chroma, hue]);
impl_color_sub!(Oklch, [l, chroma, hue]);

impl_array_casts!(Oklch<T>, [T; 3]);
impl_simd_array_conversion_hue!(Oklch, [l, chroma]);
impl_struct_of_array_traits_hue!(Oklch, OklabHueIter, [l, chroma]);

impl_eq_hue!(Oklch, OklabHue, [l, chroma, hue]);

#[allow(deprecated)]
impl<T> crate::RelativeContrast for Oklch<T>
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

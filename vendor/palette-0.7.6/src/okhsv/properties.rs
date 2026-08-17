use crate::hues::OklabHueIter;
use crate::num::{Arithmetics, Real};

use crate::stimulus::Stimulus;
use crate::{ok_utils, OklabHue};

use super::Okhsv;

impl_is_within_bounds! {
    Okhsv {
        saturation => [Self::min_saturation(), Self::max_saturation()+ T::from_f64(ok_utils::MAX_SRGB_SATURATION_INACCURACY)],
        value => [Self::min_value(), Self::max_value()+ T::from_f64(ok_utils::MAX_SRGB_SATURATION_INACCURACY)]
    }
    where T: Real+Arithmetics+Stimulus
}

impl_clamp! {
    Okhsv {
        saturation => [Self::min_saturation(), Self::max_saturation()+ T::from_f64(ok_utils::MAX_SRGB_SATURATION_INACCURACY)],
        value => [Self::min_value(), Self::max_value()+ T::from_f64(ok_utils::MAX_SRGB_SATURATION_INACCURACY)]
    }
    other {hue}
    where T: Real+Arithmetics+Stimulus
}

impl_mix_hue!(Okhsv { saturation, value });
impl_lighten!(Okhsv increase {value => [Self::min_value(), Self::max_value()]} other {hue, saturation}  where T: Real+Stimulus);
impl_saturate!(Okhsv increase {saturation => [Self::min_saturation(), Self::max_saturation()]} other {hue, value}  where T:Real+ Stimulus);
impl_hue_ops!(Okhsv, OklabHue);

impl_color_add!(Okhsv, [hue, saturation, value]);
impl_color_sub!(Okhsv, [hue, saturation, value]);

impl_array_casts!(Okhsv<T>, [T; 3]);
impl_simd_array_conversion_hue!(Okhsv, [saturation, value]);
impl_struct_of_array_traits_hue!(Okhsv, OklabHueIter, [saturation, value]);

impl_eq_hue!(Okhsv, OklabHue, [hue, saturation, value]);

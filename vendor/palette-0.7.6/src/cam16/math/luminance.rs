use crate::{
    bool_mask::LazySelect,
    cam16::BakedParameters,
    num::{Arithmetics, FromScalar, PartialCmp, Real, Sqrt, Zero},
};
/// One the apparent luminance metrics of CAM16.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub(crate) enum LuminanceType<T> {
    /// The [lightness](https://en.wikipedia.org/wiki/Lightness) (J) of a color.
    Lightness(T),

    /// The [brightness](https://en.wikipedia.org/wiki/Brightness) (Q) of a
    /// color.
    Brightness(T),
}

impl<T> LuminanceType<T> {
    pub(crate) fn into_cam16<Wp>(self, parameters: BakedParameters<Wp, T::Scalar>) -> (T, T)
    where
        T: Real + FromScalar + Zero + Arithmetics + Sqrt + PartialCmp + Clone,
        T::Mask: LazySelect<T> + Clone,
    {
        let parameters = parameters.inner;

        match self {
            LuminanceType::Lightness(lightness) => {
                let is_black = lightness.eq(&T::zero());
                let brightness = lazy_select! {
                        if is_black => T::zero(),
                        else => crate::cam16::math::lightness_to_brightness(
                        lightness.clone(),
                        T::from_scalar(parameters.c),
                        T::from_scalar(parameters.a_w),
                        T::from_scalar(parameters.f_l_4),
                    )
                };

                (lightness, brightness)
            }
            LuminanceType::Brightness(brightness) => {
                let is_black = brightness.eq(&T::zero());
                let lightness = lazy_select! {
                        if is_black => T::zero(),
                        else => crate::cam16::math::brightness_to_lightness(
                        brightness.clone(),
                        T::from_scalar(parameters.c),
                        T::from_scalar(parameters.a_w),
                        T::from_scalar(parameters.f_l_4),
                    )
                };

                (lightness, brightness)
            }
        }
    }
}

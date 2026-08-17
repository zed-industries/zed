use crate::{
    bool_mask::{LazySelect, Select},
    cam16::{
        math::{self, DependentParameters},
        BakedParameters,
    },
    num::{Arithmetics, FromScalar, PartialCmp, Real, Sqrt, Zero},
};

/// One the apparent chromatic intensity metrics of CAM16.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum ChromaticityType<T> {
    /// The [chroma](https://en.wikipedia.org/wiki/Colorfulness#Chroma) (C) of a
    /// color.
    Chroma(T),

    /// The [colorfulness](https://en.wikipedia.org/wiki/Colorfulness) (M) of a
    /// color.
    Colorfulness(T),

    /// The [saturation](https://en.wikipedia.org/wiki/Colorfulness#Saturation)
    /// (s) of a color.
    Saturation(T),
}

impl<T> ChromaticityType<T> {
    pub(crate) fn into_cam16<Wp>(
        self,
        lightness: T,
        parameters: BakedParameters<Wp, T::Scalar>,
    ) -> (T, T, T)
    where
        T: Real + FromScalar + Zero + Arithmetics + Sqrt + PartialCmp + Clone,
        T::Mask: LazySelect<T> + Clone,
    {
        let DependentParameters { c, a_w, f_l_4, .. } = parameters.inner;
        let is_black = lightness.eq(&T::zero());

        match self {
            ChromaticityType::Chroma(chroma) => {
                let colorfulness = lazy_select! {
                    if is_black.clone() => T::zero(),
                    else => math::chroma_to_colorfulness(chroma.clone(), T::from_scalar(f_l_4))
                };
                let saturation = lazy_select! {
                        if is_black.clone() => T::zero(),
                        else => math::chroma_to_saturation(
                        chroma.clone(),
                        lightness,
                        T::from_scalar(c),
                        T::from_scalar(a_w),
                    )
                };
                let chroma = is_black.select(T::zero(), chroma);

                (chroma, colorfulness, saturation)
            }
            ChromaticityType::Colorfulness(colorfulness) => {
                let chroma = lazy_select! {
                    if is_black.clone() => T::zero(),
                    else => math::colorfulness_to_chroma(colorfulness.clone(), T::from_scalar(f_l_4))
                };
                let saturation = lazy_select! {
                        if is_black.clone() => T::zero(),
                        else => math::chroma_to_saturation(
                        chroma.clone(),
                        lightness,
                        T::from_scalar(c),
                        T::from_scalar(a_w),
                    )
                };
                let colorfulness = is_black.select(T::zero(), colorfulness);

                (chroma, colorfulness, saturation)
            }
            ChromaticityType::Saturation(saturation) => {
                let chroma = lazy_select! {
                        if is_black.clone() => T::zero(),
                        else => math::saturation_to_chroma(
                        saturation.clone(),
                        lightness,
                        T::from_scalar(c),
                        T::from_scalar(a_w),
                    )
                };
                let colorfulness = lazy_select! {
                    if is_black.clone() => T::zero(),
                    else => math::chroma_to_colorfulness(chroma.clone(), T::from_scalar(f_l_4))
                };
                let saturation = is_black.select(T::zero(), saturation);

                (chroma, colorfulness, saturation)
            }
        }
    }
}

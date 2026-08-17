use crate::{Okhsl, OklabHue};

impl_rand_traits_hsl_bicone!(
    UniformOkhsl,
    Okhsl {
        hue: UniformOklabHue => OklabHue,
        height: lightness,
        radius: saturation
    }
);

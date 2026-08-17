use super::Oklch;

use crate::OklabHue;

impl_rand_traits_cylinder!(
    UniformOklch,
    Oklch {
        hue: UniformOklabHue => OklabHue,
        height: l,
        radius: chroma // FIXME: Same as with Oklab: The limit of chroma has no meaning
    }
);

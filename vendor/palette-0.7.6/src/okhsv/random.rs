use crate::{Okhsv, OklabHue};

impl_rand_traits_hsv_cone!(
    UniformOkhsv,
    Okhsv {
        hue: UniformOklabHue => OklabHue,
        height: value,
        radius: saturation
    }
);

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Zeroable for Okhsv<T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Pod for Okhsv<T> where T: bytemuck::Pod {}

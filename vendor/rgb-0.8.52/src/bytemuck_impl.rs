use crate::{Abgr, Argb, Bgr, Bgra, Grb, Rgb, Rgba};
use crate::formats::gray_a::GrayA;
use crate::formats::gray_alpha::GrayAlpha_v08;
use crate::formats::gray::Gray_v08;

#[cfg(feature = "unstable-experimental")]
use crate::formats::gray::Gray_v09;

macro_rules! bytemuck {
    ($name:ident) => {
        unsafe impl<T: ::bytemuck::Zeroable> ::bytemuck::Zeroable for $name<T> {}
        unsafe impl<T: ::bytemuck::Pod> ::bytemuck::Pod for $name<T> {}
    };
}

bytemuck!(Rgb);
bytemuck!(Bgr);
bytemuck!(Grb);
bytemuck!(Rgba);
bytemuck!(Argb);
bytemuck!(Bgra);
bytemuck!(Abgr);
bytemuck!(GrayA);

bytemuck!(GrayAlpha_v08);
bytemuck!(Gray_v08);

#[cfg(feature = "unstable-experimental")]
bytemuck!(Gray_v09);

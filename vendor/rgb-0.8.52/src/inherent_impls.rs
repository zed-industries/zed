use crate::{Abgr, Argb, Bgr, Bgra, Grb, Rgb, Rgba};
use crate::formats::gray_a::GrayA;
use crate::formats::gray::Gray_v08;
#[cfg(feature = "unstable-experimental")]
use crate::formats::gray::Gray_v09;
use crate::formats::gray_alpha::GrayAlpha_v08;

macro_rules! inherent_impls {
    ($name:ident, $new_fn:ident, [$($field:tt $var:ident),*]) => {
        impl<T: Copy> $name<T> {
            #[doc=concat!("Creates a new [`", stringify!($name), "`] pixel type from its components.")]
            ///
            /// Alternatively, you can use struct literal syntax to
            /// create the new pixel type:
            ///```not_rust
            #[doc=concat!("use rgb::", stringify!($name), ";")]
            ///
            #[doc=concat!("let pixel = ", stringify!($name), " {", stringify!($($field: $var),*), "};")]
            ///```
            #[allow(deprecated)]
            pub const fn $new_fn($($var: T),*) -> Self {
                Self {$($field: $var),*}
            }
        }
    }
}

inherent_impls!(Rgb, new, [r red, g green, b blue]);
inherent_impls!(Bgr, new_bgr, [b blue, g green, r red]);
inherent_impls!(Grb, new_grb, [g green, r red, b blue]);

inherent_impls!(Gray_v08, new, [0 value]);
#[cfg(feature = "unstable-experimental")]
inherent_impls!(Gray_v09, new, [v value]);

inherent_impls!(Rgba, new, [r red, g green, b blue, a alpha]);
inherent_impls!(Argb, new_argb, [a alpha, r red, g green, b blue]);
inherent_impls!(Bgra, new_bgra, [b blue, g green, r red, a alpha]);
inherent_impls!(Abgr, new_abgr, [a alpha, b blue, g green, r red]);
inherent_impls!(GrayA, new, [v value, a alpha]);

inherent_impls!(GrayAlpha_v08, new, [0 value, 1 alpha]);

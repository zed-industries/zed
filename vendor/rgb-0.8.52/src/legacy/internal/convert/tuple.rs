
use crate::formats::gray::Gray_v08;
use crate::{Abgr, Argb, Bgr, Bgra, Grb, Rgb, Rgba, alt::GrayAlpha};

#[cfg(feature = "unstable-experimental")]
use crate::GrayA;
#[cfg(feature = "unstable-experimental")]
use crate::formats::gray::Gray_v09;

macro_rules! tuple_conversion {
    ($name:ident, 1, [$($bit:tt:$num:tt),*]) => {
        impl<R, S> From<$name<R>> for (S,) where R: Into<S> {
            #[allow(deprecated)]
            fn from(value: $name<R>) -> Self {
                ($(value.$bit.into()),*,)
            }
        }
    };
    ($name:ident, 2, [$($bit:tt:$num:tt),*]) => {
        impl<R, S> From<$name<R>> for (S, S) where R: Into<S> {
            #[allow(deprecated)]
            fn from(value: $name<R>) -> Self {
                ($(value.$bit.into()),*)
            }
        }
        impl<R, S> From<(R, R)> for $name<S> where R: Into<S> {
            #[allow(deprecated)]
            fn from(value: (R, R)) -> Self {
                Self{$($bit: value.$num.into()),*}
            }
        }
    };
    ($name:ident, 3, [$($bit:tt:$num:tt),*]) => {
        impl<R, S> From<$name<R>> for (S, S, S) where R: Into<S> {
            fn from(value: $name<R>) -> Self {
                ($(value.$bit.into()),*)
            }
        }
        impl<R, S> From<(R, R, R)> for $name<S> where R: Into<S> {
            fn from(value: (R, R, R)) -> Self {
                Self{$($bit: value.$num.into()),*}
            }
        }
    };
    ($name:ident, 4, [$($bit:tt:$num:tt),*]) => {
        impl<R, S> From<$name<R>> for (S, S, S, S) where R: Into<S> {
            fn from(value: $name<R>) -> Self {
                ($(value.$bit.into()),*)
            }
        }
        impl<R, S> From<(R, R, R, R)> for $name<S> where R: Into<S> {
            fn from(value: (R, R, R, R)) -> Self {
                Self{$($bit: value.$num.into()),*}
            }
        }
    };
}


tuple_conversion!(Rgb, 3, [r:0, g:1, b:2]);
tuple_conversion!(Bgr, 3, [b:0, g:1, r:2]);
tuple_conversion!(Grb, 3, [g:0, r:1, b:2]);
#[cfg(feature = "unstable-experimental")]
tuple_conversion!(Gray_v09, 1, [v:0]);


tuple_conversion!(Rgba, 4, [r:0, g:1, b:2, a:3]);
tuple_conversion!(Argb, 4, [a:0, r:1, g:2, b:3]);
tuple_conversion!(Bgra, 4, [b:0, g:1, r:2, a:3]);
tuple_conversion!(Abgr, 4, [a:0, b:1, g:2, r:3]);
#[cfg(feature = "unstable-experimental")]
tuple_conversion!(GrayA, 2, [v:0, a:1]);

tuple_conversion!(Gray_v08, 1, [0:0]);
tuple_conversion!(GrayAlpha, 2, [0:0, 1:1]);

#[test]
fn converts() {
    assert_eq!((1,2,3), Rgb {r:1u8,g:2,b:3}.into());
    assert_eq!(Rgb {r:1u8,g:2,b:3}, (1,2,3).into());
    assert_eq!((1,2,3,4), Rgba {r:1,g:2,b:3,a:4}.into());
    assert_eq!(Rgba {r:1u8,g:2,b:3,a:4}, (1,2,3,4).into());
    assert_eq!(Bgra {r:1u8,g:2,b:3,a:4}, (3,2,1,4).into());
    assert_eq!(Bgr {r:1u8,g:2,b:3}, (3,2,1).into());
}

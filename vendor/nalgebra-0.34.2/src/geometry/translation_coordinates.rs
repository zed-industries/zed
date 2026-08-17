use std::ops::{Deref, DerefMut};

use crate::base::Scalar;
use crate::base::coordinates::{X, XY, XYZ, XYZW, XYZWA, XYZWAB};

use crate::geometry::Translation;

/*
 *
 * Give coordinates to Translation{1 .. 6}
 *
 */

macro_rules! deref_impl(
    ($D: expr_2021, $Target: ident $(, $comps: ident)*) => {
        impl<T: Scalar> Deref for Translation<T, $D> {
            type Target = $Target<T>;

            #[inline]
            fn deref(&self) -> &Self::Target {
                unsafe { &*(self as *const Translation<T, $D> as *const Self::Target) }
            }
        }

        impl<T: Scalar> DerefMut for Translation<T, $D> {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe { &mut *(self as *mut Translation<T, $D> as *mut Self::Target) }
            }
        }
    }
);

deref_impl!(1, X, x);
deref_impl!(2, XY, x, y);
deref_impl!(3, XYZ, x, y, z);
deref_impl!(4, XYZW, x, y, z, w);
deref_impl!(5, XYZWA, x, y, z, w, a);
deref_impl!(6, XYZWAB, x, y, z, w, a, b);

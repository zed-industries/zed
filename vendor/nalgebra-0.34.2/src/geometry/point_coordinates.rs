use std::ops::{Deref, DerefMut};

use crate::base::coordinates::{X, XY, XYZ, XYZW, XYZWA, XYZWAB};
use crate::base::{Scalar, U1, U2, U3, U4, U5, U6};

use crate::geometry::OPoint;

/*
 *
 * Give coordinates to Point{1 .. 6}
 *
 */

macro_rules! deref_impl(
    ($D: ty, $Target: ident $(, $comps: ident)*) => {
        impl<T: Scalar> Deref for OPoint<T, $D>
        {
            type Target = $Target<T>;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &*self.coords
            }
        }

        impl<T: Scalar> DerefMut for OPoint<T, $D>
        {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut *self.coords
            }
        }
    }
);

deref_impl!(U1, X, x);
deref_impl!(U2, XY, x, y);
deref_impl!(U3, XYZ, x, y, z);
deref_impl!(U4, XYZW, x, y, z, w);
deref_impl!(U5, XYZWA, x, y, z, w, a);
deref_impl!(U6, XYZWAB, x, y, z, w, a, b);

#![allow(missing_docs)]

//! Structures to which matrices and vector can be auto-dereferenced (through `Deref`) to access
//! components using their names. For example, if `v` is a 3D vector, one can write `v.z` instead
//! of `v[2]`.

use std::ops::{Deref, DerefMut};

use crate::base::dimension::{U1, U2, U3, U4, U5, U6};
use crate::base::storage::{IsContiguous, RawStorage, RawStorageMut};
use crate::base::{Matrix, Scalar};

/*
 *
 * Give coordinates to owned Vector{1 .. 6} and Matrix{1 .. 6}
 *
 */

macro_rules! coords_impl(
    ($T: ident; $($comps: ident),*) => {
        /// Data structure used to provide access to matrix and vector coordinates with the dot
        /// notation, e.g., `v.x` is the same as `v[0]` for a vector.
        #[repr(C)]
        #[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
        #[cfg_attr(feature = "serde-serialize-no-std", derive(Serialize, Deserialize))]
        pub struct $T<T: Scalar> {
            $(pub $comps: T),*
        }
    }
);

macro_rules! deref_impl(
    ($R: ty, $C: ty; $Target: ident) => {
        impl<T: Scalar, S> Deref for Matrix<T, $R, $C, S>
            where S: RawStorage<T, $R, $C> + IsContiguous {
            type Target = $Target<T>;

            #[inline]
            fn deref(&self) -> &Self::Target {
                // Safety: this is OK because of the IsContiguous trait.
                unsafe { &*(self.data.ptr() as *const Self::Target) }
            }
        }

        impl<T: Scalar, S> DerefMut for Matrix<T, $R, $C, S>
            where S: RawStorageMut<T, $R, $C> + IsContiguous {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                // Safety: this is OK because of the IsContiguous trait.
                unsafe { &mut *(self.data.ptr_mut() as *mut Self::Target) }
            }
        }
    }
);

/*
 *
 * Vector coordinates.
 *
 */
coords_impl!(X; x);
coords_impl!(XY; x, y);
coords_impl!(XYZ; x, y, z);
coords_impl!(XYZW; x, y, z, w);
coords_impl!(XYZWA; x, y, z, w, a);
coords_impl!(XYZWAB; x, y, z, w, a, b);
coords_impl!(IJKW; i, j, k, w);

/*
 * Rectangular matrices with 2 rows.
 */
coords_impl!(M2x2; m11, m21,
                   m12, m22);
coords_impl!(M2x3; m11, m21,
                   m12, m22,
                   m13, m23);
coords_impl!(M2x4; m11, m21,
                   m12, m22,
                   m13, m23,
                   m14, m24);
coords_impl!(M2x5; m11, m21,
                   m12, m22,
                   m13, m23,
                   m14, m24,
                   m15, m25);
coords_impl!(M2x6; m11, m21,
                   m12, m22,
                   m13, m23,
                   m14, m24,
                   m15, m25,
                   m16, m26);

/*
 * Rectangular matrices with 3 rows.
 */
coords_impl!(M3x2; m11, m21, m31,
                   m12, m22, m32);
coords_impl!(M3x3; m11, m21, m31,
                   m12, m22, m32,
                   m13, m23, m33);
coords_impl!(M3x4; m11, m21, m31,
                   m12, m22, m32,
                   m13, m23, m33,
                   m14, m24, m34);
coords_impl!(M3x5; m11, m21, m31,
                   m12, m22, m32,
                   m13, m23, m33,
                   m14, m24, m34,
                   m15, m25, m35);
coords_impl!(M3x6; m11, m21, m31,
                   m12, m22, m32,
                   m13, m23, m33,
                   m14, m24, m34,
                   m15, m25, m35,
                   m16, m26, m36);

/*
 * Rectangular matrices with 4 rows.
 */
coords_impl!(M4x2; m11, m21, m31, m41,
                   m12, m22, m32, m42);
coords_impl!(M4x3; m11, m21, m31, m41,
                   m12, m22, m32, m42,
                   m13, m23, m33, m43);
coords_impl!(M4x4; m11, m21, m31, m41,
                   m12, m22, m32, m42,
                   m13, m23, m33, m43,
                   m14, m24, m34, m44);
coords_impl!(M4x5; m11, m21, m31, m41,
                   m12, m22, m32, m42,
                   m13, m23, m33, m43,
                   m14, m24, m34, m44,
                   m15, m25, m35, m45);
coords_impl!(M4x6; m11, m21, m31, m41,
                   m12, m22, m32, m42,
                   m13, m23, m33, m43,
                   m14, m24, m34, m44,
                   m15, m25, m35, m45,
                   m16, m26, m36, m46);

/*
 * Rectangular matrices with 5 rows.
 */
coords_impl!(M5x2; m11, m21, m31, m41, m51,
                   m12, m22, m32, m42, m52);
coords_impl!(M5x3; m11, m21, m31, m41, m51,
                   m12, m22, m32, m42, m52,
                   m13, m23, m33, m43, m53);
coords_impl!(M5x4; m11, m21, m31, m41, m51,
                   m12, m22, m32, m42, m52,
                   m13, m23, m33, m43, m53,
                   m14, m24, m34, m44, m54);
coords_impl!(M5x5; m11, m21, m31, m41, m51,
                   m12, m22, m32, m42, m52,
                   m13, m23, m33, m43, m53,
                   m14, m24, m34, m44, m54,
                   m15, m25, m35, m45, m55);
coords_impl!(M5x6; m11, m21, m31, m41, m51,
                   m12, m22, m32, m42, m52,
                   m13, m23, m33, m43, m53,
                   m14, m24, m34, m44, m54,
                   m15, m25, m35, m45, m55,
                   m16, m26, m36, m46, m56);

/*
 * Rectangular matrices with 6 rows.
 */

coords_impl!(M6x2; m11, m21, m31, m41, m51, m61,
                   m12, m22, m32, m42, m52, m62);
coords_impl!(M6x3; m11, m21, m31, m41, m51, m61,
                   m12, m22, m32, m42, m52, m62,
                   m13, m23, m33, m43, m53, m63);
coords_impl!(M6x4; m11, m21, m31, m41, m51, m61,
                   m12, m22, m32, m42, m52, m62,
                   m13, m23, m33, m43, m53, m63,
                   m14, m24, m34, m44, m54, m64);
coords_impl!(M6x5; m11, m21, m31, m41, m51, m61,
                   m12, m22, m32, m42, m52, m62,
                   m13, m23, m33, m43, m53, m63,
                   m14, m24, m34, m44, m54, m64,
                   m15, m25, m35, m45, m55, m65);
coords_impl!(M6x6; m11, m21, m31, m41, m51, m61,
                   m12, m22, m32, m42, m52, m62,
                   m13, m23, m33, m43, m53, m63,
                   m14, m24, m34, m44, m54, m64,
                   m15, m25, m35, m45, m55, m65,
                   m16, m26, m36, m46, m56, m66);

/*
 *
 * Attach coordinates to matrices.
 *
 */
deref_impl!(U1, U1; X);
deref_impl!(U2, U1; XY);
deref_impl!(U3, U1; XYZ);
deref_impl!(U4, U1; XYZW);
deref_impl!(U5, U1; XYZWA);
deref_impl!(U6, U1; XYZWAB);

deref_impl!(U1, U2; XY);
deref_impl!(U1, U3; XYZ);
deref_impl!(U1, U4; XYZW);
deref_impl!(U1, U5; XYZWA);
deref_impl!(U1, U6; XYZWAB);

deref_impl!(U2, U2; M2x2);
deref_impl!(U2, U3; M2x3);
deref_impl!(U2, U4; M2x4);
deref_impl!(U2, U5; M2x5);
deref_impl!(U2, U6; M2x6);

deref_impl!(U3, U2; M3x2);
deref_impl!(U3, U3; M3x3);
deref_impl!(U3, U4; M3x4);
deref_impl!(U3, U5; M3x5);
deref_impl!(U3, U6; M3x6);

deref_impl!(U4, U2; M4x2);
deref_impl!(U4, U3; M4x3);
deref_impl!(U4, U4; M4x4);
deref_impl!(U4, U5; M4x5);
deref_impl!(U4, U6; M4x6);

deref_impl!(U5, U2; M5x2);
deref_impl!(U5, U3; M5x3);
deref_impl!(U5, U4; M5x4);
deref_impl!(U5, U5; M5x5);
deref_impl!(U5, U6; M5x6);

deref_impl!(U6, U2; M6x2);
deref_impl!(U6, U3; M6x3);
deref_impl!(U6, U4; M6x4);
deref_impl!(U6, U5; M6x5);
deref_impl!(U6, U6; M6x6);

use crate::base::storage::{RawStorage, RawStorageMut};
use crate::{OVector, Point, Scalar};
use std::convert::{AsMut, AsRef};

macro_rules! impl_from_into_mint_1D(
    ($($NRows: expr => $PT:ident, $VT:ident [$SZ: expr]);* $(;)*) => {$(
        impl<T> From<mint::$PT<T>> for Point<T, $NRows>
        where T: Scalar {
            #[inline]
            fn from(p: mint::$PT<T>) -> Self {
                Self {
                    coords: OVector::from(mint::$VT::from(p)),
                }
            }
        }

        impl<T> Into<mint::$PT<T>> for Point<T, $NRows>
        where T: Scalar {
            #[inline]
            fn into(self) -> mint::$PT<T> {
                let mint_vec: mint::$VT<T> = self.coords.into();
                mint::$PT::from(mint_vec)
            }
        }

        impl<T> AsRef<mint::$PT<T>> for Point<T, $NRows>
        where T: Scalar {
            #[inline]
            fn as_ref(&self) -> &mint::$PT<T> {
                unsafe {
                    &*(self.coords.data.ptr() as *const mint::$PT<T>)
                }
            }
        }

        impl<T> AsMut<mint::$PT<T>> for Point<T, $NRows>
        where T: Scalar {
            #[inline]
            fn as_mut(&mut self) -> &mut mint::$PT<T> {
                unsafe {
                    &mut *(self.coords.data.ptr_mut() as *mut mint::$PT<T>)
                }
            }
        }
    )*}
);

// Implement for points of dimension 2, 3.
impl_from_into_mint_1D!(
    2 => Point2, Vector2[2];
    3 => Point3, Vector3[3];
);

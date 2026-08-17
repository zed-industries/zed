use crate::base::{DimName, Scalar, ToTypenum, Vector, Vector2, Vector3};
use crate::storage::RawStorage;
use typenum::{self, Cmp, Greater};

macro_rules! impl_swizzle {
    ($( where $BaseDim: ident: $( $name: ident() -> $Result: ident[$($i: expr_2021),+] ),+ ;)* ) => {
        $(
            $(
                /// Builds a new vector from components of `self`.
                #[inline]
                #[must_use]
                pub fn $name(&self) -> $Result<T>
                where D::Typenum: Cmp<typenum::$BaseDim, Output=Greater> {
                    $Result::new($(self[$i].clone()),*)
                }
            )*
        )*
    }
}

/// # Swizzling
impl<T: Scalar, D, S: RawStorage<T, D>> Vector<T, D, S>
where
    D: DimName + ToTypenum,
{
    impl_swizzle!(
        where U0: xx()  -> Vector2[0, 0],
                  xxx() -> Vector3[0, 0, 0];

        where U1: xy()  -> Vector2[0, 1],
                  yx()  -> Vector2[1, 0],
                  yy()  -> Vector2[1, 1],
                  xxy() -> Vector3[0, 0, 1],
                  xyx() -> Vector3[0, 1, 0],
                  xyy() -> Vector3[0, 1, 1],
                  yxx() -> Vector3[1, 0, 0],
                  yxy() -> Vector3[1, 0, 1],
                  yyx() -> Vector3[1, 1, 0],
                  yyy() -> Vector3[1, 1, 1];

        where U2: xz()  -> Vector2[0, 2],
                  yz()  -> Vector2[1, 2],
                  zx()  -> Vector2[2, 0],
                  zy()  -> Vector2[2, 1],
                  zz()  -> Vector2[2, 2],
                  xxz() -> Vector3[0, 0, 2],
                  xyz() -> Vector3[0, 1, 2],
                  xzx() -> Vector3[0, 2, 0],
                  xzy() -> Vector3[0, 2, 1],
                  xzz() -> Vector3[0, 2, 2],
                  yxz() -> Vector3[1, 0, 2],
                  yyz() -> Vector3[1, 1, 2],
                  yzx() -> Vector3[1, 2, 0],
                  yzy() -> Vector3[1, 2, 1],
                  yzz() -> Vector3[1, 2, 2],
                  zxx() -> Vector3[2, 0, 0],
                  zxy() -> Vector3[2, 0, 1],
                  zxz() -> Vector3[2, 0, 2],
                  zyx() -> Vector3[2, 1, 0],
                  zyy() -> Vector3[2, 1, 1],
                  zyz() -> Vector3[2, 1, 2],
                  zzx() -> Vector3[2, 2, 0],
                  zzy() -> Vector3[2, 2, 1],
                  zzz() -> Vector3[2, 2, 2];
    );
}

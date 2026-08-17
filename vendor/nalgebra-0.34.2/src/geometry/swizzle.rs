use crate::base::{Const, Scalar, ToTypenum};
use crate::geometry::{Point, Point2, Point3};
use typenum::{self, Cmp, Greater};

macro_rules! impl_swizzle {
    ($( where $BaseDim: ident: $( $name: ident() -> $Result: ident[$($i: expr_2021),+] ),+ ;)* ) => {
        $(
            $(
                /// Builds a new point from components of `self`.
                #[inline]
                #[must_use]
                pub fn $name(&self) -> $Result<T>
                 where <Const<D> as ToTypenum>::Typenum: Cmp<typenum::$BaseDim, Output=Greater> {
                    $Result::new($(self[$i].clone()),*)
                }
            )*
        )*
    }
}

/// # Swizzling
impl<T: Scalar, const D: usize> Point<T, D>
where
    Const<D>: ToTypenum,
{
    impl_swizzle!(
        where U0: xx()  -> Point2[0, 0],
                  xxx() -> Point3[0, 0, 0];

        where U1: xy()  -> Point2[0, 1],
                  yx()  -> Point2[1, 0],
                  yy()  -> Point2[1, 1],
                  xxy() -> Point3[0, 0, 1],
                  xyx() -> Point3[0, 1, 0],
                  xyy() -> Point3[0, 1, 1],
                  yxx() -> Point3[1, 0, 0],
                  yxy() -> Point3[1, 0, 1],
                  yyx() -> Point3[1, 1, 0],
                  yyy() -> Point3[1, 1, 1];

        where U2: xz()  -> Point2[0, 2],
                  yz()  -> Point2[1, 2],
                  zx()  -> Point2[2, 0],
                  zy()  -> Point2[2, 1],
                  zz()  -> Point2[2, 2],
                  xxz() -> Point3[0, 0, 2],
                  xyz() -> Point3[0, 1, 2],
                  xzx() -> Point3[0, 2, 0],
                  xzy() -> Point3[0, 2, 1],
                  xzz() -> Point3[0, 2, 2],
                  yxz() -> Point3[1, 0, 2],
                  yyz() -> Point3[1, 1, 2],
                  yzx() -> Point3[1, 2, 0],
                  yzy() -> Point3[1, 2, 1],
                  yzz() -> Point3[1, 2, 2],
                  zxx() -> Point3[2, 0, 0],
                  zxy() -> Point3[2, 0, 1],
                  zxz() -> Point3[2, 0, 2],
                  zyx() -> Point3[2, 1, 0],
                  zyy() -> Point3[2, 1, 1],
                  zyz() -> Point3[2, 1, 2],
                  zzx() -> Point3[2, 2, 0],
                  zzy() -> Point3[2, 2, 1],
                  zzz() -> Point3[2, 2, 2];
    );
}

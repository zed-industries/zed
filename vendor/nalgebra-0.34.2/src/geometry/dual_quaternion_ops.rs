// The macros break if the references are taken out, for some reason.
#![allow(clippy::op_ref)]

/*
 * This file provides:
 *
 * NOTE: Work in progress https://github.com/dimforge/nalgebra/issues/487
 *
 * (Dual Quaternion)
 *
 * Index<usize>
 * IndexMut<usize>
 *
 * (Assignment Operators)
 *
 * -DualQuaternion
 * DualQuaternion × Scalar
 * DualQuaternion × DualQuaternion
 * DualQuaternion + DualQuaternion
 * DualQuaternion - DualQuaternion
 * DualQuaternion × UnitDualQuaternion
 * DualQuaternion ÷ UnitDualQuaternion
 * -UnitDualQuaternion
 * UnitDualQuaternion × DualQuaternion
 * UnitDualQuaternion × UnitDualQuaternion
 * UnitDualQuaternion ÷ UnitDualQuaternion
 * UnitDualQuaternion × Translation3
 * UnitDualQuaternion ÷ Translation3
 * UnitDualQuaternion × UnitQuaternion
 * UnitDualQuaternion ÷ UnitQuaternion
 * Translation3 × UnitDualQuaternion
 * Translation3 ÷ UnitDualQuaternion
 * UnitQuaternion × UnitDualQuaternion
 * UnitQuaternion ÷ UnitDualQuaternion
 * UnitDualQuaternion × Isometry3
 * UnitDualQuaternion ÷ Isometry3
 * Isometry3 × UnitDualQuaternion
 * Isometry3 ÷ UnitDualQuaternion
 * UnitDualQuaternion × Point
 * UnitDualQuaternion × Vector
 * UnitDualQuaternion × Unit<Vector>
 *
 * ---
 *
 * References:
 *   Multiplication:
 *   - https://cs.gmu.edu/~jmlien/teaching/cs451/uploads/Main/dual-quaternion.pdf
 */

use crate::base::storage::Storage;
use crate::{
    DualQuaternion, Isometry3, Point, Point3, Quaternion, SimdRealField, Translation3, U3, Unit,
    UnitDualQuaternion, UnitQuaternion, Vector, Vector3,
};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

impl<T: SimdRealField> AsRef<[T; 8]> for DualQuaternion<T> {
    #[inline]
    fn as_ref(&self) -> &[T; 8] {
        unsafe { &*(self as *const Self as *const [T; 8]) }
    }
}

impl<T: SimdRealField> AsMut<[T; 8]> for DualQuaternion<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T; 8] {
        unsafe { &mut *(self as *mut Self as *mut [T; 8]) }
    }
}

impl<T: SimdRealField> Index<usize> for DualQuaternion<T> {
    type Output = T;

    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        &self.as_ref()[i]
    }
}

impl<T: SimdRealField> IndexMut<usize> for DualQuaternion<T> {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut T {
        &mut self.as_mut()[i]
    }
}

impl<T: SimdRealField> Neg for DualQuaternion<T>
where
    T::Element: SimdRealField,
{
    type Output = DualQuaternion<T>;

    #[inline]
    fn neg(self) -> Self::Output {
        DualQuaternion::from_real_and_dual(-self.real, -self.dual)
    }
}

impl<T: SimdRealField> Neg for &DualQuaternion<T>
where
    T::Element: SimdRealField,
{
    type Output = DualQuaternion<T>;

    #[inline]
    fn neg(self) -> Self::Output {
        DualQuaternion::from_real_and_dual(-&self.real, -&self.dual)
    }
}

impl<T: SimdRealField> Neg for UnitDualQuaternion<T>
where
    T::Element: SimdRealField,
{
    type Output = UnitDualQuaternion<T>;

    #[inline]
    fn neg(self) -> Self::Output {
        UnitDualQuaternion::new_unchecked(-self.into_inner())
    }
}

impl<T: SimdRealField> Neg for &UnitDualQuaternion<T>
where
    T::Element: SimdRealField,
{
    type Output = UnitDualQuaternion<T>;

    #[inline]
    fn neg(self) -> Self::Output {
        UnitDualQuaternion::new_unchecked(-self.as_ref())
    }
}

macro_rules! dual_quaternion_op_impl(
    ($Op: ident, $op: ident;
     ($LhsRDim: ident, $LhsCDim: ident), ($RhsRDim: ident, $RhsCDim: ident)
     $(for $Storage: ident: $StoragesBound: ident $(<$($BoundParam: ty),*>)*),*;
     $lhs: ident: $Lhs: ty, $rhs: ident: $Rhs: ty, Output = $Result: ty $(=> $VDimA: ty, $VDimB: ty)*;
     $action: expr_2021; $($lives: tt),*) => {
        impl<$($lives ,)* T: SimdRealField $(, $Storage: $StoragesBound $(<$($BoundParam),*>)*)*> $Op<$Rhs> for $Lhs
            where T::Element: SimdRealField, {
            type Output = $Result;

            #[inline]
            fn $op($lhs, $rhs: $Rhs) -> Self::Output {
                $action
            }
        }
    }
);

// DualQuaternion + DualQuaternion
dual_quaternion_op_impl!(
    Add, add;
    (U4, U1), (U4, U1);
    self: &'a DualQuaternion<T>, rhs: &'b DualQuaternion<T>, Output = DualQuaternion<T>;
    DualQuaternion::from_real_and_dual(
        &self.real + &rhs.real,
        &self.dual + &rhs.dual,
    );
    'a, 'b);

dual_quaternion_op_impl!(
    Add, add;
    (U4, U1), (U4, U1);
    self: &'a DualQuaternion<T>, rhs: DualQuaternion<T>, Output = DualQuaternion<T>;
    DualQuaternion::from_real_and_dual(
        &self.real + rhs.real,
        &self.dual + rhs.dual,
    );
    'a);

dual_quaternion_op_impl!(
    Add, add;
    (U4, U1), (U4, U1);
    self: DualQuaternion<T>, rhs: &'b DualQuaternion<T>, Output = DualQuaternion<T>;
    DualQuaternion::from_real_and_dual(
        self.real + &rhs.real,
        self.dual + &rhs.dual,
    );
    'b);

dual_quaternion_op_impl!(
    Add, add;
    (U4, U1), (U4, U1);
    self: DualQuaternion<T>, rhs: DualQuaternion<T>, Output = DualQuaternion<T>;
    DualQuaternion::from_real_and_dual(
        self.real + rhs.real,
        self.dual + rhs.dual,
    ); );

// DualQuaternion - DualQuaternion
dual_quaternion_op_impl!(
    Sub, sub;
    (U4, U1), (U4, U1);
    self: &'a DualQuaternion<T>, rhs: &'b DualQuaternion<T>, Output = DualQuaternion<T>;
    DualQuaternion::from_real_and_dual(
        &self.real - &rhs.real,
        &self.dual - &rhs.dual,
    );
    'a, 'b);

dual_quaternion_op_impl!(
    Sub, sub;
    (U4, U1), (U4, U1);
    self: &'a DualQuaternion<T>, rhs: DualQuaternion<T>, Output = DualQuaternion<T>;
    DualQuaternion::from_real_and_dual(
        &self.real - rhs.real,
        &self.dual - rhs.dual,
    );
    'a);

dual_quaternion_op_impl!(
    Sub, sub;
    (U4, U1), (U4, U1);
    self: DualQuaternion<T>, rhs: &'b DualQuaternion<T>, Output = DualQuaternion<T>;
    DualQuaternion::from_real_and_dual(
        self.real - &rhs.real,
        self.dual - &rhs.dual,
    );
    'b);

dual_quaternion_op_impl!(
    Sub, sub;
    (U4, U1), (U4, U1);
    self: DualQuaternion<T>, rhs: DualQuaternion<T>, Output = DualQuaternion<T>;
    DualQuaternion::from_real_and_dual(
        self.real - rhs.real,
        self.dual - rhs.dual,
    ); );

// DualQuaternion × DualQuaternion
dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U4, U1);
    self: &'a DualQuaternion<T>, rhs: &'b DualQuaternion<T>, Output = DualQuaternion<T>;
    DualQuaternion::from_real_and_dual(
        &self.real * &rhs.real,
        &self.real * &rhs.dual + &self.dual * &rhs.real,
    );
    'a, 'b);

dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U4, U1);
    self: &'a DualQuaternion<T>, rhs: DualQuaternion<T>, Output = DualQuaternion<T>;
    self * &rhs;
    'a);

dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U4, U1);
    self: DualQuaternion<T>, rhs: &'b DualQuaternion<T>, Output = DualQuaternion<T>;
    &self * rhs;
    'b);

dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U4, U1);
    self: DualQuaternion<T>, rhs: DualQuaternion<T>, Output = DualQuaternion<T>;
    &self * &rhs; );

// DualQuaternion × UnitDualQuaternion
dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U4, U1);
    self: &'a DualQuaternion<T>, rhs: &'b UnitDualQuaternion<T>, Output = DualQuaternion<T>;
    self * rhs.dual_quaternion();
    'a, 'b);

dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U4, U1);
    self: &'a DualQuaternion<T>, rhs: UnitDualQuaternion<T>, Output = DualQuaternion<T>;
    self * rhs.dual_quaternion();
    'a);

dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U4, U1);
    self: DualQuaternion<T>, rhs: &'b UnitDualQuaternion<T>, Output = DualQuaternion<T>;
    self * rhs.dual_quaternion();
    'b);

dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U4, U1);
    self: DualQuaternion<T>, rhs: UnitDualQuaternion<T>, Output = DualQuaternion<T>;
    self * rhs.dual_quaternion(););

// DualQuaternion ÷ UnitDualQuaternion
dual_quaternion_op_impl!(
    Div, div;
    (U4, U1), (U4, U1);
    self: &'a DualQuaternion<T>, rhs: &'b UnitDualQuaternion<T>, Output = DualQuaternion<T>;
    #[allow(clippy::suspicious_arithmetic_impl)]
    { self * rhs.inverse().dual_quaternion() };
    'a, 'b);

dual_quaternion_op_impl!(
    Div, div;
    (U4, U1), (U4, U1);
    self: &'a DualQuaternion<T>, rhs: UnitDualQuaternion<T>, Output = DualQuaternion<T>;
    #[allow(clippy::suspicious_arithmetic_impl)]
    { self * rhs.inverse().dual_quaternion() };
    'a);

dual_quaternion_op_impl!(
    Div, div;
    (U4, U1), (U4, U1);
    self: DualQuaternion<T>, rhs: &'b UnitDualQuaternion<T>, Output = DualQuaternion<T>;
    #[allow(clippy::suspicious_arithmetic_impl)]
    { self * rhs.inverse().dual_quaternion() };
    'b);

dual_quaternion_op_impl!(
    Div, div;
    (U4, U1), (U4, U1);
    self: DualQuaternion<T>, rhs: UnitDualQuaternion<T>, Output = DualQuaternion<T>;
    #[allow(clippy::suspicious_arithmetic_impl)]
    { self * rhs.inverse().dual_quaternion() };);

// UnitDualQuaternion × UnitDualQuaternion
dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U4, U1);
    self: &'a UnitDualQuaternion<T>, rhs: &'b UnitDualQuaternion<T>, Output = UnitDualQuaternion<T>;
    UnitDualQuaternion::new_unchecked(self.as_ref() * rhs.as_ref());
    'a, 'b);

dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U4, U1);
    self: &'a UnitDualQuaternion<T>, rhs: UnitDualQuaternion<T>, Output = UnitDualQuaternion<T>;
    self * &rhs;
    'a);

dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U4, U1);
    self: UnitDualQuaternion<T>, rhs: &'b UnitDualQuaternion<T>, Output = UnitDualQuaternion<T>;
    &self * rhs;
    'b);

dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U4, U1);
    self: UnitDualQuaternion<T>, rhs: UnitDualQuaternion<T>, Output = UnitDualQuaternion<T>;
    &self * &rhs; );

// UnitDualQuaternion ÷ UnitDualQuaternion
dual_quaternion_op_impl!(
    Div, div;
    (U4, U1), (U4, U1);
    self: &'a UnitDualQuaternion<T>, rhs: &'b UnitDualQuaternion<T>, Output = UnitDualQuaternion<T>;
    #[allow(clippy::suspicious_arithmetic_impl)] { self * rhs.inverse() };
    'a, 'b);

dual_quaternion_op_impl!(
    Div, div;
    (U4, U1), (U4, U1);
    self: &'a UnitDualQuaternion<T>, rhs: UnitDualQuaternion<T>, Output = UnitDualQuaternion<T>;
    self / &rhs;
    'a);

dual_quaternion_op_impl!(
    Div, div;
    (U4, U1), (U4, U1);
    self: UnitDualQuaternion<T>, rhs: &'b UnitDualQuaternion<T>, Output = UnitDualQuaternion<T>;
    &self / rhs;
    'b);

dual_quaternion_op_impl!(
    Div, div;
    (U4, U1), (U4, U1);
    self: UnitDualQuaternion<T>, rhs: UnitDualQuaternion<T>, Output = UnitDualQuaternion<T>;
    &self / &rhs; );

// UnitDualQuaternion × DualQuaternion
dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U4, U1);
    self: &'a UnitDualQuaternion<T>, rhs: &'b DualQuaternion<T>,
    Output = DualQuaternion<T> => U1, U4;
    self.dual_quaternion() * rhs;
    'a, 'b);

dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U4, U1);
    self: &'a UnitDualQuaternion<T>, rhs: DualQuaternion<T>,
    Output = DualQuaternion<T> => U3, U3;
    self.dual_quaternion() * rhs;
    'a);

dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U4, U1);
    self: UnitDualQuaternion<T>, rhs: &'b DualQuaternion<T>,
    Output = DualQuaternion<T> => U3, U3;
    self.dual_quaternion() * rhs;
    'b);

dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U4, U1);
    self: UnitDualQuaternion<T>, rhs: DualQuaternion<T>,
    Output = DualQuaternion<T> => U3, U3;
    self.dual_quaternion() * rhs;);

// UnitDualQuaternion × UnitQuaternion
dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U4, U1);
    self: &'a UnitDualQuaternion<T>, rhs: &'b UnitQuaternion<T>,
    Output = UnitDualQuaternion<T> => U1, U4;
    self * UnitDualQuaternion::<T>::new_unchecked(DualQuaternion::from_real(rhs.clone().into_inner()));
    'a, 'b);

dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U4, U1);
    self: &'a UnitDualQuaternion<T>, rhs: UnitQuaternion<T>,
    Output = UnitDualQuaternion<T> => U3, U3;
    self * UnitDualQuaternion::<T>::new_unchecked(DualQuaternion::from_real(rhs.into_inner()));
    'a);

dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U4, U1);
    self: UnitDualQuaternion<T>, rhs: &'b UnitQuaternion<T>,
    Output = UnitDualQuaternion<T> => U3, U3;
    self * UnitDualQuaternion::<T>::new_unchecked(DualQuaternion::from_real(rhs.clone().into_inner()));
    'b);

dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U4, U1);
    self: UnitDualQuaternion<T>, rhs: UnitQuaternion<T>,
    Output = UnitDualQuaternion<T> => U3, U3;
    self * UnitDualQuaternion::<T>::new_unchecked(DualQuaternion::from_real(rhs.into_inner())););

// UnitQuaternion × UnitDualQuaternion
dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U4, U1);
    self: &'a UnitQuaternion<T>, rhs: &'b UnitDualQuaternion<T>,
    Output = UnitDualQuaternion<T> => U1, U4;
    UnitDualQuaternion::<T>::new_unchecked(DualQuaternion::from_real(self.clone().into_inner())) * rhs;
    'a, 'b);

dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U4, U1);
    self: &'a UnitQuaternion<T>, rhs: UnitDualQuaternion<T>,
    Output = UnitDualQuaternion<T> => U3, U3;
    UnitDualQuaternion::<T>::new_unchecked(DualQuaternion::from_real(self.clone().into_inner())) * rhs;
    'a);

dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U4, U1);
    self: UnitQuaternion<T>, rhs: &'b UnitDualQuaternion<T>,
    Output = UnitDualQuaternion<T> => U3, U3;
    UnitDualQuaternion::<T>::new_unchecked(DualQuaternion::from_real(self.into_inner())) * rhs;
    'b);

dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U4, U1);
    self: UnitQuaternion<T>, rhs: UnitDualQuaternion<T>,
    Output = UnitDualQuaternion<T> => U3, U3;
    UnitDualQuaternion::<T>::new_unchecked(DualQuaternion::from_real(self.into_inner())) * rhs;);

// UnitDualQuaternion ÷ UnitQuaternion
dual_quaternion_op_impl!(
    Div, div;
    (U4, U1), (U4, U1);
    self: &'a UnitDualQuaternion<T>, rhs: &'b UnitQuaternion<T>,
    Output = UnitDualQuaternion<T> => U1, U4;
    #[allow(clippy::suspicious_arithmetic_impl)]
    { self * UnitDualQuaternion::<T>::from_rotation(rhs.inverse()) };
    'a, 'b);

dual_quaternion_op_impl!(
    Div, div;
    (U4, U1), (U4, U1);
    self: &'a UnitDualQuaternion<T>, rhs: UnitQuaternion<T>,
    Output = UnitDualQuaternion<T> => U3, U3;
    #[allow(clippy::suspicious_arithmetic_impl)]
    { self * UnitDualQuaternion::<T>::from_rotation(rhs.inverse()) };
    'a);

dual_quaternion_op_impl!(
    Div, div;
    (U4, U1), (U4, U1);
    self: UnitDualQuaternion<T>, rhs: &'b UnitQuaternion<T>,
    Output = UnitDualQuaternion<T> => U3, U3;
    #[allow(clippy::suspicious_arithmetic_impl)]
    { self * UnitDualQuaternion::<T>::from_rotation(rhs.inverse()) };
    'b);

dual_quaternion_op_impl!(
    Div, div;
    (U4, U1), (U4, U1);
    self: UnitDualQuaternion<T>, rhs: UnitQuaternion<T>,
    Output = UnitDualQuaternion<T> => U3, U3;
    #[allow(clippy::suspicious_arithmetic_impl)]
    { self * UnitDualQuaternion::<T>::from_rotation(rhs.inverse()) };);

// UnitQuaternion ÷ UnitDualQuaternion
dual_quaternion_op_impl!(
    Div, div;
    (U4, U1), (U4, U1);
    self: &'a UnitQuaternion<T>, rhs: &'b UnitDualQuaternion<T>,
    Output = UnitDualQuaternion<T> => U1, U4;
    #[allow(clippy::suspicious_arithmetic_impl)]
    {
        UnitDualQuaternion::<T>::new_unchecked(
            DualQuaternion::from_real(self.clone().into_inner())
        ) * rhs.inverse()
    }; 'a, 'b);

dual_quaternion_op_impl!(
    Div, div;
    (U4, U1), (U4, U1);
    self: &'a UnitQuaternion<T>, rhs: UnitDualQuaternion<T>,
    Output = UnitDualQuaternion<T> => U3, U3;
    #[allow(clippy::suspicious_arithmetic_impl)]
    {
        UnitDualQuaternion::<T>::new_unchecked(
            DualQuaternion::from_real(self.clone().into_inner())
        ) * rhs.inverse()
    }; 'a);

dual_quaternion_op_impl!(
    Div, div;
    (U4, U1), (U4, U1);
    self: UnitQuaternion<T>, rhs: &'b UnitDualQuaternion<T>,
    Output = UnitDualQuaternion<T> => U3, U3;
    #[allow(clippy::suspicious_arithmetic_impl)]
    {
        UnitDualQuaternion::<T>::new_unchecked(
            DualQuaternion::from_real(self.into_inner())
        ) * rhs.inverse()
    }; 'b);

dual_quaternion_op_impl!(
    Div, div;
    (U4, U1), (U4, U1);
    self: UnitQuaternion<T>, rhs: UnitDualQuaternion<T>,
    Output = UnitDualQuaternion<T> => U3, U3;
    #[allow(clippy::suspicious_arithmetic_impl)]
    {
        UnitDualQuaternion::<T>::new_unchecked(
            DualQuaternion::from_real(self.into_inner())
        ) * rhs.inverse()
    };);

// UnitDualQuaternion × Translation3
dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U3, U1);
    self: &'a UnitDualQuaternion<T>, rhs: &'b Translation3<T>,
    Output = UnitDualQuaternion<T> => U3, U1;
    self * UnitDualQuaternion::<T>::from_parts(rhs.clone(), UnitQuaternion::identity());
    'a, 'b);

dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U3, U3);
    self: &'a UnitDualQuaternion<T>, rhs: Translation3<T>,
    Output = UnitDualQuaternion<T> => U3, U1;
    self * UnitDualQuaternion::<T>::from_parts(rhs, UnitQuaternion::identity());
    'a);

dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U3, U3);
    self: UnitDualQuaternion<T>, rhs: &'b Translation3<T>,
    Output = UnitDualQuaternion<T> => U3, U1;
    self * UnitDualQuaternion::<T>::from_parts(rhs.clone(), UnitQuaternion::identity());
    'b);

dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U3, U3);
    self: UnitDualQuaternion<T>, rhs: Translation3<T>,
    Output = UnitDualQuaternion<T> => U3, U1;
    self * UnitDualQuaternion::<T>::from_parts(rhs, UnitQuaternion::identity()); );

// UnitDualQuaternion ÷ Translation3
dual_quaternion_op_impl!(
    Div, div;
    (U4, U1), (U3, U1);
    self: &'a UnitDualQuaternion<T>, rhs: &'b Translation3<T>,
    Output = UnitDualQuaternion<T> => U3, U1;
    #[allow(clippy::suspicious_arithmetic_impl)]
    { self * UnitDualQuaternion::<T>::from_parts(rhs.inverse(), UnitQuaternion::identity()) };
    'a, 'b);

dual_quaternion_op_impl!(
    Div, div;
    (U4, U1), (U3, U3);
    self: &'a UnitDualQuaternion<T>, rhs: Translation3<T>,
    Output = UnitDualQuaternion<T> => U3, U1;
    #[allow(clippy::suspicious_arithmetic_impl)]
    { self * UnitDualQuaternion::<T>::from_parts(rhs.inverse(), UnitQuaternion::identity()) };
    'a);

dual_quaternion_op_impl!(
    Div, div;
    (U4, U1), (U3, U3);
    self: UnitDualQuaternion<T>, rhs: &'b Translation3<T>,
    Output = UnitDualQuaternion<T> => U3, U1;
    #[allow(clippy::suspicious_arithmetic_impl)]
    { self * UnitDualQuaternion::<T>::from_parts(rhs.inverse(), UnitQuaternion::identity()) };
    'b);

dual_quaternion_op_impl!(
    Div, div;
    (U4, U1), (U3, U3);
    self: UnitDualQuaternion<T>, rhs: Translation3<T>,
    Output = UnitDualQuaternion<T> => U3, U1;
    #[allow(clippy::suspicious_arithmetic_impl)]
    { self * UnitDualQuaternion::<T>::from_parts(rhs.inverse(), UnitQuaternion::identity()) };);

// Translation3 × UnitDualQuaternion
dual_quaternion_op_impl!(
    Mul, mul;
    (U3, U1), (U4, U1);
    self: &'b Translation3<T>, rhs: &'a UnitDualQuaternion<T>,
    Output = UnitDualQuaternion<T> => U3, U1;
    UnitDualQuaternion::<T>::from_parts(self.clone(), UnitQuaternion::identity()) * rhs;
    'a, 'b);

dual_quaternion_op_impl!(
    Mul, mul;
    (U3, U1), (U4, U1);
    self: &'a Translation3<T>, rhs: UnitDualQuaternion<T>,
    Output = UnitDualQuaternion<T> => U3, U1;
    UnitDualQuaternion::<T>::from_parts(self.clone(), UnitQuaternion::identity()) * rhs;
    'a);

dual_quaternion_op_impl!(
    Mul, mul;
    (U3, U1), (U4, U1);
    self: Translation3<T>, rhs: &'b UnitDualQuaternion<T>,
    Output = UnitDualQuaternion<T> => U3, U1;
    UnitDualQuaternion::<T>::from_parts(self, UnitQuaternion::identity()) * rhs;
    'b);

dual_quaternion_op_impl!(
    Mul, mul;
    (U3, U1), (U4, U1);
    self: Translation3<T>, rhs: UnitDualQuaternion<T>,
    Output = UnitDualQuaternion<T> => U3, U1;
    UnitDualQuaternion::<T>::from_parts(self, UnitQuaternion::identity()) * rhs;);

// Translation3 ÷ UnitDualQuaternion
dual_quaternion_op_impl!(
    Div, div;
    (U3, U1), (U4, U1);
    self: &'b Translation3<T>, rhs: &'a UnitDualQuaternion<T>,
    Output = UnitDualQuaternion<T> => U3, U1;
    UnitDualQuaternion::<T>::from_parts(self.clone(), UnitQuaternion::identity()) / rhs;
    'a, 'b);

dual_quaternion_op_impl!(
    Div, div;
    (U3, U1), (U4, U1);
    self: &'a Translation3<T>, rhs: UnitDualQuaternion<T>,
    Output = UnitDualQuaternion<T> => U3, U1;
    UnitDualQuaternion::<T>::from_parts(self.clone(), UnitQuaternion::identity()) / rhs;
    'a);

dual_quaternion_op_impl!(
    Div, div;
    (U3, U1), (U4, U1);
    self: Translation3<T>, rhs: &'b UnitDualQuaternion<T>,
    Output = UnitDualQuaternion<T> => U3, U1;
    UnitDualQuaternion::<T>::from_parts(self, UnitQuaternion::identity()) / rhs;
    'b);

dual_quaternion_op_impl!(
    Div, div;
    (U3, U1), (U4, U1);
    self: Translation3<T>, rhs: UnitDualQuaternion<T>,
    Output = UnitDualQuaternion<T> => U3, U1;
    UnitDualQuaternion::<T>::from_parts(self, UnitQuaternion::identity()) / rhs;);

// UnitDualQuaternion × Isometry3
dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U3, U1);
    self: &'a UnitDualQuaternion<T>, rhs: &'b Isometry3<T>,
    Output = UnitDualQuaternion<T> => U3, U1;
    self * UnitDualQuaternion::<T>::from_isometry(rhs);
    'a, 'b);

dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U3, U3);
    self: &'a UnitDualQuaternion<T>, rhs: Isometry3<T>,
    Output = UnitDualQuaternion<T> => U3, U1;
    self * UnitDualQuaternion::<T>::from_isometry(&rhs);
    'a);

dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U3, U3);
    self: UnitDualQuaternion<T>, rhs: &'b Isometry3<T>,
    Output = UnitDualQuaternion<T> => U3, U1;
    self * UnitDualQuaternion::<T>::from_isometry(rhs);
    'b);

dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U3, U3);
    self: UnitDualQuaternion<T>, rhs: Isometry3<T>,
    Output = UnitDualQuaternion<T> => U3, U1;
    self * UnitDualQuaternion::<T>::from_isometry(&rhs); );

// UnitDualQuaternion ÷ Isometry3
dual_quaternion_op_impl!(
    Div, div;
    (U4, U1), (U3, U1);
    self: &'a UnitDualQuaternion<T>, rhs: &'b Isometry3<T>,
    Output = UnitDualQuaternion<T> => U3, U1;
    // TODO: can we avoid the conversion to a rotation matrix?
    self / UnitDualQuaternion::<T>::from_isometry(rhs);
    'a, 'b);

dual_quaternion_op_impl!(
    Div, div;
    (U4, U1), (U3, U3);
    self: &'a UnitDualQuaternion<T>, rhs: Isometry3<T>,
    Output = UnitDualQuaternion<T> => U3, U1;
    self / UnitDualQuaternion::<T>::from_isometry(&rhs);
    'a);

dual_quaternion_op_impl!(
    Div, div;
    (U4, U1), (U3, U3);
    self: UnitDualQuaternion<T>, rhs: &'b Isometry3<T>,
    Output = UnitDualQuaternion<T> => U3, U1;
    self / UnitDualQuaternion::<T>::from_isometry(rhs);
    'b);

dual_quaternion_op_impl!(
    Div, div;
    (U4, U1), (U3, U3);
    self: UnitDualQuaternion<T>, rhs: Isometry3<T>,
    Output = UnitDualQuaternion<T> => U3, U1;
    self / UnitDualQuaternion::<T>::from_isometry(&rhs); );

// Isometry × UnitDualQuaternion
dual_quaternion_op_impl!(
    Mul, mul;
    (U3, U1), (U4, U1);
    self: &'a Isometry3<T>, rhs: &'b UnitDualQuaternion<T>,
    Output = UnitDualQuaternion<T> => U3, U1;
    UnitDualQuaternion::<T>::from_isometry(self) * rhs;
    'a, 'b);

dual_quaternion_op_impl!(
    Mul, mul;
    (U3, U1), (U4, U1);
    self: &'a Isometry3<T>, rhs: UnitDualQuaternion<T>,
    Output = UnitDualQuaternion<T> => U3, U1;
    UnitDualQuaternion::<T>::from_isometry(self) * rhs;
    'a);

dual_quaternion_op_impl!(
    Mul, mul;
    (U3, U1), (U4, U1);
    self: Isometry3<T>, rhs: &'b UnitDualQuaternion<T>,
    Output = UnitDualQuaternion<T> => U3, U1;
    UnitDualQuaternion::<T>::from_isometry(&self) * rhs;
    'b);

dual_quaternion_op_impl!(
    Mul, mul;
    (U3, U1), (U4, U1);
    self: Isometry3<T>, rhs: UnitDualQuaternion<T>,
    Output = UnitDualQuaternion<T> => U3, U1;
    UnitDualQuaternion::<T>::from_isometry(&self) * rhs; );

// Isometry ÷ UnitDualQuaternion
dual_quaternion_op_impl!(
    Div, div;
    (U3, U1), (U4, U1);
    self: &'a Isometry3<T>, rhs: &'b UnitDualQuaternion<T>,
    Output = UnitDualQuaternion<T> => U3, U1;
    // TODO: can we avoid the conversion from a rotation matrix?
    UnitDualQuaternion::<T>::from_isometry(self) / rhs;
    'a, 'b);

dual_quaternion_op_impl!(
    Div, div;
    (U3, U1), (U4, U1);
    self: &'a Isometry3<T>, rhs: UnitDualQuaternion<T>,
    Output = UnitDualQuaternion<T> => U3, U1;
    UnitDualQuaternion::<T>::from_isometry(self) / rhs;
    'a);

dual_quaternion_op_impl!(
    Div, div;
    (U3, U1), (U4, U1);
    self: Isometry3<T>, rhs: &'b UnitDualQuaternion<T>,
    Output = UnitDualQuaternion<T> => U3, U1;
    UnitDualQuaternion::<T>::from_isometry(&self) / rhs;
    'b);

dual_quaternion_op_impl!(
    Div, div;
    (U3, U1), (U4, U1);
    self: Isometry3<T>, rhs: UnitDualQuaternion<T>,
    Output = UnitDualQuaternion<T> => U3, U1;
    UnitDualQuaternion::<T>::from_isometry(&self) / rhs; );

// UnitDualQuaternion × Vector
dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U3, U1) for SB: Storage<T, U3> ;
    self: &'a UnitDualQuaternion<T>, rhs: &'b Vector<T, U3, SB>,
    Output = Vector3<T> => U3, U1;
    Unit::new_unchecked(self.as_ref().real.clone()) * rhs;
    'a, 'b);

dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U3, U1) for SB: Storage<T, U3> ;
    self: &'a UnitDualQuaternion<T>, rhs: Vector<T, U3, SB>,
    Output = Vector3<T> => U3, U1;
    self * &rhs;
    'a);

dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U3, U1) for SB: Storage<T, U3> ;
    self: UnitDualQuaternion<T>, rhs: &'b Vector<T, U3, SB>,
    Output = Vector3<T> => U3, U1;
    &self * rhs;
    'b);

dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U3, U1) for SB: Storage<T, U3> ;
    self: UnitDualQuaternion<T>, rhs: Vector<T, U3, SB>,
    Output = Vector3<T> => U3, U1;
    &self * &rhs; );

// UnitDualQuaternion × Point
dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U3, U1);
    self: &'a UnitDualQuaternion<T>, rhs: &'b Point3<T>,
    Output = Point3<T> => U3, U1;
    {
        let two: T = crate::convert(2.0f64);
        let q_point = Quaternion::from_parts(T::zero(), rhs.coords.clone());
        Point::from(
            ((self.as_ref().real.clone() * q_point + self.as_ref().dual.clone() * two) * self.as_ref().real.clone().conjugate())
                .vector()
                .into_owned(),
        )
    };
    'a, 'b);

dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U3, U1);
    self: &'a UnitDualQuaternion<T>, rhs: Point3<T>,
    Output = Point3<T> => U3, U1;
    self * &rhs;
    'a);

dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U3, U1);
    self: UnitDualQuaternion<T>, rhs: &'b Point3<T>,
    Output = Point3<T> => U3, U1;
    &self * rhs;
    'b);

dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U3, U1);
    self: UnitDualQuaternion<T>, rhs: Point3<T>,
    Output = Point3<T> => U3, U1;
    &self * &rhs; );

// UnitDualQuaternion × Unit<Vector>
dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U3, U1) for SB: Storage<T, U3> ;
    self: &'a UnitDualQuaternion<T>, rhs: &'b Unit<Vector<T, U3, SB>>,
    Output = Unit<Vector3<T>> => U3, U4;
    Unit::new_unchecked(self * rhs.as_ref());
    'a, 'b);

dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U3, U1) for SB: Storage<T, U3> ;
    self: &'a UnitDualQuaternion<T>, rhs: Unit<Vector<T, U3, SB>>,
    Output = Unit<Vector3<T>> => U3, U4;
    Unit::new_unchecked(self * rhs.into_inner());
    'a);

dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U3, U1) for SB: Storage<T, U3> ;
    self: UnitDualQuaternion<T>, rhs: &'b Unit<Vector<T, U3, SB>>,
    Output = Unit<Vector3<T>> => U3, U4;
    Unit::new_unchecked(self * rhs.as_ref());
    'b);

dual_quaternion_op_impl!(
    Mul, mul;
    (U4, U1), (U3, U1) for SB: Storage<T, U3> ;
    self: UnitDualQuaternion<T>, rhs: Unit<Vector<T, U3, SB>>,
    Output = Unit<Vector3<T>> => U3, U4;
    Unit::new_unchecked(self * rhs.into_inner()); );

macro_rules! left_scalar_mul_impl(
    ($($T: ty),* $(,)*) => {$(
        impl Mul<DualQuaternion<$T>> for $T {
            type Output = DualQuaternion<$T>;

            #[inline]
            fn mul(self, right: DualQuaternion<$T>) -> Self::Output {
                DualQuaternion::from_real_and_dual(
                    self * right.real,
                    self * right.dual
                )
            }
        }

        impl<'b> Mul<&'b DualQuaternion<$T>> for $T {
            type Output = DualQuaternion<$T>;

            #[inline]
            fn mul(self, right: &'b DualQuaternion<$T>) -> Self::Output {
                DualQuaternion::from_real_and_dual(
                    self * &right.real,
                    self * &right.dual
                )
            }
        }
    )*}
);

left_scalar_mul_impl!(f32, f64);

macro_rules! dual_quaternion_op_impl(
    ($OpAssign: ident, $op_assign: ident;
     ($LhsRDim: ident, $LhsCDim: ident), ($RhsRDim: ident, $RhsCDim: ident);
     $lhs: ident: $Lhs: ty, $rhs: ident: $Rhs: ty $(=> $VDimA: ty, $VDimB: ty)*;
     $action: expr_2021; $($lives: tt),*) => {
        impl<$($lives ,)* T: SimdRealField> $OpAssign<$Rhs> for $Lhs
            where T::Element: SimdRealField {

            #[inline]
            fn $op_assign(&mut $lhs, $rhs: $Rhs) {
                $action
            }
        }
    }
);

// DualQuaternion += DualQuaternion
dual_quaternion_op_impl!(
    AddAssign, add_assign;
    (U4, U1), (U4, U1);
    self: DualQuaternion<T>, rhs: &'b DualQuaternion<T>;
    {
        self.real += &rhs.real;
        self.dual += &rhs.dual;
    };
    'b);

dual_quaternion_op_impl!(
    AddAssign, add_assign;
    (U4, U1), (U4, U1);
    self: DualQuaternion<T>, rhs: DualQuaternion<T>;
    {
        self.real += rhs.real;
        self.dual += rhs.dual;
    };);

// DualQuaternion -= DualQuaternion
dual_quaternion_op_impl!(
    SubAssign, sub_assign;
    (U4, U1), (U4, U1);
    self: DualQuaternion<T>, rhs: &'b DualQuaternion<T>;
    {
        self.real -= &rhs.real;
        self.dual -= &rhs.dual;
    };
    'b);

dual_quaternion_op_impl!(
    SubAssign, sub_assign;
    (U4, U1), (U4, U1);
    self: DualQuaternion<T>, rhs: DualQuaternion<T>;
    {
        self.real -= rhs.real;
        self.dual -= rhs.dual;
    };);

// DualQuaternion ×= DualQuaternion
dual_quaternion_op_impl!(
    MulAssign, mul_assign;
    (U4, U1), (U4, U1);
    self: DualQuaternion<T>, rhs: &'b DualQuaternion<T>;
    {
        let res = &*self * rhs;
        self.real.coords.copy_from(&res.real.coords);
        self.dual.coords.copy_from(&res.dual.coords);
    };
    'b);

dual_quaternion_op_impl!(
    MulAssign, mul_assign;
    (U4, U1), (U4, U1);
    self: DualQuaternion<T>, rhs: DualQuaternion<T>;
    *self *= &rhs;);

// DualQuaternion ×= UnitDualQuaternion
dual_quaternion_op_impl!(
    MulAssign, mul_assign;
    (U4, U1), (U4, U1);
    self: DualQuaternion<T>, rhs: &'b UnitDualQuaternion<T>;
    {
        let res = &*self * rhs;
        self.real.coords.copy_from(&res.real.coords);
        self.dual.coords.copy_from(&res.dual.coords);
    };
    'b);

dual_quaternion_op_impl!(
    MulAssign, mul_assign;
    (U4, U1), (U4, U1);
    self: DualQuaternion<T>, rhs: UnitDualQuaternion<T>;
    *self *= &rhs; );

// DualQuaternion ÷= UnitDualQuaternion
dual_quaternion_op_impl!(
    DivAssign, div_assign;
    (U4, U1), (U4, U1);
    self: DualQuaternion<T>, rhs: &'b UnitDualQuaternion<T>;
    {
        let res = &*self / rhs;
        self.real.coords.copy_from(&res.real.coords);
        self.dual.coords.copy_from(&res.dual.coords);
    };
    'b);

dual_quaternion_op_impl!(
    DivAssign, div_assign;
    (U4, U1), (U4, U1);
    self: DualQuaternion<T>, rhs: UnitDualQuaternion<T>;
    *self /= &rhs; );

// UnitDualQuaternion ×= UnitDualQuaternion
dual_quaternion_op_impl!(
    MulAssign, mul_assign;
    (U4, U1), (U4, U1);
    self: UnitDualQuaternion<T>, rhs: &'b UnitDualQuaternion<T>;
    {
        let res = &*self * rhs;
        self.as_mut_unchecked().real.coords.copy_from(&res.as_ref().real.coords);
        self.as_mut_unchecked().dual.coords.copy_from(&res.as_ref().dual.coords);
    };
    'b);

dual_quaternion_op_impl!(
    MulAssign, mul_assign;
    (U4, U1), (U4, U1);
    self: UnitDualQuaternion<T>, rhs: UnitDualQuaternion<T>;
    *self *= &rhs; );

// UnitDualQuaternion ÷= UnitDualQuaternion
dual_quaternion_op_impl!(
    DivAssign, div_assign;
    (U4, U1), (U4, U1);
    self: UnitDualQuaternion<T>, rhs: &'b UnitDualQuaternion<T>;
    {
        let res = &*self / rhs;
        self.as_mut_unchecked().real.coords.copy_from(&res.as_ref().real.coords);
        self.as_mut_unchecked().dual.coords.copy_from(&res.as_ref().dual.coords);
    };
    'b);

dual_quaternion_op_impl!(
    DivAssign, div_assign;
    (U4, U1), (U4, U1);
    self: UnitDualQuaternion<T>, rhs: UnitDualQuaternion<T>;
    *self /= &rhs; );

// UnitDualQuaternion ×= UnitQuaternion
dual_quaternion_op_impl!(
    MulAssign, mul_assign;
    (U4, U1), (U4, U1);
    self: UnitDualQuaternion<T>, rhs: UnitQuaternion<T>;
    {
        let res = &*self * UnitDualQuaternion::from_rotation(rhs);
        self.as_mut_unchecked().real.coords.copy_from(&res.as_ref().real.coords);
        self.as_mut_unchecked().dual.coords.copy_from(&res.as_ref().dual.coords);
    };);

dual_quaternion_op_impl!(
    MulAssign, mul_assign;
    (U4, U1), (U4, U1);
    self: UnitDualQuaternion<T>, rhs: &'b UnitQuaternion<T>;
    *self *= rhs.clone(); 'b);

// UnitDualQuaternion ÷= UnitQuaternion
dual_quaternion_op_impl!(
    DivAssign, div_assign;
    (U4, U1), (U4, U1);
    self: UnitDualQuaternion<T>, rhs: &'b UnitQuaternion<T>;
    #[allow(clippy::suspicious_op_assign_impl)]
    {
        let res = &*self * UnitDualQuaternion::from_rotation(rhs.inverse());
        self.as_mut_unchecked().real.coords.copy_from(&res.as_ref().real.coords);
        self.as_mut_unchecked().dual.coords.copy_from(&res.as_ref().dual.coords);
    };
    'b);

dual_quaternion_op_impl!(
    DivAssign, div_assign;
    (U4, U1), (U4, U1);
    self: UnitDualQuaternion<T>, rhs: UnitQuaternion<T>;
    *self /= &rhs; );

// UnitDualQuaternion ×= Translation3
dual_quaternion_op_impl!(
    MulAssign, mul_assign;
    (U4, U1), (U4, U1);
    self: UnitDualQuaternion<T>, rhs: Translation3<T>;
    {
        let res = &*self * UnitDualQuaternion::from_parts(rhs, UnitQuaternion::identity());
        self.as_mut_unchecked().real.coords.copy_from(&res.as_ref().real.coords);
        self.as_mut_unchecked().dual.coords.copy_from(&res.as_ref().dual.coords);
    };);

dual_quaternion_op_impl!(
    MulAssign, mul_assign;
    (U4, U1), (U4, U1);
    self: UnitDualQuaternion<T>, rhs: &'b Translation3<T>;
    *self *= rhs.clone(); 'b);

// UnitDualQuaternion ÷= Translation3
dual_quaternion_op_impl!(
    DivAssign, div_assign;
    (U4, U1), (U4, U1);
    self: UnitDualQuaternion<T>, rhs: &'b Translation3<T>;
    #[allow(clippy::suspicious_op_assign_impl)]
    {
        let res = &*self * UnitDualQuaternion::from_parts(rhs.inverse(), UnitQuaternion::identity());
        self.as_mut_unchecked().real.coords.copy_from(&res.as_ref().real.coords);
        self.as_mut_unchecked().dual.coords.copy_from(&res.as_ref().dual.coords);
    };
    'b);

dual_quaternion_op_impl!(
    DivAssign, div_assign;
    (U4, U1), (U4, U1);
    self: UnitDualQuaternion<T>, rhs: Translation3<T>;
    *self /= &rhs; );

// UnitDualQuaternion ×= Isometry3
dual_quaternion_op_impl!(
    MulAssign, mul_assign;
    (U4, U1), (U3, U1);
    self: UnitDualQuaternion<T>, rhs: &'b Isometry3<T> => U3, U1;
    {
        let res = &*self * rhs;
        self.as_mut_unchecked().real.coords.copy_from(&res.as_ref().real.coords);
        self.as_mut_unchecked().dual.coords.copy_from(&res.as_ref().dual.coords);
    };
    'b);

dual_quaternion_op_impl!(
    MulAssign, mul_assign;
    (U4, U1), (U3, U1);
    self: UnitDualQuaternion<T>, rhs: Isometry3<T> => U3, U1;
    *self *= &rhs; );

// UnitDualQuaternion ÷= Isometry3
dual_quaternion_op_impl!(
    DivAssign, div_assign;
    (U4, U1), (U3, U1);
    self: UnitDualQuaternion<T>, rhs: &'b Isometry3<T> => U3, U1;
    {
        let res = &*self / rhs;
        self.as_mut_unchecked().real.coords.copy_from(&res.as_ref().real.coords);
        self.as_mut_unchecked().dual.coords.copy_from(&res.as_ref().dual.coords);
    };
    'b);

dual_quaternion_op_impl!(
    DivAssign, div_assign;
    (U4, U1), (U3, U1);
    self: UnitDualQuaternion<T>, rhs: Isometry3<T> => U3, U1;
    *self /= &rhs; );

macro_rules! scalar_op_impl(
    ($($Op: ident, $op: ident, $OpAssign: ident, $op_assign: ident);* $(;)*) => {$(
        impl<T: SimdRealField> $Op<T> for DualQuaternion<T>
         where T::Element: SimdRealField {
            type Output = DualQuaternion<T>;

            #[inline]
            fn $op(self, n: T) -> Self::Output {
                DualQuaternion::from_real_and_dual(
                    self.real.clone().$op(n.clone()),
                    self.dual.clone().$op(n)
                )
            }
        }

        impl<'a, T: SimdRealField> $Op<T> for &'a DualQuaternion<T>
         where T::Element: SimdRealField {
            type Output = DualQuaternion<T>;

            #[inline]
            fn $op(self, n: T) -> Self::Output {
                DualQuaternion::from_real_and_dual(
                    self.real.clone().$op(n.clone()),
                    self.dual.clone().$op(n)
                )
            }
        }

        impl<T: SimdRealField> $OpAssign<T> for DualQuaternion<T>
         where T::Element: SimdRealField {

            #[inline]
            fn $op_assign(&mut self, n: T) {
                self.real.$op_assign(n.clone());
                self.dual.$op_assign(n);
            }
        }
    )*}
);

scalar_op_impl!(
    Mul, mul, MulAssign, mul_assign;
    Div, div, DivAssign, div_assign;
);

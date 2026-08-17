// The macros break if the references are taken out, for some reason.
#![allow(clippy::op_ref)]

use std::ops::{Div, DivAssign, Mul, MulAssign};

use crate::base::storage::Storage;
use crate::base::{Const, Unit, Vector, Vector2};
use crate::geometry::{Isometry, Point2, Rotation, Similarity, Translation, UnitComplex};
use simba::simd::SimdRealField;

/*
 * This file provides:
 * ===================
 *
 * UnitComplex  × UnitComplex
 * UnitComplex  × Rotation -> UnitComplex
 * Rotation × UnitComplex  -> UnitComplex
 *
 * UnitComplex  ÷ UnitComplex
 * UnitComplex  ÷ Rotation -> UnitComplex
 * Rotation ÷ UnitComplex  -> UnitComplex
 *
 *
 * UnitComplex × Point
 * UnitComplex × Vector
 * UnitComplex × Unit<T>
 *
 * UnitComplex × Isometry<UnitComplex>
 * UnitComplex × Similarity<UnitComplex>
 * UnitComplex × Translation -> Isometry<UnitComplex>
 *
 * (Assignment Operators)
 *
 * UnitComplex  ×= UnitComplex
 * UnitComplex  ×= Rotation
 *
 * UnitComplex  ÷= UnitComplex
 * UnitComplex  ÷= Rotation
 *
 * Rotation ×= UnitComplex
 * Rotation ÷= UnitComplex
 *
 */

// UnitComplex × UnitComplex
impl<T: SimdRealField> Mul<Self> for UnitComplex<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Unit::new_unchecked(self.into_inner() * rhs.into_inner())
    }
}

impl<T: SimdRealField> Mul<UnitComplex<T>> for &UnitComplex<T>
where
    T::Element: SimdRealField,
{
    type Output = UnitComplex<T>;

    #[inline]
    fn mul(self, rhs: UnitComplex<T>) -> Self::Output {
        Unit::new_unchecked(self.complex() * rhs.into_inner())
    }
}

impl<'b, T: SimdRealField> Mul<&'b UnitComplex<T>> for UnitComplex<T>
where
    T::Element: SimdRealField,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: &'b UnitComplex<T>) -> Self::Output {
        Unit::new_unchecked(self.into_inner() * rhs.as_ref())
    }
}

impl<'b, T: SimdRealField> Mul<&'b UnitComplex<T>> for &UnitComplex<T>
where
    T::Element: SimdRealField,
{
    type Output = UnitComplex<T>;

    #[inline]
    fn mul(self, rhs: &'b UnitComplex<T>) -> Self::Output {
        Unit::new_unchecked(self.complex() * rhs.as_ref())
    }
}

// UnitComplex ÷ UnitComplex
impl<T: SimdRealField> Div<Self> for UnitComplex<T>
where
    T::Element: SimdRealField,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        #[allow(clippy::suspicious_arithmetic_impl)]
        Unit::new_unchecked(self.into_inner() * rhs.conjugate().into_inner())
    }
}

impl<T: SimdRealField> Div<UnitComplex<T>> for &UnitComplex<T>
where
    T::Element: SimdRealField,
{
    type Output = UnitComplex<T>;

    #[inline]
    fn div(self, rhs: UnitComplex<T>) -> Self::Output {
        #[allow(clippy::suspicious_arithmetic_impl)]
        Unit::new_unchecked(self.complex() * rhs.conjugate().into_inner())
    }
}

impl<'b, T: SimdRealField> Div<&'b UnitComplex<T>> for UnitComplex<T>
where
    T::Element: SimdRealField,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: &'b UnitComplex<T>) -> Self::Output {
        #[allow(clippy::suspicious_arithmetic_impl)]
        Unit::new_unchecked(self.into_inner() * rhs.conjugate().into_inner())
    }
}

impl<'b, T: SimdRealField> Div<&'b UnitComplex<T>> for &UnitComplex<T>
where
    T::Element: SimdRealField,
{
    type Output = UnitComplex<T>;

    #[inline]
    fn div(self, rhs: &'b UnitComplex<T>) -> Self::Output {
        #[allow(clippy::suspicious_arithmetic_impl)]
        Unit::new_unchecked(self.complex() * rhs.conjugate().into_inner())
    }
}

macro_rules! complex_op_impl(
    ($Op: ident, $op: ident;
     $($Storage: ident: $StoragesBound: ident $(<$($BoundParam: ty),*>)*),*;
     $lhs: ident: $Lhs: ty, $rhs: ident: $Rhs: ty, Output = $Result: ty;
     $action: expr_2021; $($lives: tt),*) => {
        impl<$($lives ,)* T: SimdRealField $(, $Storage: $StoragesBound $(<$($BoundParam),*>)*)*> $Op<$Rhs> for $Lhs
            where T::Element: SimdRealField {
            type Output = $Result;

            #[inline]
            fn $op($lhs, $rhs: $Rhs) -> Self::Output {
                $action
            }
        }
    }
);

macro_rules! complex_op_impl_all(
    ($Op: ident, $op: ident;
     $($Storage: ident: $StoragesBound: ident $(<$($BoundParam: ty),*>)*),*;
     $lhs: ident: $Lhs: ty, $rhs: ident: $Rhs: ty, Output = $Result: ty;
     [val val] => $action_val_val: expr_2021;
     [ref val] => $action_ref_val: expr_2021;
     [val ref] => $action_val_ref: expr_2021;
     [ref ref] => $action_ref_ref: expr_2021;) => {

    complex_op_impl!($Op, $op;
                     $($Storage: $StoragesBound $(<$($BoundParam),*>)*),*;
                     $lhs: $Lhs, $rhs: $Rhs, Output = $Result;
                     $action_val_val; );

    complex_op_impl!($Op, $op;
                     $($Storage: $StoragesBound $(<$($BoundParam),*>)*),*;
                     $lhs: &'a $Lhs, $rhs: $Rhs, Output = $Result;
                     $action_ref_val; 'a);

    complex_op_impl!($Op, $op;
                     $($Storage: $StoragesBound $(<$($BoundParam),*>)*),*;
                     $lhs: $Lhs, $rhs: &'b $Rhs, Output = $Result;
                     $action_val_ref; 'b);

    complex_op_impl!($Op, $op;
                     $($Storage: $StoragesBound $(<$($BoundParam),*>)*),*;
                     $lhs: &'a $Lhs, $rhs: &'b $Rhs, Output = $Result;
                     $action_ref_ref; 'a, 'b);


    }
);

// UnitComplex × Rotation
complex_op_impl_all!(
    Mul, mul;
    ;
    self: UnitComplex<T>, rhs: Rotation<T, 2>, Output = UnitComplex<T>;
    [val val] => &self * &rhs;
    [ref val] =>  self * &rhs;
    [val ref] => &self *  rhs;
    [ref ref] =>  self * UnitComplex::from_rotation_matrix(rhs);
);

// UnitComplex ÷ Rotation
complex_op_impl_all!(
    Div, div;
    ;
    self: UnitComplex<T>, rhs: Rotation<T, 2>, Output = UnitComplex<T>;
    [val val] => &self / &rhs;
    [ref val] =>  self / &rhs;
    [val ref] => &self /  rhs;
    [ref ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self * UnitComplex::from_rotation_matrix(rhs).inverse() };
);

// Rotation × UnitComplex
complex_op_impl_all!(
    Mul, mul;
    ;
    self: Rotation<T, 2>, rhs: UnitComplex<T>, Output = UnitComplex<T>;
    [val val] => &self * &rhs;
    [ref val] =>  self * &rhs;
    [val ref] => &self *  rhs;
    [ref ref] => UnitComplex::from_rotation_matrix(self) * rhs;
);

// Rotation ÷ UnitComplex
complex_op_impl_all!(
    Div, div;
    ;
    self: Rotation<T, 2>, rhs: UnitComplex<T>, Output = UnitComplex<T>;
    [val val] => &self / &rhs;
    [ref val] =>  self / &rhs;
    [val ref] => &self /  rhs;
    [ref ref] => #[allow(clippy::suspicious_arithmetic_impl)] { UnitComplex::from_rotation_matrix(self) * rhs.inverse() };
);

// UnitComplex × Point
complex_op_impl_all!(
    Mul, mul;
    ;
    self: UnitComplex<T>, rhs: Point2<T>, Output = Point2<T>;
    [val val] => &self * &rhs;
    [ref val] =>  self * &rhs;
    [val ref] => &self *  rhs;
    [ref ref] => Point2::from(self * &rhs.coords);
);

// UnitComplex × Vector
complex_op_impl_all!(
    Mul, mul;
    S: Storage<T, Const<2>>;
    self: UnitComplex<T>, rhs: Vector<T, Const<2>, S>, Output = Vector2<T>;
    [val val] => &self * &rhs;
    [ref val] =>  self * &rhs;
    [val ref] => &self *  rhs;
    [ref ref] => {
        let i = self.as_ref().im.clone();
        let r = self.as_ref().re.clone();
        Vector2::new(r.clone() * rhs[0].clone() - i.clone() * rhs[1].clone(), i * rhs[0].clone() + r * rhs[1].clone())
    };
);

// UnitComplex × Unit<Vector>
complex_op_impl_all!(
    Mul, mul;
    S: Storage<T, Const<2>>;
    self: UnitComplex<T>, rhs: Unit<Vector<T, Const<2>, S>>, Output = Unit<Vector2<T>>;
    [val val] => &self * &rhs;
    [ref val] =>  self * &rhs;
    [val ref] => &self *  rhs;
    [ref ref] => Unit::new_unchecked(self * rhs.as_ref());
);

// UnitComplex × Isometry<UnitComplex>
complex_op_impl_all!(
    Mul, mul;
    ;
    self: UnitComplex<T>, rhs: Isometry<T, UnitComplex<T>, 2>,
    Output = Isometry<T, UnitComplex<T>, 2>;
    [val val] => &self * &rhs;
    [ref val] =>  self * &rhs;
    [val ref] => &self *  rhs;
    [ref ref] => {
        let shift = self * &rhs.translation.vector;
        Isometry::from_parts(Translation::from(shift), self * &rhs.rotation)
    };
);

// UnitComplex × Similarity<UnitComplex>
complex_op_impl_all!(
    Mul, mul;
    ;
    self: UnitComplex<T>, rhs: Similarity<T, UnitComplex<T>, 2>,
    Output = Similarity<T, UnitComplex<T>, 2>;
    [val val] => &self * &rhs;
    [ref val] =>  self * &rhs;
    [val ref] => &self *  rhs;
    [ref ref] => Similarity::from_isometry(self * &rhs.isometry, rhs.scaling());
);

// UnitComplex × Translation
complex_op_impl_all!(
    Mul, mul;
    ;
    self: UnitComplex<T>, rhs: Translation<T, 2>,
    Output = Isometry<T, UnitComplex<T>, 2>;
    [val val] => Isometry::from_parts(Translation::from(&self *  rhs.vector), self);
    [ref val] => Isometry::from_parts(Translation::from( self *  rhs.vector), self.clone());
    [val ref] => Isometry::from_parts(Translation::from(&self * &rhs.vector), self);
    [ref ref] => Isometry::from_parts(Translation::from( self * &rhs.vector), self.clone());
);

// Translation × UnitComplex
complex_op_impl_all!(
    Mul, mul;
    ;
    self: Translation<T, 2>, right: UnitComplex<T>,
    Output = Isometry<T, UnitComplex<T>, 2>;
    [val val] => Isometry::from_parts(self,   right);
    [ref val] => Isometry::from_parts(self.clone(),  right);
    [val ref] => Isometry::from_parts(self,  right.clone());
    [ref ref] => Isometry::from_parts(self.clone(), right.clone());
);

// UnitComplex ×= UnitComplex
impl<T: SimdRealField> MulAssign<UnitComplex<T>> for UnitComplex<T>
where
    T::Element: SimdRealField,
{
    #[inline]
    fn mul_assign(&mut self, rhs: UnitComplex<T>) {
        *self = self.clone() * rhs
    }
}

impl<'b, T: SimdRealField> MulAssign<&'b UnitComplex<T>> for UnitComplex<T>
where
    T::Element: SimdRealField,
{
    #[inline]
    fn mul_assign(&mut self, rhs: &'b UnitComplex<T>) {
        *self = self.clone() * rhs
    }
}

// UnitComplex /= UnitComplex
impl<T: SimdRealField> DivAssign<UnitComplex<T>> for UnitComplex<T>
where
    T::Element: SimdRealField,
{
    #[inline]
    fn div_assign(&mut self, rhs: UnitComplex<T>) {
        *self = self.clone() / rhs
    }
}

impl<'b, T: SimdRealField> DivAssign<&'b UnitComplex<T>> for UnitComplex<T>
where
    T::Element: SimdRealField,
{
    #[inline]
    fn div_assign(&mut self, rhs: &'b UnitComplex<T>) {
        *self = self.clone() / rhs
    }
}

// UnitComplex ×= Rotation
impl<T: SimdRealField> MulAssign<Rotation<T, 2>> for UnitComplex<T>
where
    T::Element: SimdRealField,
{
    #[inline]
    fn mul_assign(&mut self, rhs: Rotation<T, 2>) {
        *self = self.clone() * rhs
    }
}

impl<'b, T: SimdRealField> MulAssign<&'b Rotation<T, 2>> for UnitComplex<T>
where
    T::Element: SimdRealField,
{
    #[inline]
    fn mul_assign(&mut self, rhs: &'b Rotation<T, 2>) {
        *self = self.clone() * rhs
    }
}

// UnitComplex ÷= Rotation
impl<T: SimdRealField> DivAssign<Rotation<T, 2>> for UnitComplex<T>
where
    T::Element: SimdRealField,
{
    #[inline]
    fn div_assign(&mut self, rhs: Rotation<T, 2>) {
        *self = self.clone() / rhs
    }
}

impl<'b, T: SimdRealField> DivAssign<&'b Rotation<T, 2>> for UnitComplex<T>
where
    T::Element: SimdRealField,
{
    #[inline]
    fn div_assign(&mut self, rhs: &'b Rotation<T, 2>) {
        *self = self.clone() / rhs
    }
}

// Rotation ×= UnitComplex
impl<T: SimdRealField> MulAssign<UnitComplex<T>> for Rotation<T, 2>
where
    T::Element: SimdRealField,
{
    #[inline]
    fn mul_assign(&mut self, rhs: UnitComplex<T>) {
        self.mul_assign(rhs.to_rotation_matrix())
    }
}

impl<'b, T: SimdRealField> MulAssign<&'b UnitComplex<T>> for Rotation<T, 2>
where
    T::Element: SimdRealField,
{
    #[inline]
    fn mul_assign(&mut self, rhs: &'b UnitComplex<T>) {
        self.mul_assign(rhs.clone().to_rotation_matrix())
    }
}

// Rotation ÷= UnitComplex
impl<T: SimdRealField> DivAssign<UnitComplex<T>> for Rotation<T, 2>
where
    T::Element: SimdRealField,
{
    #[inline]
    fn div_assign(&mut self, rhs: UnitComplex<T>) {
        self.div_assign(rhs.to_rotation_matrix())
    }
}

impl<'b, T: SimdRealField> DivAssign<&'b UnitComplex<T>> for Rotation<T, 2>
where
    T::Element: SimdRealField,
{
    #[inline]
    fn div_assign(&mut self, rhs: &'b UnitComplex<T>) {
        self.div_assign(rhs.clone().to_rotation_matrix())
    }
}

// The macros break if the references are taken out, for some reason.
#![allow(clippy::op_ref)]

use num::{One, Zero};
use std::ops::{Div, DivAssign, Mul, MulAssign};

use simba::scalar::{ClosedAddAssign, ClosedMulAssign};
use simba::simd::SimdRealField;

use crate::Scalar;
use crate::base::{SVector, Unit};

use crate::geometry::{
    AbstractRotation, Isometry, Point, Rotation, Translation, UnitComplex, UnitQuaternion,
};

// TODO: there are several cloning of rotations that we could probably get rid of (but we didn't
// yet because that would require to add a bound like `where for<'a, 'b> &'a R: Mul<&'b R, Output = R>`
// which is quite ugly.

/*
 *
 * In this file, we provide:
 * =========================
 *
 *
 * (Operators)
 *
 * Isometry × Isometry
 * Isometry × R
 *
 *
 * Isometry ÷ Isometry
 * Isometry ÷ R
 *
 * Isometry × Point
 * Isometry × Vector
 * Isometry × Unit<Vector>
 *
 *
 * Isometry    × Translation
 * Translation × Isometry
 * Translation × R           -> Isometry<R>
 *
 * NOTE: The following are provided explicitly because we can't have R × Isometry.
 * Rotation       × Isometry<Rotation>
 * UnitQuaternion × Isometry<UnitQuaternion>
 *
 * Rotation       ÷ Isometry<Rotation>
 * UnitQuaternion ÷ Isometry<UnitQuaternion>
 *
 * Rotation       × Translation -> Isometry<Rotation>
 * UnitQuaternion × Translation -> Isometry<UnitQuaternion>
 *
 *
 * (Assignment Operators)
 *
 * Isometry ×= Translation
 *
 * Isometry ×= Isometry
 * Isometry ×= R
 *
 * Isometry ÷= Isometry
 * Isometry ÷= R
 *
 */

macro_rules! isometry_binop_impl(
    ($Op: ident, $op: ident;
     $lhs: ident: $Lhs: ty, $rhs: ident: $Rhs: ty, Output = $Output: ty;
     $action: expr_2021; $($lives: tt),*) => {
        impl<$($lives ,)* T: SimdRealField, R, const D: usize> $Op<$Rhs> for $Lhs
            where T::Element: SimdRealField,
                  R: AbstractRotation<T, D>, {
            type Output = $Output;

            #[inline]
            fn $op($lhs, $rhs: $Rhs) -> Self::Output {
                $action
            }
        }
    }
);

macro_rules! isometry_binop_impl_all(
    ($Op: ident, $op: ident;
     $lhs: ident: $Lhs: ty, $rhs: ident: $Rhs: ty, Output = $Output: ty;
     [val val] => $action_val_val: expr_2021;
     [ref val] => $action_ref_val: expr_2021;
     [val ref] => $action_val_ref: expr_2021;
     [ref ref] => $action_ref_ref: expr_2021;) => {
        isometry_binop_impl!(
            $Op, $op;
            $lhs: $Lhs, $rhs: $Rhs, Output = $Output;
            $action_val_val; );

        isometry_binop_impl!(
            $Op, $op;
            $lhs: &'a $Lhs, $rhs: $Rhs, Output = $Output;
            $action_ref_val; 'a);

        isometry_binop_impl!(
            $Op, $op;
            $lhs: $Lhs, $rhs: &'b $Rhs, Output = $Output;
            $action_val_ref; 'b);

        isometry_binop_impl!(
            $Op, $op;
            $lhs: &'a $Lhs, $rhs: &'b $Rhs, Output = $Output;
            $action_ref_ref; 'a, 'b);
    }
);

macro_rules! isometry_binop_assign_impl_all(
    ($OpAssign: ident, $op_assign: ident;
     $lhs: ident: $Lhs: ty, $rhs: ident: $Rhs: ty;
     [val] => $action_val: expr_2021;
     [ref] => $action_ref: expr_2021;) => {
        impl<T: SimdRealField, R, const D: usize> $OpAssign<$Rhs> for $Lhs
            where T::Element: SimdRealField,
                  R: AbstractRotation<T, D> {
            #[inline]
            fn $op_assign(&mut $lhs, $rhs: $Rhs) {
                $action_val
            }
        }

        impl<'b, T: SimdRealField, R, const D: usize> $OpAssign<&'b $Rhs> for $Lhs
            where T::Element: SimdRealField,
                  R: AbstractRotation<T, D> {
            #[inline]
            fn $op_assign(&mut $lhs, $rhs: &'b $Rhs) {
                $action_ref
            }
        }
    }
);

// Isometry × Isometry
// Isometry ÷ Isometry
isometry_binop_impl_all!(
    Mul, mul;
    self: Isometry<T, R, D>, rhs: Isometry<T, R, D>, Output = Isometry<T, R, D>;
    [val val] => &self * &rhs;
    [ref val] => self * &rhs;
    [val ref] => &self * rhs;
    [ref ref] => {
        let shift = self.rotation.transform_vector(&rhs.translation.vector);

        #[allow(clippy::suspicious_arithmetic_impl)]
        Isometry::from_parts(Translation::from(&self.translation.vector + shift),
                             self.rotation.clone() * rhs.rotation.clone()) // TODO: too bad we have to clone.
    };
);

isometry_binop_impl_all!(
    Div, div;
    self: Isometry<T, R, D>, rhs: Isometry<T, R, D>, Output = Isometry<T, R, D>;
    [val val] => #[allow(clippy::suspicious_arithmetic_impl)] { self * rhs.inverse() };
    [ref val] => #[allow(clippy::suspicious_arithmetic_impl)] { self * rhs.inverse() };
    [val ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self * rhs.inverse() };
    [ref ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self * rhs.inverse() };
);

// Isometry ×= Translation
isometry_binop_assign_impl_all!(
    MulAssign, mul_assign;
    self: Isometry<T, R, D>, rhs: Translation<T, D>;
    [val] => *self *= &rhs;
    [ref] => #[allow(clippy::suspicious_op_assign_impl)] {
        let shift = self.rotation.transform_vector(&rhs.vector);
        self.translation.vector += shift;
    };
);

// Isometry ×= Isometry
// Isometry ÷= Isometry
isometry_binop_assign_impl_all!(
    MulAssign, mul_assign;
    self: Isometry<T, R, D>, rhs: Isometry<T, R, D>;
    [val] => *self *= &rhs;
    [ref] => {
        let shift = self.rotation.transform_vector(&rhs.translation.vector);
        self.translation.vector += shift;
        self.rotation *= rhs.rotation.clone();
    };
);

isometry_binop_assign_impl_all!(
    DivAssign, div_assign;
    self: Isometry<T, R, D>, rhs: Isometry<T, R, D>;
    [val] => *self /= &rhs;
    [ref] => #[allow(clippy::suspicious_op_assign_impl)] { *self *= rhs.inverse() };
);

// Isometry ×= R
// Isometry ÷= R
md_assign_impl_all!(
    MulAssign, mul_assign where T: SimdRealField for T::Element: SimdRealField;
    (Const<D>, U1), (Const<D>, Const<D>)
    const D; for; where;
    self: Isometry<T, Rotation<T, D>, D>, rhs: Rotation<T, D>;
    [val] => self.rotation *= rhs;
    [ref] => self.rotation *= rhs.clone();
);

md_assign_impl_all!(
    DivAssign, div_assign where T: SimdRealField for T::Element: SimdRealField;
    (Const<D>, U1), (Const<D>, Const<D>)
    const D; for; where;
    self: Isometry<T, Rotation<T, D>, D>, rhs: Rotation<T, D>;
    // TODO: don't invert explicitly?
    [val] => #[allow(clippy::suspicious_op_assign_impl)] { *self *= rhs.inverse() };
    [ref] => #[allow(clippy::suspicious_op_assign_impl)] { *self *= rhs.inverse() };
);

md_assign_impl_all!(
    MulAssign, mul_assign where T: SimdRealField for T::Element: SimdRealField;
    (U3, U3), (U3, U3)
    const; for; where;
    self: Isometry<T, UnitQuaternion<T>, 3>, rhs: UnitQuaternion<T>;
    [val] => self.rotation *= rhs;
    [ref] => self.rotation *= rhs.clone();
);

md_assign_impl_all!(
    DivAssign, div_assign where T: SimdRealField for T::Element: SimdRealField;
    (U3, U3), (U3, U3)
    const; for; where;
    self: Isometry<T, UnitQuaternion<T>, 3>, rhs: UnitQuaternion<T>;
    // TODO: don't invert explicitly?
    [val] => #[allow(clippy::suspicious_op_assign_impl)] { *self *= rhs.inverse() };
    [ref] => #[allow(clippy::suspicious_op_assign_impl)] { *self *= rhs.inverse() };
);

md_assign_impl_all!(
    MulAssign, mul_assign where T: SimdRealField for T::Element: SimdRealField;
    (U2, U2), (U2, U2)
    const; for; where;
    self: Isometry<T, UnitComplex<T>, 2>, rhs: UnitComplex<T>;
    [val] => self.rotation *= rhs;
    [ref] => self.rotation *= rhs.clone();
);

md_assign_impl_all!(
    DivAssign, div_assign where T: SimdRealField for T::Element: SimdRealField;
    (U2, U2), (U2, U2)
    const; for; where;
    self: Isometry<T, UnitComplex<T>, 2>, rhs: UnitComplex<T>;
    // TODO: don't invert explicitly?
    [val] => #[allow(clippy::suspicious_op_assign_impl)] { *self *= rhs.inverse() };
    [ref] => #[allow(clippy::suspicious_op_assign_impl)] { *self *= rhs.inverse() };
);

// Isometry × Point
isometry_binop_impl_all!(
    Mul, mul;
    self: Isometry<T, R, D>, right: Point<T, D>, Output = Point<T, D>;
    [val val] => self.translation  * self.rotation.transform_point(&right);
    [ref val] => &self.translation * self.rotation.transform_point(&right);
    [val ref] => self.translation  * self.rotation.transform_point(right);
    [ref ref] => &self.translation * self.rotation.transform_point(right);
);

// Isometry × Vector
isometry_binop_impl_all!(
    Mul, mul;
    // TODO: because of `transform_vector`, we cant use a generic storage type for the rhs vector,
    // i.e., right: Vector<T, D, S> where S: Storage<T, D>.
    self: Isometry<T, R, D>, right: SVector<T, D>, Output = SVector<T, D>;
    [val val] => self.rotation.transform_vector(&right);
    [ref val] => self.rotation.transform_vector(&right);
    [val ref] => self.rotation.transform_vector(right);
    [ref ref] => self.rotation.transform_vector(right);
);

// Isometry × Unit<Vector>
isometry_binop_impl_all!(
    Mul, mul;
    // TODO: because of `transform_vector`, we cant use a generic storage type for the rhs vector,
    // i.e., right: Vector<T, D, S> where S: Storage<T, D>.
    self: Isometry<T, R, D>, right: Unit<SVector<T, D>>, Output = Unit<SVector<T, D>>;
    [val val] => Unit::new_unchecked(self.rotation.transform_vector(right.as_ref()));
    [ref val] => Unit::new_unchecked(self.rotation.transform_vector(right.as_ref()));
    [val ref] => Unit::new_unchecked(self.rotation.transform_vector(right.as_ref()));
    [ref ref] => Unit::new_unchecked(self.rotation.transform_vector(right.as_ref()));
);

// Isometry × Translation
isometry_binop_impl_all!(
    Mul, mul;
    self: Isometry<T, R, D>, right: Translation<T, D>, Output = Isometry<T, R, D>;
    [val val] => &self * &right;
    [ref val] => self * &right;
    [val ref] => &self * right;
    [ref ref] => {
        #[allow(clippy::suspicious_arithmetic_impl)]
        let new_tr = &self.translation.vector + self.rotation.transform_vector(&right.vector);
        Isometry::from_parts(Translation::from(new_tr), self.rotation.clone())
    };
);

// Translation × Isometry
isometry_binop_impl_all!(
    Mul, mul;
    self: Translation<T, D>, right: Isometry<T, R, D>, Output = Isometry<T, R, D>;
    [val val] => Isometry::from_parts(self * right.translation, right.rotation);
    [ref val] => Isometry::from_parts(self * &right.translation, right.rotation);
    [val ref] => Isometry::from_parts(self * &right.translation, right.rotation.clone());
    [ref ref] => Isometry::from_parts(self * &right.translation, right.rotation.clone());
);

macro_rules! isometry_from_composition_impl(
    ($Op: ident, $op: ident;
     $($Dims: ident),*;
     $lhs: ident: $Lhs: ty, $rhs: ident: $Rhs: ty, Output = $Output: ty;
     $action: expr_2021; $($lives: tt),*) => {
        impl<$($lives ,)* T: SimdRealField $(, const $Dims: usize)*> $Op<$Rhs> for $Lhs
            where T::Element: SimdRealField {
            type Output = $Output;

            #[inline]
            fn $op($lhs, $rhs: $Rhs) -> Self::Output {
                $action
            }
        }
    }
);

macro_rules! isometry_from_composition_impl_all(
    ($Op: ident, $op: ident;
     $($Dims: ident),*;
     $lhs: ident: $Lhs: ty, $rhs: ident: $Rhs: ty, Output = $Output: ty;
     [val val] => $action_val_val: expr_2021;
     [ref val] => $action_ref_val: expr_2021;
     [val ref] => $action_val_ref: expr_2021;
     [ref ref] => $action_ref_ref: expr_2021;) => {

        isometry_from_composition_impl!(
            $Op, $op;
            $($Dims),*;
            $lhs: $Lhs, $rhs: $Rhs, Output = $Output;
            $action_val_val; );

        isometry_from_composition_impl!(
            $Op, $op;
            $($Dims),*;
            $lhs: &'a $Lhs, $rhs: $Rhs, Output = $Output;
            $action_ref_val; 'a);

        isometry_from_composition_impl!(
            $Op, $op;
            $($Dims),*;
            $lhs: $Lhs, $rhs: &'b $Rhs, Output = $Output;
            $action_val_ref; 'b);

        isometry_from_composition_impl!(
            $Op, $op;
            $($Dims),*;
            $lhs: &'a $Lhs, $rhs: &'b $Rhs, Output = $Output;
            $action_ref_ref; 'a, 'b);
    }
);

// Rotation × Translation
isometry_from_composition_impl_all!(
    Mul, mul;
    D;
    self: Rotation<T, D>, right: Translation<T, D>, Output = Isometry<T, Rotation<T, D>, D>;
    [val val] => Isometry::from_parts(Translation::from(&self * right.vector),  self);
    [ref val] => Isometry::from_parts(Translation::from(self * right.vector),   self.clone());
    [val ref] => Isometry::from_parts(Translation::from(&self * &right.vector), self);
    [ref ref] => Isometry::from_parts(Translation::from(self * &right.vector),  self.clone());
);

// UnitQuaternion × Translation
isometry_from_composition_impl_all!(
    Mul, mul;
    ;
    self: UnitQuaternion<T>, right: Translation<T, 3>,
    Output = Isometry<T, UnitQuaternion<T>, 3>;
    [val val] => Isometry::from_parts(Translation::from(&self *  right.vector), self);
    [ref val] => Isometry::from_parts(Translation::from( self *  right.vector), self.clone());
    [val ref] => Isometry::from_parts(Translation::from(&self * &right.vector), self);
    [ref ref] => Isometry::from_parts(Translation::from( self * &right.vector), self.clone());
);

// Isometry × Rotation
isometry_from_composition_impl_all!(
    Mul, mul;
    D;
    self: Isometry<T, Rotation<T, D>, D>, rhs: Rotation<T, D>,
    Output = Isometry<T, Rotation<T, D>, D>;
    [val val] => Isometry::from_parts(self.translation, self.rotation * rhs);
    [ref val] => Isometry::from_parts(self.translation.clone(), self.rotation.clone() * rhs);
    [val ref] => Isometry::from_parts(self.translation, self.rotation * rhs.clone());
    [ref ref] => Isometry::from_parts(self.translation.clone(), self.rotation.clone() * rhs.clone());
);

// Rotation × Isometry
isometry_from_composition_impl_all!(
    Mul, mul;
    D;
    self: Rotation<T, D>, right: Isometry<T, Rotation<T, D>, D>,
    Output = Isometry<T, Rotation<T, D>, D>;
    [val val] => &self * &right;
    [ref val] =>  self * &right;
    [val ref] => &self * right;
    [ref ref] => {
        let shift = self * &right.translation.vector;
        Isometry::from_parts(Translation::from(shift), self * &right.rotation)
    };
);

// Isometry ÷ Rotation
isometry_from_composition_impl_all!(
    Div, div;
    D;
    self: Isometry<T, Rotation<T, D>, D>, rhs: Rotation<T, D>,
    Output = Isometry<T, Rotation<T, D>, D>;
    [val val] => Isometry::from_parts(self.translation, self.rotation / rhs);
    [ref val] => Isometry::from_parts(self.translation.clone(), self.rotation.clone() / rhs);
    [val ref] => Isometry::from_parts(self.translation, self.rotation / rhs.clone());
    [ref ref] => Isometry::from_parts(self.translation.clone(), self.rotation.clone() / rhs.clone());
);

// Rotation ÷ Isometry
isometry_from_composition_impl_all!(
    Div, div;
    D;
    self: Rotation<T, D>, right: Isometry<T, Rotation<T, D>, D>,
    Output = Isometry<T, Rotation<T, D>, D>;
    // TODO: don't call inverse explicitly?
    [val val] => #[allow(clippy::suspicious_arithmetic_impl)] { self * right.inverse() };
    [ref val] => #[allow(clippy::suspicious_arithmetic_impl)] { self * right.inverse() };
    [val ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self * right.inverse() };
    [ref ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self * right.inverse() };
);

// Isometry × UnitQuaternion
isometry_from_composition_impl_all!(
    Mul, mul;
    ;
    self: Isometry<T, UnitQuaternion<T>, 3>, rhs: UnitQuaternion<T>,
    Output = Isometry<T, UnitQuaternion<T>, 3>;
    [val val] => Isometry::from_parts(self.translation, self.rotation * rhs);
    [ref val] => Isometry::from_parts(self.translation.clone(), self.rotation.clone() * rhs);
    [val ref] => Isometry::from_parts(self.translation, self.rotation * rhs.clone());
    [ref ref] => Isometry::from_parts(self.translation.clone(), self.rotation.clone() * rhs.clone());
);

// UnitQuaternion × Isometry
isometry_from_composition_impl_all!(
    Mul, mul;
    ;
    self: UnitQuaternion<T>, right: Isometry<T, UnitQuaternion<T>, 3>,
    Output = Isometry<T, UnitQuaternion<T>, 3>;
    [val val] => &self * &right;
    [ref val] =>  self * &right;
    [val ref] => &self * right;
    [ref ref] => {
        let shift = self * &right.translation.vector;
        Isometry::from_parts(Translation::from(shift), self * &right.rotation)
    };
);

// Isometry ÷ UnitQuaternion
isometry_from_composition_impl_all!(
    Div, div;
    ;
    self: Isometry<T, UnitQuaternion<T>, 3>, rhs: UnitQuaternion<T>,
    Output = Isometry<T, UnitQuaternion<T>, 3>;
    [val val] => Isometry::from_parts(self.translation, self.rotation / rhs);
    [ref val] => Isometry::from_parts(self.translation.clone(), self.rotation.clone() / rhs);
    [val ref] => Isometry::from_parts(self.translation, self.rotation / rhs.clone());
    [ref ref] => Isometry::from_parts(self.translation.clone(), self.rotation.clone() / rhs.clone());
);

// UnitQuaternion ÷ Isometry
isometry_from_composition_impl_all!(
    Div, div;
    ;
    self: UnitQuaternion<T>, right: Isometry<T, UnitQuaternion<T>, 3>,
    Output = Isometry<T, UnitQuaternion<T>, 3>;
    // TODO: don't call inverse explicitly?
    [val val] => #[allow(clippy::suspicious_arithmetic_impl)] { self * right.inverse() };
    [ref val] => #[allow(clippy::suspicious_arithmetic_impl)] { self * right.inverse() };
    [val ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self * right.inverse() };
    [ref ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self * right.inverse() };
);

// Translation × Rotation
isometry_from_composition_impl_all!(
    Mul, mul;
    D;
    self: Translation<T, D>, right: Rotation<T, D>, Output = Isometry<T, Rotation<T, D>, D>;
    [val val] => Isometry::from_parts(self, right);
    [ref val] => Isometry::from_parts(self.clone(), right);
    [val ref] => Isometry::from_parts(self, right.clone());
    [ref ref] => Isometry::from_parts(self.clone(), right.clone());
);

// Translation × UnitQuaternion
isometry_from_composition_impl_all!(
    Mul, mul;
    ;
    self: Translation<T, 3>, right: UnitQuaternion<T>, Output = Isometry<T, UnitQuaternion<T>, 3>;
    [val val] => Isometry::from_parts(self, right);
    [ref val] => Isometry::from_parts(self.clone(), right);
    [val ref] => Isometry::from_parts(self, right.clone());
    [ref ref] => Isometry::from_parts(self.clone(), right.clone());
);

// Isometry × UnitComplex
isometry_from_composition_impl_all!(
    Mul, mul;
    ;
    self: Isometry<T, UnitComplex<T>, 2>, rhs: UnitComplex<T>,
    Output = Isometry<T, UnitComplex<T>, 2>;
    [val val] => Isometry::from_parts(self.translation, self.rotation * rhs);
    [ref val] => Isometry::from_parts(self.translation.clone(), self.rotation.clone() * rhs);
    [val ref] => Isometry::from_parts(self.translation, self.rotation * rhs.clone());
    [ref ref] => Isometry::from_parts(self.translation.clone(), self.rotation.clone() * rhs.clone());
);

// Isometry ÷ UnitComplex
isometry_from_composition_impl_all!(
    Div, div;
    ;
    self: Isometry<T, UnitComplex<T>, 2>, rhs: UnitComplex<T>,
    Output = Isometry<T, UnitComplex<T>, 2>;
    [val val] => Isometry::from_parts(self.translation, self.rotation / rhs);
    [ref val] => Isometry::from_parts(self.translation.clone(), self.rotation.clone() / rhs);
    [val ref] => Isometry::from_parts(self.translation, self.rotation / rhs.clone());
    [ref ref] => Isometry::from_parts(self.translation.clone(), self.rotation.clone() / rhs.clone());
);

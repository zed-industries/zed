// The macros break if the references are taken out, for some reason.
#![allow(clippy::op_ref)]

use num::{One, Zero};
use std::ops::{Div, DivAssign, Mul, MulAssign};

use simba::scalar::{ClosedAddAssign, ClosedMulAssign};
use simba::simd::SimdRealField;

use crate::base::{SVector, Scalar};

use crate::geometry::{
    AbstractRotation, Isometry, Point, Rotation, Similarity, Translation, UnitComplex,
    UnitQuaternion,
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
 * Similarity × Similarity
 * Similarity × R
 * Similarity × Isometry
 *
 * Isometry × Similarity
 * Isometry ÷ Similarity
 *
 *
 * Similarity ÷ Similarity
 * Similarity ÷ R
 * Similarity ÷ Isometry
 *
 * Similarity × Point
 * Similarity × Vector
 *
 *
 * Similarity  × Translation
 * Translation × Similarity
 *
 * NOTE: The following are provided explicitly because we can't have R × Similarity.
 * Rotation   × Similarity<Rotation>
 * UnitQuaternion × Similarity<UnitQuaternion>
 *
 * Rotation   ÷ Similarity<Rotation>
 * UnitQuaternion ÷ Similarity<UnitQuaternion>
 *
 * (Assignment Operators)
 *
 * Similarity ×= Translation
 *
 * Similarity ×= Similarity
 * Similarity ×= Isometry
 * Similarity ×= R
 *
 * Similarity ÷= Similarity
 * Similarity ÷= Isometry
 * Similarity ÷= R
 *
 */

// XXX: code duplication: those macros are the same as for the isometry.
macro_rules! similarity_binop_impl(
    ($Op: ident, $op: ident;
     $lhs: ident: $Lhs: ty, $rhs: ident: $Rhs: ty, Output = $Output: ty;
     $action: expr_2021; $($lives: tt),*) => {
        impl<$($lives ,)* T: SimdRealField, R, const D: usize> $Op<$Rhs> for $Lhs
            where T::Element: SimdRealField,
                  R: AbstractRotation<T, D> {
            type Output = $Output;

            #[inline]
            fn $op($lhs, $rhs: $Rhs) -> Self::Output {
                $action
            }
        }
    }
);

macro_rules! similarity_binop_impl_all(
    ($Op: ident, $op: ident;
     $lhs: ident: $Lhs: ty, $rhs: ident: $Rhs: ty, Output = $Output: ty;
     [val val] => $action_val_val: expr_2021;
     [ref val] => $action_ref_val: expr_2021;
     [val ref] => $action_val_ref: expr_2021;
     [ref ref] => $action_ref_ref: expr_2021;) => {
        similarity_binop_impl!(
            $Op, $op;
            $lhs: $Lhs, $rhs: $Rhs, Output = $Output;
            $action_val_val; );

        similarity_binop_impl!(
            $Op, $op;
            $lhs: &'a $Lhs, $rhs: $Rhs, Output = $Output;
            $action_ref_val; 'a);

        similarity_binop_impl!(
            $Op, $op;
            $lhs: $Lhs, $rhs: &'b $Rhs, Output = $Output;
            $action_val_ref; 'b);

        similarity_binop_impl!(
            $Op, $op;
            $lhs: &'a $Lhs, $rhs: &'b $Rhs, Output = $Output;
            $action_ref_ref; 'a, 'b);
    }
);

macro_rules! similarity_binop_assign_impl_all(
    ($OpAssign: ident, $op_assign: ident;
     $lhs: ident: $Lhs: ty, $rhs: ident: $Rhs: ty;
     [val] => $action_val: expr_2021;
     [ref] => $action_ref: expr_2021;) => {
        impl<T: SimdRealField, R, const D: usize> $OpAssign<$Rhs> for $Lhs
            where T::Element: SimdRealField,
                  R: AbstractRotation<T, D>{
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

// Similarity × Similarity
// Similarity ÷ Similarity
similarity_binop_impl_all!(
    Mul, mul;
    self: Similarity<T, R, D>, rhs: Similarity<T, R, D>, Output = Similarity<T, R, D>;
    [val val] => &self * &rhs;
    [ref val] =>  self * &rhs;
    [val ref] => &self *  rhs;
    [ref ref] => {
        let mut res = self * &rhs.isometry;
        res.prepend_scaling_mut(rhs.scaling());
        res
    };
);

similarity_binop_impl_all!(
    Div, div;
    self: Similarity<T, R, D>, rhs: Similarity<T, R, D>, Output = Similarity<T, R, D>;
    [val val] => #[allow(clippy::suspicious_arithmetic_impl)] { self * rhs.inverse() };
    [ref val] => #[allow(clippy::suspicious_arithmetic_impl)] { self * rhs.inverse() };
    [val ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self * rhs.inverse() };
    [ref ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self * rhs.inverse() };
);

// Similarity ×= Translation
similarity_binop_assign_impl_all!(
    MulAssign, mul_assign;
    self: Similarity<T, R, D>, rhs: Translation<T, D>;
    [val] => *self *= &rhs;
    [ref] => {
        let shift = self.isometry.rotation.transform_vector(&rhs.vector) * self.scaling();
        self.isometry.translation.vector += shift;
    };
);

// Similarity ×= Similarity
// Similarity ÷= Similarity
similarity_binop_assign_impl_all!(
    MulAssign, mul_assign;
    self: Similarity<T, R, D>, rhs: Similarity<T, R, D>;
    [val] => *self *= &rhs;
    [ref] => {
        *self *= &rhs.isometry;
        self.prepend_scaling_mut(rhs.scaling());
    };
);

similarity_binop_assign_impl_all!(
    DivAssign, div_assign;
    self: Similarity<T, R, D>, rhs: Similarity<T, R, D>;
    [val] => *self /= &rhs;
    // TODO: don't invert explicitly.
    [ref] => #[allow(clippy::suspicious_op_assign_impl)] { *self *= rhs.inverse() };
);

// Similarity ×= Isometry
// Similarity ÷= Isometry
similarity_binop_assign_impl_all!(
    MulAssign, mul_assign;
    self: Similarity<T, R, D>, rhs: Isometry<T, R, D>;
    [val] => *self *= &rhs;
    [ref] => {
        let shift = self.isometry.rotation.transform_vector(&rhs.translation.vector) * self.scaling();
        self.isometry.translation.vector += shift;
        self.isometry.rotation *= rhs.rotation.clone();
    };
);

similarity_binop_assign_impl_all!(
    DivAssign, div_assign;
    self: Similarity<T, R, D>, rhs: Isometry<T, R, D>;
    [val] => *self /= &rhs;
    // TODO: don't invert explicitly.
    [ref] => #[allow(clippy::suspicious_op_assign_impl)] { *self *= rhs.inverse() };
);

// Similarity ×= R
// Similarity ÷= R
md_assign_impl_all!(
    MulAssign, mul_assign where T: SimdRealField for T::Element: SimdRealField;
    (Const<D>, U1), (Const<D>, Const<D>)
    const D; for; where;
    self: Similarity<T, Rotation<T, D>, D>, rhs: Rotation<T, D>;
    [val] => self.isometry.rotation *= rhs;
    [ref] => self.isometry.rotation *= rhs.clone();
);

md_assign_impl_all!(
    DivAssign, div_assign where T: SimdRealField for T::Element: SimdRealField;
    (Const<D>, U1), (Const<D>, Const<D>)
    const D; for; where;
    self: Similarity<T, Rotation<T, D>, D>, rhs: Rotation<T, D>;
    // TODO: don't invert explicitly?
    [val] => #[allow(clippy::suspicious_op_assign_impl)] { *self *= rhs.inverse() };
    [ref] => #[allow(clippy::suspicious_op_assign_impl)] { *self *= rhs.inverse() };
);

md_assign_impl_all!(
    MulAssign, mul_assign where T: SimdRealField for T::Element: SimdRealField;
    (U3, U3), (U3, U3)
    const; for; where;
    self: Similarity<T, UnitQuaternion<T>, 3>, rhs: UnitQuaternion<T>;
    [val] => self.isometry.rotation *= rhs;
    [ref] => self.isometry.rotation *= rhs.clone();
);

md_assign_impl_all!(
    DivAssign, div_assign where T: SimdRealField for T::Element: SimdRealField;
    (U3, U3), (U3, U3)
    const; for; where;
    self: Similarity<T, UnitQuaternion<T>, 3>, rhs: UnitQuaternion<T>;
    // TODO: don't invert explicitly?
    [val] => #[allow(clippy::suspicious_op_assign_impl)] { *self *= rhs.inverse() };
    [ref] => #[allow(clippy::suspicious_op_assign_impl)] { *self *= rhs.inverse() };
);

md_assign_impl_all!(
    MulAssign, mul_assign where T: SimdRealField for T::Element: SimdRealField;
    (U2, U2), (U2, U2)
    const; for; where;
    self: Similarity<T, UnitComplex<T>, 2>, rhs: UnitComplex<T>;
    [val] => self.isometry.rotation *= rhs;
    [ref] => self.isometry.rotation *= rhs.clone();
);

md_assign_impl_all!(
    DivAssign, div_assign where T: SimdRealField for T::Element: SimdRealField;
    (U2, U2), (U2, U2)
    const; for; where;
    self: Similarity<T, UnitComplex<T>, 2>, rhs: UnitComplex<T>;
    // TODO: don't invert explicitly?
    [val] => #[allow(clippy::suspicious_op_assign_impl)] { *self *= rhs.inverse() };
    [ref] => #[allow(clippy::suspicious_op_assign_impl)] { *self *= rhs.inverse() };
);

// Similarity × Isometry
// Similarity ÷ Isometry
similarity_binop_impl_all!(
    Mul, mul;
    self: Similarity<T, R, D>, rhs: Isometry<T, R, D>, Output = Similarity<T, R, D>;
    [val val] => &self * &rhs;
    [ref val] => self * &rhs;
    [val ref] => &self * rhs;
    [ref ref] => {
        let shift = self.isometry.rotation.transform_vector(&rhs.translation.vector) * self.scaling();
        Similarity::from_parts(
            #[allow(clippy::suspicious_arithmetic_impl)]
            Translation::from(&self.isometry.translation.vector + shift),
            self.isometry.rotation.clone() * rhs.rotation.clone(),
            self.scaling())
    };
);

similarity_binop_impl_all!(
    Div, div;
    self: Similarity<T, R, D>, rhs: Isometry<T, R, D>, Output = Similarity<T, R, D>;
    [val val] => #[allow(clippy::suspicious_arithmetic_impl)] { self * rhs.inverse() };
    [ref val] => #[allow(clippy::suspicious_arithmetic_impl)] { self * rhs.inverse() };
    [val ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self * rhs.inverse() };
    [ref ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self * rhs.inverse() };
);

// Isometry × Similarity
// Isometry ÷ Similarity
similarity_binop_impl_all!(
    Mul, mul;
    self: Isometry<T, R, D>, rhs: Similarity<T, R, D>, Output = Similarity<T, R, D>;
    [val val] => {
        let scaling = rhs.scaling();
        Similarity::from_isometry(self * rhs.isometry, scaling)
    };
    [ref val] => {
        let scaling = rhs.scaling();
        Similarity::from_isometry(self * rhs.isometry, scaling)
    };
    [val ref] => {
        let scaling = rhs.scaling();
        Similarity::from_isometry(self * &rhs.isometry, scaling)
    };
    [ref ref] => {
        let scaling = rhs.scaling();
        Similarity::from_isometry(self * &rhs.isometry, scaling)
    };
);

similarity_binop_impl_all!(
    Div, div;
    self: Isometry<T, R, D>, rhs: Similarity<T, R, D>, Output = Similarity<T, R, D>;
    [val val] => #[allow(clippy::suspicious_arithmetic_impl)] { self * rhs.inverse() };
    [ref val] => #[allow(clippy::suspicious_arithmetic_impl)] { self * rhs.inverse() };
    [val ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self * rhs.inverse() };
    [ref ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self * rhs.inverse() };
);

// Similarity × Point
similarity_binop_impl_all!(
    Mul, mul;
    self: Similarity<T, R, D>, right: Point<T, D>, Output = Point<T, D>;
    [val val] => {
        let scaling = self.scaling();
        self.isometry.translation * (self.isometry.rotation.transform_point(&right) * scaling)
    };
    [ref val] => &self.isometry.translation * (self.isometry.rotation.transform_point(&right) * self.scaling());
    [val ref] => {
        let scaling = self.scaling();
        self.isometry.translation * (self.isometry.rotation.transform_point(right) * scaling)
    };
    [ref ref] => &self.isometry.translation * (self.isometry.rotation.transform_point(right) * self.scaling());
);

// Similarity × Vector
similarity_binop_impl_all!(
    Mul, mul;
    self: Similarity<T, R, D>, right: SVector<T, D>, Output = SVector<T, D>;
    [val val] => self.isometry.rotation.transform_vector(&right) * self.scaling();
    [ref val] => self.isometry.rotation.transform_vector(&right) * self.scaling();
    [val ref] => self.isometry.rotation.transform_vector(right) * self.scaling();
    [ref ref] => self.isometry.rotation.transform_vector(right) * self.scaling();
);

// Similarity × Translation
similarity_binop_impl_all!(
    Mul, mul;
    self: Similarity<T, R, D>, right: Translation<T, D>, Output = Similarity<T, R, D>;
    [val val] => &self * &right;
    [ref val] => self * &right;
    [val ref] => &self * right;
    [ref ref] => {
        let shift = self.isometry.rotation.transform_vector(&right.vector) * self.scaling();
        Similarity::from_parts(
            #[allow(clippy::suspicious_arithmetic_impl)]
            Translation::from(&self.isometry.translation.vector + shift),
            self.isometry.rotation.clone(),
            self.scaling())
    };
);

// Translation × Similarity
similarity_binop_impl_all!(
    Mul, mul;
    self: Translation<T, D>, right: Similarity<T, R, D>, Output = Similarity<T, R, D>;
    [val val] => {
        let scaling = right.scaling();
        Similarity::from_isometry(self * right.isometry, scaling)
    };
    [ref val] => {
        let scaling = right.scaling();
        Similarity::from_isometry(self * right.isometry, scaling)
    };
    [val ref] => Similarity::from_isometry(self * &right.isometry, right.scaling());
    [ref ref] => Similarity::from_isometry(self * &right.isometry, right.scaling());
);

macro_rules! similarity_from_composition_impl(
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

macro_rules! similarity_from_composition_impl_all(
    ($Op: ident, $op: ident;
     $($Dims: ident),*;
     $lhs: ident: $Lhs: ty, $rhs: ident: $Rhs: ty, Output = $Output: ty;
     [val val] => $action_val_val: expr_2021;
     [ref val] => $action_ref_val: expr_2021;
     [val ref] => $action_val_ref: expr_2021;
     [ref ref] => $action_ref_ref: expr_2021;) => {

        similarity_from_composition_impl!(
            $Op, $op;
            $($Dims),*;
            $lhs: $Lhs, $rhs: $Rhs, Output = $Output;
            $action_val_val; );

        similarity_from_composition_impl!(
            $Op, $op;
            $($Dims),*;
            $lhs: &'a $Lhs, $rhs: $Rhs, Output = $Output;
            $action_ref_val; 'a);

        similarity_from_composition_impl!(
            $Op, $op;
            $($Dims),*;
            $lhs: $Lhs, $rhs: &'b $Rhs, Output = $Output;
            $action_val_ref; 'b);

        similarity_from_composition_impl!(
            $Op, $op;
            $($Dims),*;
            $lhs: &'a $Lhs, $rhs: &'b $Rhs, Output = $Output;
            $action_ref_ref; 'a, 'b);
    }
);

// Similarity × Rotation
similarity_from_composition_impl_all!(
    Mul, mul;
    D;
    self: Similarity<T, Rotation<T, D>, D>, rhs: Rotation<T, D>,
    Output = Similarity<T, Rotation<T, D>, D>;
    [val val] => {
        let scaling = self.scaling();
        Similarity::from_isometry(self.isometry * rhs, scaling)
    };
    [ref val] => Similarity::from_isometry(&self.isometry * rhs, self.scaling());
    [val ref] => {
        let scaling = self.scaling();
        Similarity::from_isometry(self.isometry * rhs, scaling)
    };
    [ref ref] => Similarity::from_isometry(&self.isometry * rhs, self.scaling());
);

// Rotation × Similarity
similarity_from_composition_impl_all!(
    Mul, mul;
    D;
    self: Rotation<T, D>, right: Similarity<T, Rotation<T, D>, D>,
    Output = Similarity<T, Rotation<T, D>, D>;
    [val val] => &self * &right;
    [ref val] =>  self * &right;
    [val ref] => &self *  right;
    [ref ref] => Similarity::from_isometry(self * &right.isometry, right.scaling());
);

// Similarity ÷ Rotation
similarity_from_composition_impl_all!(
    Div, div;
    D;
    self: Similarity<T, Rotation<T, D>, D>, rhs: Rotation<T, D>,
    Output = Similarity<T, Rotation<T, D>, D>;
    [val val] => {
        let scaling = self.scaling();
        Similarity::from_isometry(self.isometry / rhs, scaling)
    };
    [ref val] => Similarity::from_isometry(&self.isometry / rhs, self.scaling());
    [val ref] => {
        let scaling = self.scaling();
        Similarity::from_isometry(self.isometry / rhs, scaling)
    };
    [ref ref] => Similarity::from_isometry(&self.isometry / rhs, self.scaling());
);

// Rotation ÷ Similarity
similarity_from_composition_impl_all!(
    Div, div;
    D;
    self: Rotation<T, D>, right: Similarity<T, Rotation<T, D>, D>,
    Output = Similarity<T, Rotation<T, D>, D>;
    // TODO: don't call inverse explicitly?
    [val val] => #[allow(clippy::suspicious_arithmetic_impl)] { self * right.inverse() };
    [ref val] => #[allow(clippy::suspicious_arithmetic_impl)] { self * right.inverse() };
    [val ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self * right.inverse() };
    [ref ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self * right.inverse() };
);

// Similarity × UnitQuaternion
similarity_from_composition_impl_all!(
    Mul, mul;
    ;
    self: Similarity<T, UnitQuaternion<T>, 3>, rhs: UnitQuaternion<T>,
    Output = Similarity<T, UnitQuaternion<T>, 3>;
    [val val] => {
        let scaling = self.scaling();
        Similarity::from_isometry(self.isometry * rhs, scaling)
    };
    [ref val] => Similarity::from_isometry(&self.isometry * rhs, self.scaling());
    [val ref] => {
        let scaling = self.scaling();
        Similarity::from_isometry(self.isometry * rhs, scaling)
    };
    [ref ref] => Similarity::from_isometry(&self.isometry * rhs, self.scaling());
);

// UnitQuaternion × Similarity
similarity_from_composition_impl_all!(
    Mul, mul;
    ;
    self: UnitQuaternion<T>, right: Similarity<T, UnitQuaternion<T>, 3>,
    Output = Similarity<T, UnitQuaternion<T>, 3>;
    [val val] => &self * &right;
    [ref val] =>  self * &right;
    [val ref] => &self *  right;
    [ref ref] => Similarity::from_isometry(self * &right.isometry, right.scaling());
);

// Similarity ÷ UnitQuaternion
similarity_from_composition_impl_all!(
    Div, div;
    ;
    self: Similarity<T, UnitQuaternion<T>, 3>, rhs: UnitQuaternion<T>,
    Output = Similarity<T, UnitQuaternion<T>, 3>;
    [val val] => {
        let scaling = self.scaling();
        Similarity::from_isometry(self.isometry / rhs, scaling)
    };
    [ref val] => Similarity::from_isometry(&self.isometry / rhs, self.scaling());
    [val ref] => {
        let scaling = self.scaling();
        Similarity::from_isometry(self.isometry / rhs, scaling)
    };
    [ref ref] => Similarity::from_isometry(&self.isometry / rhs, self.scaling());
);

// UnitQuaternion ÷ Similarity
similarity_from_composition_impl_all!(
    Div, div;
    ;
    self: UnitQuaternion<T>, right: Similarity<T, UnitQuaternion<T>, 3>,
    Output = Similarity<T, UnitQuaternion<T>, 3>;
    // TODO: don't call inverse explicitly?
    [val val] => #[allow(clippy::suspicious_arithmetic_impl)] { self * right.inverse() };
    [ref val] => #[allow(clippy::suspicious_arithmetic_impl)] { self * right.inverse() };
    [val ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self * right.inverse() };
    [ref ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self * right.inverse() };
);

// Similarity × UnitComplex
similarity_from_composition_impl_all!(
    Mul, mul;
    ;
    self: Similarity<T, UnitComplex<T>, 2>, rhs: UnitComplex<T>,
    Output = Similarity<T, UnitComplex<T>, 2>;
    [val val] => {
        let scaling = self.scaling();
        Similarity::from_isometry(self.isometry * rhs, scaling)
    };
    [ref val] => Similarity::from_isometry(&self.isometry * rhs, self.scaling());
    [val ref] => {
        let scaling = self.scaling();
        Similarity::from_isometry(self.isometry * rhs, scaling)
    };
    [ref ref] => Similarity::from_isometry(&self.isometry * rhs, self.scaling());
);

// Similarity ÷ UnitComplex
similarity_from_composition_impl_all!(
    Div, div;
    ;
    self: Similarity<T, UnitComplex<T>, 2>, rhs: UnitComplex<T>,
    Output = Similarity<T, UnitComplex<T>, 2>;
    [val val] => {
        let scaling = self.scaling();
        Similarity::from_isometry(self.isometry / rhs, scaling)
    };
    [ref val] => Similarity::from_isometry(&self.isometry / rhs, self.scaling());
    [val ref] => {
        let scaling = self.scaling();
        Similarity::from_isometry(self.isometry / rhs, scaling)
    };
    [ref ref] => Similarity::from_isometry(&self.isometry / rhs, self.scaling());
);

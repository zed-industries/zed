use std::ops::{Div, DivAssign, Mul, MulAssign};

use simba::scalar::{ClosedAddAssign, ClosedSubAssign};

use crate::base::constraint::{SameNumberOfColumns, SameNumberOfRows, ShapeConstraint};
use crate::base::dimension::U1;
use crate::base::{Const, Scalar};

use crate::geometry::{Point, Translation};

// Translation × Translation
add_sub_impl!(Mul, mul, ClosedAddAssign;
    (Const<D>, U1), (Const<D>, U1) -> (Const<D>, U1)
    const D; for; where;
    self: &'a Translation<T, D>, right: &'b Translation<T, D>, Output = Translation<T, D>;
    #[allow(clippy::suspicious_arithmetic_impl)] { Translation::from(&self.vector + &right.vector) };
    'a, 'b);

add_sub_impl!(Mul, mul, ClosedAddAssign;
    (Const<D>, U1), (Const<D>, U1) -> (Const<D>, U1)
    const D; for; where;
    self: &'a Translation<T, D>, right: Translation<T, D>, Output = Translation<T, D>;
    #[allow(clippy::suspicious_arithmetic_impl)] { Translation::from(&self.vector + right.vector) };
    'a);

add_sub_impl!(Mul, mul, ClosedAddAssign;
    (Const<D>, U1), (Const<D>, U1) -> (Const<D>, U1)
    const D; for; where;
    self: Translation<T, D>, right: &'b Translation<T, D>, Output = Translation<T, D>;
    #[allow(clippy::suspicious_arithmetic_impl)] { Translation::from(self.vector + &right.vector) };
    'b);

add_sub_impl!(Mul, mul, ClosedAddAssign;
    (Const<D>, U1), (Const<D>, U1) -> (Const<D>, U1)
    const D; for; where;
    self: Translation<T, D>, right: Translation<T, D>, Output = Translation<T, D>;
    #[allow(clippy::suspicious_arithmetic_impl)] { Translation::from(self.vector + right.vector) }; );

// Translation ÷ Translation
// TODO: instead of calling inverse explicitly, could we just add a `mul_tr` or `mul_inv` method?
add_sub_impl!(Div, div, ClosedSubAssign;
    (Const<D>, U1), (Const<D>, U1) -> (Const<D>, U1)
    const D; for; where;
    self: &'a Translation<T, D>, right: &'b Translation<T, D>, Output = Translation<T, D>;
    #[allow(clippy::suspicious_arithmetic_impl)] { Translation::from(&self.vector - &right.vector) };
    'a, 'b);

add_sub_impl!(Div, div, ClosedSubAssign;
    (Const<D>, U1), (Const<D>, U1) -> (Const<D>, U1)
    const D; for; where;
    self: &'a Translation<T, D>, right: Translation<T, D>, Output = Translation<T, D>;
    #[allow(clippy::suspicious_arithmetic_impl)] { Translation::from(&self.vector - right.vector) };
    'a);

add_sub_impl!(Div, div, ClosedSubAssign;
    (Const<D>, U1), (Const<D>, U1) -> (Const<D>, U1)
    const D; for; where;
    self: Translation<T, D>, right: &'b Translation<T, D>, Output = Translation<T, D>;
    #[allow(clippy::suspicious_arithmetic_impl)] { Translation::from(self.vector - &right.vector) };
    'b);

add_sub_impl!(Div, div, ClosedSubAssign;
    (Const<D>, U1), (Const<D>, U1) -> (Const<D>, U1)
    const D; for; where;
    self: Translation<T, D>, right: Translation<T, D>, Output = Translation<T, D>;
    #[allow(clippy::suspicious_arithmetic_impl)] { Translation::from(self.vector - right.vector) }; );

// Translation × Point
// TODO: we don't handle properly non-zero origins here. Do we want this to be the intended
// behavior?
add_sub_impl!(Mul, mul, ClosedAddAssign;
    (Const<D>, U1), (Const<D>, U1) -> (Const<D>, U1)
    const D; for; where;
    self: &'a Translation<T, D>, right: &'b Point<T, D>, Output = Point<T, D>;
    #[allow(clippy::suspicious_arithmetic_impl)] { right + &self.vector };
    'a, 'b);

add_sub_impl!(Mul, mul, ClosedAddAssign;
    (Const<D>, U1), (Const<D>, U1) -> (Const<D>, U1)
    const D; for; where;
    self: &'a Translation<T, D>, right: Point<T, D>, Output = Point<T, D>;
    #[allow(clippy::suspicious_arithmetic_impl)] { right + &self.vector };
    'a);

add_sub_impl!(Mul, mul, ClosedAddAssign;
    (Const<D>, U1), (Const<D>, U1) -> (Const<D>, U1)
    const D; for; where;
    self: Translation<T, D>, right: &'b Point<T, D>, Output = Point<T, D>;
    #[allow(clippy::suspicious_arithmetic_impl)] { right + self.vector };
    'b);

add_sub_impl!(Mul, mul, ClosedAddAssign;
    (Const<D>, U1), (Const<D>, U1) -> (Const<D>, U1)
    const D; for; where;
    self: Translation<T, D>, right: Point<T, D>, Output = Point<T, D>;
    #[allow(clippy::suspicious_arithmetic_impl)] { right + self.vector }; );

// Translation *= Translation
add_sub_assign_impl!(MulAssign, mul_assign, ClosedAddAssign;
    const D;
    self: Translation<T, D>, right: &'b Translation<T, D>;
    #[allow(clippy::suspicious_op_assign_impl)] { self.vector += &right.vector };
    'b);

add_sub_assign_impl!(MulAssign, mul_assign, ClosedAddAssign;
    const D;
    self: Translation<T, D>, right: Translation<T, D>;
    #[allow(clippy::suspicious_op_assign_impl)] { self.vector += right.vector }; );

add_sub_assign_impl!(DivAssign, div_assign, ClosedSubAssign;
    const D;
    self: Translation<T, D>, right: &'b Translation<T, D>;
    #[allow(clippy::suspicious_op_assign_impl)] { self.vector -= &right.vector };
    'b);

add_sub_assign_impl!(DivAssign, div_assign, ClosedSubAssign;
    const D;
    self: Translation<T, D>, right: Translation<T, D>;
    #[allow(clippy::suspicious_op_assign_impl)] { self.vector -= right.vector }; );

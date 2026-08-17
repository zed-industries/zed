use std::ops::{Mul, MulAssign};

use simba::scalar::ClosedMulAssign;

use crate::base::constraint::{SameNumberOfColumns, SameNumberOfRows, ShapeConstraint};
use crate::base::dimension::U1;
use crate::base::{Const, SVector, Scalar};

use crate::geometry::{Point, Scale};

// Scale × Scale
add_sub_impl!(Mul, mul, ClosedMulAssign;
    (Const<D>, U1), (Const<D>, U1) -> (Const<D>, U1)
    const D; for; where;
    self: &'a Scale<T, D>, right: &'b Scale<T, D>, Output = Scale<T, D>;
    Scale::from(self.vector.component_mul(&right.vector));
    'a, 'b);

add_sub_impl!(Mul, mul, ClosedMulAssign;
    (Const<D>, U1), (Const<D>, U1) -> (Const<D>, U1)
    const D; for; where;
    self: &'a Scale<T, D>, right: Scale<T, D>, Output = Scale<T, D>;
    Scale::from(self.vector.component_mul(&right.vector));
    'a);

add_sub_impl!(Mul, mul, ClosedMulAssign;
    (Const<D>, U1), (Const<D>, U1) -> (Const<D>, U1)
    const D; for; where;
    self: Scale<T, D>, right: &'b Scale<T, D>, Output = Scale<T, D>;
    Scale::from(self.vector.component_mul(&right.vector));
    'b);

add_sub_impl!(Mul, mul, ClosedMulAssign;
    (Const<D>, U1), (Const<D>, U1) -> (Const<D>, U1)
    const D; for; where;
    self: Scale<T, D>, right: Scale<T, D>, Output = Scale<T, D>;
    Scale::from(self.vector.component_mul(&right.vector)); );

// Scale × scalar
add_sub_impl!(Mul, mul, ClosedMulAssign;
    (Const<D>, U1), (Const<D>, U1) -> (Const<D>, U1)
    const D; for; where;
    self: &'a Scale<T, D>, right: T, Output = Scale<T, D>;
    Scale::from(&self.vector * right);
    'a);

add_sub_impl!(Mul, mul, ClosedMulAssign;
    (Const<D>, U1), (Const<D>, U1) -> (Const<D>, U1)
    const D; for; where;
    self: Scale<T, D>, right: T, Output = Scale<T, D>;
    Scale::from(self.vector * right); );

// Scale × Point
add_sub_impl!(Mul, mul, ClosedMulAssign;
    (Const<D>, U1), (Const<D>, U1) -> (Const<D>, U1)
    const D; for; where;
    self: &'a Scale<T, D>, right: &'b Point<T, D>, Output = Point<T, D>;
    Point::from(self.vector.component_mul(&right.coords));
    'a, 'b);

add_sub_impl!(Mul, mul, ClosedMulAssign;
    (Const<D>, U1), (Const<D>, U1) -> (Const<D>, U1)
    const D; for; where;
    self: &'a Scale<T, D>, right: Point<T, D>, Output = Point<T, D>;
    Point::from(self.vector.component_mul(&right.coords));
    'a);

add_sub_impl!(Mul, mul, ClosedMulAssign;
    (Const<D>, U1), (Const<D>, U1) -> (Const<D>, U1)
    const D; for; where;
    self: Scale<T, D>, right: &'b Point<T, D>, Output = Point<T, D>;
    Point::from(self.vector.component_mul(&right.coords));
    'b);

add_sub_impl!(Mul, mul, ClosedMulAssign;
    (Const<D>, U1), (Const<D>, U1) -> (Const<D>, U1)
    const D; for; where;
    self: Scale<T, D>, right: Point<T, D>, Output = Point<T, D>;
    Point::from(self.vector.component_mul(&right.coords)); );

// Scale * Vector
add_sub_impl!(Mul, mul, ClosedMulAssign;
    (Const<D>, U1), (Const<D>, U1) -> (Const<D>, U1)
    const D; for; where;
    self: &'a Scale<T, D>, right: &'b SVector<T, D>, Output = SVector<T, D>;
    self.vector.component_mul(right);
    'a, 'b);

add_sub_impl!(Mul, mul, ClosedMulAssign;
    (Const<D>, U1), (Const<D>, U1) -> (Const<D>, U1)
    const D; for; where;
    self: &'a Scale<T, D>, right: SVector<T, D>, Output = SVector<T, D>;
    self.vector.component_mul(&right);
    'a);

add_sub_impl!(Mul, mul, ClosedMulAssign;
    (Const<D>, U1), (Const<D>, U1) -> (Const<D>, U1)
    const D; for; where;
    self: Scale<T, D>, right: &'b SVector<T, D>, Output = SVector<T, D>;
    self.vector.component_mul(right);
    'b);

add_sub_impl!(Mul, mul, ClosedMulAssign;
    (Const<D>, U1), (Const<D>, U1) -> (Const<D>, U1)
    const D; for; where;
    self: Scale<T, D>, right: SVector<T, D>, Output = SVector<T, D>;
    self.vector.component_mul(&right); );

// Scale *= Scale
add_sub_assign_impl!(MulAssign, mul_assign, ClosedMulAssign;
    const D;
    self: Scale<T, D>, right: &'b Scale<T, D>;
    self.vector.component_mul_assign(&right.vector);
    'b);

add_sub_assign_impl!(MulAssign, mul_assign, ClosedMulAssign;
    const D;
    self: Scale<T, D>, right: Scale<T, D>;
    self.vector.component_mul_assign(&right.vector); );

// Scale ×= scalar
add_sub_assign_impl!(MulAssign, mul_assign, ClosedMulAssign;
    const D;
    self: Scale<T, D>, right: T;
    self.vector *= right; );

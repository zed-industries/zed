/*
 *
 * This provides the following operator overladings:
 *
 * Index<(usize, usize)>
 *
 * Rotation × Rotation
 * Rotation ÷ Rotation
 * Rotation × Matrix
 * Matrix   × Rotation
 * Matrix   ÷ Rotation
 * Rotation × Point
 * Rotation × Unit<Vector>
 *
 *
 * Rotation ×= Rotation
 * Matrix   ×= Rotation
 */

use num::{One, Zero};
use std::ops::{Div, DivAssign, Index, Mul, MulAssign};

use simba::scalar::{ClosedAddAssign, ClosedMulAssign};

use crate::base::allocator::Allocator;
use crate::base::constraint::{AreMultipliable, ShapeConstraint};
use crate::base::dimension::{Dim, U1};
use crate::base::storage::Storage;
use crate::base::{
    Const, DefaultAllocator, Matrix, OMatrix, SMatrix, SVector, Scalar, Unit, Vector,
};

use crate::geometry::{Point, Rotation};

impl<T: Scalar, const D: usize> Index<(usize, usize)> for Rotation<T, D> {
    type Output = T;

    #[inline]
    fn index(&self, row_col: (usize, usize)) -> &T {
        self.matrix().index(row_col)
    }
}

// Rotation × Rotation
md_impl_all!(
    Mul, mul;
    (Const<D>, Const<D>), (Const<D>, Const<D>)
    const D;
    for;
    where;
    self: Rotation<T, D>, right: Rotation<T, D>, Output = Rotation<T, D>;
    [val val] => Rotation::from_matrix_unchecked(self.into_inner() * right.into_inner());
    [ref val] => Rotation::from_matrix_unchecked(self.matrix() * right.into_inner());
    [val ref] => Rotation::from_matrix_unchecked(self.into_inner() * right.matrix());
    [ref ref] => Rotation::from_matrix_unchecked(self.matrix() * right.matrix());
);

// Rotation ÷ Rotation
// TODO: instead of calling inverse explicitly, could we just add a `mul_tr` or `mul_inv` method?
md_impl_all!(
    Div, div;
    (Const<D>, Const<D>), (Const<D>, Const<D>)
    const D;
    for;
    where;
    self: Rotation<T, D>, right: Rotation<T, D>, Output = Rotation<T, D>;
    [val val] => #[allow(clippy::suspicious_arithmetic_impl)] { self * right.inverse() };
    [ref val] => #[allow(clippy::suspicious_arithmetic_impl)] { self * right.inverse() };
    [val ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self * right.inverse() };
    [ref ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self * right.inverse() };
);

// Rotation × Matrix
md_impl_all!(
    Mul, mul;
    (Const<D1>, Const<D1>), (R2, C2)
    const D1;
    for R2, C2, SB;
    where R2: Dim, C2: Dim, SB: Storage<T, R2, C2>,
          DefaultAllocator: Allocator<Const<D1>, C2>,
          ShapeConstraint: AreMultipliable<Const<D1>, Const<D1>, R2, C2>;
    self: Rotation<T, D1>, right: Matrix<T, R2, C2, SB>, Output = OMatrix<T, Const<D1>, C2>;
    [val val] => self.into_inner() * right;
    [ref val] => self.matrix() * right;
    [val ref] => self.into_inner() * right;
    [ref ref] => self.matrix() * right;
);

// Matrix × Rotation
md_impl_all!(
    Mul, mul;
    (R1, C1), (Const<D2>, Const<D2>)
    const D2;
    for R1, C1, SA;
    where R1: Dim, C1: Dim, SA: Storage<T, R1, C1>,
          DefaultAllocator: Allocator<R1, Const<D2>>,
          ShapeConstraint:  AreMultipliable<R1, C1, Const<D2>, Const<D2>>;
    self: Matrix<T, R1, C1, SA>, right: Rotation<T, D2>, Output = OMatrix<T, R1, Const<D2>>;
    [val val] => self * right.into_inner();
    [ref val] => self * right.into_inner();
    [val ref] => self * right.matrix();
    [ref ref] => self * right.matrix();
);

// Matrix ÷ Rotation
md_impl_all!(
    Div, div;
    (R1, C1), (Const<D2>, Const<D2>)
    const D2;
    for R1, C1, SA;
    where R1: Dim, C1: Dim, SA: Storage<T, R1, C1>,
          DefaultAllocator: Allocator<R1, Const<D2>>,
          ShapeConstraint: AreMultipliable<R1, C1, Const<D2>, Const<D2>>;
    self: Matrix<T, R1, C1, SA>, right: Rotation<T, D2>, Output = OMatrix<T, R1, Const<D2>>;
    [val val] => #[allow(clippy::suspicious_arithmetic_impl)] { self * right.inverse() };
    [ref val] => #[allow(clippy::suspicious_arithmetic_impl)] { self * right.inverse() };
    [val ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self * right.inverse() };
    [ref ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self * right.inverse() };
);

// Rotation × Point
// TODO: we don't handle properly non-zero origins here. Do we want this to be the intended
// behavior?
md_impl_all!(
    Mul, mul;
    (Const<D>, Const<D>), (Const<D>, U1)
    const D;
    for;
    where ShapeConstraint:  AreMultipliable<Const<D>, Const<D>, Const<D>, U1>;
    self: Rotation<T, D>, right: Point<T, D>, Output = Point<T, D>;
    [val val] => self.into_inner() * right;
    [ref val] => self.matrix() * right;
    [val ref] => self.into_inner() * right;
    [ref ref] => self.matrix() * right;
);

// Rotation × Unit<Vector>
md_impl_all!(
    Mul, mul;
    (Const<D>, Const<D>), (Const<D>, U1)
    const D;
    for S;
    where S: Storage<T, Const<D>>,
          ShapeConstraint: AreMultipliable<Const<D>, Const<D>, Const<D>, U1>;
    self: Rotation<T, D>, right: Unit<Vector<T, Const<D>, S>>, Output = Unit<SVector<T, D>>;
    [val val] => Unit::new_unchecked(self.into_inner() * right.into_inner());
    [ref val] => Unit::new_unchecked(self.matrix() * right.into_inner());
    [val ref] => Unit::new_unchecked(self.into_inner() * right.as_ref());
    [ref ref] => Unit::new_unchecked(self.matrix() * right.as_ref());
);

// Rotation ×= Rotation
// TODO: try not to call `inverse()` explicitly.

md_assign_impl_all!(
    MulAssign, mul_assign;
    (Const<D>, Const<D>), (Const<D>, Const<D>)
    const D; for; where;
    self: Rotation<T, D>, right: Rotation<T, D>;
    [val] => self.matrix_mut_unchecked().mul_assign(right.into_inner());
    [ref] => self.matrix_mut_unchecked().mul_assign(right.matrix());
);

md_assign_impl_all!(
    DivAssign, div_assign;
    (Const<D>, Const<D>), (Const<D>, Const<D>)
    const D; for; where;
    self: Rotation<T, D>, right: Rotation<T, D>;
    [val] => self.matrix_mut_unchecked().mul_assign(right.inverse().into_inner());
    [ref] => self.matrix_mut_unchecked().mul_assign(right.inverse().matrix());
);

// Matrix *= Rotation
// TODO: try not to call `inverse()` explicitly.
// TODO: this shares the same limitations as for the current impl. of MulAssign for matrices.
// (In particular the number of matrix column must be equal to the number of rotation columns,
// i.e., equal to the rotation dimension.

md_assign_impl_all!(
    MulAssign, mul_assign;
    (Const<R1>, Const<C1>), (Const<C1>, Const<C1>)
    const R1, C1; for; where;
    self: SMatrix<T, R1, C1>, right: Rotation<T, C1>;
    [val] => self.mul_assign(right.into_inner());
    [ref] => self.mul_assign(right.matrix());
);

md_assign_impl_all!(
    DivAssign, div_assign;
    (Const<R1>, Const<C1>), (Const<C1>, Const<C1>)
    const R1, C1; for; where;
    self: SMatrix<T, R1, C1>, right: Rotation<T, C1>;
    [val] => self.mul_assign(right.inverse().into_inner());
    [ref] => self.mul_assign(right.inverse().matrix());
);

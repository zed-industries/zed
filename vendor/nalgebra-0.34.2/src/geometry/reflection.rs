use crate::base::constraint::{AreMultipliable, DimEq, SameNumberOfRows, ShapeConstraint};
use crate::base::{Const, Matrix, Unit, Vector};
use crate::dimension::{Dim, U1};
use crate::storage::{Storage, StorageMut};
use simba::scalar::ComplexField;

use crate::geometry::Point;

/// A reflection wrt. a plane.
pub struct Reflection<T, D, S> {
    axis: Vector<T, D, S>,
    bias: T,
}

impl<T: ComplexField, S: Storage<T, Const<D>>, const D: usize> Reflection<T, Const<D>, S> {
    /// Creates a new reflection wrt. the plane orthogonal to the given axis and that contains the
    /// point `pt`.
    pub fn new_containing_point(axis: Unit<Vector<T, Const<D>, S>>, pt: &Point<T, D>) -> Self {
        let bias = axis.dotc(&pt.coords);
        Self::new(axis, bias)
    }
}

impl<T: ComplexField, D: Dim, S: Storage<T, D>> Reflection<T, D, S> {
    /// Creates a new reflection wrt. the plane orthogonal to the given axis and bias.
    ///
    /// The bias is the position of the plane on the axis. In particular, a bias equal to zero
    /// represents a plane that passes through the origin.
    pub fn new(axis: Unit<Vector<T, D, S>>, bias: T) -> Self {
        Self {
            axis: axis.into_inner(),
            bias,
        }
    }

    /// The reflection axis.
    #[must_use]
    pub const fn axis(&self) -> &Vector<T, D, S> {
        &self.axis
    }

    /// The reflection bias.
    ///
    /// The bias is the position of the plane on the axis. In particular, a bias equal to zero
    /// represents a plane that passes through the origin.
    #[must_use]
    pub fn bias(&self) -> T {
        self.bias.clone()
    }

    // TODO: naming convention: reflect_to, reflect_assign ?
    /// Applies the reflection to the columns of `rhs`.
    pub fn reflect<R2: Dim, C2: Dim, S2>(&self, rhs: &mut Matrix<T, R2, C2, S2>)
    where
        S2: StorageMut<T, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        for i in 0..rhs.ncols() {
            // NOTE: we borrow the column twice here. First it is borrowed immutably for the
            // dot product, and then mutably. Somehow, this allows significantly
            // better optimizations of the dot product from the compiler.
            let m_two: T = crate::convert(-2.0f64);
            let factor = (self.axis.dotc(&rhs.column(i)) - self.bias.clone()) * m_two;
            rhs.column_mut(i).axpy(factor, &self.axis, T::one());
        }
    }

    // TODO: naming convention: reflect_to, reflect_assign ?
    /// Applies the reflection to the columns of `rhs`.
    pub fn reflect_with_sign<R2: Dim, C2: Dim, S2>(&self, rhs: &mut Matrix<T, R2, C2, S2>, sign: T)
    where
        S2: StorageMut<T, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        for i in 0..rhs.ncols() {
            // NOTE: we borrow the column twice here. First it is borrowed immutably for the
            // dot product, and then mutably. Somehow, this allows significantly
            // better optimizations of the dot product from the compiler.
            let m_two = sign.clone().scale(crate::convert(-2.0f64));
            let factor = (self.axis.dotc(&rhs.column(i)) - self.bias.clone()) * m_two;
            rhs.column_mut(i).axpy(factor, &self.axis, sign.clone());
        }
    }

    /// Applies the reflection to the rows of `lhs`.
    pub fn reflect_rows<R2: Dim, C2: Dim, S2, S3>(
        &self,
        lhs: &mut Matrix<T, R2, C2, S2>,
        work: &mut Vector<T, R2, S3>,
    ) where
        S2: StorageMut<T, R2, C2>,
        S3: StorageMut<T, R2>,
        ShapeConstraint: DimEq<C2, D> + AreMultipliable<R2, C2, D, U1>,
    {
        lhs.mul_to(&self.axis, work);

        if !self.bias.is_zero() {
            work.add_scalar_mut(-self.bias.clone());
        }

        let m_two: T = crate::convert(-2.0f64);
        lhs.gerc(m_two, work, &self.axis, T::one());
    }

    /// Applies the reflection to the rows of `lhs`.
    pub fn reflect_rows_with_sign<R2: Dim, C2: Dim, S2, S3>(
        &self,
        lhs: &mut Matrix<T, R2, C2, S2>,
        work: &mut Vector<T, R2, S3>,
        sign: T,
    ) where
        S2: StorageMut<T, R2, C2>,
        S3: StorageMut<T, R2>,
        ShapeConstraint: DimEq<C2, D> + AreMultipliable<R2, C2, D, U1>,
    {
        lhs.mul_to(&self.axis, work);

        if !self.bias.is_zero() {
            work.add_scalar_mut(-self.bias.clone());
        }

        let m_two = sign.clone().scale(crate::convert(-2.0f64));
        lhs.gerc(m_two, work, &self.axis, sign);
    }
}

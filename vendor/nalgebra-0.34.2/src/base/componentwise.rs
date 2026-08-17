// Non-conventional component-wise operators.

use num::{Signed, Zero};
use std::ops::{Add, Mul};

use simba::scalar::{ClosedDivAssign, ClosedMulAssign};
use simba::simd::SimdPartialOrd;

use crate::ClosedAddAssign;
use crate::base::allocator::{Allocator, SameShapeAllocator};
use crate::base::constraint::{SameNumberOfColumns, SameNumberOfRows, ShapeConstraint};
use crate::base::dimension::Dim;
use crate::base::storage::{Storage, StorageMut};
use crate::base::{DefaultAllocator, Matrix, MatrixSum, OMatrix, Scalar};

/// The type of the result of a matrix component-wise operation.
pub type MatrixComponentOp<T, R1, C1, R2, C2> = MatrixSum<T, R1, C1, R2, C2>;

impl<T: Scalar, R: Dim, C: Dim, S: Storage<T, R, C>> Matrix<T, R, C, S> {
    /// Computes the component-wise absolute value.
    ///
    /// # Example
    ///
    /// ```
    /// # use nalgebra::Matrix2;
    /// let a = Matrix2::new(0.0, 1.0,
    ///                      -2.0, -3.0);
    /// assert_eq!(a.abs(), Matrix2::new(0.0, 1.0, 2.0, 3.0))
    /// ```
    #[inline]
    #[must_use]
    pub fn abs(&self) -> OMatrix<T, R, C>
    where
        T: Signed,
        DefaultAllocator: Allocator<R, C>,
    {
        let mut res = self.clone_owned();

        for e in res.iter_mut() {
            *e = e.abs();
        }

        res
    }

    // TODO: add other operators like component_ln, component_pow, etc. ?
}

macro_rules! component_binop_impl(
    ($($binop: ident, $binop_mut: ident, $binop_assign: ident, $cmpy: ident, $Trait: ident . $op: ident . $op_assign: ident, $desc:expr_2021, $desc_cmpy:expr_2021, $desc_mut:expr_2021);* $(;)*) => {$(
        #[doc = $desc]
        #[inline]
        #[must_use]
        pub fn $binop<R2, C2, SB>(&self, rhs: &Matrix<T, R2, C2, SB>) -> MatrixComponentOp<T, R1, C1, R2, C2>
            where T: $Trait,
                  R2: Dim, C2: Dim,
                  SB: Storage<T, R2, C2>,
                  DefaultAllocator: SameShapeAllocator<R1, C1, R2, C2>,
                  ShapeConstraint:  SameNumberOfRows<R1, R2> + SameNumberOfColumns<C1, C2> {

            assert_eq!(self.shape(), rhs.shape(), "Componentwise mul/div: mismatched matrix dimensions.");
            let mut res = self.clone_owned_sum();

            for j in 0 .. res.ncols() {
                for i in 0 .. res.nrows() {
                    unsafe {
                        res.get_unchecked_mut((i, j)).$op_assign(rhs.get_unchecked((i, j)).clone());
                    }
                }
            }

            res
        }

        // componentwise binop plus Y.
        #[doc = $desc_cmpy]
        #[inline]
        pub fn $cmpy<R2, C2, SB, R3, C3, SC>(&mut self, alpha: T, a: &Matrix<T, R2, C2, SB>, b: &Matrix<T, R3, C3, SC>, beta: T)
            where T: $Trait + Zero + Mul<T, Output = T> + Add<T, Output = T>,
                  R2: Dim, C2: Dim,
                  R3: Dim, C3: Dim,
                  SA: StorageMut<T, R1, C1>,
                  SB: Storage<T, R2, C2>,
                  SC: Storage<T, R3, C3>,
                  ShapeConstraint: SameNumberOfRows<R1, R2> + SameNumberOfColumns<C1, C2> +
                                   SameNumberOfRows<R1, R3> + SameNumberOfColumns<C1, C3> {
            assert_eq!(self.shape(), a.shape(), "Componentwise mul/div: mismatched matrix dimensions.");
            assert_eq!(self.shape(), b.shape(), "Componentwise mul/div: mismatched matrix dimensions.");

            if beta.is_zero() {
                for j in 0 .. self.ncols() {
                    for i in 0 .. self.nrows() {
                        unsafe {
                            let res = alpha.clone() * a.get_unchecked((i, j)).clone().$op(b.get_unchecked((i, j)).clone());
                            *self.get_unchecked_mut((i, j)) = res;
                        }
                    }
                }
            }
            else {
                for j in 0 .. self.ncols() {
                    for i in 0 .. self.nrows() {
                        unsafe {
                            let res = alpha.clone() * a.get_unchecked((i, j)).clone().$op(b.get_unchecked((i, j)).clone());
                            *self.get_unchecked_mut((i, j)) = beta.clone() * self.get_unchecked((i, j)).clone() + res;
                        }
                    }
                }
            }
        }

        #[doc = $desc_mut]
        #[inline]
        pub fn $binop_assign<R2, C2, SB>(&mut self, rhs: &Matrix<T, R2, C2, SB>)
            where T: $Trait,
                  R2: Dim,
                  C2: Dim,
                  SA: StorageMut<T, R1, C1>,
                  SB: Storage<T, R2, C2>,
                  ShapeConstraint: SameNumberOfRows<R1, R2> + SameNumberOfColumns<C1, C2> {

            assert_eq!(self.shape(), rhs.shape(), "Componentwise mul/div: mismatched matrix dimensions.");

            for j in 0 .. self.ncols() {
                for i in 0 .. self.nrows() {
                    unsafe {
                        self.get_unchecked_mut((i, j)).$op_assign(rhs.get_unchecked((i, j)).clone());
                    }
                }
            }
        }

        #[doc = $desc_mut]
        #[inline]
        #[deprecated(note = "This is renamed using the `_assign` suffix instead of the `_mut` suffix.")]
        pub fn $binop_mut<R2, C2, SB>(&mut self, rhs: &Matrix<T, R2, C2, SB>)
            where T: $Trait,
                  R2: Dim,
                  C2: Dim,
                  SA: StorageMut<T, R1, C1>,
                  SB: Storage<T, R2, C2>,
                  ShapeConstraint: SameNumberOfRows<R1, R2> + SameNumberOfColumns<C1, C2> {
            self.$binop_assign(rhs)
        }
    )*}
);

/// # Componentwise operations
impl<T: Scalar, R1: Dim, C1: Dim, SA: Storage<T, R1, C1>> Matrix<T, R1, C1, SA> {
    component_binop_impl!(
        component_mul, component_mul_mut, component_mul_assign, cmpy, ClosedMulAssign.mul.mul_assign,
        r"
        Componentwise matrix or vector multiplication.

        # Example

        ```
        # use nalgebra::Matrix2;
        let a = Matrix2::new(0.0, 1.0, 2.0, 3.0);
        let b = Matrix2::new(4.0, 5.0, 6.0, 7.0);
        let expected = Matrix2::new(0.0, 5.0, 12.0, 21.0);

        assert_eq!(a.component_mul(&b), expected);
        ```
        ",
        r"
        Computes componentwise `self[i] = alpha * a[i] * b[i] + beta * self[i]`.

        # Example
        ```
        # use nalgebra::Matrix2;
        let mut m = Matrix2::new(0.0, 1.0, 2.0, 3.0);
        let a = Matrix2::new(0.0, 1.0, 2.0, 3.0);
        let b = Matrix2::new(4.0, 5.0, 6.0, 7.0);
        let expected = (a.component_mul(&b) * 5.0) + m * 10.0;

        m.cmpy(5.0, &a, &b, 10.0);
        assert_eq!(m, expected);
        ```
        ",
        r"
        Inplace componentwise matrix or vector multiplication.

        # Example
        ```
        # use nalgebra::Matrix2;
        let mut a = Matrix2::new(0.0, 1.0, 2.0, 3.0);
        let b = Matrix2::new(4.0, 5.0, 6.0, 7.0);
        let expected = Matrix2::new(0.0, 5.0, 12.0, 21.0);

        a.component_mul_assign(&b);

        assert_eq!(a, expected);
        ```
        ";
        component_div, component_div_mut, component_div_assign, cdpy, ClosedDivAssign.div.div_assign,
        r"
        Componentwise matrix or vector division.

        # Example

        ```
        # use nalgebra::Matrix2;
        let a = Matrix2::new(0.0, 1.0, 2.0, 3.0);
        let b = Matrix2::new(4.0, 5.0, 6.0, 7.0);
        let expected = Matrix2::new(0.0, 1.0 / 5.0, 2.0 / 6.0, 3.0 / 7.0);

        assert_eq!(a.component_div(&b), expected);
        ```
        ",
        r"
        Computes componentwise `self[i] = alpha * a[i] / b[i] + beta * self[i]`.

        # Example
        ```
        # use nalgebra::Matrix2;
        let mut m = Matrix2::new(0.0, 1.0, 2.0, 3.0);
        let a = Matrix2::new(4.0, 5.0, 6.0, 7.0);
        let b = Matrix2::new(4.0, 5.0, 6.0, 7.0);
        let expected = (a.component_div(&b) * 5.0) + m * 10.0;

        m.cdpy(5.0, &a, &b, 10.0);
        assert_eq!(m, expected);
        ```
        ",
        r"
        Inplace componentwise matrix or vector division.

        # Example
        ```
        # use nalgebra::Matrix2;
        let mut a = Matrix2::new(0.0, 1.0, 2.0, 3.0);
        let b = Matrix2::new(4.0, 5.0, 6.0, 7.0);
        let expected = Matrix2::new(0.0, 1.0 / 5.0, 2.0 / 6.0, 3.0 / 7.0);

        a.component_div_assign(&b);

        assert_eq!(a, expected);
        ```
        ";
        // TODO: add other operators like bitshift, etc. ?
    );

    /// Computes the infimum (aka. componentwise min) of two matrices/vectors.
    ///
    /// # Example
    ///
    /// ```
    /// # use nalgebra::Matrix2;
    /// let u = Matrix2::new(4.0, 2.0, 1.0, -2.0);
    /// let v = Matrix2::new(2.0, 4.0, -2.0, 1.0);
    /// let expected = Matrix2::new(2.0, 2.0, -2.0, -2.0);
    /// assert_eq!(u.inf(&v), expected)
    /// ```
    #[inline]
    #[must_use]
    pub fn inf(&self, other: &Self) -> OMatrix<T, R1, C1>
    where
        T: SimdPartialOrd,
        DefaultAllocator: Allocator<R1, C1>,
    {
        self.zip_map(other, |a, b| a.simd_min(b))
    }

    /// Computes the supremum (aka. componentwise max) of two matrices/vectors.
    ///
    /// # Example
    ///
    /// ```
    /// # use nalgebra::Matrix2;
    /// let u = Matrix2::new(4.0, 2.0, 1.0, -2.0);
    /// let v = Matrix2::new(2.0, 4.0, -2.0, 1.0);
    /// let expected = Matrix2::new(4.0, 4.0, 1.0, 1.0);
    /// assert_eq!(u.sup(&v), expected)
    /// ```
    #[inline]
    #[must_use]
    pub fn sup(&self, other: &Self) -> OMatrix<T, R1, C1>
    where
        T: SimdPartialOrd,
        DefaultAllocator: Allocator<R1, C1>,
    {
        self.zip_map(other, |a, b| a.simd_max(b))
    }

    /// Computes the (infimum, supremum) of two matrices/vectors.
    ///
    /// # Example
    ///
    /// ```
    /// # use nalgebra::Matrix2;
    /// let u = Matrix2::new(4.0, 2.0, 1.0, -2.0);
    /// let v = Matrix2::new(2.0, 4.0, -2.0, 1.0);
    /// let expected = (Matrix2::new(2.0, 2.0, -2.0, -2.0), Matrix2::new(4.0, 4.0, 1.0, 1.0));
    /// assert_eq!(u.inf_sup(&v), expected)
    /// ```
    #[inline]
    #[must_use]
    pub fn inf_sup(&self, other: &Self) -> (OMatrix<T, R1, C1>, OMatrix<T, R1, C1>)
    where
        T: SimdPartialOrd,
        DefaultAllocator: Allocator<R1, C1>,
    {
        // TODO: can this be optimized?
        (self.inf(other), self.sup(other))
    }

    /// Adds a scalar to `self`.
    ///
    /// # Example
    ///
    /// ```
    /// # use nalgebra::Matrix2;
    /// let u = Matrix2::new(1.0, 2.0, 3.0, 4.0);
    /// let s = 10.0;
    /// let expected = Matrix2::new(11.0, 12.0, 13.0, 14.0);
    /// assert_eq!(u.add_scalar(s), expected)
    /// ```
    #[inline]
    #[must_use = "Did you mean to use add_scalar_mut()?"]
    pub fn add_scalar(&self, rhs: T) -> OMatrix<T, R1, C1>
    where
        T: ClosedAddAssign,
        DefaultAllocator: Allocator<R1, C1>,
    {
        let mut res = self.clone_owned();
        res.add_scalar_mut(rhs);
        res
    }

    /// Adds a scalar to `self` in-place.
    ///
    /// # Example
    ///
    /// ```
    /// # use nalgebra::Matrix2;
    /// let mut u = Matrix2::new(1.0, 2.0, 3.0, 4.0);
    /// let s = 10.0;
    /// u.add_scalar_mut(s);
    /// let expected = Matrix2::new(11.0, 12.0, 13.0, 14.0);
    /// assert_eq!(u, expected)
    /// ```
    #[inline]
    pub fn add_scalar_mut(&mut self, rhs: T)
    where
        T: ClosedAddAssign,
        SA: StorageMut<T, R1, C1>,
    {
        for e in self.iter_mut() {
            *e += rhs.clone()
        }
    }
}

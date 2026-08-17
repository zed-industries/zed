#[cfg(feature = "serde-serialize-no-std")]
use serde::{Deserialize, Serialize};

use num::{One, Zero};
use simba::scalar::ComplexField;
use simba::simd::SimdComplexField;

use crate::allocator::Allocator;
use crate::base::{Const, DefaultAllocator, Matrix, OMatrix, Vector};
use crate::constraint::{SameNumberOfRows, ShapeConstraint};
use crate::dimension::{Dim, DimAdd, DimDiff, DimSub, DimSum, U1};
use crate::storage::{Storage, StorageMut};

/// The Cholesky decomposition of a symmetric-definite-positive matrix.
#[cfg_attr(feature = "serde-serialize-no-std", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde-serialize-no-std",
    serde(bound(serialize = "DefaultAllocator: Allocator<D>,
         OMatrix<T, D, D>: Serialize"))
)]
#[cfg_attr(
    feature = "serde-serialize-no-std",
    serde(bound(deserialize = "DefaultAllocator: Allocator<D>,
         OMatrix<T, D, D>: Deserialize<'de>"))
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Debug)]
pub struct Cholesky<T: SimdComplexField, D: Dim>
where
    DefaultAllocator: Allocator<D, D>,
{
    chol: OMatrix<T, D, D>,
}

impl<T: SimdComplexField, D: Dim> Copy for Cholesky<T, D>
where
    DefaultAllocator: Allocator<D, D>,
    OMatrix<T, D, D>: Copy,
{
}

impl<T: SimdComplexField, D: Dim> Cholesky<T, D>
where
    DefaultAllocator: Allocator<D, D>,
{
    /// Computes the Cholesky decomposition of `matrix` without checking that the matrix is definite-positive.
    ///
    /// If the input matrix is not definite-positive, the decomposition may contain trash values (Inf, NaN, etc.)
    pub fn new_unchecked(mut matrix: OMatrix<T, D, D>) -> Self {
        assert!(matrix.is_square(), "The input matrix must be square.");

        let n = matrix.nrows();

        for j in 0..n {
            for k in 0..j {
                let factor = unsafe { -matrix.get_unchecked((j, k)).clone() };

                let (mut col_j, col_k) = matrix.columns_range_pair_mut(j, k);
                let mut col_j = col_j.rows_range_mut(j..);
                let col_k = col_k.rows_range(j..);
                col_j.axpy(factor.simd_conjugate(), &col_k, T::one());
            }

            let diag = unsafe { matrix.get_unchecked((j, j)).clone() };
            let denom = diag.simd_sqrt();

            unsafe {
                *matrix.get_unchecked_mut((j, j)) = denom.clone();
            }

            let mut col = matrix.view_range_mut(j + 1.., j);
            col /= denom;
        }

        Cholesky { chol: matrix }
    }

    /// Uses the given matrix as-is without any checks or modifications as the
    /// Cholesky decomposition.
    ///
    /// It is up to the user to ensure all invariants hold.
    pub const fn pack_dirty(matrix: OMatrix<T, D, D>) -> Self {
        Cholesky { chol: matrix }
    }

    /// Retrieves the lower-triangular factor of the Cholesky decomposition with its strictly
    /// upper-triangular part filled with zeros.
    pub fn unpack(mut self) -> OMatrix<T, D, D> {
        self.chol.fill_upper_triangle(T::zero(), 1);
        self.chol
    }

    /// Retrieves the lower-triangular factor of the Cholesky decomposition, without zeroing-out
    /// its strict upper-triangular part.
    ///
    /// The values of the strict upper-triangular part are garbage and should be ignored by further
    /// computations.
    pub fn unpack_dirty(self) -> OMatrix<T, D, D> {
        self.chol
    }

    /// Retrieves the lower-triangular factor of the Cholesky decomposition with its strictly
    /// uppen-triangular part filled with zeros.
    #[must_use]
    pub fn l(&self) -> OMatrix<T, D, D> {
        self.chol.lower_triangle()
    }

    /// Retrieves the lower-triangular factor of the Cholesky decomposition, without zeroing-out
    /// its strict upper-triangular part.
    ///
    /// This is an allocation-less version of `self.l()`. The values of the strict upper-triangular
    /// part are garbage and should be ignored by further computations.
    #[must_use]
    pub const fn l_dirty(&self) -> &OMatrix<T, D, D> {
        &self.chol
    }

    /// Solves the system `self * x = b` where `self` is the decomposed matrix and `x` the unknown.
    ///
    /// The result is stored on `b`.
    pub fn solve_mut<R2: Dim, C2: Dim, S2>(&self, b: &mut Matrix<T, R2, C2, S2>)
    where
        S2: StorageMut<T, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        self.chol.solve_lower_triangular_unchecked_mut(b);
        self.chol.ad_solve_lower_triangular_unchecked_mut(b);
    }

    /// Returns the solution of the system `self * x = b` where `self` is the decomposed matrix and
    /// `x` the unknown.
    #[must_use = "Did you mean to use solve_mut()?"]
    pub fn solve<R2: Dim, C2: Dim, S2>(&self, b: &Matrix<T, R2, C2, S2>) -> OMatrix<T, R2, C2>
    where
        S2: Storage<T, R2, C2>,
        DefaultAllocator: Allocator<R2, C2>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        let mut res = b.clone_owned();
        self.solve_mut(&mut res);
        res
    }

    /// Computes the inverse of the decomposed matrix.
    #[must_use]
    pub fn inverse(&self) -> OMatrix<T, D, D> {
        let shape = self.chol.shape_generic();
        let mut res = OMatrix::identity_generic(shape.0, shape.1);

        self.solve_mut(&mut res);
        res
    }

    /// Computes the determinant of the decomposed matrix.
    #[must_use]
    pub fn determinant(&self) -> T::SimdRealField {
        let dim = self.chol.nrows();
        let mut prod_diag = T::one();
        for i in 0..dim {
            prod_diag *= unsafe { self.chol.get_unchecked((i, i)).clone() };
        }
        prod_diag.simd_modulus_squared()
    }

    /// Computes the natural logarithm of determinant of the decomposed matrix.
    ///
    /// This method is more robust than `.determinant()` to very small or very
    /// large determinants since it returns the natural logarithm of the
    /// determinant rather than the determinant itself.
    #[must_use]
    pub fn ln_determinant(&self) -> T::SimdRealField {
        let dim = self.chol.nrows();
        let mut sum_diag = T::SimdRealField::zero();
        for i in 0..dim {
            sum_diag += unsafe {
                self.chol
                    .get_unchecked((i, i))
                    .clone()
                    .simd_modulus_squared()
                    .simd_ln()
            };
        }
        sum_diag
    }
}

impl<T: ComplexField, D: Dim> Cholesky<T, D>
where
    DefaultAllocator: Allocator<D, D>,
{
    /// Attempts to compute the Cholesky decomposition of `matrix`.
    ///
    /// Returns `None` if the input matrix is not definite-positive. The input matrix is assumed
    /// to be symmetric and only the lower-triangular part is read.
    pub fn new(matrix: OMatrix<T, D, D>) -> Option<Self> {
        Self::new_internal(matrix, None)
    }

    /// Attempts to approximate the Cholesky decomposition of `matrix` by
    /// replacing non-positive values on the diagonals during the decomposition
    /// with the given `substitute`.
    ///
    /// [`try_sqrt`](ComplexField::try_sqrt) will be applied to the `substitute`
    /// when it has to be used.
    ///
    /// If your input matrix results only in positive values on the diagonals
    /// during the decomposition, `substitute` is unused and the result is just
    /// the same as if you used [`new`](Cholesky::new).
    ///
    /// This method allows to compensate for matrices with very small or even
    /// negative values due to numerical errors but necessarily results in only
    /// an approximation: it is basically a hack. If you don't specifically need
    /// Cholesky, it may be better to consider alternatives like the
    /// [`LU`](crate::linalg::LU) decomposition/factorization.
    pub fn new_with_substitute(matrix: OMatrix<T, D, D>, substitute: T) -> Option<Self> {
        Self::new_internal(matrix, Some(substitute))
    }

    /// Common implementation for `new` and `new_with_substitute`.
    fn new_internal(mut matrix: OMatrix<T, D, D>, substitute: Option<T>) -> Option<Self> {
        assert!(matrix.is_square(), "The input matrix must be square.");

        let n = matrix.nrows();

        for j in 0..n {
            for k in 0..j {
                let factor = unsafe { -matrix.get_unchecked((j, k)).clone() };

                let (mut col_j, col_k) = matrix.columns_range_pair_mut(j, k);
                let mut col_j = col_j.rows_range_mut(j..);
                let col_k = col_k.rows_range(j..);

                col_j.axpy(factor.conjugate(), &col_k, T::one());
            }

            let sqrt_denom = |v: T| {
                if v.is_zero() {
                    return None;
                }
                v.try_sqrt()
            };

            let diag = unsafe { matrix.get_unchecked((j, j)).clone() };

            if let Some(denom) =
                sqrt_denom(diag).or_else(|| substitute.clone().and_then(sqrt_denom))
            {
                unsafe {
                    *matrix.get_unchecked_mut((j, j)) = denom.clone();
                }

                let mut col = matrix.view_range_mut(j + 1.., j);
                col /= denom;
                continue;
            }

            // The diagonal element is either zero or its square root could not
            // be taken (e.g. for negative real numbers).
            return None;
        }

        Some(Cholesky { chol: matrix })
    }

    /// Given the Cholesky decomposition of a matrix `M`, a scalar `sigma` and a vector `v`,
    /// performs a rank one update such that we end up with the decomposition of `M + sigma * (v * v.adjoint())`.
    #[inline]
    pub fn rank_one_update<R2: Dim, S2>(&mut self, x: &Vector<T, R2, S2>, sigma: T::RealField)
    where
        S2: Storage<T, R2, U1>,
        DefaultAllocator: Allocator<R2, U1>,
        ShapeConstraint: SameNumberOfRows<R2, D>,
    {
        Self::xx_rank_one_update(&mut self.chol, &mut x.clone_owned(), sigma)
    }

    /// Updates the decomposition such that we get the decomposition of a matrix with the given column `col` in the `j`th position.
    /// Since the matrix is square, an identical row will be added in the `j`th row.
    pub fn insert_column<R2, S2>(
        &self,
        j: usize,
        col: Vector<T, R2, S2>,
    ) -> Cholesky<T, DimSum<D, U1>>
    where
        D: DimAdd<U1>,
        R2: Dim,
        S2: Storage<T, R2, U1>,
        DefaultAllocator: Allocator<DimSum<D, U1>, DimSum<D, U1>> + Allocator<R2>,
        ShapeConstraint: SameNumberOfRows<R2, DimSum<D, U1>>,
    {
        let mut col = col.into_owned();
        // for an explanation of the formulas, see https://en.wikipedia.org/wiki/Cholesky_decomposition#Updating_the_decomposition
        let n = col.nrows();
        assert_eq!(
            n,
            self.chol.nrows() + 1,
            "The new column must have the size of the factored matrix plus one."
        );
        assert!(j < n, "j needs to be within the bound of the new matrix.");

        // loads the data into a new matrix with an additional jth row/column
        // TODO: would it be worth it to avoid the zero-initialization?
        let mut chol = Matrix::zeros_generic(
            self.chol.shape_generic().0.add(Const::<1>),
            self.chol.shape_generic().1.add(Const::<1>),
        );
        chol.view_range_mut(..j, ..j)
            .copy_from(&self.chol.view_range(..j, ..j));
        chol.view_range_mut(..j, j + 1..)
            .copy_from(&self.chol.view_range(..j, j..));
        chol.view_range_mut(j + 1.., ..j)
            .copy_from(&self.chol.view_range(j.., ..j));
        chol.view_range_mut(j + 1.., j + 1..)
            .copy_from(&self.chol.view_range(j.., j..));

        // update the jth row
        let top_left_corner = self.chol.view_range(..j, ..j);

        let col_j = col[j].clone();
        let (mut new_rowj_adjoint, mut new_colj) = col.rows_range_pair_mut(..j, j + 1..);
        assert!(
            top_left_corner.solve_lower_triangular_mut(&mut new_rowj_adjoint),
            "Cholesky::insert_column : Unable to solve lower triangular system!"
        );

        new_rowj_adjoint.adjoint_to(&mut chol.view_range_mut(j, ..j));

        // update the center element
        let center_element = T::sqrt(col_j - T::from_real(new_rowj_adjoint.norm_squared()));
        chol[(j, j)] = center_element.clone();

        // update the jth column
        let bottom_left_corner = self.chol.view_range(j.., ..j);
        // new_colj = (col_jplus - bottom_left_corner * new_rowj.adjoint()) / center_element;
        new_colj.gemm(
            -T::one() / center_element.clone(),
            &bottom_left_corner,
            &new_rowj_adjoint,
            T::one() / center_element,
        );
        chol.view_range_mut(j + 1.., j).copy_from(&new_colj);

        // update the bottom right corner
        let mut bottom_right_corner = chol.view_range_mut(j + 1.., j + 1..);
        Self::xx_rank_one_update(
            &mut bottom_right_corner,
            &mut new_colj,
            -T::RealField::one(),
        );

        Cholesky { chol }
    }

    /// Updates the decomposition such that we get the decomposition of the factored matrix with its `j`th column removed.
    /// Since the matrix is square, the `j`th row will also be removed.
    #[must_use]
    pub fn remove_column(&self, j: usize) -> Cholesky<T, DimDiff<D, U1>>
    where
        D: DimSub<U1>,
        DefaultAllocator: Allocator<DimDiff<D, U1>, DimDiff<D, U1>> + Allocator<D>,
    {
        let n = self.chol.nrows();
        assert!(n > 0, "The matrix needs at least one column.");
        assert!(j < n, "j needs to be within the bound of the matrix.");

        // loads the data into a new matrix except for the jth row/column
        // TODO: would it be worth it to avoid this zero initialization?
        let mut chol = Matrix::zeros_generic(
            self.chol.shape_generic().0.sub(Const::<1>),
            self.chol.shape_generic().1.sub(Const::<1>),
        );
        chol.view_range_mut(..j, ..j)
            .copy_from(&self.chol.view_range(..j, ..j));
        chol.view_range_mut(..j, j..)
            .copy_from(&self.chol.view_range(..j, j + 1..));
        chol.view_range_mut(j.., ..j)
            .copy_from(&self.chol.view_range(j + 1.., ..j));
        chol.view_range_mut(j.., j..)
            .copy_from(&self.chol.view_range(j + 1.., j + 1..));

        // updates the bottom right corner
        let mut bottom_right_corner = chol.view_range_mut(j.., j..);
        let mut workspace = self.chol.column(j).clone_owned();
        let mut old_colj = workspace.rows_range_mut(j + 1..);
        Self::xx_rank_one_update(&mut bottom_right_corner, &mut old_colj, T::RealField::one());

        Cholesky { chol }
    }

    /// Given the Cholesky decomposition of a matrix `M`, a scalar `sigma` and a vector `x`,
    /// performs a rank one update such that we end up with the decomposition of `M + sigma * (x * x.adjoint())`.
    ///
    /// This helper method is called by `rank_one_update` but also `insert_column` and `remove_column`
    /// where it is used on a square view of the decomposition
    fn xx_rank_one_update<Dm, Sm, Rx, Sx>(
        chol: &mut Matrix<T, Dm, Dm, Sm>,
        x: &mut Vector<T, Rx, Sx>,
        sigma: T::RealField,
    ) where
        //T: ComplexField,
        Dm: Dim,
        Rx: Dim,
        Sm: StorageMut<T, Dm, Dm>,
        Sx: StorageMut<T, Rx, U1>,
    {
        // heavily inspired by Eigen's `llt_rank_update_lower` implementation https://eigen.tuxfamily.org/dox/LLT_8h_source.html
        let n = x.nrows();
        assert_eq!(
            n,
            chol.nrows(),
            "The input vector must be of the same size as the factorized matrix."
        );

        let mut beta = crate::one::<T::RealField>();

        for j in 0..n {
            // updates the diagonal
            let diag = T::real(unsafe { chol.get_unchecked((j, j)).clone() });
            let diag2 = diag.clone() * diag.clone();
            let xj = unsafe { x.get_unchecked(j).clone() };
            let sigma_xj2 = sigma.clone() * T::modulus_squared(xj.clone());
            let gamma = diag2.clone() * beta.clone() + sigma_xj2.clone();
            let new_diag = (diag2.clone() + sigma_xj2.clone() / beta.clone()).sqrt();
            unsafe { *chol.get_unchecked_mut((j, j)) = T::from_real(new_diag.clone()) };
            beta += sigma_xj2 / diag2;
            // updates the terms of L
            let mut xjplus = x.rows_range_mut(j + 1..);
            let mut col_j = chol.view_range_mut(j + 1.., j);
            // temp_jplus -= (wj / T::from_real(diag)) * col_j;
            xjplus.axpy(-xj.clone() / T::from_real(diag.clone()), &col_j, T::one());
            if gamma != crate::zero::<T::RealField>() {
                // col_j = T::from_real(nljj / diag) * col_j  + (T::from_real(nljj * sigma / gamma) * T::conjugate(wj)) * temp_jplus;
                col_j.axpy(
                    T::from_real(new_diag.clone() * sigma.clone() / gamma) * T::conjugate(xj),
                    &xjplus,
                    T::from_real(new_diag / diag),
                );
            }
        }
    }
}

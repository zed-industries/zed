use crate::{RawStorage, SimdComplexField};
use num::{One, Zero};
use simba::scalar::{ClosedAddAssign, ClosedMulAssign};

use crate::base::allocator::Allocator;
use crate::base::blas_uninit::{axcpy_uninit, gemm_uninit, gemv_uninit};
use crate::base::constraint::{
    AreMultipliable, DimEq, SameNumberOfColumns, SameNumberOfRows, ShapeConstraint,
};
use crate::base::dimension::{Const, Dim, Dyn, U1, U2, U3, U4};
use crate::base::storage::{Storage, StorageMut};
use crate::base::uninit::Init;
use crate::base::{
    DVectorView, DefaultAllocator, Matrix, Scalar, SquareMatrix, Vector, VectorView,
};

/// # Dot/scalar product
impl<T, R: Dim, C: Dim, S: RawStorage<T, R, C>> Matrix<T, R, C, S>
where
    T: Scalar + Zero + ClosedAddAssign + ClosedMulAssign,
{
    #[inline(always)]
    fn dotx<R2: Dim, C2: Dim, SB>(
        &self,
        rhs: &Matrix<T, R2, C2, SB>,
        conjugate: impl Fn(T) -> T,
    ) -> T
    where
        SB: RawStorage<T, R2, C2>,
        ShapeConstraint: DimEq<R, R2> + DimEq<C, C2>,
    {
        assert!(
            self.nrows() == rhs.nrows(),
            "Dot product dimensions mismatch for shapes {:?} and {:?}: left rows != right rows.",
            self.shape(),
            rhs.shape(),
        );

        assert!(
            self.ncols() == rhs.ncols(),
            "Dot product dimensions mismatch for shapes {:?} and {:?}: left cols != right cols.",
            self.shape(),
            rhs.shape(),
        );

        // So we do some special cases for common fixed-size vectors of dimension lower than 8
        // because the `for` loop below won't be very efficient on those.
        if (R::is::<U2>() || R2::is::<U2>()) && (C::is::<U1>() || C2::is::<U1>()) {
            unsafe {
                let a = conjugate(self.get_unchecked((0, 0)).clone())
                    * rhs.get_unchecked((0, 0)).clone();
                let b = conjugate(self.get_unchecked((1, 0)).clone())
                    * rhs.get_unchecked((1, 0)).clone();

                return a + b;
            }
        }
        if (R::is::<U3>() || R2::is::<U3>()) && (C::is::<U1>() || C2::is::<U1>()) {
            unsafe {
                let a = conjugate(self.get_unchecked((0, 0)).clone())
                    * rhs.get_unchecked((0, 0)).clone();
                let b = conjugate(self.get_unchecked((1, 0)).clone())
                    * rhs.get_unchecked((1, 0)).clone();
                let c = conjugate(self.get_unchecked((2, 0)).clone())
                    * rhs.get_unchecked((2, 0)).clone();

                return a + b + c;
            }
        }
        if (R::is::<U4>() || R2::is::<U4>()) && (C::is::<U1>() || C2::is::<U1>()) {
            unsafe {
                let mut a = conjugate(self.get_unchecked((0, 0)).clone())
                    * rhs.get_unchecked((0, 0)).clone();
                let mut b = conjugate(self.get_unchecked((1, 0)).clone())
                    * rhs.get_unchecked((1, 0)).clone();
                let c = conjugate(self.get_unchecked((2, 0)).clone())
                    * rhs.get_unchecked((2, 0)).clone();
                let d = conjugate(self.get_unchecked((3, 0)).clone())
                    * rhs.get_unchecked((3, 0)).clone();

                a += c;
                b += d;

                return a + b;
            }
        }

        // All this is inspired from the "unrolled version" discussed in:
        // https://blog.theincredibleholk.org/blog/2012/12/10/optimizing-dot-product/
        //
        // And this comment from bluss:
        // https://users.rust-lang.org/t/how-to-zip-two-slices-efficiently/2048/12
        let mut res = T::zero();

        // We have to define them outside of the loop (and not inside at first assignment)
        // otherwise vectorization won't kick in for some reason.
        let mut acc0;
        let mut acc1;
        let mut acc2;
        let mut acc3;
        let mut acc4;
        let mut acc5;
        let mut acc6;
        let mut acc7;

        for j in 0..self.ncols() {
            let mut i = 0;

            acc0 = T::zero();
            acc1 = T::zero();
            acc2 = T::zero();
            acc3 = T::zero();
            acc4 = T::zero();
            acc5 = T::zero();
            acc6 = T::zero();
            acc7 = T::zero();

            while self.nrows() - i >= 8 {
                acc0 += unsafe {
                    conjugate(self.get_unchecked((i, j)).clone())
                        * rhs.get_unchecked((i, j)).clone()
                };
                acc1 += unsafe {
                    conjugate(self.get_unchecked((i + 1, j)).clone())
                        * rhs.get_unchecked((i + 1, j)).clone()
                };
                acc2 += unsafe {
                    conjugate(self.get_unchecked((i + 2, j)).clone())
                        * rhs.get_unchecked((i + 2, j)).clone()
                };
                acc3 += unsafe {
                    conjugate(self.get_unchecked((i + 3, j)).clone())
                        * rhs.get_unchecked((i + 3, j)).clone()
                };
                acc4 += unsafe {
                    conjugate(self.get_unchecked((i + 4, j)).clone())
                        * rhs.get_unchecked((i + 4, j)).clone()
                };
                acc5 += unsafe {
                    conjugate(self.get_unchecked((i + 5, j)).clone())
                        * rhs.get_unchecked((i + 5, j)).clone()
                };
                acc6 += unsafe {
                    conjugate(self.get_unchecked((i + 6, j)).clone())
                        * rhs.get_unchecked((i + 6, j)).clone()
                };
                acc7 += unsafe {
                    conjugate(self.get_unchecked((i + 7, j)).clone())
                        * rhs.get_unchecked((i + 7, j)).clone()
                };
                i += 8;
            }

            res += acc0 + acc4;
            res += acc1 + acc5;
            res += acc2 + acc6;
            res += acc3 + acc7;

            for k in i..self.nrows() {
                res += unsafe {
                    conjugate(self.get_unchecked((k, j)).clone())
                        * rhs.get_unchecked((k, j)).clone()
                }
            }
        }

        res
    }

    /// The dot product between two vectors or matrices (seen as vectors).
    ///
    /// This is equal to `self.transpose() * rhs`. For the sesquilinear complex dot product, use
    /// `self.dotc(rhs)`.
    ///
    /// Note that this is **not** the matrix multiplication as in, e.g., numpy. For matrix
    /// multiplication, use one of: `.gemm`, `.mul_to`, `.mul`, the `*` operator.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Vector3, Matrix2x3};
    /// let vec1 = Vector3::new(1.0, 2.0, 3.0);
    /// let vec2 = Vector3::new(0.1, 0.2, 0.3);
    /// assert_eq!(vec1.dot(&vec2), 1.4);
    ///
    /// let mat1 = Matrix2x3::new(1.0, 2.0, 3.0,
    ///                           4.0, 5.0, 6.0);
    /// let mat2 = Matrix2x3::new(0.1, 0.2, 0.3,
    ///                           0.4, 0.5, 0.6);
    /// assert_eq!(mat1.dot(&mat2), 9.1);
    /// ```
    ///
    #[inline]
    #[must_use]
    pub fn dot<R2: Dim, C2: Dim, SB>(&self, rhs: &Matrix<T, R2, C2, SB>) -> T
    where
        SB: RawStorage<T, R2, C2>,
        ShapeConstraint: DimEq<R, R2> + DimEq<C, C2>,
    {
        self.dotx(rhs, |e| e)
    }

    /// The conjugate-linear dot product between two vectors or matrices (seen as vectors).
    ///
    /// This is equal to `self.adjoint() * rhs`.
    /// For real vectors, this is identical to `self.dot(&rhs)`.
    /// Note that this is **not** the matrix multiplication as in, e.g., numpy. For matrix
    /// multiplication, use one of: `.gemm`, `.mul_to`, `.mul`, the `*` operator.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Vector2, Complex};
    /// let vec1 = Vector2::new(Complex::new(1.0, 2.0), Complex::new(3.0, 4.0));
    /// let vec2 = Vector2::new(Complex::new(0.4, 0.3), Complex::new(0.2, 0.1));
    /// assert_eq!(vec1.dotc(&vec2), Complex::new(2.0, -1.0));
    ///
    /// // Note that for complex vectors, we generally have:
    /// // vec1.dotc(&vec2) != vec2.dot(&vec2)
    /// assert_ne!(vec1.dotc(&vec2), vec1.dot(&vec2));
    /// ```
    #[inline]
    #[must_use]
    pub fn dotc<R2: Dim, C2: Dim, SB>(&self, rhs: &Matrix<T, R2, C2, SB>) -> T
    where
        T: SimdComplexField,
        SB: RawStorage<T, R2, C2>,
        ShapeConstraint: DimEq<R, R2> + DimEq<C, C2>,
    {
        self.dotx(rhs, T::simd_conjugate)
    }

    /// The dot product between the transpose of `self` and `rhs`.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Vector3, RowVector3, Matrix2x3, Matrix3x2};
    /// let vec1 = Vector3::new(1.0, 2.0, 3.0);
    /// let vec2 = RowVector3::new(0.1, 0.2, 0.3);
    /// assert_eq!(vec1.tr_dot(&vec2), 1.4);
    ///
    /// let mat1 = Matrix2x3::new(1.0, 2.0, 3.0,
    ///                           4.0, 5.0, 6.0);
    /// let mat2 = Matrix3x2::new(0.1, 0.4,
    ///                           0.2, 0.5,
    ///                           0.3, 0.6);
    /// assert_eq!(mat1.tr_dot(&mat2), 9.1);
    /// ```
    #[inline]
    #[must_use]
    pub fn tr_dot<R2: Dim, C2: Dim, SB>(&self, rhs: &Matrix<T, R2, C2, SB>) -> T
    where
        SB: RawStorage<T, R2, C2>,
        ShapeConstraint: DimEq<C, R2> + DimEq<R, C2>,
    {
        let (nrows, ncols) = self.shape();
        assert_eq!(
            (ncols, nrows),
            rhs.shape(),
            "Transposed dot product dimension mismatch."
        );

        let mut res = T::zero();

        for j in 0..self.nrows() {
            for i in 0..self.ncols() {
                res += unsafe {
                    self.get_unchecked((j, i)).clone() * rhs.get_unchecked((i, j)).clone()
                }
            }
        }

        res
    }
}

/// # BLAS functions
impl<T, D: Dim, S> Vector<T, D, S>
where
    T: Scalar + Zero + ClosedAddAssign + ClosedMulAssign,
    S: StorageMut<T, D>,
{
    /// Computes `self = a * x * c + b * self`.
    ///
    /// If `b` is zero, `self` is never read from.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::Vector3;
    /// let mut vec1 = Vector3::new(1.0, 2.0, 3.0);
    /// let vec2 = Vector3::new(0.1, 0.2, 0.3);
    /// vec1.axcpy(5.0, &vec2, 2.0, 5.0);
    /// assert_eq!(vec1, Vector3::new(6.0, 12.0, 18.0));
    /// ```
    #[inline]
    #[allow(clippy::many_single_char_names)]
    pub fn axcpy<D2: Dim, SB>(&mut self, a: T, x: &Vector<T, D2, SB>, c: T, b: T)
    where
        SB: Storage<T, D2>,
        ShapeConstraint: DimEq<D, D2>,
    {
        unsafe { axcpy_uninit(Init, self, a, x, c, b) };
    }

    /// Computes `self = a * x + b * self`.
    ///
    /// If `b` is zero, `self` is never read from.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::Vector3;
    /// let mut vec1 = Vector3::new(1.0, 2.0, 3.0);
    /// let vec2 = Vector3::new(0.1, 0.2, 0.3);
    /// vec1.axpy(10.0, &vec2, 5.0);
    /// assert_eq!(vec1, Vector3::new(6.0, 12.0, 18.0));
    /// ```
    #[inline]
    pub fn axpy<D2: Dim, SB>(&mut self, a: T, x: &Vector<T, D2, SB>, b: T)
    where
        T: One,
        SB: Storage<T, D2>,
        ShapeConstraint: DimEq<D, D2>,
    {
        assert_eq!(self.nrows(), x.nrows(), "Axpy: mismatched vector shapes.");
        self.axcpy(a, x, T::one(), b)
    }

    /// Computes `self = alpha * a * x + beta * self`, where `a` is a matrix, `x` a vector, and
    /// `alpha, beta` two scalars.
    ///
    /// If `beta` is zero, `self` is never read.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Matrix2, Vector2};
    /// let mut vec1 = Vector2::new(1.0, 2.0);
    /// let vec2 = Vector2::new(0.1, 0.2);
    /// let mat = Matrix2::new(1.0, 2.0,
    ///                        3.0, 4.0);
    /// vec1.gemv(10.0, &mat, &vec2, 5.0);
    /// assert_eq!(vec1, Vector2::new(10.0, 21.0));
    /// ```
    #[inline]
    pub fn gemv<R2: Dim, C2: Dim, D3: Dim, SB, SC>(
        &mut self,
        alpha: T,
        a: &Matrix<T, R2, C2, SB>,
        x: &Vector<T, D3, SC>,
        beta: T,
    ) where
        T: One,
        SB: Storage<T, R2, C2>,
        SC: Storage<T, D3>,
        ShapeConstraint: DimEq<D, R2> + AreMultipliable<R2, C2, D3, U1>,
    {
        // Safety: this is safe because we are passing Status == Init.
        unsafe { gemv_uninit(Init, self, alpha, a, x, beta) }
    }

    #[inline(always)]
    fn xxgemv<D2: Dim, D3: Dim, SB, SC>(
        &mut self,
        alpha: T,
        a: &SquareMatrix<T, D2, SB>,
        x: &Vector<T, D3, SC>,
        beta: T,
        dot: impl Fn(
            &DVectorView<'_, T, SB::RStride, SB::CStride>,
            &DVectorView<'_, T, SC::RStride, SC::CStride>,
        ) -> T,
    ) where
        T: One,
        SB: Storage<T, D2, D2>,
        SC: Storage<T, D3>,
        ShapeConstraint: DimEq<D, D2> + AreMultipliable<D2, D2, D3, U1>,
    {
        let dim1 = self.nrows();
        let dim2 = a.nrows();
        let dim3 = x.nrows();

        assert!(
            a.is_square(),
            "Symmetric cgemv: the input matrix must be square."
        );
        assert!(
            dim2 == dim3 && dim1 == dim2,
            "Symmetric cgemv: dimensions mismatch."
        );

        if dim2 == 0 {
            return;
        }

        // TODO: avoid bound checks.
        let col2 = a.column(0);
        let val = unsafe { x.vget_unchecked(0).clone() };
        self.axpy(alpha.clone() * val, &col2, beta);
        self[0] += alpha.clone() * dot(&a.view_range(1.., 0), &x.rows_range(1..));

        for j in 1..dim2 {
            let col2 = a.column(j);
            let dot = dot(&col2.rows_range(j..), &x.rows_range(j..));

            let val;
            unsafe {
                val = x.vget_unchecked(j).clone();
                *self.vget_unchecked_mut(j) += alpha.clone() * dot;
            }
            self.rows_range_mut(j + 1..).axpy(
                alpha.clone() * val,
                &col2.rows_range(j + 1..),
                T::one(),
            );
        }
    }

    /// Computes `self = alpha * a * x + beta * self`, where `a` is a **symmetric** matrix, `x` a
    /// vector, and `alpha, beta` two scalars.
    ///
    /// For hermitian matrices, use `.hegemv` instead.
    /// If `beta` is zero, `self` is never read. If `self` is read, only its lower-triangular part
    /// (including the diagonal) is actually read.
    ///
    /// # Examples
    /// ```
    /// # use nalgebra::{Matrix2, Vector2};
    /// let mat = Matrix2::new(1.0, 2.0,
    ///                        2.0, 4.0);
    /// let mut vec1 = Vector2::new(1.0, 2.0);
    /// let vec2 = Vector2::new(0.1, 0.2);
    /// vec1.sygemv(10.0, &mat, &vec2, 5.0);
    /// assert_eq!(vec1, Vector2::new(10.0, 20.0));
    ///
    ///
    /// // The matrix upper-triangular elements can be garbage because it is never
    /// // read by this method. Therefore, it is not necessary for the caller to
    /// // fill the matrix struct upper-triangle.
    /// let mat = Matrix2::new(1.0, 9999999.9999999,
    ///                        2.0, 4.0);
    /// let mut vec1 = Vector2::new(1.0, 2.0);
    /// vec1.sygemv(10.0, &mat, &vec2, 5.0);
    /// assert_eq!(vec1, Vector2::new(10.0, 20.0));
    /// ```
    #[inline]
    pub fn sygemv<D2: Dim, D3: Dim, SB, SC>(
        &mut self,
        alpha: T,
        a: &SquareMatrix<T, D2, SB>,
        x: &Vector<T, D3, SC>,
        beta: T,
    ) where
        T: One,
        SB: Storage<T, D2, D2>,
        SC: Storage<T, D3>,
        ShapeConstraint: DimEq<D, D2> + AreMultipliable<D2, D2, D3, U1>,
    {
        self.xxgemv(alpha, a, x, beta, |a, b| a.dot(b))
    }

    /// Computes `self = alpha * a * x + beta * self`, where `a` is an **hermitian** matrix, `x` a
    /// vector, and `alpha, beta` two scalars.
    ///
    /// If `beta` is zero, `self` is never read. If `self` is read, only its lower-triangular part
    /// (including the diagonal) is actually read.
    ///
    /// # Examples
    /// ```
    /// # use nalgebra::{Matrix2, Vector2, Complex};
    /// let mat = Matrix2::new(Complex::new(1.0, 0.0), Complex::new(2.0, -0.1),
    ///                        Complex::new(2.0, 1.0), Complex::new(4.0, 0.0));
    /// let mut vec1 = Vector2::new(Complex::new(1.0, 2.0), Complex::new(3.0, 4.0));
    /// let vec2 = Vector2::new(Complex::new(0.1, 0.2), Complex::new(0.3, 0.4));
    /// vec1.sygemv(Complex::new(10.0, 20.0), &mat, &vec2, Complex::new(5.0, 15.0));
    /// assert_eq!(vec1, Vector2::new(Complex::new(-48.0, 44.0), Complex::new(-75.0, 110.0)));
    ///
    ///
    /// // The matrix upper-triangular elements can be garbage because it is never
    /// // read by this method. Therefore, it is not necessary for the caller to
    /// // fill the matrix struct upper-triangle.
    ///
    /// let mat = Matrix2::new(Complex::new(1.0, 0.0), Complex::new(99999999.9, 999999999.9),
    ///                        Complex::new(2.0, 1.0), Complex::new(4.0, 0.0));
    /// let mut vec1 = Vector2::new(Complex::new(1.0, 2.0), Complex::new(3.0, 4.0));
    /// let vec2 = Vector2::new(Complex::new(0.1, 0.2), Complex::new(0.3, 0.4));
    /// vec1.sygemv(Complex::new(10.0, 20.0), &mat, &vec2, Complex::new(5.0, 15.0));
    /// assert_eq!(vec1, Vector2::new(Complex::new(-48.0, 44.0), Complex::new(-75.0, 110.0)));
    /// ```
    #[inline]
    pub fn hegemv<D2: Dim, D3: Dim, SB, SC>(
        &mut self,
        alpha: T,
        a: &SquareMatrix<T, D2, SB>,
        x: &Vector<T, D3, SC>,
        beta: T,
    ) where
        T: SimdComplexField,
        SB: Storage<T, D2, D2>,
        SC: Storage<T, D3>,
        ShapeConstraint: DimEq<D, D2> + AreMultipliable<D2, D2, D3, U1>,
    {
        self.xxgemv(alpha, a, x, beta, |a, b| a.dotc(b))
    }

    #[inline(always)]
    fn gemv_xx<R2: Dim, C2: Dim, D3: Dim, SB, SC>(
        &mut self,
        alpha: T,
        a: &Matrix<T, R2, C2, SB>,
        x: &Vector<T, D3, SC>,
        beta: T,
        dot: impl Fn(&VectorView<'_, T, R2, SB::RStride, SB::CStride>, &Vector<T, D3, SC>) -> T,
    ) where
        T: One,
        SB: Storage<T, R2, C2>,
        SC: Storage<T, D3>,
        ShapeConstraint: DimEq<D, C2> + AreMultipliable<C2, R2, D3, U1>,
    {
        let dim1 = self.nrows();
        let (nrows2, ncols2) = a.shape();
        let dim3 = x.nrows();

        assert!(
            nrows2 == dim3 && dim1 == ncols2,
            "Gemv: dimensions mismatch."
        );

        if ncols2 == 0 {
            return;
        }

        if beta.is_zero() {
            for j in 0..ncols2 {
                let val = unsafe { self.vget_unchecked_mut(j) };
                *val = alpha.clone() * dot(&a.column(j), x)
            }
        } else {
            for j in 0..ncols2 {
                let val = unsafe { self.vget_unchecked_mut(j) };
                *val = alpha.clone() * dot(&a.column(j), x) + beta.clone() * val.clone();
            }
        }
    }

    /// Computes `self = alpha * a.transpose() * x + beta * self`, where `a` is a matrix, `x` a vector, and
    /// `alpha, beta` two scalars.
    ///
    /// If `beta` is zero, `self` is never read.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Matrix2, Vector2};
    /// let mat = Matrix2::new(1.0, 3.0,
    ///                        2.0, 4.0);
    /// let mut vec1 = Vector2::new(1.0, 2.0);
    /// let vec2 = Vector2::new(0.1, 0.2);
    /// let expected = mat.transpose() * vec2 * 10.0 + vec1 * 5.0;
    ///
    /// vec1.gemv_tr(10.0, &mat, &vec2, 5.0);
    /// assert_eq!(vec1, expected);
    /// ```
    #[inline]
    pub fn gemv_tr<R2: Dim, C2: Dim, D3: Dim, SB, SC>(
        &mut self,
        alpha: T,
        a: &Matrix<T, R2, C2, SB>,
        x: &Vector<T, D3, SC>,
        beta: T,
    ) where
        T: One,
        SB: Storage<T, R2, C2>,
        SC: Storage<T, D3>,
        ShapeConstraint: DimEq<D, C2> + AreMultipliable<C2, R2, D3, U1>,
    {
        self.gemv_xx(alpha, a, x, beta, |a, b| a.dot(b))
    }

    /// Computes `self = alpha * a.adjoint() * x + beta * self`, where `a` is a matrix, `x` a vector, and
    /// `alpha, beta` two scalars.
    ///
    /// For real matrices, this is the same as `.gemv_tr`.
    /// If `beta` is zero, `self` is never read.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Matrix2, Vector2, Complex};
    /// let mat = Matrix2::new(Complex::new(1.0, 2.0), Complex::new(3.0, 4.0),
    ///                        Complex::new(5.0, 6.0), Complex::new(7.0, 8.0));
    /// let mut vec1 = Vector2::new(Complex::new(1.0, 2.0), Complex::new(3.0, 4.0));
    /// let vec2 = Vector2::new(Complex::new(0.1, 0.2), Complex::new(0.3, 0.4));
    /// let expected = mat.adjoint() * vec2 * Complex::new(10.0, 20.0) + vec1 * Complex::new(5.0, 15.0);
    ///
    /// vec1.gemv_ad(Complex::new(10.0, 20.0), &mat, &vec2, Complex::new(5.0, 15.0));
    /// assert_eq!(vec1, expected);
    /// ```
    #[inline]
    pub fn gemv_ad<R2: Dim, C2: Dim, D3: Dim, SB, SC>(
        &mut self,
        alpha: T,
        a: &Matrix<T, R2, C2, SB>,
        x: &Vector<T, D3, SC>,
        beta: T,
    ) where
        T: SimdComplexField,
        SB: Storage<T, R2, C2>,
        SC: Storage<T, D3>,
        ShapeConstraint: DimEq<D, C2> + AreMultipliable<C2, R2, D3, U1>,
    {
        self.gemv_xx(alpha, a, x, beta, |a, b| a.dotc(b))
    }
}

impl<T, R1: Dim, C1: Dim, S: StorageMut<T, R1, C1>> Matrix<T, R1, C1, S>
where
    T: Scalar + Zero + ClosedAddAssign + ClosedMulAssign,
{
    #[inline(always)]
    fn gerx<D2: Dim, D3: Dim, SB, SC>(
        &mut self,
        alpha: T,
        x: &Vector<T, D2, SB>,
        y: &Vector<T, D3, SC>,
        beta: T,
        conjugate: impl Fn(T) -> T,
    ) where
        T: One,
        SB: Storage<T, D2>,
        SC: Storage<T, D3>,
        ShapeConstraint: DimEq<R1, D2> + DimEq<C1, D3>,
    {
        let (nrows1, ncols1) = self.shape();
        let dim2 = x.nrows();
        let dim3 = y.nrows();

        assert!(
            nrows1 == dim2 && ncols1 == dim3,
            "ger: dimensions mismatch."
        );

        for j in 0..ncols1 {
            // TODO: avoid bound checks.
            let val = unsafe { conjugate(y.vget_unchecked(j).clone()) };
            self.column_mut(j)
                .axpy(alpha.clone() * val, x, beta.clone());
        }
    }

    /// Computes `self = alpha * x * y.transpose() + beta * self`.
    ///
    /// If `beta` is zero, `self` is never read.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Matrix2x3, Vector2, Vector3};
    /// let mut mat = Matrix2x3::repeat(4.0);
    /// let vec1 = Vector2::new(1.0, 2.0);
    /// let vec2 = Vector3::new(0.1, 0.2, 0.3);
    /// let expected = vec1 * vec2.transpose() * 10.0 + mat * 5.0;
    ///
    /// mat.ger(10.0, &vec1, &vec2, 5.0);
    /// assert_eq!(mat, expected);
    /// ```
    #[inline]
    pub fn ger<D2: Dim, D3: Dim, SB, SC>(
        &mut self,
        alpha: T,
        x: &Vector<T, D2, SB>,
        y: &Vector<T, D3, SC>,
        beta: T,
    ) where
        T: One,
        SB: Storage<T, D2>,
        SC: Storage<T, D3>,
        ShapeConstraint: DimEq<R1, D2> + DimEq<C1, D3>,
    {
        self.gerx(alpha, x, y, beta, |e| e)
    }

    /// Computes `self = alpha * x * y.adjoint() + beta * self`.
    ///
    /// If `beta` is zero, `self` is never read.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{Matrix2x3, Vector2, Vector3, Complex};
    /// let mut mat = Matrix2x3::repeat(Complex::new(4.0, 5.0));
    /// let vec1 = Vector2::new(Complex::new(1.0, 2.0), Complex::new(3.0, 4.0));
    /// let vec2 = Vector3::new(Complex::new(0.6, 0.5), Complex::new(0.4, 0.5), Complex::new(0.2, 0.1));
    /// let expected = vec1 * vec2.adjoint() * Complex::new(10.0, 20.0) + mat * Complex::new(5.0, 15.0);
    ///
    /// mat.gerc(Complex::new(10.0, 20.0), &vec1, &vec2, Complex::new(5.0, 15.0));
    /// assert_eq!(mat, expected);
    /// ```
    #[inline]
    pub fn gerc<D2: Dim, D3: Dim, SB, SC>(
        &mut self,
        alpha: T,
        x: &Vector<T, D2, SB>,
        y: &Vector<T, D3, SC>,
        beta: T,
    ) where
        T: SimdComplexField,
        SB: Storage<T, D2>,
        SC: Storage<T, D3>,
        ShapeConstraint: DimEq<R1, D2> + DimEq<C1, D3>,
    {
        self.gerx(alpha, x, y, beta, SimdComplexField::simd_conjugate)
    }

    /// Computes `self = alpha * a * b + beta * self`, where `a, b, self` are matrices.
    /// `alpha` and `beta` are scalar.
    ///
    /// If `beta` is zero, `self` is never read.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{Matrix2x3, Matrix3x4, Matrix2x4};
    /// let mut mat1 = Matrix2x4::identity();
    /// let mat2 = Matrix2x3::new(1.0, 2.0, 3.0,
    ///                           4.0, 5.0, 6.0);
    /// let mat3 = Matrix3x4::new(0.1, 0.2, 0.3, 0.4,
    ///                           0.5, 0.6, 0.7, 0.8,
    ///                           0.9, 1.0, 1.1, 1.2);
    /// let expected = mat2 * mat3 * 10.0 + mat1 * 5.0;
    ///
    /// mat1.gemm(10.0, &mat2, &mat3, 5.0);
    /// assert_relative_eq!(mat1, expected);
    /// ```
    #[inline]
    pub fn gemm<R2: Dim, C2: Dim, R3: Dim, C3: Dim, SB, SC>(
        &mut self,
        alpha: T,
        a: &Matrix<T, R2, C2, SB>,
        b: &Matrix<T, R3, C3, SC>,
        beta: T,
    ) where
        T: One,
        SB: Storage<T, R2, C2>,
        SC: Storage<T, R3, C3>,
        ShapeConstraint: SameNumberOfRows<R1, R2>
            + SameNumberOfColumns<C1, C3>
            + AreMultipliable<R2, C2, R3, C3>,
    {
        // SAFETY: this is valid because our matrices are initialized and
        // we are using status = Init.
        unsafe { gemm_uninit(Init, self, alpha, a, b, beta) }
    }

    /// Computes `self = alpha * a.transpose() * b + beta * self`, where `a, b, self` are matrices.
    /// `alpha` and `beta` are scalar.
    ///
    /// If `beta` is zero, `self` is never read.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{Matrix3x2, Matrix3x4, Matrix2x4};
    /// let mut mat1 = Matrix2x4::identity();
    /// let mat2 = Matrix3x2::new(1.0, 4.0,
    ///                           2.0, 5.0,
    ///                           3.0, 6.0);
    /// let mat3 = Matrix3x4::new(0.1, 0.2, 0.3, 0.4,
    ///                           0.5, 0.6, 0.7, 0.8,
    ///                           0.9, 1.0, 1.1, 1.2);
    /// let expected = mat2.transpose() * mat3 * 10.0 + mat1 * 5.0;
    ///
    /// mat1.gemm_tr(10.0, &mat2, &mat3, 5.0);
    /// assert_eq!(mat1, expected);
    /// ```
    #[inline]
    pub fn gemm_tr<R2: Dim, C2: Dim, R3: Dim, C3: Dim, SB, SC>(
        &mut self,
        alpha: T,
        a: &Matrix<T, R2, C2, SB>,
        b: &Matrix<T, R3, C3, SC>,
        beta: T,
    ) where
        T: One,
        SB: Storage<T, R2, C2>,
        SC: Storage<T, R3, C3>,
        ShapeConstraint: SameNumberOfRows<R1, C2>
            + SameNumberOfColumns<C1, C3>
            + AreMultipliable<C2, R2, R3, C3>,
    {
        let (nrows1, ncols1) = self.shape();
        let (nrows2, ncols2) = a.shape();
        let (nrows3, ncols3) = b.shape();

        assert_eq!(
            nrows2, nrows3,
            "gemm: dimensions mismatch for multiplication."
        );
        assert_eq!(
            (nrows1, ncols1),
            (ncols2, ncols3),
            "gemm: dimensions mismatch for addition."
        );

        for j1 in 0..ncols1 {
            // TODO: avoid bound checks.
            self.column_mut(j1)
                .gemv_tr(alpha.clone(), a, &b.column(j1), beta.clone());
        }
    }

    /// Computes `self = alpha * a.adjoint() * b + beta * self`, where `a, b, self` are matrices.
    /// `alpha` and `beta` are scalar.
    ///
    /// If `beta` is zero, `self` is never read.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{Matrix3x2, Matrix3x4, Matrix2x4, Complex};
    /// let mut mat1 = Matrix2x4::identity();
    /// let mat2 = Matrix3x2::new(Complex::new(1.0, 4.0), Complex::new(7.0, 8.0),
    ///                           Complex::new(2.0, 5.0), Complex::new(9.0, 10.0),
    ///                           Complex::new(3.0, 6.0), Complex::new(11.0, 12.0));
    /// let mat3 = Matrix3x4::new(Complex::new(0.1, 1.3), Complex::new(0.2, 1.4), Complex::new(0.3, 1.5), Complex::new(0.4, 1.6),
    ///                           Complex::new(0.5, 1.7), Complex::new(0.6, 1.8), Complex::new(0.7, 1.9), Complex::new(0.8, 2.0),
    ///                           Complex::new(0.9, 2.1), Complex::new(1.0, 2.2), Complex::new(1.1, 2.3), Complex::new(1.2, 2.4));
    /// let expected = mat2.adjoint() * mat3 * Complex::new(10.0, 20.0) + mat1 * Complex::new(5.0, 15.0);
    ///
    /// mat1.gemm_ad(Complex::new(10.0, 20.0), &mat2, &mat3, Complex::new(5.0, 15.0));
    /// assert_eq!(mat1, expected);
    /// ```
    #[inline]
    pub fn gemm_ad<R2: Dim, C2: Dim, R3: Dim, C3: Dim, SB, SC>(
        &mut self,
        alpha: T,
        a: &Matrix<T, R2, C2, SB>,
        b: &Matrix<T, R3, C3, SC>,
        beta: T,
    ) where
        T: SimdComplexField,
        SB: Storage<T, R2, C2>,
        SC: Storage<T, R3, C3>,
        ShapeConstraint: SameNumberOfRows<R1, C2>
            + SameNumberOfColumns<C1, C3>
            + AreMultipliable<C2, R2, R3, C3>,
    {
        let (nrows1, ncols1) = self.shape();
        let (nrows2, ncols2) = a.shape();
        let (nrows3, ncols3) = b.shape();

        assert_eq!(
            nrows2, nrows3,
            "gemm: dimensions mismatch for multiplication."
        );
        assert_eq!(
            (nrows1, ncols1),
            (ncols2, ncols3),
            "gemm: dimensions mismatch for addition."
        );

        for j1 in 0..ncols1 {
            // TODO: avoid bound checks.
            self.column_mut(j1)
                .gemv_ad(alpha.clone(), a, &b.column(j1), beta.clone());
        }
    }
}

impl<T, R1: Dim, C1: Dim, S: StorageMut<T, R1, C1>> Matrix<T, R1, C1, S>
where
    T: Scalar + Zero + ClosedAddAssign + ClosedMulAssign,
{
    #[inline(always)]
    fn xxgerx<D2: Dim, D3: Dim, SB, SC>(
        &mut self,
        alpha: T,
        x: &Vector<T, D2, SB>,
        y: &Vector<T, D3, SC>,
        beta: T,
        conjugate: impl Fn(T) -> T,
    ) where
        T: One,
        SB: Storage<T, D2>,
        SC: Storage<T, D3>,
        ShapeConstraint: DimEq<R1, D2> + DimEq<C1, D3>,
    {
        let dim1 = self.nrows();
        let dim2 = x.nrows();
        let dim3 = y.nrows();

        assert!(
            self.is_square(),
            "Symmetric ger: the input matrix must be square."
        );
        assert!(dim1 == dim2 && dim1 == dim3, "ger: dimensions mismatch.");

        for j in 0..dim1 {
            let val = unsafe { conjugate(y.vget_unchecked(j).clone()) };
            let subdim = Dyn(dim1 - j);
            // TODO: avoid bound checks.
            self.generic_view_mut((j, j), (subdim, Const::<1>)).axpy(
                alpha.clone() * val,
                &x.rows_range(j..),
                beta.clone(),
            );
        }
    }

    /// Computes `self = alpha * x * y.transpose() + beta * self`, where `self` is a **symmetric**
    /// matrix.
    ///
    /// If `beta` is zero, `self` is never read. The result is symmetric. Only the lower-triangular
    /// (including the diagonal) part of `self` is read/written.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Matrix2, Vector2};
    /// let mut mat = Matrix2::identity();
    /// let vec1 = Vector2::new(1.0, 2.0);
    /// let vec2 = Vector2::new(0.1, 0.2);
    /// let expected = vec1 * vec2.transpose() * 10.0 + mat * 5.0;
    /// mat.m12 = 99999.99999; // This component is on the upper-triangular part and will not be read/written.
    ///
    /// mat.ger_symm(10.0, &vec1, &vec2, 5.0);
    /// assert_eq!(mat.lower_triangle(), expected.lower_triangle());
    /// assert_eq!(mat.m12, 99999.99999); // This was untouched.
    /// ```
    #[inline]
    #[deprecated(note = "This is renamed `syger` to match the original BLAS terminology.")]
    pub fn ger_symm<D2: Dim, D3: Dim, SB, SC>(
        &mut self,
        alpha: T,
        x: &Vector<T, D2, SB>,
        y: &Vector<T, D3, SC>,
        beta: T,
    ) where
        T: One,
        SB: Storage<T, D2>,
        SC: Storage<T, D3>,
        ShapeConstraint: DimEq<R1, D2> + DimEq<C1, D3>,
    {
        self.syger(alpha, x, y, beta)
    }

    /// Computes `self = alpha * x * y.transpose() + beta * self`, where `self` is a **symmetric**
    /// matrix.
    ///
    /// For hermitian complex matrices, use `.hegerc` instead.
    /// If `beta` is zero, `self` is never read. The result is symmetric. Only the lower-triangular
    /// (including the diagonal) part of `self` is read/written.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Matrix2, Vector2};
    /// let mut mat = Matrix2::identity();
    /// let vec1 = Vector2::new(1.0, 2.0);
    /// let vec2 = Vector2::new(0.1, 0.2);
    /// let expected = vec1 * vec2.transpose() * 10.0 + mat * 5.0;
    /// mat.m12 = 99999.99999; // This component is on the upper-triangular part and will not be read/written.
    ///
    /// mat.syger(10.0, &vec1, &vec2, 5.0);
    /// assert_eq!(mat.lower_triangle(), expected.lower_triangle());
    /// assert_eq!(mat.m12, 99999.99999); // This was untouched.
    /// ```
    #[inline]
    pub fn syger<D2: Dim, D3: Dim, SB, SC>(
        &mut self,
        alpha: T,
        x: &Vector<T, D2, SB>,
        y: &Vector<T, D3, SC>,
        beta: T,
    ) where
        T: One,
        SB: Storage<T, D2>,
        SC: Storage<T, D3>,
        ShapeConstraint: DimEq<R1, D2> + DimEq<C1, D3>,
    {
        self.xxgerx(alpha, x, y, beta, |e| e)
    }

    /// Computes `self = alpha * x * y.adjoint() + beta * self`, where `self` is an **hermitian**
    /// matrix.
    ///
    /// If `beta` is zero, `self` is never read. The result is symmetric. Only the lower-triangular
    /// (including the diagonal) part of `self` is read/written.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Matrix2, Vector2, Complex};
    /// let mut mat = Matrix2::identity();
    /// let vec1 = Vector2::new(Complex::new(1.0, 3.0), Complex::new(2.0, 4.0));
    /// let vec2 = Vector2::new(Complex::new(0.2, 0.4), Complex::new(0.1, 0.3));
    /// let expected = vec1 * vec2.adjoint() * Complex::new(10.0, 20.0) + mat * Complex::new(5.0, 15.0);
    /// mat.m12 = Complex::new(99999.99999, 88888.88888); // This component is on the upper-triangular part and will not be read/written.
    ///
    /// mat.hegerc(Complex::new(10.0, 20.0), &vec1, &vec2, Complex::new(5.0, 15.0));
    /// assert_eq!(mat.lower_triangle(), expected.lower_triangle());
    /// assert_eq!(mat.m12, Complex::new(99999.99999, 88888.88888)); // This was untouched.
    /// ```
    #[inline]
    pub fn hegerc<D2: Dim, D3: Dim, SB, SC>(
        &mut self,
        alpha: T,
        x: &Vector<T, D2, SB>,
        y: &Vector<T, D3, SC>,
        beta: T,
    ) where
        T: SimdComplexField,
        SB: Storage<T, D2>,
        SC: Storage<T, D3>,
        ShapeConstraint: DimEq<R1, D2> + DimEq<C1, D3>,
    {
        self.xxgerx(alpha, x, y, beta, SimdComplexField::simd_conjugate)
    }
}

impl<T, D1: Dim, S: StorageMut<T, D1, D1>> SquareMatrix<T, D1, S>
where
    T: Scalar + Zero + One + ClosedAddAssign + ClosedMulAssign,
{
    /// Computes the quadratic form `self = alpha * lhs * mid * lhs.transpose() + beta * self`.
    ///
    /// This uses the provided workspace `work` to avoid allocations for intermediate results.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{DMatrix, DVector};
    /// // Note that all those would also work with statically-sized matrices.
    /// // We use DMatrix/DVector since that's the only case where pre-allocating the
    /// // workspace is actually useful (assuming the same workspace is re-used for
    /// // several computations) because it avoids repeated dynamic allocations.
    /// let mut mat = DMatrix::identity(2, 2);
    /// let lhs = DMatrix::from_row_slice(2, 3, &[1.0, 2.0, 3.0,
    ///                                           4.0, 5.0, 6.0]);
    /// let mid = DMatrix::from_row_slice(3, 3, &[0.1, 0.2, 0.3,
    ///                                           0.5, 0.6, 0.7,
    ///                                           0.9, 1.0, 1.1]);
    /// // The random shows that values on the workspace do not
    /// // matter as they will be overwritten.
    /// let mut workspace = DVector::new_random(2);
    /// let expected = &lhs * &mid * lhs.transpose() * 10.0 + &mat * 5.0;
    ///
    /// mat.quadform_tr_with_workspace(&mut workspace, 10.0, &lhs, &mid, 5.0);
    /// assert_relative_eq!(mat, expected);
    /// ```
    pub fn quadform_tr_with_workspace<D2, S2, R3, C3, S3, D4, S4>(
        &mut self,
        work: &mut Vector<T, D2, S2>,
        alpha: T,
        lhs: &Matrix<T, R3, C3, S3>,
        mid: &SquareMatrix<T, D4, S4>,
        beta: T,
    ) where
        D2: Dim,
        R3: Dim,
        C3: Dim,
        D4: Dim,
        S2: StorageMut<T, D2>,
        S3: Storage<T, R3, C3>,
        S4: Storage<T, D4, D4>,
        ShapeConstraint: DimEq<D1, D2> + DimEq<D1, R3> + DimEq<D2, R3> + DimEq<C3, D4>,
    {
        work.gemv(T::one(), lhs, &mid.column(0), T::zero());
        self.ger(alpha.clone(), work, &lhs.column(0), beta);

        for j in 1..mid.ncols() {
            work.gemv(T::one(), lhs, &mid.column(j), T::zero());
            self.ger(alpha.clone(), work, &lhs.column(j), T::one());
        }
    }

    /// Computes the quadratic form `self = alpha * lhs * mid * lhs.transpose() + beta * self`.
    ///
    /// This allocates a workspace vector of dimension D1 for intermediate results.
    /// If `D1` is a type-level integer, then the allocation is performed on the stack.
    /// Use `.quadform_tr_with_workspace(...)` instead to avoid allocations.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{Matrix2, Matrix3, Matrix2x3, Vector2};
    /// let mut mat = Matrix2::identity();
    /// let lhs = Matrix2x3::new(1.0, 2.0, 3.0,
    ///                          4.0, 5.0, 6.0);
    /// let mid = Matrix3::new(0.1, 0.2, 0.3,
    ///                        0.5, 0.6, 0.7,
    ///                        0.9, 1.0, 1.1);
    /// let expected = lhs * mid * lhs.transpose() * 10.0 + mat * 5.0;
    ///
    /// mat.quadform_tr(10.0, &lhs, &mid, 5.0);
    /// assert_relative_eq!(mat, expected);
    /// ```
    pub fn quadform_tr<R3, C3, S3, D4, S4>(
        &mut self,
        alpha: T,
        lhs: &Matrix<T, R3, C3, S3>,
        mid: &SquareMatrix<T, D4, S4>,
        beta: T,
    ) where
        R3: Dim,
        C3: Dim,
        D4: Dim,
        S3: Storage<T, R3, C3>,
        S4: Storage<T, D4, D4>,
        ShapeConstraint: DimEq<D1, D1> + DimEq<D1, R3> + DimEq<C3, D4>,
        DefaultAllocator: Allocator<D1>,
    {
        // TODO: would it be useful to avoid the zero-initialization of the workspace data?
        let mut work = Matrix::zeros_generic(self.shape_generic().0, Const::<1>);
        self.quadform_tr_with_workspace(&mut work, alpha, lhs, mid, beta)
    }

    /// Computes the quadratic form `self = alpha * rhs.transpose() * mid * rhs + beta * self`.
    ///
    /// This uses the provided workspace `work` to avoid allocations for intermediate results.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{DMatrix, DVector};
    /// // Note that all those would also work with statically-sized matrices.
    /// // We use DMatrix/DVector since that's the only case where pre-allocating the
    /// // workspace is actually useful (assuming the same workspace is re-used for
    /// // several computations) because it avoids repeated dynamic allocations.
    /// let mut mat = DMatrix::identity(2, 2);
    /// let rhs = DMatrix::from_row_slice(3, 2, &[1.0, 2.0,
    ///                                           3.0, 4.0,
    ///                                           5.0, 6.0]);
    /// let mid = DMatrix::from_row_slice(3, 3, &[0.1, 0.2, 0.3,
    ///                                           0.5, 0.6, 0.7,
    ///                                           0.9, 1.0, 1.1]);
    /// // The random shows that values on the workspace do not
    /// // matter as they will be overwritten.
    /// let mut workspace = DVector::new_random(3);
    /// let expected = rhs.transpose() * &mid * &rhs * 10.0 + &mat * 5.0;
    ///
    /// mat.quadform_with_workspace(&mut workspace, 10.0, &mid, &rhs, 5.0);
    /// assert_relative_eq!(mat, expected);
    /// ```
    pub fn quadform_with_workspace<D2, S2, D3, S3, R4, C4, S4>(
        &mut self,
        work: &mut Vector<T, D2, S2>,
        alpha: T,
        mid: &SquareMatrix<T, D3, S3>,
        rhs: &Matrix<T, R4, C4, S4>,
        beta: T,
    ) where
        D2: Dim,
        D3: Dim,
        R4: Dim,
        C4: Dim,
        S2: StorageMut<T, D2>,
        S3: Storage<T, D3, D3>,
        S4: Storage<T, R4, C4>,
        ShapeConstraint:
            DimEq<D3, R4> + DimEq<D1, C4> + DimEq<D2, D3> + AreMultipliable<C4, R4, D2, U1>,
    {
        work.gemv(T::one(), mid, &rhs.column(0), T::zero());
        self.column_mut(0)
            .gemv_tr(alpha.clone(), rhs, work, beta.clone());

        for j in 1..rhs.ncols() {
            work.gemv(T::one(), mid, &rhs.column(j), T::zero());
            self.column_mut(j)
                .gemv_tr(alpha.clone(), rhs, work, beta.clone());
        }
    }

    /// Computes the quadratic form `self = alpha * rhs.transpose() * mid * rhs + beta * self`.
    ///
    /// This allocates a workspace vector of dimension D2 for intermediate results.
    /// If `D2` is a type-level integer, then the allocation is performed on the stack.
    /// Use `.quadform_with_workspace(...)` instead to avoid allocations.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{Matrix2, Matrix3x2, Matrix3};
    /// let mut mat = Matrix2::identity();
    /// let rhs = Matrix3x2::new(1.0, 2.0,
    ///                          3.0, 4.0,
    ///                          5.0, 6.0);
    /// let mid = Matrix3::new(0.1, 0.2, 0.3,
    ///                        0.5, 0.6, 0.7,
    ///                        0.9, 1.0, 1.1);
    /// let expected = rhs.transpose() * mid * rhs * 10.0 + mat * 5.0;
    ///
    /// mat.quadform(10.0, &mid, &rhs, 5.0);
    /// assert_relative_eq!(mat, expected);
    /// ```
    pub fn quadform<D2, S2, R3, C3, S3>(
        &mut self,
        alpha: T,
        mid: &SquareMatrix<T, D2, S2>,
        rhs: &Matrix<T, R3, C3, S3>,
        beta: T,
    ) where
        D2: Dim,
        R3: Dim,
        C3: Dim,
        S2: Storage<T, D2, D2>,
        S3: Storage<T, R3, C3>,
        ShapeConstraint: DimEq<D2, R3> + DimEq<D1, C3> + AreMultipliable<C3, R3, D2, U1>,
        DefaultAllocator: Allocator<D2>,
    {
        // TODO: would it be useful to avoid the zero-initialization of the workspace data?
        let mut work = Vector::zeros_generic(mid.shape_generic().0, Const::<1>);
        self.quadform_with_workspace(&mut work, alpha, mid, rhs, beta)
    }
}

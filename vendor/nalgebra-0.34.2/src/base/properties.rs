// Matrix properties checks.
use approx::RelativeEq;
use num::{One, Zero};

use simba::scalar::{ClosedAddAssign, ClosedMulAssign, ComplexField, RealField};

use crate::RawStorage;
use crate::base::allocator::Allocator;
use crate::base::dimension::{Dim, DimMin};
use crate::base::storage::Storage;
use crate::base::{DefaultAllocator, Matrix, SquareMatrix};

impl<T, R: Dim, C: Dim, S: RawStorage<T, R, C>> Matrix<T, R, C, S> {
    /// The total number of elements of this matrix.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use nalgebra::Matrix3x4;
    /// let mat = Matrix3x4::<f32>::zeros();
    /// assert_eq!(mat.len(), 12);
    /// ```
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        let (nrows, ncols) = self.shape();
        nrows * ncols
    }

    /// Returns true if the matrix contains no elements.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use nalgebra::Matrix3x4;
    /// let mat = Matrix3x4::<f32>::zeros();
    /// assert!(!mat.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Indicates if this is a square matrix.
    #[inline]
    #[must_use]
    pub fn is_square(&self) -> bool {
        let (nrows, ncols) = self.shape();
        nrows == ncols
    }

    // TODO: RelativeEq prevents us from using those methods on integer matrices…
    /// Indicated if this is the identity matrix within a relative error of `eps`.
    ///
    /// If the matrix is diagonal, this checks that diagonal elements (i.e. at coordinates `(i, i)`
    /// for i from `0` to `min(R, C)`) are equal one; and that all other elements are zero.
    #[inline]
    #[must_use]
    pub fn is_identity(&self, eps: T::Epsilon) -> bool
    where
        T: Zero + One + RelativeEq,
        T::Epsilon: Clone,
    {
        let (nrows, ncols) = self.shape();

        for j in 0..ncols {
            for i in 0..nrows {
                let el = unsafe { self.get_unchecked((i, j)) };
                if (i == j && !relative_eq!(*el, T::one(), epsilon = eps.clone()))
                    || (i != j && !relative_eq!(*el, T::zero(), epsilon = eps.clone()))
                {
                    return false;
                }
            }
        }

        true
    }
}

impl<T: ComplexField, R: Dim, C: Dim, S: Storage<T, R, C>> Matrix<T, R, C, S> {
    /// Checks that `Mᵀ × M = Id`.
    ///
    /// In this definition `Id` is approximately equal to the identity matrix with a relative error
    /// equal to `eps`.
    #[inline]
    #[must_use]
    pub fn is_orthogonal(&self, eps: T::Epsilon) -> bool
    where
        T: Zero + One + ClosedAddAssign + ClosedMulAssign + RelativeEq,
        S: Storage<T, R, C>,
        T::Epsilon: Clone,
        DefaultAllocator: Allocator<R, C> + Allocator<C, C>,
    {
        (self.ad_mul(self)).is_identity(eps)
    }
}

impl<T: RealField, D: Dim, S: Storage<T, D, D>> SquareMatrix<T, D, S>
where
    DefaultAllocator: Allocator<D, D>,
{
    /// Checks that this matrix is orthogonal and has a determinant equal to 1.
    #[inline]
    #[must_use]
    pub fn is_special_orthogonal(&self, eps: T) -> bool
    where
        D: DimMin<D, Output = D>,
        DefaultAllocator: Allocator<D>,
    {
        self.is_square() && self.is_orthogonal(eps) && self.determinant() > T::zero()
    }

    /// Returns `true` if this matrix is invertible.
    #[inline]
    #[must_use]
    pub fn is_invertible(&self) -> bool {
        // TODO: improve this?
        self.clone_owned().try_inverse().is_some()
    }
}

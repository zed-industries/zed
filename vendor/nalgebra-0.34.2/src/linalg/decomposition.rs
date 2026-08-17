use crate::storage::Storage;
use crate::{
    Allocator, Bidiagonal, Cholesky, ColPivQR, ComplexField, DefaultAllocator, Dim, DimDiff,
    DimMin, DimMinimum, DimSub, FullPivLU, Hessenberg, LU, Matrix, OMatrix, QR, RealField, SVD,
    Schur, SymmetricEigen, SymmetricTridiagonal, U1, UDU,
};

/// # Rectangular matrix decomposition
///
/// This section contains the methods for computing some common decompositions of rectangular
/// matrices with real or complex components. The following are currently supported:
///
/// | Decomposition            | Factors             | Details |
/// | -------------------------|---------------------|--------------|
/// | QR                       | `Q * R`             | `Q` is an unitary matrix, and `R` is upper-triangular. |
/// | QR with column pivoting  | `Q * R * P⁻¹`       | `Q` is an unitary matrix, and `R` is upper-triangular. `P` is a permutation matrix. |
/// | LU with partial pivoting | `P⁻¹ * L * U`       | `L` is lower-triangular with a diagonal filled with `1` and `U` is upper-triangular. `P` is a permutation matrix. |
/// | LU with full pivoting    | `P⁻¹ * L * U * Q⁻¹` | `L` is lower-triangular with a diagonal filled with `1` and `U` is upper-triangular. `P` and `Q` are permutation matrices. |
/// | SVD                      | `U * Σ * Vᵀ`        | `U` and `V` are two orthogonal matrices and `Σ` is a diagonal matrix containing the singular values. |
/// | Polar (Left Polar)       | `P' * U`            | `U` is semi-unitary/unitary and `P'` is a positive semi-definite Hermitian Matrix
impl<T: ComplexField, R: Dim, C: Dim, S: Storage<T, R, C>> Matrix<T, R, C, S> {
    /// Computes the bidiagonalization using householder reflections.
    pub fn bidiagonalize(self) -> Bidiagonal<T, R, C>
    where
        R: DimMin<C>,
        DimMinimum<R, C>: DimSub<U1>,
        DefaultAllocator: Allocator<R, C>
            + Allocator<C>
            + Allocator<R>
            + Allocator<DimMinimum<R, C>>
            + Allocator<DimDiff<DimMinimum<R, C>, U1>>,
    {
        Bidiagonal::new(self.into_owned())
    }

    /// Computes the LU decomposition with full pivoting of `matrix`.
    ///
    /// This effectively computes `P, L, U, Q` such that `P * matrix * Q = LU`.
    pub fn full_piv_lu(self) -> FullPivLU<T, R, C>
    where
        R: DimMin<C>,
        DefaultAllocator: Allocator<R, C> + Allocator<DimMinimum<R, C>>,
    {
        FullPivLU::new(self.into_owned())
    }

    /// Computes the LU decomposition with partial (row) pivoting of `matrix`.
    pub fn lu(self) -> LU<T, R, C>
    where
        R: DimMin<C>,
        DefaultAllocator: Allocator<R, C> + Allocator<DimMinimum<R, C>>,
    {
        LU::new(self.into_owned())
    }

    /// Computes the QR decomposition of this matrix.
    pub fn qr(self) -> QR<T, R, C>
    where
        R: DimMin<C>,
        DefaultAllocator: Allocator<R, C> + Allocator<R> + Allocator<DimMinimum<R, C>>,
    {
        QR::new(self.into_owned())
    }

    /// Computes the QR decomposition (with column pivoting) of this matrix.
    pub fn col_piv_qr(self) -> ColPivQR<T, R, C>
    where
        R: DimMin<C>,
        DefaultAllocator: Allocator<R, C>
            + Allocator<R>
            + Allocator<DimMinimum<R, C>>
            + Allocator<DimMinimum<R, C>>,
    {
        ColPivQR::new(self.into_owned())
    }

    /// Computes the Singular Value Decomposition using implicit shift.
    /// The singular values are guaranteed to be sorted in descending order.
    /// If this order is not required consider using `svd_unordered`.
    pub fn svd(self, compute_u: bool, compute_v: bool) -> SVD<T, R, C>
    where
        R: DimMin<C>,
        DimMinimum<R, C>: DimSub<U1>, // for Bidiagonal.
        DefaultAllocator: Allocator<R, C>
            + Allocator<C>
            + Allocator<R>
            + Allocator<DimDiff<DimMinimum<R, C>, U1>>
            + Allocator<DimMinimum<R, C>, C>
            + Allocator<R, DimMinimum<R, C>>
            + Allocator<DimMinimum<R, C>>,
    {
        SVD::new(self.into_owned(), compute_u, compute_v)
    }

    /// Computes the Singular Value Decomposition using implicit shift.
    /// The singular values are not guaranteed to be sorted in any particular order.
    /// If a descending order is required, consider using `svd` instead.
    pub fn svd_unordered(self, compute_u: bool, compute_v: bool) -> SVD<T, R, C>
    where
        R: DimMin<C>,
        DimMinimum<R, C>: DimSub<U1>, // for Bidiagonal.
        DefaultAllocator: Allocator<R, C>
            + Allocator<C>
            + Allocator<R>
            + Allocator<DimDiff<DimMinimum<R, C>, U1>>
            + Allocator<DimMinimum<R, C>, C>
            + Allocator<R, DimMinimum<R, C>>
            + Allocator<DimMinimum<R, C>>,
    {
        SVD::new_unordered(self.into_owned(), compute_u, compute_v)
    }

    /// Attempts to compute the Singular Value Decomposition of `matrix` using implicit shift.
    /// The singular values are guaranteed to be sorted in descending order.
    /// If this order is not required consider using `try_svd_unordered`.
    ///
    /// # Arguments
    ///
    /// * `compute_u` − set this to `true` to enable the computation of left-singular vectors.
    /// * `compute_v` − set this to `true` to enable the computation of right-singular vectors.
    /// * `eps`       − tolerance used to determine when a value converged to 0.
    /// * `max_niter` − maximum total number of iterations performed by the algorithm. If this
    ///   number of iteration is exceeded, `None` is returned. If `niter == 0`, then the algorithm
    ///   continues indefinitely until convergence.
    pub fn try_svd(
        self,
        compute_u: bool,
        compute_v: bool,
        eps: T::RealField,
        max_niter: usize,
    ) -> Option<SVD<T, R, C>>
    where
        R: DimMin<C>,
        DimMinimum<R, C>: DimSub<U1>, // for Bidiagonal.
        DefaultAllocator: Allocator<R, C>
            + Allocator<C>
            + Allocator<R>
            + Allocator<DimDiff<DimMinimum<R, C>, U1>>
            + Allocator<DimMinimum<R, C>, C>
            + Allocator<R, DimMinimum<R, C>>
            + Allocator<DimMinimum<R, C>>,
    {
        SVD::try_new(self.into_owned(), compute_u, compute_v, eps, max_niter)
    }

    /// Attempts to compute the Singular Value Decomposition of `matrix` using implicit shift.
    /// The singular values are not guaranteed to be sorted in any particular order.
    /// If a descending order is required, consider using `try_svd` instead.
    ///
    /// # Arguments
    ///
    /// * `compute_u` − set this to `true` to enable the computation of left-singular vectors.
    /// * `compute_v` − set this to `true` to enable the computation of right-singular vectors.
    /// * `eps`       − tolerance used to determine when a value converged to 0.
    /// * `max_niter` − maximum total number of iterations performed by the algorithm. If this
    ///   number of iteration is exceeded, `None` is returned. If `niter == 0`, then the algorithm
    ///   continues indefinitely until convergence.
    pub fn try_svd_unordered(
        self,
        compute_u: bool,
        compute_v: bool,
        eps: T::RealField,
        max_niter: usize,
    ) -> Option<SVD<T, R, C>>
    where
        R: DimMin<C>,
        DimMinimum<R, C>: DimSub<U1>, // for Bidiagonal.
        DefaultAllocator: Allocator<R, C>
            + Allocator<C>
            + Allocator<R>
            + Allocator<DimDiff<DimMinimum<R, C>, U1>>
            + Allocator<DimMinimum<R, C>, C>
            + Allocator<R, DimMinimum<R, C>>
            + Allocator<DimMinimum<R, C>>
            + Allocator<DimMinimum<R, C>>
            + Allocator<DimDiff<DimMinimum<R, C>, U1>>,
    {
        SVD::try_new_unordered(self.into_owned(), compute_u, compute_v, eps, max_niter)
    }

    /// Computes the Polar Decomposition of  a `matrix` (indirectly uses SVD).
    pub fn polar(self) -> (OMatrix<T, R, R>, OMatrix<T, R, C>)
    where
        R: DimMin<C>,
        DimMinimum<R, C>: DimSub<U1>, // for Bidiagonal.
        DefaultAllocator: Allocator<R, C>
            + Allocator<DimMinimum<R, C>, R>
            + Allocator<DimMinimum<R, C>>
            + Allocator<R, R>
            + Allocator<DimMinimum<R, C>, DimMinimum<R, C>>
            + Allocator<C>
            + Allocator<R>
            + Allocator<DimDiff<DimMinimum<R, C>, U1>>
            + Allocator<DimMinimum<R, C>, C>
            + Allocator<R, DimMinimum<R, C>>
            + Allocator<DimMinimum<R, C>>
            + Allocator<DimMinimum<R, C>>
            + Allocator<DimDiff<DimMinimum<R, C>, U1>>,
    {
        SVD::new_unordered(self.into_owned(), true, true)
            .to_polar()
            .unwrap()
    }

    /// Attempts to compute the Polar Decomposition of  a `matrix` (indirectly uses SVD).
    ///
    /// # Arguments
    ///
    /// * `eps`       − tolerance used to determine when a value converged to 0 when computing the SVD.
    /// * `max_niter` − maximum total number of iterations performed by the SVD computation algorithm.
    pub fn try_polar(
        self,
        eps: T::RealField,
        max_niter: usize,
    ) -> Option<(OMatrix<T, R, R>, OMatrix<T, R, C>)>
    where
        R: DimMin<C>,
        DimMinimum<R, C>: DimSub<U1>, // for Bidiagonal.
        DefaultAllocator: Allocator<R, C>
            + Allocator<DimMinimum<R, C>, R>
            + Allocator<DimMinimum<R, C>>
            + Allocator<R, R>
            + Allocator<DimMinimum<R, C>, DimMinimum<R, C>>
            + Allocator<C>
            + Allocator<R>
            + Allocator<DimDiff<DimMinimum<R, C>, U1>>
            + Allocator<DimMinimum<R, C>, C>
            + Allocator<R, DimMinimum<R, C>>
            + Allocator<DimMinimum<R, C>>
            + Allocator<DimMinimum<R, C>>
            + Allocator<DimDiff<DimMinimum<R, C>, U1>>,
    {
        SVD::try_new_unordered(self.into_owned(), true, true, eps, max_niter)
            .and_then(|svd| svd.to_polar())
    }
}

/// # Square matrix decomposition
///
/// This section contains the methods for computing some common decompositions of square
/// matrices with real or complex components. The following are currently supported:
///
/// | Decomposition            | Factors                   | Details |
/// | -------------------------|---------------------------|--------------|
/// | Hessenberg               | `Q * H * Qᵀ`             | `Q` is a unitary matrix and `H` an upper-Hessenberg matrix. |
/// | Cholesky                 | `L * Lᵀ`                 | `L` is a lower-triangular matrix. |
/// | UDU                      | `U * D * Uᵀ`             | `U` is a upper-triangular matrix, and `D` a diagonal matrix. |
/// | Schur decomposition      | `Q * T * Qᵀ`             | `Q` is an unitary matrix and `T` a quasi-upper-triangular matrix. |
/// | Symmetric eigendecomposition | `Q ~ Λ ~ Qᵀ`   | `Q` is an unitary matrix, and `Λ` is a real diagonal matrix. |
/// | Symmetric tridiagonalization | `Q ~ T ~ Qᵀ`   | `Q` is an unitary matrix, and `T` is a tridiagonal matrix. |
impl<T: ComplexField, D: Dim, S: Storage<T, D, D>> Matrix<T, D, D, S> {
    /// Attempts to compute the Cholesky decomposition of this matrix.
    ///
    /// Returns `None` if the input matrix is not definite-positive. The input matrix is assumed
    /// to be symmetric and only the lower-triangular part is read.
    pub fn cholesky(self) -> Option<Cholesky<T, D>>
    where
        DefaultAllocator: Allocator<D, D>,
    {
        Cholesky::new(self.into_owned())
    }

    /// Attempts to compute the UDU decomposition of this matrix.
    ///
    /// The input matrix `self` is assumed to be symmetric and this decomposition will only read
    /// the upper-triangular part of `self`.
    pub fn udu(self) -> Option<UDU<T, D>>
    where
        T: RealField,
        DefaultAllocator: Allocator<D> + Allocator<D, D>,
    {
        UDU::new(self.into_owned())
    }

    /// Computes the Hessenberg decomposition of this matrix using householder reflections.
    pub fn hessenberg(self) -> Hessenberg<T, D>
    where
        D: DimSub<U1>,
        DefaultAllocator: Allocator<D, D> + Allocator<D> + Allocator<DimDiff<D, U1>>,
    {
        Hessenberg::new(self.into_owned())
    }

    /// Computes the Schur decomposition of a square matrix.
    pub fn schur(self) -> Schur<T, D>
    where
        D: DimSub<U1>, // For Hessenberg.
        DefaultAllocator: Allocator<D, DimDiff<D, U1>>
            + Allocator<DimDiff<D, U1>>
            + Allocator<D, D>
            + Allocator<D>,
    {
        Schur::new(self.into_owned())
    }

    /// Attempts to compute the Schur decomposition of a square matrix.
    ///
    /// If only eigenvalues are needed, it is more efficient to call the matrix method
    /// `.eigenvalues()` instead.
    ///
    /// # Arguments
    ///
    /// * `eps`       − tolerance used to determine when a value converged to 0.
    /// * `max_niter` − maximum total number of iterations performed by the algorithm. If this
    ///   number of iteration is exceeded, `None` is returned. If `niter == 0`, then the algorithm
    ///   continues indefinitely until convergence.
    pub fn try_schur(self, eps: T::RealField, max_niter: usize) -> Option<Schur<T, D>>
    where
        D: DimSub<U1>, // For Hessenberg.
        DefaultAllocator: Allocator<D, DimDiff<D, U1>>
            + Allocator<DimDiff<D, U1>>
            + Allocator<D, D>
            + Allocator<D>,
    {
        Schur::try_new(self.into_owned(), eps, max_niter)
    }

    /// Computes the eigendecomposition of this symmetric matrix.
    ///
    /// Only the lower-triangular part (including the diagonal) of `m` is read.
    pub fn symmetric_eigen(self) -> SymmetricEigen<T, D>
    where
        D: DimSub<U1>,
        DefaultAllocator:
            Allocator<D, D> + Allocator<DimDiff<D, U1>> + Allocator<D> + Allocator<DimDiff<D, U1>>,
    {
        SymmetricEigen::new(self.into_owned())
    }

    /// Computes the eigendecomposition of the given symmetric matrix with user-specified
    /// convergence parameters.
    ///
    /// Only the lower-triangular part (including the diagonal) of `m` is read.
    ///
    /// # Arguments
    ///
    /// * `eps`       − tolerance used to determine when a value converged to 0.
    /// * `max_niter` − maximum total number of iterations performed by the algorithm. If this
    ///   number of iteration is exceeded, `None` is returned. If `niter == 0`, then the algorithm
    ///   continues indefinitely until convergence.
    pub fn try_symmetric_eigen(
        self,
        eps: T::RealField,
        max_niter: usize,
    ) -> Option<SymmetricEigen<T, D>>
    where
        D: DimSub<U1>,
        DefaultAllocator:
            Allocator<D, D> + Allocator<DimDiff<D, U1>> + Allocator<D> + Allocator<DimDiff<D, U1>>,
    {
        SymmetricEigen::try_new(self.into_owned(), eps, max_niter)
    }

    /// Computes the tridiagonalization of this symmetric matrix.
    ///
    /// Only the lower-triangular part (including the diagonal) of `m` is read.
    pub fn symmetric_tridiagonalize(self) -> SymmetricTridiagonal<T, D>
    where
        D: DimSub<U1>,
        DefaultAllocator: Allocator<D, D> + Allocator<DimDiff<D, U1>>,
    {
        SymmetricTridiagonal::new(self.into_owned())
    }
}

#[cfg(feature = "serde-serialize-no-std")]
use serde::{Deserialize, Serialize};

use crate::allocator::Allocator;
use crate::base::{Const, DefaultAllocator, OMatrix, OVector};
use crate::dimension::Dim;
use simba::scalar::RealField;

/// UDU factorization.
#[cfg_attr(feature = "serde-serialize-no-std", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde-serialize-no-std",
    serde(bound(serialize = "OVector<T, D>: Serialize, OMatrix<T, D, D>: Serialize"))
)]
#[cfg_attr(
    feature = "serde-serialize-no-std",
    serde(bound(
        deserialize = "OVector<T, D>: Deserialize<'de>, OMatrix<T, D, D>: Deserialize<'de>"
    ))
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Debug)]
pub struct UDU<T: RealField, D: Dim>
where
    DefaultAllocator: Allocator<D> + Allocator<D, D>,
{
    /// The upper triangular matrix resulting from the factorization
    pub u: OMatrix<T, D, D>,
    /// The diagonal matrix resulting from the factorization
    pub d: OVector<T, D>,
}

impl<T: RealField, D: Dim> Copy for UDU<T, D>
where
    DefaultAllocator: Allocator<D> + Allocator<D, D>,
    OVector<T, D>: Copy,
    OMatrix<T, D, D>: Copy,
{
}

impl<T: RealField, D: Dim> UDU<T, D>
where
    DefaultAllocator: Allocator<D> + Allocator<D, D>,
{
    /// Computes the UDU^T factorization.
    ///
    /// The input matrix `p` is assumed to be symmetric and this decomposition will only read
    /// the upper-triangular part of `p`.
    ///
    /// Ref.: "Optimal control and estimation-Dover Publications", Robert F. Stengel, (1994) page 360
    pub fn new(p: OMatrix<T, D, D>) -> Option<Self> {
        let n = p.ncols();
        let n_dim = p.shape_generic().1;

        let mut d = OVector::zeros_generic(n_dim, Const::<1>);
        let mut u = OMatrix::zeros_generic(n_dim, n_dim);

        d[n - 1] = p[(n - 1, n - 1)].clone();

        if d[n - 1].is_zero() {
            return None;
        }

        u.column_mut(n - 1)
            .axpy(T::one() / d[n - 1].clone(), &p.column(n - 1), T::zero());

        for j in (0..n - 1).rev() {
            let mut d_j = d[j].clone();
            for k in j + 1..n {
                d_j += d[k].clone() * u[(j, k)].clone().powi(2);
            }

            d[j] = p[(j, j)].clone() - d_j;

            if d[j].is_zero() {
                return None;
            }

            for i in (0..=j).rev() {
                let mut u_ij = u[(i, j)].clone();
                for k in j + 1..n {
                    u_ij += d[k].clone() * u[(j, k)].clone() * u[(i, k)].clone();
                }

                u[(i, j)] = (p[(i, j)].clone() - u_ij) / d[j].clone();
            }

            u[(j, j)] = T::one();
        }

        Some(Self { u, d })
    }

    /// Returns the diagonal elements as a matrix
    #[must_use]
    pub fn d_matrix(&self) -> OMatrix<T, D, D> {
        OMatrix::from_diagonal(&self.d)
    }
}

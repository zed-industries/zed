#[cfg(feature = "serde-serialize-no-std")]
use serde::{Deserialize, Serialize};

use num_complex::Complex;
use simba::scalar::ComplexField;
use std::cmp;
use std::fmt::Display;
use std::ops::MulAssign;

use crate::allocator::Allocator;
use crate::base::dimension::{Dim, DimDiff, DimSub, Dyn, U1, U2, U3};
use crate::base::storage::Storage;
use crate::base::{
    DefaultAllocator, Hessenberg, OMatrix, OVector, SquareMatrix, Unit, Vector2, Vector3,
};
use crate::constraint::{DimEq, ShapeConstraint};

use crate::geometry::{Reflection, UnitComplex};
use crate::linalg::householder;
use crate::linalg::Schur;

/// Eigendecomposition of a real matrix with real eigenvalues (or complex eigen values for complex matrices).
#[cfg_attr(feature = "serde-serialize-no-std", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde-serialize-no-std",
    serde(bound(serialize = "DefaultAllocator: Allocator<D>,
         OVector<T, D>: Serialize,
         OMatrix<T, D, D>: Serialize"))
)]
#[cfg_attr(
    feature = "serde-serialize-no-std",
    serde(bound(deserialize = "DefaultAllocator: Allocator<D>,
         OVector<T, D>: Serialize,
         OMatrix<T, D, D>: Deserialize<'de>"))
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Debug)]
pub struct Eigen<T: ComplexField, D: Dim>
where
    DefaultAllocator: Allocator<D, D> + Allocator<D>,
{
    pub eigenvectors: OMatrix<T, D, D>,
    pub eigenvalues: OVector<T, D>,
}

impl<T: ComplexField, D: Dim> Copy for Eigen<T, D>
where
    DefaultAllocator: Allocator<D, D> + Allocator<D>,
    OMatrix<T, D, D>: Copy,
    OVector<T, D>: Copy,
{
}

impl<T: ComplexField, D: Dim> Eigen<T, D>
where
    D: DimSub<U1>,                               // For Hessenberg.
    ShapeConstraint: DimEq<Dyn, DimDiff<D, U1>>, // For Hessenberg.
    DefaultAllocator:
        Allocator<D, DimDiff<D, U1>> + Allocator<DimDiff<D, U1>> + Allocator<D, D> + Allocator<D>,
    // XXX: for debug
    DefaultAllocator: Allocator<D, D>,
    OMatrix<T, D, D>: Display,
{
    /// Computes the eigendecomposition of a diagonalizable matrix with Complex eigenvalues.
    pub fn new(m: OMatrix<T, D, D>) -> Option<Eigen<T, D>> {
        assert!(
            m.is_square(),
            "Unable to compute the eigendecomposition of a non-square matrix."
        );

        let dim = m.nrows();
        let (mut eigenvectors, mut eigenvalues) = Schur::new(m, 0).unwrap().unpack();

        println!("Schur eigenvalues: {}", eigenvalues);

        // Check that the eigenvalues are all Complex.
        for i in 0..dim - 1 {
            if !eigenvalues[(i + 1, i)].is_zero() {
                return None;
            }
        }

        for j in 1..dim {
            for i in 0..j {
                let diff = eigenvalues[(i, i)] - eigenvalues[(j, j)];

                if diff.is_zero() && !eigenvalues[(i, j)].is_zero() {
                    return None;
                }

                let z = -eigenvalues[(i, j)] / diff;

                for k in j + 1..dim {
                    eigenvalues[(i, k)] -= z * eigenvalues[(j, k)];
                }

                for k in 0..dim {
                    eigenvectors[(k, j)] += z * eigenvectors[(k, i)];
                }
            }
        }

        // Normalize the eigenvector basis.
        for i in 0..dim {
            let _ = eigenvectors.column_mut(i).normalize_mut();
        }

        Some(Eigen {
            eigenvectors,
            eigenvalues: eigenvalues.diagonal(),
        })
    }
}

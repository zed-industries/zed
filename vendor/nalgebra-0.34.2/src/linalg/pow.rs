//! This module provides the matrix exponential (pow) function to square matrices.

use crate::{
    DefaultAllocator, DimMin, Matrix, OMatrix, Scalar,
    allocator::Allocator,
    storage::{Storage, StorageMut},
};
use num::{One, Zero};
use simba::scalar::{ClosedAddAssign, ClosedMulAssign};

impl<T, D, S> Matrix<T, D, D, S>
where
    T: Scalar + Zero + One + ClosedAddAssign + ClosedMulAssign,
    D: DimMin<D, Output = D>,
    S: StorageMut<T, D, D>,
    DefaultAllocator: Allocator<D, D> + Allocator<D>,
{
    /// Raises this matrix to an integral power `exp` in-place.
    pub fn pow_mut(&mut self, mut exp: u32) {
        // A matrix raised to the zeroth power is just the identity.
        if exp == 0 {
            self.fill_with_identity();
        } else if exp > 1 {
            // We use the buffer to hold the result of multiplier^2, thus avoiding
            // extra allocations.
            let mut x = self.clone_owned();
            let mut workspace = self.clone_owned();

            if exp.is_multiple_of(2) {
                self.fill_with_identity();
            } else {
                // Avoid an useless multiplication by the identity
                // if the exponent is odd.
                exp -= 1;
            }

            // Exponentiation by squares.
            loop {
                if exp % 2 == 1 {
                    self.mul_to(&x, &mut workspace);
                    self.copy_from(&workspace);
                }

                exp /= 2;

                if exp == 0 {
                    break;
                }

                x.mul_to(&x, &mut workspace);
                x.copy_from(&workspace);
            }
        }
    }
}

impl<T, D, S: Storage<T, D, D>> Matrix<T, D, D, S>
where
    T: Scalar + Zero + One + ClosedAddAssign + ClosedMulAssign,
    D: DimMin<D, Output = D>,
    S: StorageMut<T, D, D>,
    DefaultAllocator: Allocator<D, D> + Allocator<D>,
{
    /// Raise this matrix to an integral power `exp`.
    #[must_use]
    pub fn pow(&self, exp: u32) -> OMatrix<T, D, D> {
        let mut result = self.clone_owned();
        result.pow_mut(exp);
        result
    }
}

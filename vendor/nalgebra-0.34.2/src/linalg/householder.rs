//! Construction of householder elementary reflections.

use crate::allocator::Allocator;
use crate::base::{DefaultAllocator, OMatrix, OVector, Unit, Vector};
use crate::dimension::Dim;
use crate::storage::StorageMut;
use num::Zero;
use simba::scalar::ComplexField;

use crate::geometry::Reflection;

/// Replaces `column` by the axis of the householder reflection that transforms `column` into
/// `(+/-|column|, 0, ..., 0)`.
///
/// The unit-length axis is output to `column`. Returns what would be the first component of
/// `column` after reflection and `false` if no reflection was necessary.
#[doc(hidden)]
#[inline(always)]
pub fn reflection_axis_mut<T: ComplexField, D: Dim, S: StorageMut<T, D>>(
    column: &mut Vector<T, D, S>,
) -> (T, bool) {
    let reflection_sq_norm = column.norm_squared();
    let reflection_norm = reflection_sq_norm.clone().sqrt();

    let factor;
    let signed_norm;

    unsafe {
        let (modulus, sign) = column.vget_unchecked(0).clone().to_exp();
        signed_norm = sign.scale(reflection_norm.clone());
        factor = (reflection_sq_norm + modulus * reflection_norm) * crate::convert(2.0);
        *column.vget_unchecked_mut(0) += signed_norm.clone();
    };

    if !factor.is_zero() {
        column.unscale_mut(factor.sqrt());

        // Normalize again, making sure the vector is unit-sized.
        // If `factor` had a very small value, the first normalization
        // (dividing by `factor.sqrt()`) might end up with a slightly
        // non-unit vector (especially when using 32-bits float).
        // Decompositions strongly rely on that unit-vector property,
        // so we run a second normalization (that is much more numerically
        // stable since the norm is close to 1) to ensure it has a unit
        // size.
        let _ = column.normalize_mut();

        (-signed_norm, true)
    } else {
        // TODO: not sure why we don't have a - sign here.
        (signed_norm, false)
    }
}

/// Uses an householder reflection to zero out the `icol`-th column, starting with the `shift + 1`-th
/// subdiagonal element.
///
/// Returns the signed norm of the column.
#[doc(hidden)]
#[must_use]
pub fn clear_column_unchecked<T: ComplexField, R: Dim, C: Dim>(
    matrix: &mut OMatrix<T, R, C>,
    icol: usize,
    shift: usize,
    bilateral: Option<&mut OVector<T, R>>,
) -> T
where
    DefaultAllocator: Allocator<R, C> + Allocator<R>,
{
    let (mut left, mut right) = matrix.columns_range_pair_mut(icol, icol + 1..);
    let mut axis = left.rows_range_mut(icol + shift..);

    let (reflection_norm, not_zero) = reflection_axis_mut(&mut axis);

    if not_zero {
        let refl = Reflection::new(Unit::new_unchecked(axis), T::zero());
        let sign = reflection_norm.clone().signum();
        if let Some(work) = bilateral {
            refl.reflect_rows_with_sign(&mut right, work, sign.clone());
        }
        refl.reflect_with_sign(&mut right.rows_range_mut(icol + shift..), sign.conjugate());
    }

    reflection_norm
}

/// Uses an householder reflection to zero out the `irow`-th row, ending before the `shift + 1`-th
/// superdiagonal element.
///
/// Returns the signed norm of the column.
#[doc(hidden)]
#[must_use]
pub fn clear_row_unchecked<T: ComplexField, R: Dim, C: Dim>(
    matrix: &mut OMatrix<T, R, C>,
    axis_packed: &mut OVector<T, C>,
    work: &mut OVector<T, R>,
    irow: usize,
    shift: usize,
) -> T
where
    DefaultAllocator: Allocator<R, C> + Allocator<R> + Allocator<C>,
{
    let (mut top, mut bottom) = matrix.rows_range_pair_mut(irow, irow + 1..);
    let mut axis = axis_packed.rows_range_mut(irow + shift..);
    axis.tr_copy_from(&top.columns_range(irow + shift..));

    let (reflection_norm, not_zero) = reflection_axis_mut(&mut axis);
    axis.conjugate_mut(); // So that reflect_rows actually cancels the first row.

    if not_zero {
        let refl = Reflection::new(Unit::new_unchecked(axis), T::zero());
        refl.reflect_rows_with_sign(
            &mut bottom.columns_range_mut(irow + shift..),
            &mut work.rows_range_mut(irow + 1..),
            reflection_norm.clone().signum().conjugate(),
        );
        top.columns_range_mut(irow + shift..)
            .tr_copy_from(refl.axis());
    } else {
        top.columns_range_mut(irow + shift..).tr_copy_from(&axis);
    }

    reflection_norm
}

/// Computes the orthogonal transformation described by the elementary reflector axii stored on
/// the lower-diagonal element of the given matrix.
/// matrices.
#[doc(hidden)]
pub fn assemble_q<T: ComplexField, D: Dim>(m: &OMatrix<T, D, D>, signs: &[T]) -> OMatrix<T, D, D>
where
    DefaultAllocator: Allocator<D, D>,
{
    assert!(m.is_square());
    let dim = m.shape_generic().0;

    // NOTE: we could build the identity matrix and call p_mult on it.
    // Instead we don't so that we take in account the matrix sparseness.
    let mut res = OMatrix::identity_generic(dim, dim);

    for i in (0..dim.value() - 1).rev() {
        let axis = m.view_range(i + 1.., i);
        let refl = Reflection::new(Unit::new_unchecked(axis), T::zero());

        let mut res_rows = res.view_range_mut(i + 1.., i..);
        refl.reflect_with_sign(&mut res_rows, signs[i].clone().signum());
    }

    res
}

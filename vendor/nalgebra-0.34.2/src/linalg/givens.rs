//! Construction of givens rotations.

use num::{One, Zero};
use simba::scalar::ComplexField;

use crate::base::constraint::{DimEq, ShapeConstraint};
use crate::base::dimension::{Dim, U2};
use crate::base::storage::{Storage, StorageMut};
use crate::base::{Matrix, Vector};

/// A Givens rotation.
#[derive(Debug, Clone, Copy)]
pub struct GivensRotation<T: ComplexField> {
    c: T::RealField,
    s: T,
}

// Matrix = UnitComplex * Matrix
impl<T: ComplexField> GivensRotation<T> {
    /// The Givens rotation that does nothing.
    pub fn identity() -> Self {
        Self {
            c: T::RealField::one(),
            s: T::zero(),
        }
    }

    /// Initializes a Givens rotation from its components.
    ///
    /// The components are copies as-is. It is not checked whether they describe
    /// an actually valid Givens rotation.
    pub const fn new_unchecked(c: T::RealField, s: T) -> Self {
        Self { c, s }
    }

    /// Initializes a Givens rotation from its non-normalized cosine an sine components.
    pub fn new(c: T, s: T) -> (Self, T) {
        Self::try_new(c, s, T::RealField::zero())
            .unwrap_or_else(|| (GivensRotation::identity(), T::zero()))
    }

    /// Initializes a Givens rotation form its non-normalized cosine an sine components.
    pub fn try_new(c: T, s: T, eps: T::RealField) -> Option<(Self, T)> {
        let (mod0, sign0) = c.to_exp();
        let denom = (mod0.clone() * mod0.clone() + s.clone().modulus_squared()).sqrt();

        if denom > eps {
            let norm = sign0.scale(denom.clone());
            let c = mod0 / denom;
            let s = s / norm.clone();
            Some((Self { c, s }, norm))
        } else {
            None
        }
    }

    /// Computes the rotation `R` required such that the `y` component of `R * v` is zero.
    ///
    /// Returns `None` if no rotation is needed (i.e. if `v.y == 0`). Otherwise, this returns the norm
    /// of `v` and the rotation `r` such that `R * v = [ |v|, 0.0 ]^t` where `|v|` is the norm of `v`.
    pub fn cancel_y<S: Storage<T, U2>>(v: &Vector<T, U2, S>) -> Option<(Self, T)> {
        if !v[1].is_zero() {
            let (mod0, sign0) = v[0].clone().to_exp();
            let denom = (mod0.clone() * mod0.clone() + v[1].clone().modulus_squared()).sqrt();
            let c = mod0 / denom.clone();
            let s = -v[1].clone() / sign0.clone().scale(denom.clone());
            let r = sign0.scale(denom);
            Some((Self { c, s }, r))
        } else {
            None
        }
    }

    /// Computes the rotation `R` required such that the `x` component of `R * v` is zero.
    ///
    /// Returns `None` if no rotation is needed (i.e. if `v.x == 0`). Otherwise, this returns the norm
    /// of `v` and the rotation `r` such that `R * v = [ 0.0, |v| ]^t` where `|v|` is the norm of `v`.
    pub fn cancel_x<S: Storage<T, U2>>(v: &Vector<T, U2, S>) -> Option<(Self, T)> {
        if !v[0].is_zero() {
            let (mod1, sign1) = v[1].clone().to_exp();
            let denom = (mod1.clone() * mod1.clone() + v[0].clone().modulus_squared()).sqrt();
            let c = mod1 / denom.clone();
            let s = (v[0].clone().conjugate() * sign1.clone()).unscale(denom.clone());
            let r = sign1.scale(denom);
            Some((Self { c, s }, r))
        } else {
            None
        }
    }

    /// The cos part of this rotation.
    #[must_use]
    pub fn c(&self) -> T::RealField {
        self.c.clone()
    }

    /// The sin part of this rotation.
    #[must_use]
    pub fn s(&self) -> T {
        self.s.clone()
    }

    /// The inverse of this givens rotation.
    #[must_use = "This function does not mutate self."]
    pub fn inverse(&self) -> Self {
        Self {
            c: self.c.clone(),
            s: -self.s.clone(),
        }
    }

    /// Performs the multiplication `rhs = self * rhs` in-place.
    pub fn rotate<R2: Dim, C2: Dim, S2: StorageMut<T, R2, C2>>(
        &self,
        rhs: &mut Matrix<T, R2, C2, S2>,
    ) where
        ShapeConstraint: DimEq<R2, U2>,
    {
        assert_eq!(
            rhs.nrows(),
            2,
            "Unit complex rotation: the input matrix must have exactly two rows."
        );
        let s = self.s.clone();
        let c = self.c.clone();

        for j in 0..rhs.ncols() {
            unsafe {
                let a = rhs.get_unchecked((0, j)).clone();
                let b = rhs.get_unchecked((1, j)).clone();

                *rhs.get_unchecked_mut((0, j)) =
                    a.clone().scale(c.clone()) - s.clone().conjugate() * b.clone();
                *rhs.get_unchecked_mut((1, j)) = s.clone() * a + b.scale(c.clone());
            }
        }
    }

    /// Performs the multiplication `lhs = lhs * self` in-place.
    pub fn rotate_rows<R2: Dim, C2: Dim, S2: StorageMut<T, R2, C2>>(
        &self,
        lhs: &mut Matrix<T, R2, C2, S2>,
    ) where
        ShapeConstraint: DimEq<C2, U2>,
    {
        assert_eq!(
            lhs.ncols(),
            2,
            "Unit complex rotation: the input matrix must have exactly two columns."
        );
        let s = self.s.clone();
        let c = self.c.clone();

        // TODO: can we optimize that to iterate on one column at a time ?
        for j in 0..lhs.nrows() {
            unsafe {
                let a = lhs.get_unchecked((j, 0)).clone();
                let b = lhs.get_unchecked((j, 1)).clone();

                *lhs.get_unchecked_mut((j, 0)) = a.clone().scale(c.clone()) + s.clone() * b.clone();
                *lhs.get_unchecked_mut((j, 1)) = -s.clone().conjugate() * a + b.scale(c.clone());
            }
        }
    }
}

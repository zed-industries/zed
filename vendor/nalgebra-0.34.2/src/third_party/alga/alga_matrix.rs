#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::vec::Vec;

use num::{One, Zero};

use alga::general::{
    AbstractGroup, AbstractGroupAbelian, AbstractLoop, AbstractMagma, AbstractModule,
    AbstractMonoid, AbstractQuasigroup, AbstractSemigroup, Additive, ClosedAdd, ClosedMul,
    ClosedNeg, ComplexField, Field, Identity, JoinSemilattice, Lattice, MeetSemilattice, Module,
    Multiplicative, RingCommutative, TwoSidedInverse,
};
use alga::linear::{
    FiniteDimInnerSpace, FiniteDimVectorSpace, InnerSpace, NormedSpace, VectorSpace,
};

use crate::base::allocator::Allocator;
use crate::base::dimension::{Dim, DimName};
use crate::base::storage::{RawStorage, RawStorageMut};
use crate::base::{DefaultAllocator, Matrix, OMatrix, Scalar};
use std::mem::MaybeUninit;

/*
 *
 * Additive structures.
 *
 */
impl<T, R: DimName, C: DimName> Identity<Additive> for OMatrix<T, R, C>
where
    T: Scalar + Zero,
    DefaultAllocator: Allocator<R, C>,
{
    #[inline]
    fn identity() -> Self {
        Self::from_element(T::zero())
    }
}

impl<T, R: DimName, C: DimName> AbstractMagma<Additive> for OMatrix<T, R, C>
where
    T: Scalar + ClosedAdd,
    DefaultAllocator: Allocator<R, C>,
{
    #[inline]
    fn operate(&self, other: &Self) -> Self {
        self + other
    }
}

impl<T, R: DimName, C: DimName> TwoSidedInverse<Additive> for OMatrix<T, R, C>
where
    T: Scalar + ClosedNeg,
    DefaultAllocator: Allocator<R, C>,
{
    #[inline]
    fn two_sided_inverse(&self) -> Self {
        -self
    }

    #[inline]
    fn two_sided_inverse_mut(&mut self) {
        *self = -self.clone()
    }
}

macro_rules! inherit_additive_structure(
    ($($marker: ident<$operator: ident> $(+ $bounds: ident)*),* $(,)*) => {$(
        impl<T, R: DimName, C: DimName> $marker<$operator> for OMatrix<T, R, C>
            where T: Scalar + $marker<$operator> $(+ $bounds)*,
                  DefaultAllocator: Allocator<R, C> { }
    )*}
);

inherit_additive_structure!(
    AbstractSemigroup<Additive> + ClosedAdd,
    AbstractMonoid<Additive> + Zero + ClosedAdd,
    AbstractQuasigroup<Additive> + ClosedAdd + ClosedNeg,
    AbstractLoop<Additive> + Zero + ClosedAdd + ClosedNeg,
    AbstractGroup<Additive> + Zero + ClosedAdd + ClosedNeg,
    AbstractGroupAbelian<Additive> + Zero + ClosedAdd + ClosedNeg
);

impl<T, R: DimName, C: DimName> AbstractModule for OMatrix<T, R, C>
where
    T: Scalar + RingCommutative,
    DefaultAllocator: Allocator<R, C>,
{
    type AbstractRing = T;

    #[inline]
    fn multiply_by(&self, n: T) -> Self {
        self * n
    }
}

impl<T, R: DimName, C: DimName> Module for OMatrix<T, R, C>
where
    T: Scalar + RingCommutative,
    DefaultAllocator: Allocator<R, C>,
{
    type Ring = T;
}

impl<T, R: DimName, C: DimName> VectorSpace for OMatrix<T, R, C>
where
    T: Scalar + Field,
    DefaultAllocator: Allocator<R, C>,
{
    type Field = T;
}

impl<T, R: DimName, C: DimName> FiniteDimVectorSpace for OMatrix<T, R, C>
where
    T: Scalar + Field,
    DefaultAllocator: Allocator<R, C>,
{
    #[inline]
    fn dimension() -> usize {
        R::DIM * C::DIM
    }

    #[inline]
    fn canonical_basis_element(i: usize) -> Self {
        assert!(i < Self::dimension(), "Index out of bound.");

        let mut res = Self::zero();
        unsafe {
            *res.data.get_unchecked_linear_mut(i) = T::one();
        }

        res
    }

    #[inline]
    fn dot(&self, other: &Self) -> T {
        self.dot(other)
    }

    #[inline]
    unsafe fn component_unchecked(&self, i: usize) -> &T {
        unsafe { self.data.get_unchecked_linear(i) }
    }

    #[inline]
    unsafe fn component_unchecked_mut(&mut self, i: usize) -> &mut T {
        unsafe { self.data.get_unchecked_linear_mut(i) }
    }
}

impl<
    T: ComplexField + simba::scalar::ComplexField<RealField = <T as ComplexField>::RealField>,
    R: DimName,
    C: DimName,
> NormedSpace for OMatrix<T, R, C>
where
    <T as ComplexField>::RealField: simba::scalar::RealField,
    DefaultAllocator: Allocator<R, C>,
{
    type RealField = <T as ComplexField>::RealField;
    type ComplexField = T;

    #[inline]
    fn norm_squared(&self) -> <T as ComplexField>::RealField {
        self.norm_squared()
    }

    #[inline]
    fn norm(&self) -> <T as ComplexField>::RealField {
        self.norm()
    }

    #[inline]
    fn normalize(&self) -> Self {
        self.normalize()
    }

    #[inline]
    fn normalize_mut(&mut self) -> <T as ComplexField>::RealField {
        self.normalize_mut()
    }

    #[inline]
    fn try_normalize(&self, min_norm: <T as ComplexField>::RealField) -> Option<Self> {
        self.try_normalize(min_norm)
    }

    #[inline]
    fn try_normalize_mut(
        &mut self,
        min_norm: <T as ComplexField>::RealField,
    ) -> Option<<T as ComplexField>::RealField> {
        self.try_normalize_mut(min_norm)
    }
}

impl<
    T: ComplexField + simba::scalar::ComplexField<RealField = <T as ComplexField>::RealField>,
    R: DimName,
    C: DimName,
> InnerSpace for OMatrix<T, R, C>
where
    <T as ComplexField>::RealField: simba::scalar::RealField,
    DefaultAllocator: Allocator<R, C>,
{
    #[inline]
    fn angle(&self, other: &Self) -> <T as ComplexField>::RealField {
        self.angle(other)
    }

    #[inline]
    fn inner_product(&self, other: &Self) -> T {
        self.dotc(other)
    }
}

// TODO: specialization will greatly simplify this implementation in the future.
// In particular:
//   − use `x()` instead of `::canonical_basis_element`
//   − use `::new(x, y, z)` instead of `::from_slice`
impl<
    T: ComplexField + simba::scalar::ComplexField<RealField = <T as ComplexField>::RealField>,
    R: DimName,
    C: DimName,
> FiniteDimInnerSpace for OMatrix<T, R, C>
where
    <T as ComplexField>::RealField: simba::scalar::RealField,
    DefaultAllocator: Allocator<R, C>,
{
    #[inline]
    fn orthonormalize(vs: &mut [Self]) -> usize {
        let mut nbasis_elements = 0;

        for i in 0..vs.len() {
            {
                let (elt, basis) = vs[..i + 1].split_last_mut().unwrap();

                for basis_element in &basis[..nbasis_elements] {
                    *elt -= &*basis_element * elt.dot(basis_element)
                }
            }

            if vs[i]
                .try_normalize_mut(<T as ComplexField>::RealField::zero())
                .is_some()
            {
                // TODO: this will be efficient on dynamically-allocated vectors but for
                // statically-allocated ones, `.clone_from` would be better.
                vs.swap(nbasis_elements, i);
                nbasis_elements += 1;

                // All the other vectors will be dependent.
                if nbasis_elements == Self::dimension() {
                    break;
                }
            }
        }

        nbasis_elements
    }

    #[inline]
    fn orthonormal_subspace_basis<F>(vs: &[Self], mut f: F)
    where
        F: FnMut(&Self) -> bool,
    {
        // TODO: is this necessary?
        assert!(
            vs.len() <= Self::dimension(),
            "The given set of vectors has no chance of being a free family."
        );

        match Self::dimension() {
            1 => {
                if vs.is_empty() {
                    let _ = f(&Self::canonical_basis_element(0));
                }
            }
            2 => {
                if vs.is_empty() {
                    let _ = f(&Self::canonical_basis_element(0))
                        && f(&Self::canonical_basis_element(1));
                } else if vs.len() == 1 {
                    let v = &vs[0];
                    let res = Self::from_column_slice(&[-v[1], v[0]]);

                    let _ = f(&res.normalize());
                }

                // Otherwise, nothing.
            }
            3 => {
                if vs.is_empty() {
                    let _ = f(&Self::canonical_basis_element(0))
                        && f(&Self::canonical_basis_element(1))
                        && f(&Self::canonical_basis_element(2));
                } else if vs.len() == 1 {
                    let v = &vs[0];
                    let mut a;

                    if ComplexField::norm1(v[0]) > ComplexField::norm1(v[1]) {
                        a = Self::from_column_slice(&[v[2], T::zero(), -v[0]]);
                    } else {
                        a = Self::from_column_slice(&[T::zero(), -v[2], v[1]]);
                    };

                    let _ = a.normalize_mut();

                    if f(&a.cross(v)) {
                        let _ = f(&a);
                    }
                } else if vs.len() == 2 {
                    let _ = f(&vs[0].cross(&vs[1]).normalize());
                }
            }
            _ => {
                #[cfg(any(feature = "std", feature = "alloc"))]
                {
                    // XXX: use a GenericArray instead.
                    let mut known_basis = Vec::new();

                    for v in vs.iter() {
                        known_basis.push(v.normalize())
                    }

                    for i in 0..Self::dimension() - vs.len() {
                        let mut elt = Self::canonical_basis_element(i);

                        for v in &known_basis {
                            elt -= v * elt.dot(v)
                        }

                        if let Some(subsp_elt) =
                            elt.try_normalize(<T as ComplexField>::RealField::zero())
                        {
                            if !f(&subsp_elt) {
                                return;
                            };

                            known_basis.push(subsp_elt);
                        }
                    }
                }
                #[cfg(all(not(feature = "std"), not(feature = "alloc")))]
                {
                    panic!(
                        "Cannot compute the orthogonal subspace basis of a vector with a dimension greater than 3 \
                            if #![no_std] is enabled and the 'alloc' feature is not enabled."
                    )
                }
            }
        }
    }
}

/*
 *
 *
 * Multiplicative structures.
 *
 *
 */
impl<T, D: DimName> Identity<Multiplicative> for OMatrix<T, D, D>
where
    T: Scalar + Zero + One,
    DefaultAllocator: Allocator<D, D>,
{
    #[inline]
    fn identity() -> Self {
        Self::identity()
    }
}

impl<T, D: DimName> AbstractMagma<Multiplicative> for OMatrix<T, D, D>
where
    T: Scalar + Zero + One + ClosedAdd + ClosedMul,
    DefaultAllocator: Allocator<D, D>,
{
    #[inline]
    fn operate(&self, other: &Self) -> Self {
        self * other
    }
}

macro_rules! impl_multiplicative_structure(
    ($($marker: ident<$operator: ident> $(+ $bounds: ident)*),* $(,)*) => {$(
        impl<T, D: DimName> $marker<$operator> for OMatrix<T, D, D>
            where T: Scalar + Zero + One + ClosedAdd + ClosedMul + $marker<$operator> $(+ $bounds)*,
                  DefaultAllocator: Allocator<D, D> { }
    )*}
);

impl_multiplicative_structure!(
    AbstractSemigroup<Multiplicative>,
    AbstractMonoid<Multiplicative> + One
);

/*
 *
 * Ordering
 *
 */
impl<T, R: Dim, C: Dim> MeetSemilattice for OMatrix<T, R, C>
where
    T: Scalar + MeetSemilattice,
    DefaultAllocator: Allocator<R, C>,
{
    #[inline]
    fn meet(&self, other: &Self) -> Self {
        self.zip_map(other, |a, b| a.meet(&b))
    }
}

impl<T, R: Dim, C: Dim> JoinSemilattice for OMatrix<T, R, C>
where
    T: Scalar + JoinSemilattice,
    DefaultAllocator: Allocator<R, C>,
{
    #[inline]
    fn join(&self, other: &Self) -> Self {
        self.zip_map(other, |a, b| a.join(&b))
    }
}

impl<T, R: Dim, C: Dim> Lattice for OMatrix<T, R, C>
where
    T: Scalar + Lattice,
    DefaultAllocator: Allocator<R, C>,
{
    #[inline]
    fn meet_join(&self, other: &Self) -> (Self, Self) {
        let shape = self.shape_generic();
        assert!(
            shape == other.shape_generic(),
            "Matrix meet/join error: mismatched dimensions."
        );

        let mut mres = Matrix::uninit(shape.0, shape.1);
        let mut jres = Matrix::uninit(shape.0, shape.1);

        for i in 0..shape.0.value() * shape.1.value() {
            unsafe {
                let mj = self
                    .data
                    .get_unchecked_linear(i)
                    .meet_join(other.data.get_unchecked_linear(i));
                *mres.data.get_unchecked_linear_mut(i) = MaybeUninit::new(mj.0);
                *jres.data.get_unchecked_linear_mut(i) = MaybeUninit::new(mj.1);
            }
        }

        // Safety: both mres and jres are now completely initialized.
        unsafe { (mres.assume_init(), jres.assume_init()) }
    }
}

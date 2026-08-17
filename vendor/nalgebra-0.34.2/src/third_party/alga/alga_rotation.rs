use alga::general::{
    AbstractGroup, AbstractLoop, AbstractMagma, AbstractMonoid, AbstractQuasigroup,
    AbstractSemigroup, Id, Identity, Multiplicative, RealField, TwoSidedInverse,
};
use alga::linear::{
    self, AffineTransformation, DirectIsometry, Isometry, OrthogonalTransformation,
    ProjectiveTransformation, Similarity, Transformation,
};

use crate::base::SVector;
use crate::geometry::{Point, Rotation};

/*
 *
 * Algebraic structures.
 *
 */
impl<T: RealField + simba::scalar::RealField, const D: usize> Identity<Multiplicative>
    for Rotation<T, D>
{
    #[inline]
    fn identity() -> Self {
        Self::identity()
    }
}

impl<T: RealField + simba::scalar::RealField, const D: usize> TwoSidedInverse<Multiplicative>
    for Rotation<T, D>
{
    #[inline]
    fn two_sided_inverse(&self) -> Self {
        self.transpose()
    }

    #[inline]
    fn two_sided_inverse_mut(&mut self) {
        self.transpose_mut()
    }
}

impl<T: RealField + simba::scalar::RealField, const D: usize> AbstractMagma<Multiplicative>
    for Rotation<T, D>
{
    #[inline]
    fn operate(&self, rhs: &Self) -> Self {
        self * rhs
    }
}

macro_rules! impl_multiplicative_structures(
    ($($marker: ident<$operator: ident>),* $(,)*) => {$(
        impl<T: RealField + simba::scalar::RealField, const D: usize> $marker<$operator> for Rotation<T, D>
            { }
    )*}
);

impl_multiplicative_structures!(
    AbstractSemigroup<Multiplicative>,
    AbstractMonoid<Multiplicative>,
    AbstractQuasigroup<Multiplicative>,
    AbstractLoop<Multiplicative>,
    AbstractGroup<Multiplicative>
);

/*
 *
 * Transformation groups.
 *
 */
impl<T: RealField + simba::scalar::RealField, const D: usize> Transformation<Point<T, D>>
    for Rotation<T, D>
{
    #[inline]
    fn transform_point(&self, pt: &Point<T, D>) -> Point<T, D> {
        self.transform_point(pt)
    }

    #[inline]
    fn transform_vector(&self, v: &SVector<T, D>) -> SVector<T, D> {
        self.transform_vector(v)
    }
}

impl<T: RealField + simba::scalar::RealField, const D: usize> ProjectiveTransformation<Point<T, D>>
    for Rotation<T, D>
{
    #[inline]
    fn inverse_transform_point(&self, pt: &Point<T, D>) -> Point<T, D> {
        self.inverse_transform_point(pt)
    }

    #[inline]
    fn inverse_transform_vector(&self, v: &SVector<T, D>) -> SVector<T, D> {
        self.inverse_transform_vector(v)
    }
}

impl<T: RealField + simba::scalar::RealField, const D: usize> AffineTransformation<Point<T, D>>
    for Rotation<T, D>
{
    type Rotation = Self;
    type NonUniformScaling = Id;
    type Translation = Id;

    #[inline]
    fn decompose(&self) -> (Id, Self, Id, Self) {
        (Id::new(), *self, Id::new(), Self::identity())
    }

    #[inline]
    fn append_translation(&self, _: &Self::Translation) -> Self {
        *self
    }

    #[inline]
    fn prepend_translation(&self, _: &Self::Translation) -> Self {
        *self
    }

    #[inline]
    fn append_rotation(&self, r: &Self::Rotation) -> Self {
        r * self
    }

    #[inline]
    fn prepend_rotation(&self, r: &Self::Rotation) -> Self {
        self * r
    }

    #[inline]
    fn append_scaling(&self, _: &Self::NonUniformScaling) -> Self {
        *self
    }

    #[inline]
    fn prepend_scaling(&self, _: &Self::NonUniformScaling) -> Self {
        *self
    }
}

impl<T: RealField + simba::scalar::RealField, const D: usize> Similarity<Point<T, D>>
    for Rotation<T, D>
{
    type Scaling = Id;

    #[inline]
    fn translation(&self) -> Id {
        Id::new()
    }

    #[inline]
    fn rotation(&self) -> Self {
        *self
    }

    #[inline]
    fn scaling(&self) -> Id {
        Id::new()
    }
}

macro_rules! marker_impl(
    ($($Trait: ident),*) => {$(
        impl<T: RealField + simba::scalar::RealField, const D: usize> $Trait<Point<T, D>> for Rotation<T, D>
        { }
    )*}
);

marker_impl!(Isometry, DirectIsometry, OrthogonalTransformation);

/// Subgroups of the n-dimensional rotation group `SO(n)`.
impl<T: RealField + simba::scalar::RealField, const D: usize> linear::Rotation<Point<T, D>>
    for Rotation<T, D>
{
    #[inline]
    fn powf(&self, _: T) -> Option<Self> {
        // XXX: Add the general case.
        // XXX: Use specialization for 2D and 3D.
        unimplemented!()
    }

    #[inline]
    fn rotation_between(_: &SVector<T, D>, _: &SVector<T, D>) -> Option<Self> {
        // XXX: Add the general case.
        // XXX: Use specialization for 2D and 3D.
        unimplemented!()
    }

    #[inline]
    fn scaled_rotation_between(_: &SVector<T, D>, _: &SVector<T, D>, _: T) -> Option<Self> {
        // XXX: Add the general case.
        // XXX: Use specialization for 2D and 3D.
        unimplemented!()
    }
}

/*
impl<T: RealField + simba::scalar::RealField> Matrix for Rotation<T> {
    type Field     = T;
    type Row       = Matrix<T>;
    type Column    = Matrix<T>;
    type Transpose = Self;

    #[inline]
    fn nrows(&self) -> usize {
        self.submatrix.nrows()
    }

    #[inline]
    fn ncolumns(&self) -> usize {
        self.submatrix.ncolumns()
    }

    #[inline]
    fn row(&self, i: usize) -> Self::Row {
        self.submatrix.row(i)
    }

    #[inline]
    fn column(&self, i: usize) -> Self::Column {
        self.submatrix.column(i)
    }

    #[inline]
    fn get(&self, i: usize, j: usize) -> Self::Field {
        self.submatrix[(i, j)]
    }

    #[inline]
    unsafe fn get_unchecked(&self, i: usize, j: usize) -> Self::Field {
        self.submatrix.at_fast(i, j)
    }

    #[inline]
    fn transpose(&self) -> Self::Transpose {
        Rotation::from_matrix_unchecked(self.submatrix.transpose())
    }
}

impl<T: RealField + simba::scalar::RealField> SquareMatrix for Rotation<T> {
    type Vector = Matrix<T>;

    #[inline]
    fn diagonal(&self) -> Self::Coordinates {
        self.submatrix.diagonal()
    }

    #[inline]
    fn determinant(&self) -> Self::Field {
        crate::one()
    }

    #[inline]
    fn try_inverse(&self) -> Option<Self> {
        Some(::transpose(self))
    }

    #[inline]
    fn try_inverse_mut(&mut self) -> bool {
        self.transpose_mut();
        true
    }

    #[inline]
    fn transpose_mut(&mut self) {
        self.submatrix.transpose_mut()
    }
}

impl<T: RealField + simba::scalar::RealField> InversibleSquareMatrix for Rotation<T> { }
*/

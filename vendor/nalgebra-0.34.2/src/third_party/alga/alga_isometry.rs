use alga::general::{
    AbstractGroup, AbstractLoop, AbstractMagma, AbstractMonoid, AbstractQuasigroup,
    AbstractSemigroup, Id, Identity, Multiplicative, RealField, TwoSidedInverse,
};
use alga::linear::Isometry as AlgaIsometry;
use alga::linear::{
    AffineTransformation, DirectIsometry, ProjectiveTransformation, Rotation, Similarity,
    Transformation,
};

use crate::base::SVector;

use crate::geometry::{AbstractRotation, Isometry, Point, Translation};

/*
 *
 * Algebraic structures.
 *
 */
impl<T: RealField + simba::scalar::RealField, R, const D: usize> Identity<Multiplicative>
    for Isometry<T, R, D>
where
    R: Rotation<Point<T, D>> + AbstractRotation<T, D>,
{
    #[inline]
    fn identity() -> Self {
        Self::identity()
    }
}

impl<T: RealField + simba::scalar::RealField, R, const D: usize> TwoSidedInverse<Multiplicative>
    for Isometry<T, R, D>
where
    R: Rotation<Point<T, D>> + AbstractRotation<T, D>,
{
    #[inline]
    fn two_sided_inverse(&self) -> Self {
        self.inverse()
    }

    #[inline]
    fn two_sided_inverse_mut(&mut self) {
        self.inverse_mut()
    }
}

impl<T: RealField + simba::scalar::RealField, R, const D: usize> AbstractMagma<Multiplicative>
    for Isometry<T, R, D>
where
    R: Rotation<Point<T, D>> + AbstractRotation<T, D>,
{
    #[inline]
    fn operate(&self, rhs: &Self) -> Self {
        self * rhs
    }
}

macro_rules! impl_multiplicative_structures(
    ($($marker: ident<$operator: ident>),* $(,)*) => {$(
        impl<T: RealField + simba::scalar::RealField, R, const D: usize> $marker<$operator> for Isometry<T, R, D>
            where R: Rotation<Point<T, D>> + AbstractRotation<T, D> { }
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
impl<T: RealField + simba::scalar::RealField, R, const D: usize> Transformation<Point<T, D>>
    for Isometry<T, R, D>
where
    R: Rotation<Point<T, D>> + AbstractRotation<T, D>,
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

impl<T: RealField + simba::scalar::RealField, R, const D: usize>
    ProjectiveTransformation<Point<T, D>> for Isometry<T, R, D>
where
    R: Rotation<Point<T, D>> + AbstractRotation<T, D>,
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

impl<T: RealField + simba::scalar::RealField, R, const D: usize> AffineTransformation<Point<T, D>>
    for Isometry<T, R, D>
where
    R: Rotation<Point<T, D>> + AbstractRotation<T, D>,
{
    type Rotation = R;
    type NonUniformScaling = Id;
    type Translation = Translation<T, D>;

    #[inline]
    fn decompose(&self) -> (Self::Translation, R, Id, R) {
        (
            self.translation,
            self.rotation.clone(),
            Id::new(),
            <R as AbstractRotation<T, D>>::identity(),
        )
    }

    #[inline]
    fn append_translation(&self, t: &Self::Translation) -> Self {
        t * self
    }

    #[inline]
    fn prepend_translation(&self, t: &Self::Translation) -> Self {
        self * t
    }

    #[inline]
    fn append_rotation(&self, r: &Self::Rotation) -> Self {
        let shift = Transformation::transform_vector(r, &self.translation.vector);
        Isometry::from_parts(Translation::from(shift), r.clone() * self.rotation.clone())
    }

    #[inline]
    fn prepend_rotation(&self, r: &Self::Rotation) -> Self {
        Isometry::from_parts(self.translation, self.rotation.prepend_rotation(r))
    }

    #[inline]
    fn append_scaling(&self, _: &Self::NonUniformScaling) -> Self {
        self.clone()
    }

    #[inline]
    fn prepend_scaling(&self, _: &Self::NonUniformScaling) -> Self {
        self.clone()
    }

    #[inline]
    fn append_rotation_wrt_point(&self, r: &Self::Rotation, p: &Point<T, D>) -> Option<Self> {
        let mut res = self.clone();
        res.append_rotation_wrt_point_mut(r, p);
        Some(res)
    }
}

impl<T: RealField + simba::scalar::RealField, R, const D: usize> Similarity<Point<T, D>>
    for Isometry<T, R, D>
where
    R: Rotation<Point<T, D>> + AbstractRotation<T, D>,
{
    type Scaling = Id;

    #[inline]
    fn translation(&self) -> Translation<T, D> {
        self.translation
    }

    #[inline]
    fn rotation(&self) -> R {
        self.rotation.clone()
    }

    #[inline]
    fn scaling(&self) -> Id {
        Id::new()
    }
}

macro_rules! marker_impl(
    ($($Trait: ident),*) => {$(
        impl<T: RealField + simba::scalar::RealField, R, const D: usize> $Trait<Point<T, D>> for Isometry<T, R, D>
        where R: Rotation<Point<T, D>> + AbstractRotation<T, D> { }
    )*}
);

marker_impl!(AlgaIsometry, DirectIsometry);

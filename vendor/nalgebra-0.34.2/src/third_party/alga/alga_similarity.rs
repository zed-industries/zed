use alga::general::{
    AbstractGroup, AbstractLoop, AbstractMagma, AbstractMonoid, AbstractQuasigroup,
    AbstractSemigroup, Identity, Multiplicative, RealField, TwoSidedInverse,
};
use alga::linear::Similarity as AlgaSimilarity;
use alga::linear::{AffineTransformation, ProjectiveTransformation, Rotation, Transformation};

use crate::base::SVector;
use crate::geometry::{AbstractRotation, Point, Similarity, Translation};

/*
 *
 * Algebraic structures.
 *
 */
impl<T: RealField + simba::scalar::RealField, R, const D: usize> Identity<Multiplicative>
    for Similarity<T, R, D>
where
    R: Rotation<Point<T, D>> + AbstractRotation<T, D>,
{
    #[inline]
    fn identity() -> Self {
        Self::identity()
    }
}

impl<T: RealField + simba::scalar::RealField, R, const D: usize> TwoSidedInverse<Multiplicative>
    for Similarity<T, R, D>
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
    for Similarity<T, R, D>
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
        impl<T: RealField + simba::scalar::RealField, R, const D: usize> $marker<$operator> for Similarity<T, R, D>
            where R: Rotation<Point<T, D>> + AbstractRotation<T, D>,
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
impl<T: RealField + simba::scalar::RealField, R, const D: usize> Transformation<Point<T, D>>
    for Similarity<T, R, D>
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
    ProjectiveTransformation<Point<T, D>> for Similarity<T, R, D>
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
    for Similarity<T, R, D>
where
    R: Rotation<Point<T, D>> + AbstractRotation<T, D>,
{
    type NonUniformScaling = T;
    type Rotation = R;
    type Translation = Translation<T, D>;

    #[inline]
    fn decompose(&self) -> (Translation<T, D>, R, T, R) {
        (
            self.isometry.translation,
            self.isometry.rotation.clone(),
            self.scaling(),
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
        Similarity::from_isometry(self.isometry.append_rotation(r), self.scaling())
    }

    #[inline]
    fn prepend_rotation(&self, r: &Self::Rotation) -> Self {
        Similarity::from_isometry(self.isometry.prepend_rotation(r), self.scaling())
    }

    #[inline]
    fn append_scaling(&self, s: &Self::NonUniformScaling) -> Self {
        self.append_scaling(*s)
    }

    #[inline]
    fn prepend_scaling(&self, s: &Self::NonUniformScaling) -> Self {
        self.prepend_scaling(*s)
    }

    #[inline]
    fn append_rotation_wrt_point(&self, r: &Self::Rotation, p: &Point<T, D>) -> Option<Self> {
        let mut res = self.clone();
        res.append_rotation_wrt_point_mut(r, p);
        Some(res)
    }
}

impl<T: RealField + simba::scalar::RealField, R, const D: usize> AlgaSimilarity<Point<T, D>>
    for Similarity<T, R, D>
where
    R: Rotation<Point<T, D>> + AbstractRotation<T, D>,
{
    type Scaling = T;

    #[inline]
    fn translation(&self) -> Translation<T, D> {
        self.isometry.translation()
    }

    #[inline]
    fn rotation(&self) -> R {
        self.isometry.rotation()
    }

    #[inline]
    fn scaling(&self) -> T {
        self.scaling()
    }
}

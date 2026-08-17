#[cfg(feature = "arbitrary")]
use crate::base::storage::Owned;
#[cfg(feature = "arbitrary")]
use quickcheck::{Arbitrary, Gen};

use num::{One, Zero};
#[cfg(feature = "rand-no-std")]
use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};

use simba::scalar::{ClosedAddAssign, SupersetOf};

use crate::base::{SVector, Scalar};
use crate::geometry::Translation;

impl<T: Scalar + Zero, const D: usize> Default for Translation<T, D> {
    fn default() -> Self {
        Self::identity()
    }
}

impl<T: Scalar, const D: usize> Translation<T, D> {
    /// Creates a new identity translation.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Point2, Point3, Translation2, Translation3};
    /// let t = Translation2::identity();
    /// let p = Point2::new(1.0, 2.0);
    /// assert_eq!(t * p, p);
    ///
    /// // Works in all dimensions.
    /// let t = Translation3::identity();
    /// let p = Point3::new(1.0, 2.0, 3.0);
    /// assert_eq!(t * p, p);
    /// ```
    #[inline]
    pub fn identity() -> Translation<T, D>
    where
        T: Zero,
    {
        Self::from(SVector::<T, D>::from_element(T::zero()))
    }

    /// Cast the components of `self` to another type.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::Translation2;
    /// let tra = Translation2::new(1.0f64, 2.0);
    /// let tra2 = tra.cast::<f32>();
    /// assert_eq!(tra2, Translation2::new(1.0f32, 2.0));
    /// ```
    pub fn cast<To: Scalar>(self) -> Translation<To, D>
    where
        Translation<To, D>: SupersetOf<Self>,
    {
        crate::convert(self)
    }
}

impl<T: Scalar + Zero + ClosedAddAssign, const D: usize> One for Translation<T, D> {
    #[inline]
    fn one() -> Self {
        Self::identity()
    }
}

#[cfg(feature = "rand-no-std")]
impl<T: Scalar, const D: usize> Distribution<Translation<T, D>> for StandardUniform
where
    StandardUniform: Distribution<T>,
{
    /// Generate an arbitrary random variate for testing purposes.
    #[inline]
    fn sample<G: Rng + ?Sized>(&self, rng: &mut G) -> Translation<T, D> {
        Translation::from(rng.random::<SVector<T, D>>())
    }
}

#[cfg(feature = "arbitrary")]
impl<T: Scalar + Arbitrary + Send, const D: usize> Arbitrary for Translation<T, D>
where
    Owned<T, crate::Const<D>>: Send,
{
    #[inline]
    fn arbitrary(rng: &mut Gen) -> Self {
        let v: SVector<T, D> = Arbitrary::arbitrary(rng);
        Self::from(v)
    }
}

/*
 *
 * Small translation construction from components.
 *
 */
macro_rules! componentwise_constructors_impl(
    ($($doc: expr_2021; $D: expr_2021, $($args: ident:$irow: expr_2021),*);* $(;)*) => {$(
        impl<T> Translation<T, $D>
             {
            #[doc = "Initializes this translation from its components."]
            #[doc = "# Example\n```"]
            #[doc = $doc]
            #[doc = "```"]
            #[inline]
            pub const fn new($($args: T),*) -> Self {
                Self { vector: SVector::<T, $D>::new($($args),*) }
            }
        }
    )*}
);

componentwise_constructors_impl!(
    "# use nalgebra::Translation1;\nlet t = Translation1::new(1.0);\nassert!(t.vector.x == 1.0);";
    1, x:0;
    "# use nalgebra::Translation2;\nlet t = Translation2::new(1.0, 2.0);\nassert!(t.vector.x == 1.0 && t.vector.y == 2.0);";
    2, x:0, y:1;
    "# use nalgebra::Translation3;\nlet t = Translation3::new(1.0, 2.0, 3.0);\nassert!(t.vector.x == 1.0 && t.vector.y == 2.0 && t.vector.z == 3.0);";
    3, x:0, y:1, z:2;
    "# use nalgebra::Translation4;\nlet t = Translation4::new(1.0, 2.0, 3.0, 4.0);\nassert!(t.vector.x == 1.0 && t.vector.y == 2.0 && t.vector.z == 3.0 && t.vector.w == 4.0);";
    4, x:0, y:1, z:2, w:3;
    "# use nalgebra::Translation5;\nlet t = Translation5::new(1.0, 2.0, 3.0, 4.0, 5.0);\nassert!(t.vector.x == 1.0 && t.vector.y == 2.0 && t.vector.z == 3.0 && t.vector.w == 4.0 && t.vector.a == 5.0);";
    5, x:0, y:1, z:2, w:3, a:4;
    "# use nalgebra::Translation6;\nlet t = Translation6::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);\nassert!(t.vector.x == 1.0 && t.vector.y == 2.0 && t.vector.z == 3.0 && t.vector.w == 4.0 && t.vector.a == 5.0 && t.vector.b == 6.0);";
    6, x:0, y:1, z:2, w:3, a:4, b:5;
);

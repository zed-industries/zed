#[cfg(feature = "arbitrary")]
use crate::base::storage::Owned;
#[cfg(feature = "arbitrary")]
use quickcheck::{Arbitrary, Gen};

use num::One;
#[cfg(feature = "rand-no-std")]
use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};

use simba::scalar::{ClosedMulAssign, SupersetOf};

use crate::base::{SVector, Scalar};
use crate::geometry::Scale;

impl<T: Scalar, const D: usize> Scale<T, D> {
    /// Creates a new identity scale.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Point2, Point3, Scale2, Scale3};
    /// let t = Scale2::identity();
    /// let p = Point2::new(1.0, 2.0);
    /// assert_eq!(t * p, p);
    ///
    /// // Works in all dimensions.
    /// let t = Scale3::identity();
    /// let p = Point3::new(1.0, 2.0, 3.0);
    /// assert_eq!(t * p, p);
    /// ```
    #[inline]
    pub fn identity() -> Scale<T, D>
    where
        T: One,
    {
        Scale::from(SVector::from_element(T::one()))
    }

    /// Cast the components of `self` to another type.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::Scale2;
    /// let tra = Scale2::new(1.0f64, 2.0);
    /// let tra2 = tra.cast::<f32>();
    /// assert_eq!(tra2, Scale2::new(1.0f32, 2.0));
    /// ```
    pub fn cast<To: Scalar>(self) -> Scale<To, D>
    where
        Scale<To, D>: SupersetOf<Self>,
    {
        crate::convert(self)
    }
}

impl<T: Scalar + One + ClosedMulAssign, const D: usize> One for Scale<T, D> {
    #[inline]
    fn one() -> Self {
        Self::identity()
    }
}

#[cfg(feature = "rand-no-std")]
impl<T: Scalar, const D: usize> Distribution<Scale<T, D>> for StandardUniform
where
    StandardUniform: Distribution<T>,
{
    /// Generate an arbitrary random variate for testing purposes.
    #[inline]
    fn sample<G: Rng + ?Sized>(&self, rng: &mut G) -> Scale<T, D> {
        Scale::from(rng.random::<SVector<T, D>>())
    }
}

#[cfg(feature = "arbitrary")]
impl<T: Scalar + Arbitrary + Send, const D: usize> Arbitrary for Scale<T, D>
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
 * Small Scale construction from components.
 *
 */
macro_rules! componentwise_constructors_impl(
    ($($doc: expr_2021; $D: expr_2021, $($args: ident:$irow: expr_2021),*);* $(;)*) => {$(
        impl<T> Scale<T, $D>
             {
            #[doc = "Initializes this Scale from its components."]
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
    "# use nalgebra::Scale1;\nlet t = Scale1::new(1.0);\nassert!(t.vector.x == 1.0);";
    1, x:0;
    "# use nalgebra::Scale2;\nlet t = Scale2::new(1.0, 2.0);\nassert!(t.vector.x == 1.0 && t.vector.y == 2.0);";
    2, x:0, y:1;
    "# use nalgebra::Scale3;\nlet t = Scale3::new(1.0, 2.0, 3.0);\nassert!(t.vector.x == 1.0 && t.vector.y == 2.0 && t.vector.z == 3.0);";
    3, x:0, y:1, z:2;
    "# use nalgebra::Scale4;\nlet t = Scale4::new(1.0, 2.0, 3.0, 4.0);\nassert!(t.vector.x == 1.0 && t.vector.y == 2.0 && t.vector.z == 3.0 && t.vector.w == 4.0);";
    4, x:0, y:1, z:2, w:3;
    "# use nalgebra::Scale5;\nlet t = Scale5::new(1.0, 2.0, 3.0, 4.0, 5.0);\nassert!(t.vector.x == 1.0 && t.vector.y == 2.0 && t.vector.z == 3.0 && t.vector.w == 4.0 && t.vector.a == 5.0);";
    5, x:0, y:1, z:2, w:3, a:4;
    "# use nalgebra::Scale6;\nlet t = Scale6::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);\nassert!(t.vector.x == 1.0 && t.vector.y == 2.0 && t.vector.z == 3.0 && t.vector.w == 4.0 && t.vector.a == 5.0 && t.vector.b == 6.0);";
    6, x:0, y:1, z:2, w:3, a:4, b:5;
);

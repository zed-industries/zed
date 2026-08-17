#[cfg(feature = "arbitrary")]
use quickcheck::{Arbitrary, Gen};

use num::{Bounded, One, Zero};
#[cfg(feature = "rand-no-std")]
use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};

use crate::base::allocator::Allocator;
use crate::base::dimension::{DimNameAdd, DimNameSum, U1};
use crate::base::{DefaultAllocator, Scalar};
use crate::{
    Const, DimName, OPoint, OVector, Point1, Point2, Point3, Point4, Point5, Point6, Vector1,
    Vector2, Vector3, Vector4, Vector5, Vector6,
};
use simba::scalar::{ClosedDivAssign, SupersetOf};

use crate::geometry::Point;

impl<T: Scalar + Zero, D: DimName> Default for OPoint<T, D>
where
    DefaultAllocator: Allocator<D>,
{
    fn default() -> Self {
        Self::origin()
    }
}

/// # Other construction methods
impl<T: Scalar, D: DimName> OPoint<T, D>
where
    DefaultAllocator: Allocator<D>,
{
    /// Creates a new point with all coordinates equal to zero.
    ///
    /// # Example
    ///
    /// ```
    /// # use nalgebra::{Point2, Point3};
    /// // This works in any dimension.
    /// // The explicit crate::<f32> type annotation may not always be needed,
    /// // depending on the context of type inference.
    /// let pt = Point2::<f32>::origin();
    /// assert!(pt.x == 0.0 && pt.y == 0.0);
    ///
    /// let pt = Point3::<f32>::origin();
    /// assert!(pt.x == 0.0 && pt.y == 0.0 && pt.z == 0.0);
    /// ```
    #[inline]
    pub fn origin() -> Self
    where
        T: Zero,
    {
        Self::from(OVector::from_element(T::zero()))
    }

    /// Creates a new point from a slice.
    ///
    /// # Example
    ///
    /// ```
    /// # use nalgebra::{Point2, Point3};
    /// let data = [ 1.0, 2.0, 3.0 ];
    ///
    /// let pt = Point2::from_slice(&data[..2]);
    /// assert_eq!(pt, Point2::new(1.0, 2.0));
    ///
    /// let pt = Point3::from_slice(&data);
    /// assert_eq!(pt, Point3::new(1.0, 2.0, 3.0));
    /// ```
    #[inline]
    pub fn from_slice(components: &[T]) -> Self {
        Self::from(OVector::from_row_slice(components))
    }

    /// Creates a new point from its homogeneous vector representation.
    ///
    /// In practice, this builds a D-dimensional points with the same first D component as `v`
    /// divided by the last component of `v`. Returns `None` if this divisor is zero.
    ///
    /// # Example
    ///
    /// ```
    /// # use nalgebra::{Point2, Point3, Vector3, Vector4};
    ///
    /// let coords = Vector4::new(1.0, 2.0, 3.0, 1.0);
    /// let pt = Point3::from_homogeneous(coords);
    /// assert_eq!(pt, Some(Point3::new(1.0, 2.0, 3.0)));
    ///
    /// // All component of the result will be divided by the
    /// // last component of the vector, here 2.0.
    /// let coords = Vector4::new(1.0, 2.0, 3.0, 2.0);
    /// let pt = Point3::from_homogeneous(coords);
    /// assert_eq!(pt, Some(Point3::new(0.5, 1.0, 1.5)));
    ///
    /// // Fails because the last component is zero.
    /// let coords = Vector4::new(1.0, 2.0, 3.0, 0.0);
    /// let pt = Point3::from_homogeneous(coords);
    /// assert!(pt.is_none());
    ///
    /// // Works also in other dimensions.
    /// let coords = Vector3::new(1.0, 2.0, 1.0);
    /// let pt = Point2::from_homogeneous(coords);
    /// assert_eq!(pt, Some(Point2::new(1.0, 2.0)));
    /// ```
    #[inline]
    pub fn from_homogeneous(v: OVector<T, DimNameSum<D, U1>>) -> Option<Self>
    where
        T: Scalar + Zero + One + ClosedDivAssign,
        D: DimNameAdd<U1>,
        DefaultAllocator: Allocator<DimNameSum<D, U1>>,
    {
        if !v[D::DIM].is_zero() {
            let coords = v.generic_view((0, 0), (D::name(), Const::<1>)) / v[D::DIM].clone();
            Some(Self::from(coords))
        } else {
            None
        }
    }

    /// Cast the components of `self` to another type.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::Point2;
    /// let pt = Point2::new(1.0f64, 2.0);
    /// let pt2 = pt.cast::<f32>();
    /// assert_eq!(pt2, Point2::new(1.0f32, 2.0));
    /// ```
    pub fn cast<To: Scalar>(self) -> OPoint<To, D>
    where
        OPoint<To, D>: SupersetOf<Self>,
        DefaultAllocator: Allocator<D>,
    {
        crate::convert(self)
    }
}

/*
 *
 * Traits that build points.
 *
 */
impl<T: Scalar + Bounded, D: DimName> Bounded for OPoint<T, D>
where
    DefaultAllocator: Allocator<D>,
{
    #[inline]
    fn max_value() -> Self {
        Self::from(OVector::max_value())
    }

    #[inline]
    fn min_value() -> Self {
        Self::from(OVector::min_value())
    }
}

#[cfg(feature = "rand-no-std")]
impl<T: Scalar, D: DimName> Distribution<OPoint<T, D>> for StandardUniform
where
    StandardUniform: Distribution<T>,
    DefaultAllocator: Allocator<D>,
{
    /// Generate a `Point` where each coordinate is an independent variate from `[0, 1)`.
    #[inline]
    fn sample<'a, G: Rng + ?Sized>(&self, rng: &mut G) -> OPoint<T, D> {
        OPoint::from(rng.random::<OVector<T, D>>())
    }
}

#[cfg(feature = "arbitrary")]
impl<T: Scalar + Arbitrary + Send, D: DimName> Arbitrary for OPoint<T, D>
where
    <DefaultAllocator as Allocator<D>>::Buffer<T>: Send,
    DefaultAllocator: Allocator<D>,
{
    #[inline]
    fn arbitrary(g: &mut Gen) -> Self {
        Self::from(OVector::arbitrary(g))
    }
}

/*
 *
 * Small points construction from components.
 *
 */
// NOTE: the impl for Point1 is not with the others so that we
// can add a section with the impl block comment.
/// # Construction from individual components
impl<T: Scalar> Point1<T> {
    /// Initializes this point from its components.
    ///
    /// # Example
    ///
    /// ```
    /// # use nalgebra::Point1;
    /// let p = Point1::new(1.0);
    /// assert_eq!(p.x, 1.0);
    /// ```
    #[inline]
    pub const fn new(x: T) -> Self {
        Point {
            coords: Vector1::new(x),
        }
    }
}
macro_rules! componentwise_constructors_impl(
    ($($doc: expr_2021; $Point: ident, $Vector: ident, $($args: ident:$irow: expr_2021),*);* $(;)*) => {$(
        impl<T: Scalar> $Point<T> {
            #[doc = "Initializes this point from its components."]
            #[doc = "# Example\n```"]
            #[doc = $doc]
            #[doc = "```"]
            #[inline]
            pub const fn new($($args: T),*) -> Self {
                Point { coords: $Vector::new($($args),*) }
            }
        }
    )*}
);

componentwise_constructors_impl!(
    "# use nalgebra::Point2;\nlet p = Point2::new(1.0, 2.0);\nassert!(p.x == 1.0 && p.y == 2.0);";
    Point2, Vector2, x:0, y:1;
    "# use nalgebra::Point3;\nlet p = Point3::new(1.0, 2.0, 3.0);\nassert!(p.x == 1.0 && p.y == 2.0 && p.z == 3.0);";
    Point3, Vector3, x:0, y:1, z:2;
    "# use nalgebra::Point4;\nlet p = Point4::new(1.0, 2.0, 3.0, 4.0);\nassert!(p.x == 1.0 && p.y == 2.0 && p.z == 3.0 && p.w == 4.0);";
    Point4, Vector4, x:0, y:1, z:2, w:3;
    "# use nalgebra::Point5;\nlet p = Point5::new(1.0, 2.0, 3.0, 4.0, 5.0);\nassert!(p.x == 1.0 && p.y == 2.0 && p.z == 3.0 && p.w == 4.0 && p.a == 5.0);";
    Point5, Vector5, x:0, y:1, z:2, w:3, a:4;
    "# use nalgebra::Point6;\nlet p = Point6::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);\nassert!(p.x == 1.0 && p.y == 2.0 && p.z == 3.0 && p.w == 4.0 && p.a == 5.0 && p.b == 6.0);";
    Point6, Vector6, x:0, y:1, z:2, w:3, a:4, b:5;
);

use crate::Const;
use crate::geometry::OPoint;

/// A point with `D` elements.
pub type Point<T, const D: usize> = OPoint<T, Const<D>>;

/// A statically sized 1-dimensional column point.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Point`] type too.**
pub type Point1<T> = Point<T, 1>;
/// A statically sized 2-dimensional column point.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Point`] type too.**
pub type Point2<T> = Point<T, 2>;
/// A statically sized 3-dimensional column point.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Point`] type too.**
pub type Point3<T> = Point<T, 3>;
/// A statically sized 4-dimensional column point.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Point`] type too.**
pub type Point4<T> = Point<T, 4>;
/// A statically sized 5-dimensional column point.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Point`] type too.**
pub type Point5<T> = Point<T, 5>;
/// A statically sized 6-dimensional column point.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Point`] type too.**
pub type Point6<T> = Point<T, 6>;

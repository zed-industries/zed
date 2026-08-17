//! Line and Column newtypes for strongly typed tty/grid/terminal APIs.

/// Indexing types and implementations for Grid and Line.
use std::cmp::{Ord, Ordering, max, min};
use std::fmt;
use std::ops::{Add, AddAssign, Deref, Sub, SubAssign};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::grid::Dimensions;

/// The side of a cell.
pub type Side = Direction;

/// Horizontal direction.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    #[must_use]
    pub fn opposite(self) -> Self {
        match self {
            Side::Right => Side::Left,
            Side::Left => Side::Right,
        }
    }
}

/// Terminal grid boundaries.
pub enum Boundary {
    /// Cursor's range of motion in the grid.
    ///
    /// This is equal to the viewport when the user isn't scrolled into the history.
    Cursor,

    /// Topmost line in history until the bottommost line in the terminal.
    Grid,

    /// Unbounded.
    None,
}

/// Index in the grid using row, column notation.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Point<L = Line, C = Column> {
    pub line: L,
    pub column: C,
}

impl<L, C> Point<L, C> {
    pub fn new(line: L, column: C) -> Point<L, C> {
        Point { line, column }
    }
}

impl Point {
    /// Subtract a number of columns from a point.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn sub<D>(mut self, dimensions: &D, boundary: Boundary, rhs: usize) -> Self
    where
        D: Dimensions,
    {
        let cols = dimensions.columns();
        let line_changes = (rhs + cols - 1).saturating_sub(self.column.0) / cols;
        self.line -= line_changes;
        self.column = Column((cols + self.column.0 - rhs % cols) % cols);
        self.grid_clamp(dimensions, boundary)
    }

    /// Add a number of columns to a point.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn add<D>(mut self, dimensions: &D, boundary: Boundary, rhs: usize) -> Self
    where
        D: Dimensions,
    {
        let cols = dimensions.columns();
        self.line += (rhs + self.column.0) / cols;
        self.column = Column((self.column.0 + rhs) % cols);
        self.grid_clamp(dimensions, boundary)
    }

    /// Clamp a point to a grid boundary.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn grid_clamp<D>(mut self, dimensions: &D, boundary: Boundary) -> Self
    where
        D: Dimensions,
    {
        let last_column = dimensions.last_column();
        self.column = min(self.column, last_column);

        let topmost_line = dimensions.topmost_line();
        let bottommost_line = dimensions.bottommost_line();

        match boundary {
            Boundary::Cursor if self.line < 0 => Point::new(Line(0), Column(0)),
            Boundary::Grid if self.line < topmost_line => Point::new(topmost_line, Column(0)),
            Boundary::Cursor | Boundary::Grid if self.line > bottommost_line => {
                Point::new(bottommost_line, last_column)
            },
            Boundary::None => {
                self.line = self.line.grid_clamp(dimensions, boundary);
                self
            },
            _ => self,
        }
    }
}

impl<L: Ord, C: Ord> PartialOrd for Point<L, C> {
    fn partial_cmp(&self, other: &Point<L, C>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<L: Ord, C: Ord> Ord for Point<L, C> {
    fn cmp(&self, other: &Point<L, C>) -> Ordering {
        match (self.line.cmp(&other.line), self.column.cmp(&other.column)) {
            (Ordering::Equal, ord) | (ord, _) => ord,
        }
    }
}

/// A line.
///
/// Newtype to avoid passing values incorrectly.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Line(pub i32);

impl Line {
    /// Clamp a line to a grid boundary.
    #[must_use]
    pub fn grid_clamp<D: Dimensions>(self, dimensions: &D, boundary: Boundary) -> Self {
        match boundary {
            Boundary::Cursor => max(Line(0), min(dimensions.bottommost_line(), self)),
            Boundary::Grid => {
                let bottommost_line = dimensions.bottommost_line();
                let topmost_line = dimensions.topmost_line();
                max(topmost_line, min(bottommost_line, self))
            },
            Boundary::None => {
                let screen_lines = dimensions.screen_lines() as i32;
                let total_lines = dimensions.total_lines() as i32;

                if self >= screen_lines {
                    let topmost_line = dimensions.topmost_line();
                    let extra = (self.0 - screen_lines) % total_lines;
                    topmost_line + extra
                } else {
                    let bottommost_line = dimensions.bottommost_line();
                    let extra = (self.0 - screen_lines + 1) % total_lines;
                    bottommost_line + extra
                }
            },
        }
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<usize> for Line {
    fn from(source: usize) -> Self {
        Self(source as i32)
    }
}

impl Add<usize> for Line {
    type Output = Line;

    #[inline]
    fn add(self, rhs: usize) -> Line {
        self + rhs as i32
    }
}

impl AddAssign<usize> for Line {
    #[inline]
    fn add_assign(&mut self, rhs: usize) {
        *self += rhs as i32;
    }
}

impl Sub<usize> for Line {
    type Output = Line;

    #[inline]
    fn sub(self, rhs: usize) -> Line {
        self - rhs as i32
    }
}

impl SubAssign<usize> for Line {
    #[inline]
    fn sub_assign(&mut self, rhs: usize) {
        *self -= rhs as i32;
    }
}

impl PartialOrd<usize> for Line {
    #[inline]
    fn partial_cmp(&self, other: &usize) -> Option<Ordering> {
        self.0.partial_cmp(&(*other as i32))
    }
}

impl PartialEq<usize> for Line {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        self.0.eq(&(*other as i32))
    }
}

/// A column.
///
/// Newtype to avoid passing values incorrectly.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Column(pub usize);

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

macro_rules! ops {
    ($ty:ty, $construct:expr, $primitive:ty) => {
        impl Deref for $ty {
            type Target = $primitive;

            #[inline]
            fn deref(&self) -> &$primitive {
                &self.0
            }
        }

        impl From<$primitive> for $ty {
            #[inline]
            fn from(val: $primitive) -> $ty {
                $construct(val)
            }
        }

        impl Add<$ty> for $ty {
            type Output = $ty;

            #[inline]
            fn add(self, rhs: $ty) -> $ty {
                $construct(self.0 + rhs.0)
            }
        }

        impl AddAssign<$ty> for $ty {
            #[inline]
            fn add_assign(&mut self, rhs: $ty) {
                self.0 += rhs.0;
            }
        }

        impl Add<$primitive> for $ty {
            type Output = $ty;

            #[inline]
            fn add(self, rhs: $primitive) -> $ty {
                $construct(self.0 + rhs)
            }
        }

        impl AddAssign<$primitive> for $ty {
            #[inline]
            fn add_assign(&mut self, rhs: $primitive) {
                self.0 += rhs
            }
        }

        impl Sub<$ty> for $ty {
            type Output = $ty;

            #[inline]
            fn sub(self, rhs: $ty) -> $ty {
                $construct(self.0 - rhs.0)
            }
        }

        impl SubAssign<$ty> for $ty {
            #[inline]
            fn sub_assign(&mut self, rhs: $ty) {
                self.0 -= rhs.0;
            }
        }

        impl Sub<$primitive> for $ty {
            type Output = $ty;

            #[inline]
            fn sub(self, rhs: $primitive) -> $ty {
                $construct(self.0 - rhs)
            }
        }

        impl SubAssign<$primitive> for $ty {
            #[inline]
            fn sub_assign(&mut self, rhs: $primitive) {
                self.0 -= rhs
            }
        }

        impl PartialEq<$ty> for $primitive {
            #[inline]
            fn eq(&self, other: &$ty) -> bool {
                self.eq(&other.0)
            }
        }

        impl PartialEq<$primitive> for $ty {
            #[inline]
            fn eq(&self, other: &$primitive) -> bool {
                self.0.eq(other)
            }
        }

        impl PartialOrd<$ty> for $primitive {
            #[inline]
            fn partial_cmp(&self, other: &$ty) -> Option<Ordering> {
                self.partial_cmp(&other.0)
            }
        }

        impl PartialOrd<$primitive> for $ty {
            #[inline]
            fn partial_cmp(&self, other: &$primitive) -> Option<Ordering> {
                self.0.partial_cmp(other)
            }
        }
    };
}

ops!(Column, Column, usize);
ops!(Line, Line, i32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn location_ordering() {
        assert!(Point::new(Line(0), Column(0)) == Point::new(Line(0), Column(0)));
        assert!(Point::new(Line(1), Column(0)) > Point::new(Line(0), Column(0)));
        assert!(Point::new(Line(0), Column(1)) > Point::new(Line(0), Column(0)));
        assert!(Point::new(Line(1), Column(1)) > Point::new(Line(0), Column(0)));
        assert!(Point::new(Line(1), Column(1)) > Point::new(Line(0), Column(1)));
        assert!(Point::new(Line(1), Column(1)) > Point::new(Line(1), Column(0)));
        assert!(Point::new(Line(0), Column(0)) > Point::new(Line(-1), Column(0)));
    }

    #[test]
    fn sub() {
        let size = (10, 42);
        let point = Point::new(Line(0), Column(13));

        let result = point.sub(&size, Boundary::Cursor, 1);

        assert_eq!(result, Point::new(Line(0), point.column - 1));
    }

    #[test]
    fn sub_wrap() {
        let size = (10, 42);
        let point = Point::new(Line(1), Column(0));

        let result = point.sub(&size, Boundary::Cursor, 1);

        assert_eq!(result, Point::new(Line(0), size.last_column()));
    }

    #[test]
    fn sub_clamp() {
        let size = (10, 42);
        let point = Point::new(Line(0), Column(0));

        let result = point.sub(&size, Boundary::Cursor, 1);

        assert_eq!(result, point);
    }

    #[test]
    fn sub_grid_clamp() {
        let size = (0, 42);
        let point = Point::new(Line(0), Column(0));

        let result = point.sub(&size, Boundary::Grid, 1);

        assert_eq!(result, point);
    }

    #[test]
    fn sub_none_clamp() {
        let size = (10, 42);
        let point = Point::new(Line(0), Column(0));

        let result = point.sub(&size, Boundary::None, 1);

        assert_eq!(result, Point::new(Line(9), Column(41)));
    }

    #[test]
    fn add() {
        let size = (10, 42);
        let point = Point::new(Line(0), Column(13));

        let result = point.add(&size, Boundary::Cursor, 1);

        assert_eq!(result, Point::new(Line(0), point.column + 1));
    }

    #[test]
    fn add_wrap() {
        let size = (10, 42);
        let point = Point::new(Line(0), size.last_column());

        let result = point.add(&size, Boundary::Cursor, 1);

        assert_eq!(result, Point::new(Line(1), Column(0)));
    }

    #[test]
    fn add_clamp() {
        let size = (10, 42);
        let point = Point::new(Line(9), Column(41));

        let result = point.add(&size, Boundary::Cursor, 1);

        assert_eq!(result, point);
    }

    #[test]
    fn add_grid_clamp() {
        let size = (10, 42);
        let point = Point::new(Line(9), Column(41));

        let result = point.add(&size, Boundary::Grid, 1);

        assert_eq!(result, point);
    }

    #[test]
    fn add_none_clamp() {
        let size = (10, 42);
        let point = Point::new(Line(9), Column(41));

        let result = point.add(&size, Boundary::None, 1);

        assert_eq!(result, Point::new(Line(0), Column(0)));
    }
}

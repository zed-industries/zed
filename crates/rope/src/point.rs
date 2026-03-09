use std::{
    cmp::Ordering,
    fmt::{self, Debug},
    ops::{Add, AddAssign, Range, Sub},
};

/// A zero-indexed point in a text buffer consisting of a row and column.
#[derive(Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Point {
    pub row: u32,
    pub column: u32,
}

impl Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point({}:{})", self.row, self.column)
    }
}

impl Point {
    pub const MAX: Self = Self {
        row: u32::MAX,
        column: u32::MAX,
    };

    pub fn new(row: u32, column: u32) -> Self {
        Point { row, column }
    }

    pub fn row_range(range: Range<u32>) -> Range<Self> {
        Point {
            row: range.start,
            column: 0,
        }..Point {
            row: range.end,
            column: 0,
        }
    }

    pub fn zero() -> Self {
        Point::new(0, 0)
    }

    pub fn parse_str(s: &str) -> Self {
        let mut point = Self::zero();
        for (row, line) in s.split('\n').enumerate() {
            point.row = row as u32;
            point.column = line.len() as u32;
        }
        point
    }

    pub fn is_zero(&self) -> bool {
        self.row == 0 && self.column == 0
    }

    pub fn saturating_sub(self, other: Self) -> Self {
        if self < other {
            Self::zero()
        } else {
            self - other
        }
    }
}

impl<'a> Add<&'a Self> for Point {
    type Output = Point;

    fn add(self, other: &'a Self) -> Self::Output {
        self + *other
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        if other.row == 0 {
            Point::new(self.row, self.column + other.column)
        } else {
            Point::new(self.row + other.row, other.column)
        }
    }
}

impl<'a> Sub<&'a Self> for Point {
    type Output = Point;

    fn sub(self, other: &'a Self) -> Self::Output {
        self - *other
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Self) -> Self::Output {
        debug_assert!(other <= self);

        if self.row == other.row {
            Point::new(0, self.column - other.column)
        } else {
            Point::new(self.row - other.row, self.column)
        }
    }
}

impl<'a> AddAssign<&'a Self> for Point {
    fn add_assign(&mut self, other: &'a Self) {
        *self += *other;
    }
}

impl AddAssign<Self> for Point {
    fn add_assign(&mut self, other: Self) {
        if other.row == 0 {
            self.column += other.column;
        } else {
            self.row += other.row;
            self.column = other.column;
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    #[cfg(target_pointer_width = "64")]
    fn cmp(&self, other: &Point) -> Ordering {
        let a = ((self.row as usize) << 32) | self.column as usize;
        let b = ((other.row as usize) << 32) | other.column as usize;
        a.cmp(&b)
    }

    #[cfg(target_pointer_width = "32")]
    fn cmp(&self, other: &Point) -> Ordering {
        match self.row.cmp(&other.row) {
            Ordering::Equal => self.column.cmp(&other.column),
            comparison @ _ => comparison,
        }
    }
}

use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, Sub},
};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash)]
pub struct PointUtf16 {
    pub row: u32,
    pub column: u32,
}

impl PointUtf16 {
    pub const MAX: Self = Self {
        row: u32::MAX,
        column: u32::MAX,
    };

    pub fn new(row: u32, column: u32) -> Self {
        PointUtf16 { row, column }
    }

    pub fn zero() -> Self {
        PointUtf16::new(0, 0)
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

impl<'a> Add<&'a Self> for PointUtf16 {
    type Output = PointUtf16;

    fn add(self, other: &'a Self) -> Self::Output {
        self + *other
    }
}

impl Add for PointUtf16 {
    type Output = PointUtf16;

    fn add(self, other: Self) -> Self::Output {
        if other.row == 0 {
            PointUtf16::new(self.row, self.column + other.column)
        } else {
            PointUtf16::new(self.row + other.row, other.column)
        }
    }
}

impl<'a> Sub<&'a Self> for PointUtf16 {
    type Output = PointUtf16;

    fn sub(self, other: &'a Self) -> Self::Output {
        self - *other
    }
}

impl Sub for PointUtf16 {
    type Output = PointUtf16;

    fn sub(self, other: Self) -> Self::Output {
        debug_assert!(other <= self);

        if self.row == other.row {
            PointUtf16::new(0, self.column - other.column)
        } else {
            PointUtf16::new(self.row - other.row, self.column)
        }
    }
}

impl<'a> AddAssign<&'a Self> for PointUtf16 {
    fn add_assign(&mut self, other: &'a Self) {
        *self += *other;
    }
}

impl AddAssign<Self> for PointUtf16 {
    fn add_assign(&mut self, other: Self) {
        if other.row == 0 {
            self.column += other.column;
        } else {
            self.row += other.row;
            self.column = other.column;
        }
    }
}

impl PartialOrd for PointUtf16 {
    fn partial_cmp(&self, other: &PointUtf16) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PointUtf16 {
    #[cfg(target_pointer_width = "64")]
    fn cmp(&self, other: &PointUtf16) -> Ordering {
        let a = ((self.row as usize) << 32) | self.column as usize;
        let b = ((other.row as usize) << 32) | other.column as usize;
        a.cmp(&b)
    }

    #[cfg(target_pointer_width = "32")]
    fn cmp(&self, other: &PointUtf16) -> Ordering {
        match self.row.cmp(&other.row) {
            Ordering::Equal => self.column.cmp(&other.column),
            comparison @ _ => comparison,
        }
    }
}

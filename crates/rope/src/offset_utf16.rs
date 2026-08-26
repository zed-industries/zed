use std::ops::{Add, AddAssign, Sub};

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct OffsetUtf16(pub usize);

impl<'a> Add<&'a Self> for OffsetUtf16 {
    type Output = Self;

    fn add(self, other: &'a Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl Add for OffsetUtf16 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl<'a> Sub<&'a Self> for OffsetUtf16 {
    type Output = Self;

    fn sub(self, other: &'a Self) -> Self::Output {
        debug_assert!(*other <= self);
        Self(self.0 - other.0)
    }
}

impl Sub for OffsetUtf16 {
    type Output = OffsetUtf16;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}

impl<'a> AddAssign<&'a Self> for OffsetUtf16 {
    fn add_assign(&mut self, other: &'a Self) {
        self.0 += other.0;
    }
}

impl AddAssign<Self> for OffsetUtf16 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

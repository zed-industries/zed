#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct RowDelta(pub u32);

impl RowDelta {
    pub fn saturating_sub(self, other: RowDelta) -> RowDelta {
        RowDelta(self.0.saturating_sub(other.0))
    }
}

impl ::std::ops::Add for RowDelta {
    type Output = RowDelta;

    fn add(self, rhs: RowDelta) -> Self::Output {
        RowDelta(self.0 + rhs.0)
    }
}

impl ::std::ops::Sub for RowDelta {
    type Output = RowDelta;

    fn sub(self, rhs: RowDelta) -> Self::Output {
        RowDelta(self.0 - rhs.0)
    }
}

impl ::std::ops::AddAssign for RowDelta {
    fn add_assign(&mut self, rhs: RowDelta) {
        self.0 += rhs.0;
    }
}

impl ::std::ops::SubAssign for RowDelta {
    fn sub_assign(&mut self, rhs: RowDelta) {
        self.0 -= rhs.0;
    }
}

macro_rules! impl_for_row_types {
    ($row:ident => $row_delta:ident) => {
        impl $row {
            pub fn saturating_sub(self, other: $row_delta) -> $row {
                $row(self.0.saturating_sub(other.0))
            }
        }

        impl ::std::ops::Add for $row {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }

        impl ::std::ops::Add<$row_delta> for $row {
            type Output = Self;

            fn add(self, rhs: $row_delta) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }

        impl ::std::ops::Sub for $row {
            type Output = $row_delta;

            fn sub(self, rhs: Self) -> Self::Output {
                $row_delta(self.0 - rhs.0)
            }
        }

        impl ::std::ops::Sub<$row_delta> for $row {
            type Output = $row;

            fn sub(self, rhs: $row_delta) -> Self::Output {
                $row(self.0 - rhs.0)
            }
        }

        impl ::std::ops::AddAssign for $row {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
            }
        }

        impl ::std::ops::AddAssign<$row_delta> for $row {
            fn add_assign(&mut self, rhs: $row_delta) {
                self.0 += rhs.0;
            }
        }

        impl ::std::ops::SubAssign<$row_delta> for $row {
            fn sub_assign(&mut self, rhs: $row_delta) {
                self.0 -= rhs.0;
            }
        }
    };
}

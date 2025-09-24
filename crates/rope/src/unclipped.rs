use crate::ChunkSummary;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Unclipped<T>(pub T);

impl<T> From<T> for Unclipped<T> {
    fn from(value: T) -> Self {
        Unclipped(value)
    }
}

impl<'a, T: sum_tree::Dimension<'a, ChunkSummary>> sum_tree::Dimension<'a, ChunkSummary>
    for Unclipped<T>
{
    fn zero(_: ()) -> Self {
        Self(T::zero(()))
    }

    fn add_summary(&mut self, summary: &'a ChunkSummary, _: ()) {
        self.0.add_summary(summary, ());
    }
}

impl<T: Add<T, Output = T>> Add<Unclipped<T>> for Unclipped<T> {
    type Output = Unclipped<T>;

    fn add(self, rhs: Unclipped<T>) -> Self::Output {
        Unclipped(self.0 + rhs.0)
    }
}

impl<T: Sub<T, Output = T>> Sub<Unclipped<T>> for Unclipped<T> {
    type Output = Unclipped<T>;

    fn sub(self, rhs: Unclipped<T>) -> Self::Output {
        Unclipped(self.0 - rhs.0)
    }
}

impl<T: AddAssign<T>> AddAssign<Unclipped<T>> for Unclipped<T> {
    fn add_assign(&mut self, rhs: Unclipped<T>) {
        self.0 += rhs.0;
    }
}

impl<T: SubAssign<T>> SubAssign<Unclipped<T>> for Unclipped<T> {
    fn sub_assign(&mut self, rhs: Unclipped<T>) {
        self.0 -= rhs.0;
    }
}

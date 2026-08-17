use crate::stats::float::Float;
use cast::{self, usize};

/// A "view" into the percentiles of a sample
pub struct Percentiles<A>(Box<[A]>)
where
    A: Float;

// TODO(rust-lang/rfcs#735) move this `impl` into a private percentiles module
impl<A> Percentiles<A>
where
    A: Float,
    usize: cast::From<A, Output = Result<usize, cast::Error>>,
{
    /// Returns the percentile at `p`%
    ///
    /// Safety:
    ///
    /// - Make sure that `p` is in the range `[0, 100]`
    unsafe fn at_unchecked(&self, p: A) -> A {
        let _100 = A::cast(100);
        debug_assert!(p >= A::cast(0) && p <= _100);
        debug_assert!(self.0.len() > 0);
        let len = self.0.len() - 1;

        if p == _100 {
            self.0[len]
        } else {
            let rank = (p / _100) * A::cast(len);
            let integer = rank.floor();
            let fraction = rank - integer;
            let n = usize(integer).unwrap();
            let &floor = self.0.get_unchecked(n);
            let &ceiling = self.0.get_unchecked(n + 1);

            floor + (ceiling - floor) * fraction
        }
    }

    /// Returns the percentile at `p`%
    ///
    /// # Panics
    ///
    /// Panics if `p` is outside the closed `[0, 100]` range
    pub fn at(&self, p: A) -> A {
        let _0 = A::cast(0);
        let _100 = A::cast(100);

        assert!(p >= _0 && p <= _100);
        assert!(self.0.len() > 0);

        unsafe { self.at_unchecked(p) }
    }

    /// Returns the interquartile range
    pub fn iqr(&self) -> A {
        let q1 = self.at(A::cast(25));
        let q3 = self.at(A::cast(75));

        q3 - q1
    }

    /// Returns the 50th percentile
    pub fn median(&self) -> A {
        self.at(A::cast(50))
    }

    /// Returns the 25th, 50th and 75th percentiles
    pub fn quartiles(&self) -> (A, A, A) {
        (
            self.at(A::cast(25)),
            self.at(A::cast(50)),
            self.at(A::cast(75)),
        )
    }
}

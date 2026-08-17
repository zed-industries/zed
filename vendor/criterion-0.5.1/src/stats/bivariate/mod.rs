//! Bivariate analysis

mod bootstrap;
pub mod regression;
mod resamples;

use crate::stats::bivariate::resamples::Resamples;
use crate::stats::float::Float;
use crate::stats::tuple::{Tuple, TupledDistributionsBuilder};
use crate::stats::univariate::Sample;
#[cfg(feature = "rayon")]
use rayon::iter::{IntoParallelIterator, ParallelIterator};

/// Bivariate `(X, Y)` data
///
/// Invariants:
///
/// - No `NaN`s in the data
/// - At least two data points in the set
pub struct Data<'a, X, Y>(&'a [X], &'a [Y]);

impl<'a, X, Y> Copy for Data<'a, X, Y> {}

#[cfg_attr(feature = "cargo-clippy", allow(clippy::expl_impl_clone_on_copy))]
impl<'a, X, Y> Clone for Data<'a, X, Y> {
    fn clone(&self) -> Data<'a, X, Y> {
        *self
    }
}

impl<'a, X, Y> Data<'a, X, Y> {
    /// Returns the length of the data set
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Iterate over the data set
    pub fn iter(&self) -> Pairs<'a, X, Y> {
        Pairs {
            data: *self,
            state: 0,
        }
    }
}

impl<'a, X, Y> Data<'a, X, Y>
where
    X: Float,
    Y: Float,
{
    /// Creates a new data set from two existing slices
    pub fn new(xs: &'a [X], ys: &'a [Y]) -> Data<'a, X, Y> {
        assert!(
            xs.len() == ys.len()
                && xs.len() > 1
                && xs.iter().all(|x| !x.is_nan())
                && ys.iter().all(|y| !y.is_nan())
        );

        Data(xs, ys)
    }

    // TODO Remove the `T` parameter in favor of `S::Output`
    /// Returns the bootstrap distributions of the parameters estimated by the `statistic`
    ///
    /// - Multi-threaded
    /// - Time: `O(nresamples)`
    /// - Memory: `O(nresamples)`
    pub fn bootstrap<T, S>(&self, nresamples: usize, statistic: S) -> T::Distributions
    where
        S: Fn(Data<X, Y>) -> T + Sync,
        T: Tuple + Send,
        T::Distributions: Send,
        T::Builder: Send,
    {
        #[cfg(feature = "rayon")]
        {
            (0..nresamples)
                .into_par_iter()
                .map_init(
                    || Resamples::new(*self),
                    |resamples, _| statistic(resamples.next()),
                )
                .fold(
                    || T::Builder::new(0),
                    |mut sub_distributions, sample| {
                        sub_distributions.push(sample);
                        sub_distributions
                    },
                )
                .reduce(
                    || T::Builder::new(0),
                    |mut a, mut b| {
                        a.extend(&mut b);
                        a
                    },
                )
                .complete()
        }
        #[cfg(not(feature = "rayon"))]
        {
            let mut resamples = Resamples::new(*self);
            (0..nresamples)
                .map(|_| statistic(resamples.next()))
                .fold(T::Builder::new(0), |mut sub_distributions, sample| {
                    sub_distributions.push(sample);
                    sub_distributions
                })
                .complete()
        }
    }

    /// Returns a view into the `X` data
    pub fn x(&self) -> &'a Sample<X> {
        Sample::new(self.0)
    }

    /// Returns a view into the `Y` data
    pub fn y(&self) -> &'a Sample<Y> {
        Sample::new(self.1)
    }
}

/// Iterator over `Data`
pub struct Pairs<'a, X: 'a, Y: 'a> {
    data: Data<'a, X, Y>,
    state: usize,
}

impl<'a, X, Y> Iterator for Pairs<'a, X, Y> {
    type Item = (&'a X, &'a Y);

    fn next(&mut self) -> Option<(&'a X, &'a Y)> {
        if self.state < self.data.len() {
            let i = self.state;
            self.state += 1;

            // This is safe because i will always be < self.data.{0,1}.len()
            debug_assert!(i < self.data.0.len());
            debug_assert!(i < self.data.1.len());
            unsafe { Some((self.data.0.get_unchecked(i), self.data.1.get_unchecked(i))) }
        } else {
            None
        }
    }
}

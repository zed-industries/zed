//! Univariate analysis

mod bootstrap;
mod percentiles;
mod resamples;
mod sample;

pub mod kde;
pub mod mixed;
pub mod outliers;

use crate::stats::float::Float;
use crate::stats::tuple::{Tuple, TupledDistributionsBuilder};
#[cfg(feature = "rayon")]
use rayon::prelude::*;
use std::cmp;

use self::resamples::Resamples;

pub use self::percentiles::Percentiles;
pub use self::sample::Sample;

/// Performs a two-sample bootstrap
///
/// - Multithreaded
/// - Time: `O(nresamples)`
/// - Memory: `O(nresamples)`
#[cfg_attr(feature = "cargo-clippy", allow(clippy::cast_lossless))]
pub fn bootstrap<A, B, T, S>(
    a: &Sample<A>,
    b: &Sample<B>,
    nresamples: usize,
    statistic: S,
) -> T::Distributions
where
    A: Float,
    B: Float,
    S: Fn(&Sample<A>, &Sample<B>) -> T + Sync,
    T: Tuple + Send,
    T::Distributions: Send,
    T::Builder: Send,
{
    let nresamples_sqrt = (nresamples as f64).sqrt().ceil() as usize;
    let per_chunk = (nresamples + nresamples_sqrt - 1) / nresamples_sqrt;

    #[cfg(feature = "rayon")]
    {
        (0..nresamples_sqrt)
            .into_par_iter()
            .map_init(
                || (Resamples::new(a), Resamples::new(b)),
                |(a_resamples, b_resamples), i| {
                    let start = i * per_chunk;
                    let end = cmp::min((i + 1) * per_chunk, nresamples);
                    let a_resample = a_resamples.next();

                    let mut sub_distributions: T::Builder =
                        TupledDistributionsBuilder::new(end - start);

                    for _ in start..end {
                        let b_resample = b_resamples.next();
                        sub_distributions.push(statistic(a_resample, b_resample));
                    }
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
        let mut a_resamples = Resamples::new(a);
        let mut b_resamples = Resamples::new(b);
        (0..nresamples_sqrt)
            .map(|i| {
                let start = i * per_chunk;
                let end = cmp::min((i + 1) * per_chunk, nresamples);
                let a_resample = a_resamples.next();

                let mut sub_distributions: T::Builder =
                    TupledDistributionsBuilder::new(end - start);

                for _ in start..end {
                    let b_resample = b_resamples.next();
                    sub_distributions.push(statistic(a_resample, b_resample));
                }
                sub_distributions
            })
            .fold(T::Builder::new(0), |mut a, mut b| {
                a.extend(&mut b);
                a
            })
            .complete()
    }
}

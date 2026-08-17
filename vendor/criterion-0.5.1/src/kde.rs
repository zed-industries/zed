use crate::stats::univariate::kde::kernel::Gaussian;
use crate::stats::univariate::kde::{Bandwidth, Kde};
use crate::stats::univariate::Sample;

pub fn sweep(
    sample: &Sample<f64>,
    npoints: usize,
    range: Option<(f64, f64)>,
) -> (Box<[f64]>, Box<[f64]>) {
    let (xs, ys, _) = sweep_and_estimate(sample, npoints, range, sample[0]);
    (xs, ys)
}

pub fn sweep_and_estimate(
    sample: &Sample<f64>,
    npoints: usize,
    range: Option<(f64, f64)>,
    point_to_estimate: f64,
) -> (Box<[f64]>, Box<[f64]>, f64) {
    let x_min = sample.min();
    let x_max = sample.max();

    let kde = Kde::new(sample, Gaussian, Bandwidth::Silverman);
    let h = kde.bandwidth();

    let (start, end) = match range {
        Some((start, end)) => (start, end),
        None => (x_min - 3. * h, x_max + 3. * h),
    };

    let mut xs: Vec<f64> = Vec::with_capacity(npoints);
    let step_size = (end - start) / (npoints - 1) as f64;
    for n in 0..npoints {
        xs.push(start + (step_size * n as f64));
    }

    let ys = kde.map(&xs);
    let point_estimate = kde.estimate(point_to_estimate);

    (xs.into_boxed_slice(), ys, point_estimate)
}

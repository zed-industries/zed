use crate::stats::univariate::Sample;
use crate::stats::univariate::{self, mixed};
use crate::stats::Distribution;

use crate::benchmark::BenchmarkConfig;
use crate::error::Result;
use crate::estimate::{
    build_change_estimates, ChangeDistributions, ChangeEstimates, ChangePointEstimates, Estimates,
};
use crate::measurement::Measurement;
use crate::report::BenchmarkId;
use crate::{fs, Criterion, SavedSample};

// Common comparison procedure
#[cfg_attr(feature = "cargo-clippy", allow(clippy::type_complexity))]
pub(crate) fn common<M: Measurement>(
    id: &BenchmarkId,
    avg_times: &Sample<f64>,
    config: &BenchmarkConfig,
    criterion: &Criterion<M>,
) -> Result<(
    f64,
    Distribution<f64>,
    ChangeEstimates,
    ChangeDistributions,
    Vec<f64>,
    Vec<f64>,
    Vec<f64>,
    Estimates,
)> {
    let mut sample_file = criterion.output_directory.clone();
    sample_file.push(id.as_directory_name());
    sample_file.push(&criterion.baseline_directory);
    sample_file.push("sample.json");
    let sample: SavedSample = fs::load(&sample_file)?;
    let SavedSample { iters, times, .. } = sample;

    let mut estimates_file = criterion.output_directory.clone();
    estimates_file.push(id.as_directory_name());
    estimates_file.push(&criterion.baseline_directory);
    estimates_file.push("estimates.json");
    let base_estimates: Estimates = fs::load(&estimates_file)?;

    let base_avg_times: Vec<f64> = iters
        .iter()
        .zip(times.iter())
        .map(|(iters, elapsed)| elapsed / iters)
        .collect();
    let base_avg_time_sample = Sample::new(&base_avg_times);

    let mut change_dir = criterion.output_directory.clone();
    change_dir.push(id.as_directory_name());
    change_dir.push("change");
    fs::mkdirp(&change_dir)?;
    let (t_statistic, t_distribution) = t_test(avg_times, base_avg_time_sample, config);

    let (estimates, relative_distributions) =
        estimates(id, avg_times, base_avg_time_sample, config, criterion);
    Ok((
        t_statistic,
        t_distribution,
        estimates,
        relative_distributions,
        iters,
        times,
        base_avg_times.clone(),
        base_estimates,
    ))
}

// Performs a two sample t-test
fn t_test(
    avg_times: &Sample<f64>,
    base_avg_times: &Sample<f64>,
    config: &BenchmarkConfig,
) -> (f64, Distribution<f64>) {
    let nresamples = config.nresamples;

    let t_statistic = avg_times.t(base_avg_times);
    let t_distribution = elapsed!(
        "Bootstrapping the T distribution",
        mixed::bootstrap(avg_times, base_avg_times, nresamples, |a, b| (a.t(b),))
    )
    .0;

    // HACK: Filter out non-finite numbers, which can happen sometimes when sample size is very small.
    // Downstream code doesn't like non-finite values here.
    let t_distribution = Distribution::from(
        t_distribution
            .iter()
            .filter(|a| a.is_finite())
            .cloned()
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    );

    (t_statistic, t_distribution)
}

// Estimates the relative change in the statistics of the population
fn estimates<M: Measurement>(
    id: &BenchmarkId,
    avg_times: &Sample<f64>,
    base_avg_times: &Sample<f64>,
    config: &BenchmarkConfig,
    criterion: &Criterion<M>,
) -> (ChangeEstimates, ChangeDistributions) {
    fn stats(a: &Sample<f64>, b: &Sample<f64>) -> (f64, f64) {
        (
            a.mean() / b.mean() - 1.,
            a.percentiles().median() / b.percentiles().median() - 1.,
        )
    }

    let cl = config.confidence_level;
    let nresamples = config.nresamples;

    let (dist_mean, dist_median) = elapsed!(
        "Bootstrapping the relative statistics",
        univariate::bootstrap(avg_times, base_avg_times, nresamples, stats)
    );

    let distributions = ChangeDistributions {
        mean: dist_mean,
        median: dist_median,
    };

    let (mean, median) = stats(avg_times, base_avg_times);
    let points = ChangePointEstimates { mean, median };

    let estimates = build_change_estimates(&distributions, &points, cl);

    {
        log_if_err!({
            let mut estimates_path = criterion.output_directory.clone();
            estimates_path.push(id.as_directory_name());
            estimates_path.push("change");
            estimates_path.push("estimates.json");
            fs::save(&estimates, &estimates_path)
        });
    }
    (estimates, distributions)
}

use std::path::Path;

use crate::stats::bivariate::regression::Slope;
use crate::stats::bivariate::Data;
use crate::stats::univariate::outliers::tukey;
use crate::stats::univariate::Sample;
use crate::stats::{Distribution, Tails};

use crate::benchmark::BenchmarkConfig;
use crate::connection::OutgoingMessage;
use crate::estimate::{
    build_estimates, ConfidenceInterval, Distributions, Estimate, Estimates, PointEstimates,
};
use crate::fs;
use crate::measurement::Measurement;
use crate::report::{BenchmarkId, Report, ReportContext};
use crate::routine::Routine;
use crate::{Baseline, Criterion, SavedSample, Throughput};

macro_rules! elapsed {
    ($msg:expr, $block:expr) => {{
        let start = ::std::time::Instant::now();
        let out = $block;
        let elapsed = &start.elapsed();

        info!(
            "{} took {}",
            $msg,
            crate::format::time(elapsed.as_nanos() as f64)
        );

        out
    }};
}

mod compare;

// Common analysis procedure
pub(crate) fn common<M: Measurement, T: ?Sized>(
    id: &BenchmarkId,
    routine: &mut dyn Routine<M, T>,
    config: &BenchmarkConfig,
    criterion: &Criterion<M>,
    report_context: &ReportContext,
    parameter: &T,
    throughput: Option<Throughput>,
) {
    criterion.report.benchmark_start(id, report_context);

    if let Baseline::CompareStrict = criterion.baseline {
        if !base_dir_exists(
            id,
            &criterion.baseline_directory,
            &criterion.output_directory,
        ) {
            panic!(
                "Baseline '{base}' must exist before comparison is allowed; try --save-baseline {base}",
                base=criterion.baseline_directory,
            );
        }
    }

    let (sampling_mode, iters, times);
    if let Some(baseline) = &criterion.load_baseline {
        let mut sample_path = criterion.output_directory.clone();
        sample_path.push(id.as_directory_name());
        sample_path.push(baseline);
        sample_path.push("sample.json");
        let loaded = fs::load::<SavedSample, _>(&sample_path);

        match loaded {
            Err(err) => panic!(
                "Baseline '{base}' must exist before it can be loaded; try --save-baseline {base}. Error: {err}",
                base = baseline, err = err
            ),
            Ok(samples) => {
                sampling_mode = samples.sampling_mode;
                iters = samples.iters.into_boxed_slice();
                times = samples.times.into_boxed_slice();
            }
        }
    } else {
        let sample = routine.sample(
            &criterion.measurement,
            id,
            config,
            criterion,
            report_context,
            parameter,
        );
        sampling_mode = sample.0;
        iters = sample.1;
        times = sample.2;

        if let Some(conn) = &criterion.connection {
            conn.send(&OutgoingMessage::MeasurementComplete {
                id: id.into(),
                iters: &iters,
                times: &times,
                plot_config: (&report_context.plot_config).into(),
                sampling_method: sampling_mode.into(),
                benchmark_config: config.into(),
            })
            .unwrap();

            conn.serve_value_formatter(criterion.measurement.formatter())
                .unwrap();
            return;
        }
    }

    criterion.report.analysis(id, report_context);

    if times.iter().any(|&f| f == 0.0) {
        error!(
            "At least one measurement of benchmark {} took zero time per \
            iteration. This should not be possible. If using iter_custom, please verify \
            that your routine is correctly measured.",
            id.as_title()
        );
        return;
    }

    let avg_times = iters
        .iter()
        .zip(times.iter())
        .map(|(&iters, &elapsed)| elapsed / iters)
        .collect::<Vec<f64>>();
    let avg_times = Sample::new(&avg_times);

    if criterion.should_save_baseline() {
        log_if_err!({
            let mut new_dir = criterion.output_directory.clone();
            new_dir.push(id.as_directory_name());
            new_dir.push("new");
            fs::mkdirp(&new_dir)
        });
    }

    let data = Data::new(&iters, &times);
    let labeled_sample = tukey::classify(avg_times);
    if criterion.should_save_baseline() {
        log_if_err!({
            let mut tukey_file = criterion.output_directory.to_owned();
            tukey_file.push(id.as_directory_name());
            tukey_file.push("new");
            tukey_file.push("tukey.json");
            fs::save(&labeled_sample.fences(), &tukey_file)
        });
    }
    let (mut distributions, mut estimates) = estimates(avg_times, config);
    if sampling_mode.is_linear() {
        let (distribution, slope) = regression(&data, config);

        estimates.slope = Some(slope);
        distributions.slope = Some(distribution);
    }

    if criterion.should_save_baseline() {
        log_if_err!({
            let mut sample_file = criterion.output_directory.clone();
            sample_file.push(id.as_directory_name());
            sample_file.push("new");
            sample_file.push("sample.json");
            fs::save(
                &SavedSample {
                    sampling_mode,
                    iters: data.x().as_ref().to_vec(),
                    times: data.y().as_ref().to_vec(),
                },
                &sample_file,
            )
        });
        log_if_err!({
            let mut estimates_file = criterion.output_directory.clone();
            estimates_file.push(id.as_directory_name());
            estimates_file.push("new");
            estimates_file.push("estimates.json");
            fs::save(&estimates, &estimates_file)
        });
    }

    let compare_data = if base_dir_exists(
        id,
        &criterion.baseline_directory,
        &criterion.output_directory,
    ) {
        let result = compare::common(id, avg_times, config, criterion);
        match result {
            Ok((
                t_value,
                t_distribution,
                relative_estimates,
                relative_distributions,
                base_iter_counts,
                base_sample_times,
                base_avg_times,
                base_estimates,
            )) => {
                let p_value = t_distribution.p_value(t_value, &Tails::Two);
                Some(crate::report::ComparisonData {
                    p_value,
                    t_distribution,
                    t_value,
                    relative_estimates,
                    relative_distributions,
                    significance_threshold: config.significance_level,
                    noise_threshold: config.noise_threshold,
                    base_iter_counts,
                    base_sample_times,
                    base_avg_times,
                    base_estimates,
                })
            }
            Err(e) => {
                crate::error::log_error(&e);
                None
            }
        }
    } else {
        None
    };

    let measurement_data = crate::report::MeasurementData {
        data: Data::new(&iters, &times),
        avg_times: labeled_sample,
        absolute_estimates: estimates,
        distributions,
        comparison: compare_data,
        throughput,
    };

    criterion.report.measurement_complete(
        id,
        report_context,
        &measurement_data,
        criterion.measurement.formatter(),
    );

    if criterion.should_save_baseline() {
        log_if_err!({
            let mut benchmark_file = criterion.output_directory.clone();
            benchmark_file.push(id.as_directory_name());
            benchmark_file.push("new");
            benchmark_file.push("benchmark.json");
            fs::save(&id, &benchmark_file)
        });
    }

    if criterion.connection.is_none() {
        if let Baseline::Save = criterion.baseline {
            copy_new_dir_to_base(
                id.as_directory_name(),
                &criterion.baseline_directory,
                &criterion.output_directory,
            );
        }
    }
}

fn base_dir_exists(id: &BenchmarkId, baseline: &str, output_directory: &Path) -> bool {
    let mut base_dir = output_directory.to_owned();
    base_dir.push(id.as_directory_name());
    base_dir.push(baseline);
    base_dir.exists()
}

// Performs a simple linear regression on the sample
fn regression(
    data: &Data<'_, f64, f64>,
    config: &BenchmarkConfig,
) -> (Distribution<f64>, Estimate) {
    let cl = config.confidence_level;

    let distribution = elapsed!(
        "Bootstrapped linear regression",
        data.bootstrap(config.nresamples, |d| (Slope::fit(&d).0,))
    )
    .0;

    let point = Slope::fit(data);
    let (lb, ub) = distribution.confidence_interval(config.confidence_level);
    let se = distribution.std_dev(None);

    (
        distribution,
        Estimate {
            confidence_interval: ConfidenceInterval {
                confidence_level: cl,
                lower_bound: lb,
                upper_bound: ub,
            },
            point_estimate: point.0,
            standard_error: se,
        },
    )
}

// Estimates the statistics of the population from the sample
fn estimates(avg_times: &Sample<f64>, config: &BenchmarkConfig) -> (Distributions, Estimates) {
    fn stats(sample: &Sample<f64>) -> (f64, f64, f64, f64) {
        let mean = sample.mean();
        let std_dev = sample.std_dev(Some(mean));
        let median = sample.percentiles().median();
        let mad = sample.median_abs_dev(Some(median));

        (mean, std_dev, median, mad)
    }

    let cl = config.confidence_level;
    let nresamples = config.nresamples;

    let (mean, std_dev, median, mad) = stats(avg_times);
    let points = PointEstimates {
        mean,
        median,
        std_dev,
        median_abs_dev: mad,
    };

    let (dist_mean, dist_stddev, dist_median, dist_mad) = elapsed!(
        "Bootstrapping the absolute statistics.",
        avg_times.bootstrap(nresamples, stats)
    );

    let distributions = Distributions {
        mean: dist_mean,
        slope: None,
        median: dist_median,
        median_abs_dev: dist_mad,
        std_dev: dist_stddev,
    };

    let estimates = build_estimates(&distributions, &points, cl);

    (distributions, estimates)
}

fn copy_new_dir_to_base(id: &str, baseline: &str, output_directory: &Path) {
    let root_dir = Path::new(output_directory).join(id);
    let base_dir = root_dir.join(baseline);
    let new_dir = root_dir.join("new");

    if !new_dir.exists() {
        return;
    };
    if !base_dir.exists() {
        try_else_return!(fs::mkdirp(&base_dir));
    }

    // TODO: consider using walkdir or similar to generically copy.
    try_else_return!(fs::cp(
        &new_dir.join("estimates.json"),
        &base_dir.join("estimates.json")
    ));
    try_else_return!(fs::cp(
        &new_dir.join("sample.json"),
        &base_dir.join("sample.json")
    ));
    try_else_return!(fs::cp(
        &new_dir.join("tukey.json"),
        &base_dir.join("tukey.json")
    ));
    try_else_return!(fs::cp(
        &new_dir.join("benchmark.json"),
        &base_dir.join("benchmark.json")
    ));
    #[cfg(feature = "csv_output")]
    try_else_return!(fs::cp(&new_dir.join("raw.csv"), &base_dir.join("raw.csv")));
}

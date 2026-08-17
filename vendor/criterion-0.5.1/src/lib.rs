//! A statistics-driven micro-benchmarking library written in Rust.
//!
//! This crate is a microbenchmarking library which aims to provide strong
//! statistical confidence in detecting and estimating the size of performance
//! improvements and regressions, while also being easy to use.
//!
//! See
//! [the user guide](https://bheisler.github.io/criterion.rs/book/index.html)
//! for examples as well as details on the measurement and analysis process,
//! and the output.
//!
//! ## Features:
//! * Collects detailed statistics, providing strong confidence that changes
//!   to performance are real, not measurement noise.
//! * Produces detailed charts, providing thorough understanding of your code's
//!   performance behavior.

#![warn(missing_docs)]
#![warn(bare_trait_objects)]
#![cfg_attr(feature = "real_blackbox", feature(test))]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(
        clippy::just_underscores_and_digits, // Used in the stats code
        clippy::transmute_ptr_to_ptr, // Used in the stats code
        clippy::manual_non_exhaustive, // Remove when MSRV bumped above 1.40
    )
)]

#[cfg(all(feature = "rayon", target_arch = "wasm32"))]
compile_error!("Rayon cannot be used when targeting wasi32. Try disabling default features.");

#[cfg(test)]
extern crate approx;

#[cfg(test)]
extern crate quickcheck;

use is_terminal::IsTerminal;
use regex::Regex;

#[cfg(feature = "real_blackbox")]
extern crate test;

#[macro_use]
extern crate serde_derive;

// Needs to be declared before other modules
// in order to be usable there.
#[macro_use]
mod macros_private;
#[macro_use]
mod analysis;
mod benchmark;
#[macro_use]
mod benchmark_group;
pub mod async_executor;
mod bencher;
mod connection;
#[cfg(feature = "csv_output")]
mod csv_report;
mod error;
mod estimate;
mod format;
mod fs;
mod html;
mod kde;
mod macros;
pub mod measurement;
mod plot;
pub mod profiler;
mod report;
mod routine;
mod stats;

use std::cell::RefCell;
use std::collections::HashSet;
use std::default::Default;
use std::env;
use std::io::stdout;
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, MutexGuard};
use std::time::Duration;

use criterion_plot::{Version, VersionError};
use once_cell::sync::Lazy;

use crate::benchmark::BenchmarkConfig;
use crate::connection::Connection;
use crate::connection::OutgoingMessage;
use crate::html::Html;
use crate::measurement::{Measurement, WallTime};
#[cfg(feature = "plotters")]
use crate::plot::PlottersBackend;
use crate::plot::{Gnuplot, Plotter};
use crate::profiler::{ExternalProfiler, Profiler};
use crate::report::{BencherReport, CliReport, CliVerbosity, Report, ReportContext, Reports};

#[cfg(feature = "async")]
pub use crate::bencher::AsyncBencher;
pub use crate::bencher::Bencher;
pub use crate::benchmark_group::{BenchmarkGroup, BenchmarkId};

static DEBUG_ENABLED: Lazy<bool> = Lazy::new(|| std::env::var_os("CRITERION_DEBUG").is_some());
static GNUPLOT_VERSION: Lazy<Result<Version, VersionError>> = Lazy::new(criterion_plot::version);
static DEFAULT_PLOTTING_BACKEND: Lazy<PlottingBackend> = Lazy::new(|| match &*GNUPLOT_VERSION {
    Ok(_) => PlottingBackend::Gnuplot,
    #[cfg(feature = "plotters")]
    Err(e) => {
        match e {
            VersionError::Exec(_) => eprintln!("Gnuplot not found, using plotters backend"),
            e => eprintln!(
                "Gnuplot not found or not usable, using plotters backend\n{}",
                e
            ),
        };
        PlottingBackend::Plotters
    }
    #[cfg(not(feature = "plotters"))]
    Err(_) => PlottingBackend::None,
});
static CARGO_CRITERION_CONNECTION: Lazy<Option<Mutex<Connection>>> =
    Lazy::new(|| match std::env::var("CARGO_CRITERION_PORT") {
        Ok(port_str) => {
            let port: u16 = port_str.parse().ok()?;
            let stream = TcpStream::connect(("localhost", port)).ok()?;
            Some(Mutex::new(Connection::new(stream).ok()?))
        }
        Err(_) => None,
    });
static DEFAULT_OUTPUT_DIRECTORY: Lazy<PathBuf> = Lazy::new(|| {
    // Set criterion home to (in descending order of preference):
    // - $CRITERION_HOME (cargo-criterion sets this, but other users could as well)
    // - $CARGO_TARGET_DIR/criterion
    // - the cargo target dir from `cargo metadata`
    // - ./target/criterion
    if let Some(value) = env::var_os("CRITERION_HOME") {
        PathBuf::from(value)
    } else if let Some(path) = cargo_target_directory() {
        path.join("criterion")
    } else {
        PathBuf::from("target/criterion")
    }
});

fn debug_enabled() -> bool {
    *DEBUG_ENABLED
}

/// A function that is opaque to the optimizer, used to prevent the compiler from
/// optimizing away computations in a benchmark.
///
/// This variant is backed by the (unstable) test::black_box function.
#[cfg(feature = "real_blackbox")]
pub fn black_box<T>(dummy: T) -> T {
    test::black_box(dummy)
}

/// A function that is opaque to the optimizer, used to prevent the compiler from
/// optimizing away computations in a benchmark.
///
/// This variant is stable-compatible, but it may cause some performance overhead
/// or fail to prevent code from being eliminated.
#[cfg(not(feature = "real_blackbox"))]
pub fn black_box<T>(dummy: T) -> T {
    unsafe {
        let ret = std::ptr::read_volatile(&dummy);
        std::mem::forget(dummy);
        ret
    }
}

/// Argument to [`Bencher::iter_batched`](struct.Bencher.html#method.iter_batched) and
/// [`Bencher::iter_batched_ref`](struct.Bencher.html#method.iter_batched_ref) which controls the
/// batch size.
///
/// Generally speaking, almost all benchmarks should use `SmallInput`. If the input or the result
/// of the benchmark routine is large enough that `SmallInput` causes out-of-memory errors,
/// `LargeInput` can be used to reduce memory usage at the cost of increasing the measurement
/// overhead. If the input or the result is extremely large (or if it holds some
/// limited external resource like a file handle), `PerIteration` will set the number of iterations
/// per batch to exactly one. `PerIteration` can increase the measurement overhead substantially
/// and should be avoided wherever possible.
///
/// Each value lists an estimate of the measurement overhead. This is intended as a rough guide
/// to assist in choosing an option, it should not be relied upon. In particular, it is not valid
/// to subtract the listed overhead from the measurement and assume that the result represents the
/// true runtime of a function. The actual measurement overhead for your specific benchmark depends
/// on the details of the function you're benchmarking and the hardware and operating
/// system running the benchmark.
///
/// With that said, if the runtime of your function is small relative to the measurement overhead
/// it will be difficult to take accurate measurements. In this situation, the best option is to use
/// [`Bencher::iter`](struct.Bencher.html#method.iter) which has next-to-zero measurement overhead.
#[derive(Debug, Eq, PartialEq, Copy, Hash, Clone)]
pub enum BatchSize {
    /// `SmallInput` indicates that the input to the benchmark routine (the value returned from
    /// the setup routine) is small enough that millions of values can be safely held in memory.
    /// Always prefer `SmallInput` unless the benchmark is using too much memory.
    ///
    /// In testing, the maximum measurement overhead from benchmarking with `SmallInput` is on the
    /// order of 500 picoseconds. This is presented as a rough guide; your results may vary.
    SmallInput,

    /// `LargeInput` indicates that the input to the benchmark routine or the value returned from
    /// that routine is large. This will reduce the memory usage but increase the measurement
    /// overhead.
    ///
    /// In testing, the maximum measurement overhead from benchmarking with `LargeInput` is on the
    /// order of 750 picoseconds. This is presented as a rough guide; your results may vary.
    LargeInput,

    /// `PerIteration` indicates that the input to the benchmark routine or the value returned from
    /// that routine is extremely large or holds some limited resource, such that holding many values
    /// in memory at once is infeasible. This provides the worst measurement overhead, but the
    /// lowest memory usage.
    ///
    /// In testing, the maximum measurement overhead from benchmarking with `PerIteration` is on the
    /// order of 350 nanoseconds or 350,000 picoseconds. This is presented as a rough guide; your
    /// results may vary.
    PerIteration,

    /// `NumBatches` will attempt to divide the iterations up into a given number of batches.
    /// A larger number of batches (and thus smaller batches) will reduce memory usage but increase
    /// measurement overhead. This allows the user to choose their own tradeoff between memory usage
    /// and measurement overhead, but care must be taken in tuning the number of batches. Most
    /// benchmarks should use `SmallInput` or `LargeInput` instead.
    NumBatches(u64),

    /// `NumIterations` fixes the batch size to a constant number, specified by the user. This
    /// allows the user to choose their own tradeoff between overhead and memory usage, but care must
    /// be taken in tuning the batch size. In general, the measurement overhead of `NumIterations`
    /// will be larger than that of `NumBatches`. Most benchmarks should use `SmallInput` or
    /// `LargeInput` instead.
    NumIterations(u64),

    #[doc(hidden)]
    __NonExhaustive,
}
impl BatchSize {
    /// Convert to a number of iterations per batch.
    ///
    /// We try to do a constant number of batches regardless of the number of iterations in this
    /// sample. If the measurement overhead is roughly constant regardless of the number of
    /// iterations the analysis of the results later will have an easier time separating the
    /// measurement overhead from the benchmark time.
    fn iters_per_batch(self, iters: u64) -> u64 {
        match self {
            BatchSize::SmallInput => (iters + 10 - 1) / 10,
            BatchSize::LargeInput => (iters + 1000 - 1) / 1000,
            BatchSize::PerIteration => 1,
            BatchSize::NumBatches(batches) => (iters + batches - 1) / batches,
            BatchSize::NumIterations(size) => size,
            BatchSize::__NonExhaustive => panic!("__NonExhaustive is not a valid BatchSize."),
        }
    }
}

/// Baseline describes how the baseline_directory is handled.
#[derive(Debug, Clone, Copy)]
pub enum Baseline {
    /// CompareLenient compares against a previous saved version of the baseline.
    /// If a previous baseline does not exist, the benchmark is run as normal but no comparison occurs.
    CompareLenient,
    /// CompareStrict compares against a previous saved version of the baseline.
    /// If a previous baseline does not exist, a panic occurs.
    CompareStrict,
    /// Save writes the benchmark results to the baseline directory,
    /// overwriting any results that were previously there.
    Save,
    /// Discard benchmark results.
    Discard,
}

/// Enum used to select the plotting backend.
#[derive(Debug, Clone, Copy)]
pub enum PlottingBackend {
    /// Plotting backend which uses the external `gnuplot` command to render plots. This is the
    /// default if the `gnuplot` command is installed.
    Gnuplot,
    /// Plotting backend which uses the rust 'Plotters' library. This is the default if `gnuplot`
    /// is not installed.
    Plotters,
    /// Null plotting backend which outputs nothing,
    None,
}
impl PlottingBackend {
    fn create_plotter(&self) -> Option<Box<dyn Plotter>> {
        match self {
            PlottingBackend::Gnuplot => Some(Box::<Gnuplot>::default()),
            #[cfg(feature = "plotters")]
            PlottingBackend::Plotters => Some(Box::<PlottersBackend>::default()),
            #[cfg(not(feature = "plotters"))]
            PlottingBackend::Plotters => panic!("Criterion was built without plotters support."),
            PlottingBackend::None => None,
        }
    }
}

#[derive(Debug, Clone)]
/// Enum representing the execution mode.
pub(crate) enum Mode {
    /// Run benchmarks normally.
    Benchmark,
    /// List all benchmarks but do not run them.
    List(ListFormat),
    /// Run benchmarks once to verify that they work, but otherwise do not measure them.
    Test,
    /// Iterate benchmarks for a given length of time but do not analyze or report on them.
    Profile(Duration),
}
impl Mode {
    pub fn is_benchmark(&self) -> bool {
        matches!(self, Mode::Benchmark)
    }

    pub fn is_terse(&self) -> bool {
        matches!(self, Mode::List(ListFormat::Terse))
    }
}

#[derive(Debug, Clone)]
/// Enum representing the list format.
pub(crate) enum ListFormat {
    /// The regular, default format.
    Pretty,
    /// The terse format, where nothing other than the name of the test and ": benchmark" at the end
    /// is printed out.
    Terse,
}

impl Default for ListFormat {
    fn default() -> Self {
        Self::Pretty
    }
}

/// Benchmark filtering support.
#[derive(Clone, Debug)]
pub enum BenchmarkFilter {
    /// Run all benchmarks.
    AcceptAll,
    /// Run benchmarks matching this regex.
    Regex(Regex),
    /// Run the benchmark matching this string exactly.
    Exact(String),
    /// Do not run any benchmarks.
    RejectAll,
}

/// The benchmark manager
///
/// `Criterion` lets you configure and execute benchmarks
///
/// Each benchmark consists of four phases:
///
/// - **Warm-up**: The routine is repeatedly executed, to let the CPU/OS/JIT/interpreter adapt to
/// the new load
/// - **Measurement**: The routine is repeatedly executed, and timing information is collected into
/// a sample
/// - **Analysis**: The sample is analyzed and distilled into meaningful statistics that get
/// reported to stdout, stored in files, and plotted
/// - **Comparison**: The current sample is compared with the sample obtained in the previous
/// benchmark.
pub struct Criterion<M: Measurement = WallTime> {
    config: BenchmarkConfig,
    filter: BenchmarkFilter,
    report: Reports,
    output_directory: PathBuf,
    baseline_directory: String,
    baseline: Baseline,
    load_baseline: Option<String>,
    all_directories: HashSet<String>,
    all_titles: HashSet<String>,
    measurement: M,
    profiler: Box<RefCell<dyn Profiler>>,
    connection: Option<MutexGuard<'static, Connection>>,
    mode: Mode,
}

/// Returns the Cargo target directory, possibly calling `cargo metadata` to
/// figure it out.
fn cargo_target_directory() -> Option<PathBuf> {
    #[derive(Deserialize)]
    struct Metadata {
        target_directory: PathBuf,
    }

    env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .or_else(|| {
            let output = Command::new(env::var_os("CARGO")?)
                .args(["metadata", "--format-version", "1"])
                .output()
                .ok()?;
            let metadata: Metadata = serde_json::from_slice(&output.stdout).ok()?;
            Some(metadata.target_directory)
        })
}

impl Default for Criterion {
    /// Creates a benchmark manager with the following default settings:
    ///
    /// - Sample size: 100 measurements
    /// - Warm-up time: 3 s
    /// - Measurement time: 5 s
    /// - Bootstrap size: 100 000 resamples
    /// - Noise threshold: 0.01 (1%)
    /// - Confidence level: 0.95
    /// - Significance level: 0.05
    /// - Plotting: enabled, using gnuplot if available or plotters if gnuplot is not available
    /// - No filter
    fn default() -> Criterion {
        let reports = Reports {
            cli_enabled: true,
            cli: CliReport::new(false, false, CliVerbosity::Normal),
            bencher_enabled: false,
            bencher: BencherReport,
            html: DEFAULT_PLOTTING_BACKEND.create_plotter().map(Html::new),
            csv_enabled: cfg!(feature = "csv_output"),
        };

        let mut criterion = Criterion {
            config: BenchmarkConfig {
                confidence_level: 0.95,
                measurement_time: Duration::from_secs(5),
                noise_threshold: 0.01,
                nresamples: 100_000,
                sample_size: 100,
                significance_level: 0.05,
                warm_up_time: Duration::from_secs(3),
                sampling_mode: SamplingMode::Auto,
                quick_mode: false,
            },
            filter: BenchmarkFilter::AcceptAll,
            report: reports,
            baseline_directory: "base".to_owned(),
            baseline: Baseline::Save,
            load_baseline: None,
            output_directory: DEFAULT_OUTPUT_DIRECTORY.clone(),
            all_directories: HashSet::new(),
            all_titles: HashSet::new(),
            measurement: WallTime,
            profiler: Box::new(RefCell::new(ExternalProfiler)),
            connection: CARGO_CRITERION_CONNECTION
                .as_ref()
                .map(|mtx| mtx.lock().unwrap()),
            mode: Mode::Benchmark,
        };

        if criterion.connection.is_some() {
            // disable all reports when connected to cargo-criterion; it will do the reporting.
            criterion.report.cli_enabled = false;
            criterion.report.bencher_enabled = false;
            criterion.report.csv_enabled = false;
            criterion.report.html = None;
        }
        criterion
    }
}

impl<M: Measurement> Criterion<M> {
    /// Changes the measurement for the benchmarks run with this runner. See the
    /// Measurement trait for more details
    pub fn with_measurement<M2: Measurement>(self, m: M2) -> Criterion<M2> {
        // Can't use struct update syntax here because they're technically different types.
        Criterion {
            config: self.config,
            filter: self.filter,
            report: self.report,
            baseline_directory: self.baseline_directory,
            baseline: self.baseline,
            load_baseline: self.load_baseline,
            output_directory: self.output_directory,
            all_directories: self.all_directories,
            all_titles: self.all_titles,
            measurement: m,
            profiler: self.profiler,
            connection: self.connection,
            mode: self.mode,
        }
    }

    #[must_use]
    /// Changes the internal profiler for benchmarks run with this runner. See
    /// the Profiler trait for more details.
    pub fn with_profiler<P: Profiler + 'static>(self, p: P) -> Criterion<M> {
        Criterion {
            profiler: Box::new(RefCell::new(p)),
            ..self
        }
    }

    #[must_use]
    /// Set the plotting backend. By default, Criterion will use gnuplot if available, or plotters
    /// if not.
    ///
    /// Panics if `backend` is `PlottingBackend::Gnuplot` and gnuplot is not available.
    pub fn plotting_backend(mut self, backend: PlottingBackend) -> Criterion<M> {
        if let PlottingBackend::Gnuplot = backend {
            assert!(
                !GNUPLOT_VERSION.is_err(),
                "Gnuplot plotting backend was requested, but gnuplot is not available. \
                To continue, either install Gnuplot or allow Criterion.rs to fall back \
                to using plotters."
            );
        }

        self.report.html = backend.create_plotter().map(Html::new);
        self
    }

    #[must_use]
    /// Changes the default size of the sample for benchmarks run with this runner.
    ///
    /// A bigger sample should yield more accurate results if paired with a sufficiently large
    /// measurement time.
    ///
    /// Sample size must be at least 10.
    ///
    /// # Panics
    ///
    /// Panics if n < 10
    pub fn sample_size(mut self, n: usize) -> Criterion<M> {
        assert!(n >= 10);

        self.config.sample_size = n;
        self
    }

    #[must_use]
    /// Changes the default warm up time for benchmarks run with this runner.
    ///
    /// # Panics
    ///
    /// Panics if the input duration is zero
    pub fn warm_up_time(mut self, dur: Duration) -> Criterion<M> {
        assert!(dur.as_nanos() > 0);

        self.config.warm_up_time = dur;
        self
    }

    #[must_use]
    /// Changes the default measurement time for benchmarks run with this runner.
    ///
    /// With a longer time, the measurement will become more resilient to transitory peak loads
    /// caused by external programs
    ///
    /// **Note**: If the measurement time is too "low", Criterion will automatically increase it
    ///
    /// # Panics
    ///
    /// Panics if the input duration in zero
    pub fn measurement_time(mut self, dur: Duration) -> Criterion<M> {
        assert!(dur.as_nanos() > 0);

        self.config.measurement_time = dur;
        self
    }

    #[must_use]
    /// Changes the default number of resamples for benchmarks run with this runner.
    ///
    /// Number of resamples to use for the
    /// [bootstrap](http://en.wikipedia.org/wiki/Bootstrapping_(statistics)#Case_resampling)
    ///
    /// A larger number of resamples reduces the random sampling errors, which are inherent to the
    /// bootstrap method, but also increases the analysis time
    ///
    /// # Panics
    ///
    /// Panics if the number of resamples is set to zero
    pub fn nresamples(mut self, n: usize) -> Criterion<M> {
        assert!(n > 0);
        if n <= 1000 {
            eprintln!("\nWarning: It is not recommended to reduce nresamples below 1000.");
        }

        self.config.nresamples = n;
        self
    }

    #[must_use]
    /// Changes the default noise threshold for benchmarks run with this runner. The noise threshold
    /// is used to filter out small changes in performance, even if they are statistically
    /// significant. Sometimes benchmarking the same code twice will result in small but
    /// statistically significant differences solely because of noise. This provides a way to filter
    /// out some of these false positives at the cost of making it harder to detect small changes
    /// to the true performance of the benchmark.
    ///
    /// The default is 0.01, meaning that changes smaller than 1% will be ignored.
    ///
    /// # Panics
    ///
    /// Panics if the threshold is set to a negative value
    pub fn noise_threshold(mut self, threshold: f64) -> Criterion<M> {
        assert!(threshold >= 0.0);

        self.config.noise_threshold = threshold;
        self
    }

    #[must_use]
    /// Changes the default confidence level for benchmarks run with this runner. The confidence
    /// level is the desired probability that the true runtime lies within the estimated
    /// [confidence interval](https://en.wikipedia.org/wiki/Confidence_interval). The default is
    /// 0.95, meaning that the confidence interval should capture the true value 95% of the time.
    ///
    /// # Panics
    ///
    /// Panics if the confidence level is set to a value outside the `(0, 1)` range
    pub fn confidence_level(mut self, cl: f64) -> Criterion<M> {
        assert!(cl > 0.0 && cl < 1.0);
        if cl < 0.5 {
            eprintln!("\nWarning: It is not recommended to reduce confidence level below 0.5.");
        }

        self.config.confidence_level = cl;
        self
    }

    #[must_use]
    /// Changes the default [significance level](https://en.wikipedia.org/wiki/Statistical_significance)
    /// for benchmarks run with this runner. This is used to perform a
    /// [hypothesis test](https://en.wikipedia.org/wiki/Statistical_hypothesis_testing) to see if
    /// the measurements from this run are different from the measured performance of the last run.
    /// The significance level is the desired probability that two measurements of identical code
    /// will be considered 'different' due to noise in the measurements. The default value is 0.05,
    /// meaning that approximately 5% of identical benchmarks will register as different due to
    /// noise.
    ///
    /// This presents a trade-off. By setting the significance level closer to 0.0, you can increase
    /// the statistical robustness against noise, but it also weakens Criterion.rs' ability to
    /// detect small but real changes in the performance. By setting the significance level
    /// closer to 1.0, Criterion.rs will be more able to detect small true changes, but will also
    /// report more spurious differences.
    ///
    /// See also the noise threshold setting.
    ///
    /// # Panics
    ///
    /// Panics if the significance level is set to a value outside the `(0, 1)` range
    pub fn significance_level(mut self, sl: f64) -> Criterion<M> {
        assert!(sl > 0.0 && sl < 1.0);

        self.config.significance_level = sl;
        self
    }

    #[must_use]
    /// Enables plotting
    pub fn with_plots(mut self) -> Criterion<M> {
        // If running under cargo-criterion then don't re-enable the reports; let it do the reporting.
        if self.connection.is_none() && self.report.html.is_none() {
            let default_backend = DEFAULT_PLOTTING_BACKEND.create_plotter();
            if let Some(backend) = default_backend {
                self.report.html = Some(Html::new(backend));
            } else {
                panic!("Cannot find a default plotting backend!");
            }
        }
        self
    }

    #[must_use]
    /// Disables plotting
    pub fn without_plots(mut self) -> Criterion<M> {
        self.report.html = None;
        self
    }

    #[must_use]
    /// Names an explicit baseline and enables overwriting the previous results.
    pub fn save_baseline(mut self, baseline: String) -> Criterion<M> {
        self.baseline_directory = baseline;
        self.baseline = Baseline::Save;
        self
    }

    #[must_use]
    /// Names an explicit baseline and disables overwriting the previous results.
    pub fn retain_baseline(mut self, baseline: String, strict: bool) -> Criterion<M> {
        self.baseline_directory = baseline;
        self.baseline = if strict {
            Baseline::CompareStrict
        } else {
            Baseline::CompareLenient
        };
        self
    }

    #[must_use]
    /// Filters the benchmarks. Only benchmarks with names that contain the
    /// given string will be executed.
    ///
    /// This overwrites [`Self::with_benchmark_filter`].
    pub fn with_filter<S: Into<String>>(mut self, filter: S) -> Criterion<M> {
        let filter_text = filter.into();
        let filter = Regex::new(&filter_text).unwrap_or_else(|err| {
            panic!(
                "Unable to parse '{}' as a regular expression: {}",
                filter_text, err
            )
        });
        self.filter = BenchmarkFilter::Regex(filter);

        self
    }

    /// Only run benchmarks specified by the given filter.
    ///
    /// This overwrites [`Self::with_filter`].
    pub fn with_benchmark_filter(mut self, filter: BenchmarkFilter) -> Criterion<M> {
        self.filter = filter;

        self
    }

    #[must_use]
    /// Override whether the CLI output will be colored or not. Usually you would use the `--color`
    /// CLI argument, but this is available for programmmatic use as well.
    pub fn with_output_color(mut self, enabled: bool) -> Criterion<M> {
        self.report.cli.enable_text_coloring = enabled;
        self
    }

    /// Set the output directory (currently for testing only)
    #[must_use]
    #[doc(hidden)]
    pub fn output_directory(mut self, path: &Path) -> Criterion<M> {
        self.output_directory = path.to_owned();

        self
    }

    /// Set the profile time (currently for testing only)
    #[must_use]
    #[doc(hidden)]
    pub fn profile_time(mut self, profile_time: Option<Duration>) -> Criterion<M> {
        match profile_time {
            Some(time) => self.mode = Mode::Profile(time),
            None => self.mode = Mode::Benchmark,
        }

        self
    }

    /// Generate the final summary at the end of a run.
    #[doc(hidden)]
    pub fn final_summary(&self) {
        if !self.mode.is_benchmark() {
            return;
        }

        let report_context = ReportContext {
            output_directory: self.output_directory.clone(),
            plot_config: PlotConfiguration::default(),
        };

        self.report.final_summary(&report_context);
    }

    /// Configure this criterion struct based on the command-line arguments to
    /// this process.
    #[must_use]
    #[cfg_attr(feature = "cargo-clippy", allow(clippy::cognitive_complexity))]
    pub fn configure_from_args(mut self) -> Criterion<M> {
        use clap::{value_parser, Arg, Command};
        let matches = Command::new("Criterion Benchmark")
            .arg(Arg::new("FILTER")
                .help("Skip benchmarks whose names do not contain FILTER.")
                .index(1))
            .arg(Arg::new("color")
                .short('c')
                .long("color")
                .alias("colour")
                .value_parser(["auto", "always", "never"])
                .default_value("auto")
                .help("Configure coloring of output. always = always colorize output, never = never colorize output, auto = colorize output if output is a tty and compiled for unix."))
            .arg(Arg::new("verbose")
                .short('v')
                .long("verbose")
                .num_args(0)
                .help("Print additional statistical information."))
            .arg(Arg::new("quiet")
                .long("quiet")
                .num_args(0)
                .conflicts_with("verbose")
                .help("Print only the benchmark results."))
            .arg(Arg::new("noplot")
                .short('n')
                .long("noplot")
                .num_args(0)
                .help("Disable plot and HTML generation."))
            .arg(Arg::new("save-baseline")
                .short('s')
                .long("save-baseline")
                .default_value("base")
                .help("Save results under a named baseline."))
            .arg(Arg::new("discard-baseline")
                .long("discard-baseline")
                .num_args(0)
                .conflicts_with_all(["save-baseline", "baseline", "baseline-lenient"])
                .help("Discard benchmark results."))
            .arg(Arg::new("baseline")
                .short('b')
                .long("baseline")
                .conflicts_with_all(["save-baseline", "baseline-lenient"])
                .help("Compare to a named baseline. If any benchmarks do not have the specified baseline this command fails."))
            .arg(Arg::new("baseline-lenient")
                .long("baseline-lenient")
                .conflicts_with_all(["save-baseline", "baseline"])
                .help("Compare to a named baseline. If any benchmarks do not have the specified baseline then just those benchmarks are not compared against the baseline while every other benchmark is compared against the baseline."))
            .arg(Arg::new("list")
                .long("list")
                .num_args(0)
                .help("List all benchmarks")
                .conflicts_with_all(["test", "profile-time"]))
            .arg(Arg::new("format")
                .long("format")
                .value_parser(["pretty", "terse"])
                .default_value("pretty")
                // Note that libtest's --format also works during test execution, but criterion
                // doesn't support that at the moment.
                .help("Output formatting"))
            .arg(Arg::new("ignored")
                .long("ignored")
                .num_args(0)
                .help("List or run ignored benchmarks (currently means skip all benchmarks)"))
            .arg(Arg::new("exact")
                .long("exact")
                .num_args(0)
                .help("Run benchmarks that exactly match the provided filter"))
            .arg(Arg::new("profile-time")
                .long("profile-time")
                .value_parser(value_parser!(f64))
                .help("Iterate each benchmark for approximately the given number of seconds, doing no analysis and without storing the results. Useful for running the benchmarks in a profiler.")
                .conflicts_with_all(["test", "list"]))
            .arg(Arg::new("load-baseline")
                 .long("load-baseline")
                 .conflicts_with("profile-time")
                 .requires("baseline")
                 .help("Load a previous baseline instead of sampling new data."))
            .arg(Arg::new("sample-size")
                .long("sample-size")
                .value_parser(value_parser!(usize))
                .help(format!("Changes the default size of the sample for this run. [default: {}]", self.config.sample_size)))
            .arg(Arg::new("warm-up-time")
                .long("warm-up-time")
                .value_parser(value_parser!(f64))
                .help(format!("Changes the default warm up time for this run. [default: {}]", self.config.warm_up_time.as_secs())))
            .arg(Arg::new("measurement-time")
                .long("measurement-time")
                .value_parser(value_parser!(f64))
                .help(format!("Changes the default measurement time for this run. [default: {}]", self.config.measurement_time.as_secs())))
            .arg(Arg::new("nresamples")
                .long("nresamples")
                .value_parser(value_parser!(usize))
                .help(format!("Changes the default number of resamples for this run. [default: {}]", self.config.nresamples)))
            .arg(Arg::new("noise-threshold")
                .long("noise-threshold")
                .value_parser(value_parser!(f64))
                .help(format!("Changes the default noise threshold for this run. [default: {}]", self.config.noise_threshold)))
            .arg(Arg::new("confidence-level")
                .long("confidence-level")
                .value_parser(value_parser!(f64))
                .help(format!("Changes the default confidence level for this run. [default: {}]", self.config.confidence_level)))
            .arg(Arg::new("significance-level")
                .long("significance-level")
                .value_parser(value_parser!(f64))
                .help(format!("Changes the default significance level for this run. [default: {}]", self.config.significance_level)))
            .arg(Arg::new("quick")
                .long("quick")
                .num_args(0)
                .conflicts_with("sample-size")
                .help(format!("Benchmark only until the significance level has been reached [default: {}]", self.config.quick_mode)))
            .arg(Arg::new("test")
                .hide(true)
                .long("test")
                .num_args(0)
                .help("Run the benchmarks once, to verify that they execute successfully, but do not measure or report the results.")
                .conflicts_with_all(["list", "profile-time"]))
            .arg(Arg::new("bench")
                .hide(true)
                .long("bench")
                .num_args(0))
            .arg(Arg::new("plotting-backend")
                 .long("plotting-backend")
                 .value_parser(["gnuplot", "plotters"])
                 .help("Set the plotting backend. By default, Criterion.rs will use the gnuplot backend if gnuplot is available, or the plotters backend if it isn't."))
            .arg(Arg::new("output-format")
                .long("output-format")
                .value_parser(["criterion", "bencher"])
                .default_value("criterion")
                .help("Change the CLI output format. By default, Criterion.rs will use its own format. If output format is set to 'bencher', Criterion.rs will print output in a format that resembles the 'bencher' crate."))
            .arg(Arg::new("nocapture")
                .long("nocapture")
                .num_args(0)
                .hide(true)
                .help("Ignored, but added for compatibility with libtest."))
            .arg(Arg::new("show-output")
                .long("show-output")
                .num_args(0)
                .hide(true)
                .help("Ignored, but added for compatibility with libtest."))
            .arg(Arg::new("version")
                .hide(true)
                .short('V')
                .long("version")
                .num_args(0))
            .after_help("
This executable is a Criterion.rs benchmark.
See https://github.com/bheisler/criterion.rs for more details.

To enable debug output, define the environment variable CRITERION_DEBUG.
Criterion.rs will output more debug information and will save the gnuplot
scripts alongside the generated plots.

To test that the benchmarks work, run `cargo test --benches`

NOTE: If you see an 'unrecognized option' error using any of the options above, see:
https://bheisler.github.io/criterion.rs/book/faq.html
")
            .get_matches();

        if self.connection.is_some() {
            if let Some(color) = matches.get_one::<String>("color") {
                if color != "auto" {
                    eprintln!("Warning: --color will be ignored when running with cargo-criterion. Use `cargo criterion --color {} -- <args>` instead.", color);
                }
            }
            if matches.get_flag("verbose") {
                eprintln!("Warning: --verbose will be ignored when running with cargo-criterion. Use `cargo criterion --output-format verbose -- <args>` instead.");
            }
            if matches.get_flag("noplot") {
                eprintln!("Warning: --noplot will be ignored when running with cargo-criterion. Use `cargo criterion --plotting-backend disabled -- <args>` instead.");
            }
            if let Some(backend) = matches.get_one::<String>("plotting-backend") {
                eprintln!("Warning: --plotting-backend will be ignored when running with cargo-criterion. Use `cargo criterion --plotting-backend {} -- <args>` instead.", backend);
            }
            if let Some(format) = matches.get_one::<String>("output-format") {
                if format != "criterion" {
                    eprintln!("Warning: --output-format will be ignored when running with cargo-criterion. Use `cargo criterion --output-format {} -- <args>` instead.", format);
                }
            }

            if matches.contains_id("baseline")
                || matches
                    .get_one::<String>("save-baseline")
                    .map_or(false, |base| base != "base")
                || matches.contains_id("load-baseline")
            {
                eprintln!("Error: baselines are not supported when running with cargo-criterion.");
                std::process::exit(1);
            }
        }

        let bench = matches.get_flag("bench");
        let test = matches.get_flag("test");
        let test_mode = match (bench, test) {
            (true, true) => true,   // cargo bench -- --test should run tests
            (true, false) => false, // cargo bench should run benchmarks
            (false, _) => true,     // cargo test --benches should run tests
        };

        self.mode = if matches.get_flag("list") {
            let list_format = match matches
                .get_one::<String>("format")
                .expect("a default value was provided for this")
                .as_str()
            {
                "pretty" => ListFormat::Pretty,
                "terse" => ListFormat::Terse,
                other => unreachable!(
                    "unrecognized value for --format that isn't part of possible-values: {}",
                    other
                ),
            };
            Mode::List(list_format)
        } else if test_mode {
            Mode::Test
        } else if let Some(&num_seconds) = matches.get_one("profile-time") {
            if num_seconds < 1.0 {
                eprintln!("Profile time must be at least one second.");
                std::process::exit(1);
            }

            Mode::Profile(Duration::from_secs_f64(num_seconds))
        } else {
            Mode::Benchmark
        };

        // This is kind of a hack, but disable the connection to the runner if we're not benchmarking.
        if !self.mode.is_benchmark() {
            self.connection = None;
        }

        let filter = if matches.get_flag("ignored") {
            // --ignored overwrites any name-based filters passed in.
            BenchmarkFilter::RejectAll
        } else if let Some(filter) = matches.get_one::<String>("FILTER") {
            if matches.get_flag("exact") {
                BenchmarkFilter::Exact(filter.to_owned())
            } else {
                let regex = Regex::new(filter).unwrap_or_else(|err| {
                    panic!(
                        "Unable to parse '{}' as a regular expression: {}",
                        filter, err
                    )
                });
                BenchmarkFilter::Regex(regex)
            }
        } else {
            BenchmarkFilter::AcceptAll
        };
        self = self.with_benchmark_filter(filter);

        match matches.get_one("plotting-backend").map(String::as_str) {
            // Use plotting_backend() here to re-use the panic behavior if Gnuplot is not available.
            Some("gnuplot") => self = self.plotting_backend(PlottingBackend::Gnuplot),
            Some("plotters") => self = self.plotting_backend(PlottingBackend::Plotters),
            Some(val) => panic!("Unexpected plotting backend '{}'", val),
            None => {}
        }

        if matches.get_flag("noplot") {
            self = self.without_plots();
        }

        if let Some(dir) = matches.get_one::<String>("save-baseline") {
            self.baseline = Baseline::Save;
            self.baseline_directory = dir.to_owned()
        }
        if matches.get_flag("discard-baseline") {
            self.baseline = Baseline::Discard;
        }
        if let Some(dir) = matches.get_one::<String>("baseline") {
            self.baseline = Baseline::CompareStrict;
            self.baseline_directory = dir.to_owned();
        }
        if let Some(dir) = matches.get_one::<String>("baseline-lenient") {
            self.baseline = Baseline::CompareLenient;
            self.baseline_directory = dir.to_owned();
        }

        if self.connection.is_some() {
            // disable all reports when connected to cargo-criterion; it will do the reporting.
            self.report.cli_enabled = false;
            self.report.bencher_enabled = false;
            self.report.csv_enabled = false;
            self.report.html = None;
        } else {
            match matches.get_one("output-format").map(String::as_str) {
                Some("bencher") => {
                    self.report.bencher_enabled = true;
                    self.report.cli_enabled = false;
                }
                _ => {
                    let verbose = matches.get_flag("verbose");
                    let verbosity = if verbose {
                        CliVerbosity::Verbose
                    } else if matches.get_flag("quiet") {
                        CliVerbosity::Quiet
                    } else {
                        CliVerbosity::Normal
                    };
                    let stdout_isatty = stdout().is_terminal();
                    let mut enable_text_overwrite = stdout_isatty && !verbose && !debug_enabled();
                    let enable_text_coloring;
                    match matches.get_one("color").map(String::as_str) {
                        Some("always") => {
                            enable_text_coloring = true;
                        }
                        Some("never") => {
                            enable_text_coloring = false;
                            enable_text_overwrite = false;
                        }
                        _ => enable_text_coloring = stdout_isatty,
                    };
                    self.report.bencher_enabled = false;
                    self.report.cli_enabled = true;
                    self.report.cli =
                        CliReport::new(enable_text_overwrite, enable_text_coloring, verbosity);
                }
            };
        }

        if let Some(dir) = matches.get_one::<String>("load-baseline") {
            self.load_baseline = Some(dir.to_owned());
        }

        if let Some(&num_size) = matches.get_one("sample-size") {
            assert!(num_size >= 10);
            self.config.sample_size = num_size;
        }
        if let Some(&num_seconds) = matches.get_one("warm-up-time") {
            let dur = std::time::Duration::from_secs_f64(num_seconds);
            assert!(dur.as_nanos() > 0);

            self.config.warm_up_time = dur;
        }
        if let Some(&num_seconds) = matches.get_one("measurement-time") {
            let dur = std::time::Duration::from_secs_f64(num_seconds);
            assert!(dur.as_nanos() > 0);

            self.config.measurement_time = dur;
        }
        if let Some(&num_resamples) = matches.get_one("nresamples") {
            assert!(num_resamples > 0);

            self.config.nresamples = num_resamples;
        }
        if let Some(&num_noise_threshold) = matches.get_one("noise-threshold") {
            assert!(num_noise_threshold > 0.0);

            self.config.noise_threshold = num_noise_threshold;
        }
        if let Some(&num_confidence_level) = matches.get_one("confidence-level") {
            assert!(num_confidence_level > 0.0 && num_confidence_level < 1.0);

            self.config.confidence_level = num_confidence_level;
        }
        if let Some(&num_significance_level) = matches.get_one("significance-level") {
            assert!(num_significance_level > 0.0 && num_significance_level < 1.0);

            self.config.significance_level = num_significance_level;
        }

        if matches.get_flag("quick") {
            self.config.quick_mode = true;
        }

        self
    }

    fn filter_matches(&self, id: &str) -> bool {
        match &self.filter {
            BenchmarkFilter::AcceptAll => true,
            BenchmarkFilter::Regex(regex) => regex.is_match(id),
            BenchmarkFilter::Exact(exact) => id == exact,
            BenchmarkFilter::RejectAll => false,
        }
    }

    /// Returns true iff we should save the benchmark results in
    /// json files on the local disk.
    fn should_save_baseline(&self) -> bool {
        self.connection.is_none()
            && self.load_baseline.is_none()
            && !matches!(self.baseline, Baseline::Discard)
    }

    /// Return a benchmark group. All benchmarks performed using a benchmark group will be
    /// grouped together in the final report.
    ///
    /// # Examples:
    ///
    /// ```rust
    /// #[macro_use] extern crate criterion;
    /// use self::criterion::*;
    ///
    /// fn bench_simple(c: &mut Criterion) {
    ///     let mut group = c.benchmark_group("My Group");
    ///
    ///     // Now we can perform benchmarks with this group
    ///     group.bench_function("Bench 1", |b| b.iter(|| 1 ));
    ///     group.bench_function("Bench 2", |b| b.iter(|| 2 ));
    ///    
    ///     group.finish();
    /// }
    /// criterion_group!(benches, bench_simple);
    /// criterion_main!(benches);
    /// ```
    /// # Panics:
    /// Panics if the group name is empty
    pub fn benchmark_group<S: Into<String>>(&mut self, group_name: S) -> BenchmarkGroup<'_, M> {
        let group_name = group_name.into();
        assert!(!group_name.is_empty(), "Group name must not be empty.");

        if let Some(conn) = &self.connection {
            conn.send(&OutgoingMessage::BeginningBenchmarkGroup { group: &group_name })
                .unwrap();
        }

        BenchmarkGroup::new(self, group_name)
    }
}
impl<M> Criterion<M>
where
    M: Measurement + 'static,
{
    /// Benchmarks a function. For comparing multiple functions, see `benchmark_group`.
    ///
    /// # Example
    ///
    /// ```rust
    /// #[macro_use] extern crate criterion;
    /// use self::criterion::*;
    ///
    /// fn bench(c: &mut Criterion) {
    ///     // Setup (construct data, allocate memory, etc)
    ///     c.bench_function(
    ///         "function_name",
    ///         |b| b.iter(|| {
    ///             // Code to benchmark goes here
    ///         }),
    ///     );
    /// }
    ///
    /// criterion_group!(benches, bench);
    /// criterion_main!(benches);
    /// ```
    pub fn bench_function<F>(&mut self, id: &str, f: F) -> &mut Criterion<M>
    where
        F: FnMut(&mut Bencher<'_, M>),
    {
        self.benchmark_group(id)
            .bench_function(BenchmarkId::no_function(), f);
        self
    }

    /// Benchmarks a function with an input. For comparing multiple functions or multiple inputs,
    /// see `benchmark_group`.
    ///
    /// # Example
    ///
    /// ```rust
    /// #[macro_use] extern crate criterion;
    /// use self::criterion::*;
    ///
    /// fn bench(c: &mut Criterion) {
    ///     // Setup (construct data, allocate memory, etc)
    ///     let input = 5u64;
    ///     c.bench_with_input(
    ///         BenchmarkId::new("function_name", input), &input,
    ///         |b, i| b.iter(|| {
    ///             // Code to benchmark using input `i` goes here
    ///         }),
    ///     );
    /// }
    ///
    /// criterion_group!(benches, bench);
    /// criterion_main!(benches);
    /// ```
    pub fn bench_with_input<F, I>(&mut self, id: BenchmarkId, input: &I, f: F) -> &mut Criterion<M>
    where
        F: FnMut(&mut Bencher<'_, M>, &I),
    {
        // It's possible to use BenchmarkId::from_parameter to create a benchmark ID with no function
        // name. That's intended for use with BenchmarkGroups where the function name isn't necessary,
        // but here it is.
        let group_name = id.function_name.expect(
            "Cannot use BenchmarkId::from_parameter with Criterion::bench_with_input. \
                 Consider using a BenchmarkGroup or BenchmarkId::new instead.",
        );
        // Guaranteed safe because external callers can't create benchmark IDs without a parameter
        let parameter = id.parameter.unwrap();
        self.benchmark_group(group_name).bench_with_input(
            BenchmarkId::no_function_with_input(parameter),
            input,
            f,
        );
        self
    }
}

/// Enum representing different ways of measuring the throughput of benchmarked code.
/// If the throughput setting is configured for a benchmark then the estimated throughput will
/// be reported as well as the time per iteration.
// TODO: Remove serialize/deserialize from the public API.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Throughput {
    /// Measure throughput in terms of bytes/second. The value should be the number of bytes
    /// processed by one iteration of the benchmarked code. Typically, this would be the length of
    /// an input string or `&[u8]`.
    Bytes(u64),

    /// Equivalent to Bytes, but the value will be reported in terms of
    /// kilobytes (1000 bytes) per second instead of kibibytes (1024 bytes) per
    /// second, megabytes instead of mibibytes, and gigabytes instead of gibibytes.
    BytesDecimal(u64),

    /// Measure throughput in terms of elements/second. The value should be the number of elements
    /// processed by one iteration of the benchmarked code. Typically, this would be the size of a
    /// collection, but could also be the number of lines of input text or the number of values to
    /// parse.
    Elements(u64),
}

/// Axis scaling type
#[derive(Debug, Clone, Copy)]
pub enum AxisScale {
    /// Axes scale linearly
    Linear,

    /// Axes scale logarithmically
    Logarithmic,
}

/// Contains the configuration options for the plots generated by a particular benchmark
/// or benchmark group.
///
/// ```rust
/// use self::criterion::{Bencher, Criterion, PlotConfiguration, AxisScale};
///
/// let plot_config = PlotConfiguration::default()
///     .summary_scale(AxisScale::Logarithmic);
///
/// // Using Criterion::default() for simplicity; normally you'd use the macros.
/// let mut criterion = Criterion::default();
/// let mut benchmark_group = criterion.benchmark_group("Group name");
/// benchmark_group.plot_config(plot_config);
/// // Use benchmark group
/// ```
#[derive(Debug, Clone)]
pub struct PlotConfiguration {
    summary_scale: AxisScale,
}

impl Default for PlotConfiguration {
    fn default() -> PlotConfiguration {
        PlotConfiguration {
            summary_scale: AxisScale::Linear,
        }
    }
}

impl PlotConfiguration {
    #[must_use]
    /// Set the axis scale (linear or logarithmic) for the summary plots. Typically, you would
    /// set this to logarithmic if benchmarking over a range of inputs which scale exponentially.
    /// Defaults to linear.
    pub fn summary_scale(mut self, new_scale: AxisScale) -> PlotConfiguration {
        self.summary_scale = new_scale;
        self
    }
}

/// This enum allows the user to control how Criterion.rs chooses the iteration count when sampling.
/// The default is Auto, which will choose a method automatically based on the iteration time during
/// the warm-up phase.
#[derive(Debug, Clone, Copy)]
pub enum SamplingMode {
    /// Criterion.rs should choose a sampling method automatically. This is the default, and is
    /// recommended for most users and most benchmarks.
    Auto,

    /// Scale the iteration count in each sample linearly. This is suitable for most benchmarks,
    /// but it tends to require many iterations which can make it very slow for very long benchmarks.
    Linear,

    /// Keep the iteration count the same for all samples. This is not recommended, as it affects
    /// the statistics that Criterion.rs can compute. However, it requires fewer iterations than
    /// the Linear method and therefore is more suitable for very long-running benchmarks where
    /// benchmark execution time is more of a problem and statistical precision is less important.
    Flat,
}
impl SamplingMode {
    pub(crate) fn choose_sampling_mode(
        &self,
        warmup_mean_execution_time: f64,
        sample_count: u64,
        target_time: f64,
    ) -> ActualSamplingMode {
        match self {
            SamplingMode::Linear => ActualSamplingMode::Linear,
            SamplingMode::Flat => ActualSamplingMode::Flat,
            SamplingMode::Auto => {
                // Estimate execution time with linear sampling
                let total_runs = sample_count * (sample_count + 1) / 2;
                let d =
                    (target_time / warmup_mean_execution_time / total_runs as f64).ceil() as u64;
                let expected_ns = total_runs as f64 * d as f64 * warmup_mean_execution_time;

                if expected_ns > (2.0 * target_time) {
                    ActualSamplingMode::Flat
                } else {
                    ActualSamplingMode::Linear
                }
            }
        }
    }
}

/// Enum to represent the sampling mode without Auto.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub(crate) enum ActualSamplingMode {
    Linear,
    Flat,
}
impl ActualSamplingMode {
    pub(crate) fn iteration_counts(
        &self,
        warmup_mean_execution_time: f64,
        sample_count: u64,
        target_time: &Duration,
    ) -> Vec<u64> {
        match self {
            ActualSamplingMode::Linear => {
                let n = sample_count;
                let met = warmup_mean_execution_time;
                let m_ns = target_time.as_nanos();
                // Solve: [d + 2*d + 3*d + ... + n*d] * met = m_ns
                let total_runs = n * (n + 1) / 2;
                let d = ((m_ns as f64 / met / total_runs as f64).ceil() as u64).max(1);
                let expected_ns = total_runs as f64 * d as f64 * met;

                if d == 1 {
                    let recommended_sample_size =
                        ActualSamplingMode::recommend_linear_sample_size(m_ns as f64, met);
                    let actual_time = Duration::from_nanos(expected_ns as u64);
                    eprint!("\nWarning: Unable to complete {} samples in {:.1?}. You may wish to increase target time to {:.1?}",
                            n, target_time, actual_time);

                    if recommended_sample_size != n {
                        eprintln!(
                            ", enable flat sampling, or reduce sample count to {}.",
                            recommended_sample_size
                        );
                    } else {
                        eprintln!(" or enable flat sampling.");
                    }
                }

                (1..(n + 1)).map(|a| a * d).collect::<Vec<u64>>()
            }
            ActualSamplingMode::Flat => {
                let n = sample_count;
                let met = warmup_mean_execution_time;
                let m_ns = target_time.as_nanos() as f64;
                let time_per_sample = m_ns / (n as f64);
                // This is pretty simplistic; we could do something smarter to fit into the allotted time.
                let iterations_per_sample = ((time_per_sample / met).ceil() as u64).max(1);

                let expected_ns = met * (iterations_per_sample * n) as f64;

                if iterations_per_sample == 1 {
                    let recommended_sample_size =
                        ActualSamplingMode::recommend_flat_sample_size(m_ns, met);
                    let actual_time = Duration::from_nanos(expected_ns as u64);
                    eprint!("\nWarning: Unable to complete {} samples in {:.1?}. You may wish to increase target time to {:.1?}",
                            n, target_time, actual_time);

                    if recommended_sample_size != n {
                        eprintln!(", or reduce sample count to {}.", recommended_sample_size);
                    } else {
                        eprintln!(".");
                    }
                }

                vec![iterations_per_sample; n as usize]
            }
        }
    }

    fn is_linear(&self) -> bool {
        matches!(self, ActualSamplingMode::Linear)
    }

    fn recommend_linear_sample_size(target_time: f64, met: f64) -> u64 {
        // Some math shows that n(n+1)/2 * d * met = target_time. d = 1, so it can be ignored.
        // This leaves n(n+1) = (2*target_time)/met, or n^2 + n - (2*target_time)/met = 0
        // Which can be solved with the quadratic formula. Since A and B are constant 1,
        // this simplifies to sample_size = (-1 +- sqrt(1 - 4C))/2, where C = (2*target_time)/met.
        // We don't care about the negative solution. Experimentation shows that this actually tends to
        // result in twice the desired execution time (probably because of the ceil used to calculate
        // d) so instead I use c = target_time/met.
        let c = target_time / met;
        let sample_size = (-1.0 + (4.0 * c).sqrt()) / 2.0;
        let sample_size = sample_size as u64;

        // Round down to the nearest 10 to give a margin and avoid excessive precision
        let sample_size = (sample_size / 10) * 10;

        // Clamp it to be at least 10, since criterion.rs doesn't allow sample sizes smaller than 10.
        if sample_size < 10 {
            10
        } else {
            sample_size
        }
    }

    fn recommend_flat_sample_size(target_time: f64, met: f64) -> u64 {
        let sample_size = (target_time / met) as u64;

        // Round down to the nearest 10 to give a margin and avoid excessive precision
        let sample_size = (sample_size / 10) * 10;

        // Clamp it to be at least 10, since criterion.rs doesn't allow sample sizes smaller than 10.
        if sample_size < 10 {
            10
        } else {
            sample_size
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SavedSample {
    sampling_mode: ActualSamplingMode,
    iters: Vec<f64>,
    times: Vec<f64>,
}

/// Custom-test-framework runner. Should not be called directly.
#[doc(hidden)]
pub fn runner(benches: &[&dyn Fn()]) {
    for bench in benches {
        bench();
    }
    Criterion::default().configure_from_args().final_summary();
}

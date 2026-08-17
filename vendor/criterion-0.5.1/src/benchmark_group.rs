use crate::analysis;
use crate::benchmark::PartialBenchmarkConfig;
use crate::connection::OutgoingMessage;
use crate::measurement::Measurement;
use crate::report::BenchmarkId as InternalBenchmarkId;
use crate::report::Report;
use crate::report::ReportContext;
use crate::routine::{Function, Routine};
use crate::{Bencher, Criterion, Mode, PlotConfiguration, SamplingMode, Throughput};
use std::time::Duration;

/// Structure used to group together a set of related benchmarks, along with custom configuration
/// settings for groups of benchmarks. All benchmarks performed using a benchmark group will be
/// grouped together in the final report.
///
/// # Examples:
///
/// ```no_run
/// #[macro_use] extern crate criterion;
/// use self::criterion::*;
/// use std::time::Duration;
///
/// fn bench_simple(c: &mut Criterion) {
///     let mut group = c.benchmark_group("My Group");
///
///     // Now we can perform benchmarks with this group
///     group.bench_function("Bench 1", |b| b.iter(|| 1 ));
///     group.bench_function("Bench 2", |b| b.iter(|| 2 ));
///    
///     // It's recommended to call group.finish() explicitly at the end, but if you don't it will
///     // be called automatically when the group is dropped.
///     group.finish();
/// }
///
/// fn bench_nested(c: &mut Criterion) {
///     let mut group = c.benchmark_group("My Second Group");
///     // We can override the configuration on a per-group level
///     group.measurement_time(Duration::from_secs(1));
///
///     // We can also use loops to define multiple benchmarks, even over multiple dimensions.
///     for x in 0..3 {
///         for y in 0..3 {
///             let point = (x, y);
///             let parameter_string = format!("{} * {}", x, y);
///             group.bench_with_input(BenchmarkId::new("Multiply", parameter_string), &point,
///                 |b, (p_x, p_y)| b.iter(|| p_x * p_y));
///         }
///     }
///    
///     group.finish();
/// }
///
/// fn bench_throughput(c: &mut Criterion) {
///     let mut group = c.benchmark_group("Summation");
///     
///     for size in [1024, 2048, 4096].iter() {
///         // Generate input of an appropriate size...
///         let input = vec![1u64, *size];
///
///         // We can use the throughput function to tell Criterion.rs how large the input is
///         // so it can calculate the overall throughput of the function. If we wanted, we could
///         // even change the benchmark configuration for different inputs (eg. to reduce the
///         // number of samples for extremely large and slow inputs) or even different functions.
///         group.throughput(Throughput::Elements(*size as u64));
///
///         group.bench_with_input(BenchmarkId::new("sum", *size), &input,
///             |b, i| b.iter(|| i.iter().sum::<u64>()));
///         group.bench_with_input(BenchmarkId::new("fold", *size), &input,
///             |b, i| b.iter(|| i.iter().fold(0u64, |a, b| a + b)));
///     }
///
///     group.finish();
/// }
///
/// criterion_group!(benches, bench_simple, bench_nested, bench_throughput);
/// criterion_main!(benches);
/// ```
pub struct BenchmarkGroup<'a, M: Measurement> {
    criterion: &'a mut Criterion<M>,
    group_name: String,
    all_ids: Vec<InternalBenchmarkId>,
    any_matched: bool,
    partial_config: PartialBenchmarkConfig,
    throughput: Option<Throughput>,
}
impl<'a, M: Measurement> BenchmarkGroup<'a, M> {
    /// Changes the size of the sample for this benchmark
    ///
    /// A bigger sample should yield more accurate results if paired with a sufficiently large
    /// measurement time.
    ///
    /// Sample size must be at least 10.
    ///
    /// # Panics
    ///
    /// Panics if n < 10.
    pub fn sample_size(&mut self, n: usize) -> &mut Self {
        assert!(n >= 10);

        self.partial_config.sample_size = Some(n);
        self
    }

    /// Changes the warm up time for this benchmark
    ///
    /// # Panics
    ///
    /// Panics if the input duration is zero
    pub fn warm_up_time(&mut self, dur: Duration) -> &mut Self {
        assert!(dur.as_nanos() > 0);

        self.partial_config.warm_up_time = Some(dur);
        self
    }

    /// Changes the target measurement time for this benchmark group.
    ///
    /// Criterion will attempt to spent approximately this amount of time measuring each
    /// benchmark on a best-effort basis. If it is not possible to perform the measurement in
    /// the requested time (eg. because each iteration of the benchmark is long) then Criterion
    /// will spend as long as is needed to collect the desired number of samples. With a longer
    /// time, the measurement will become more resilient to interference from other programs.
    ///
    /// # Panics
    ///
    /// Panics if the input duration is zero
    pub fn measurement_time(&mut self, dur: Duration) -> &mut Self {
        assert!(dur.as_nanos() > 0);

        self.partial_config.measurement_time = Some(dur);
        self
    }

    /// Changes the number of resamples for this benchmark group
    ///
    /// Number of resamples to use for the
    /// [bootstrap](http://en.wikipedia.org/wiki/Bootstrapping_(statistics)#Case_resampling)
    ///
    /// A larger number of resamples reduces the random sampling errors which are inherent to the
    /// bootstrap method, but also increases the analysis time.
    ///
    /// # Panics
    ///
    /// Panics if the number of resamples is set to zero
    pub fn nresamples(&mut self, n: usize) -> &mut Self {
        assert!(n > 0);
        if n <= 1000 {
            eprintln!("\nWarning: It is not recommended to reduce nresamples below 1000.");
        }

        self.partial_config.nresamples = Some(n);
        self
    }

    /// Changes the noise threshold for benchmarks in this group. The noise threshold
    /// is used to filter out small changes in performance from one run to the next, even if they
    /// are statistically significant. Sometimes benchmarking the same code twice will result in
    /// small but statistically significant differences solely because of noise. This provides a way
    /// to filter out some of these false positives at the cost of making it harder to detect small
    /// changes to the true performance of the benchmark.
    ///
    /// The default is 0.01, meaning that changes smaller than 1% will be ignored.
    ///
    /// # Panics
    ///
    /// Panics if the threshold is set to a negative value
    pub fn noise_threshold(&mut self, threshold: f64) -> &mut Self {
        assert!(threshold >= 0.0);

        self.partial_config.noise_threshold = Some(threshold);
        self
    }

    /// Changes the confidence level for benchmarks in this group. The confidence
    /// level is the desired probability that the true runtime lies within the estimated
    /// [confidence interval](https://en.wikipedia.org/wiki/Confidence_interval). The default is
    /// 0.95, meaning that the confidence interval should capture the true value 95% of the time.
    ///
    /// # Panics
    ///
    /// Panics if the confidence level is set to a value outside the `(0, 1)` range
    pub fn confidence_level(&mut self, cl: f64) -> &mut Self {
        assert!(cl > 0.0 && cl < 1.0);
        if cl < 0.5 {
            eprintln!("\nWarning: It is not recommended to reduce confidence level below 0.5.");
        }

        self.partial_config.confidence_level = Some(cl);
        self
    }

    /// Changes the [significance level](https://en.wikipedia.org/wiki/Statistical_significance)
    /// for benchmarks in this group. This is used to perform a
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
    pub fn significance_level(&mut self, sl: f64) -> &mut Self {
        assert!(sl > 0.0 && sl < 1.0);

        self.partial_config.significance_level = Some(sl);
        self
    }

    /// Changes the plot configuration for this benchmark group.
    pub fn plot_config(&mut self, new_config: PlotConfiguration) -> &mut Self {
        self.partial_config.plot_config = new_config;
        self
    }

    /// Set the input size for this benchmark group. Used for reporting the
    /// throughput.
    pub fn throughput(&mut self, throughput: Throughput) -> &mut Self {
        self.throughput = Some(throughput);
        self
    }

    /// Set the sampling mode for this benchmark group.
    pub fn sampling_mode(&mut self, new_mode: SamplingMode) -> &mut Self {
        self.partial_config.sampling_mode = Some(new_mode);
        self
    }

    pub(crate) fn new(criterion: &mut Criterion<M>, group_name: String) -> BenchmarkGroup<'_, M> {
        BenchmarkGroup {
            criterion,
            group_name,
            all_ids: vec![],
            any_matched: false,
            partial_config: PartialBenchmarkConfig::default(),
            throughput: None,
        }
    }

    /// Benchmark the given parameterless function inside this benchmark group.
    pub fn bench_function<ID: IntoBenchmarkId, F>(&mut self, id: ID, mut f: F) -> &mut Self
    where
        F: FnMut(&mut Bencher<'_, M>),
    {
        self.run_bench(id.into_benchmark_id(), &(), |b, _| f(b));
        self
    }

    /// Benchmark the given parameterized function inside this benchmark group.
    pub fn bench_with_input<ID: IntoBenchmarkId, F, I>(
        &mut self,
        id: ID,
        input: &I,
        f: F,
    ) -> &mut Self
    where
        F: FnMut(&mut Bencher<'_, M>, &I),
        I: ?Sized,
    {
        self.run_bench(id.into_benchmark_id(), input, f);
        self
    }

    fn run_bench<F, I>(&mut self, id: BenchmarkId, input: &I, f: F)
    where
        F: FnMut(&mut Bencher<'_, M>, &I),
        I: ?Sized,
    {
        let config = self.partial_config.to_complete(&self.criterion.config);
        let report_context = ReportContext {
            output_directory: self.criterion.output_directory.clone(),
            plot_config: self.partial_config.plot_config.clone(),
        };

        let mut id = InternalBenchmarkId::new(
            self.group_name.clone(),
            id.function_name,
            id.parameter,
            self.throughput.clone(),
        );

        assert!(
            !self.all_ids.contains(&id),
            "Benchmark IDs must be unique within a group. Encountered duplicated benchmark ID {}",
            &id
        );

        id.ensure_directory_name_unique(&self.criterion.all_directories);
        self.criterion
            .all_directories
            .insert(id.as_directory_name().to_owned());
        id.ensure_title_unique(&self.criterion.all_titles);
        self.criterion.all_titles.insert(id.as_title().to_owned());

        let do_run = self.criterion.filter_matches(id.id());
        self.any_matched |= do_run;
        let mut func = Function::new(f);

        match &self.criterion.mode {
            Mode::Benchmark => {
                if let Some(conn) = &self.criterion.connection {
                    if do_run {
                        conn.send(&OutgoingMessage::BeginningBenchmark { id: (&id).into() })
                            .unwrap();
                    } else {
                        conn.send(&OutgoingMessage::SkippingBenchmark { id: (&id).into() })
                            .unwrap();
                    }
                }
                if do_run {
                    analysis::common(
                        &id,
                        &mut func,
                        &config,
                        self.criterion,
                        &report_context,
                        input,
                        self.throughput.clone(),
                    );
                }
            }
            Mode::List(_) => {
                if do_run {
                    println!("{}: benchmark", id);
                }
            }
            Mode::Test => {
                if do_run {
                    // In test mode, run the benchmark exactly once, then exit.
                    self.criterion.report.test_start(&id, &report_context);
                    func.test(&self.criterion.measurement, input);
                    self.criterion.report.test_pass(&id, &report_context);
                }
            }
            &Mode::Profile(duration) => {
                if do_run {
                    func.profile(
                        &self.criterion.measurement,
                        &id,
                        self.criterion,
                        &report_context,
                        duration,
                        input,
                    );
                }
            }
        }

        self.all_ids.push(id);
    }

    /// Consume the benchmark group and generate the summary reports for the group.
    ///
    /// It is recommended to call this explicitly, but if you forget it will be called when the
    /// group is dropped.
    pub fn finish(self) {
        ::std::mem::drop(self);
    }
}
impl<'a, M: Measurement> Drop for BenchmarkGroup<'a, M> {
    fn drop(&mut self) {
        // I don't really like having a bunch of non-trivial code in drop, but this is the only way
        // to really write linear types like this in Rust...
        if let Some(conn) = &mut self.criterion.connection {
            conn.send(&OutgoingMessage::FinishedBenchmarkGroup {
                group: &self.group_name,
            })
            .unwrap();

            conn.serve_value_formatter(self.criterion.measurement.formatter())
                .unwrap();
        }

        if self.all_ids.len() > 1 && self.any_matched && self.criterion.mode.is_benchmark() {
            let report_context = ReportContext {
                output_directory: self.criterion.output_directory.clone(),
                plot_config: self.partial_config.plot_config.clone(),
            };

            self.criterion.report.summarize(
                &report_context,
                &self.all_ids,
                self.criterion.measurement.formatter(),
            );
        }
        if self.any_matched && !self.criterion.mode.is_terse() {
            self.criterion.report.group_separator();
        }
    }
}

/// Simple structure representing an ID for a benchmark. The ID must be unique within a benchmark
/// group.
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct BenchmarkId {
    pub(crate) function_name: Option<String>,
    pub(crate) parameter: Option<String>,
}
impl BenchmarkId {
    /// Construct a new benchmark ID from a string function name and a parameter value.
    ///
    /// Note that the parameter value need not be the same as the parameter passed to your
    /// actual benchmark. For instance, you might have a benchmark that takes a 1MB string as
    /// input. It would be impractical to embed the whole string in the benchmark ID, so instead
    /// your parameter value might be a descriptive string like "1MB Alphanumeric".
    ///
    /// # Examples
    /// ```
    /// # use criterion::{BenchmarkId, Criterion};
    /// // A basic benchmark ID is typically constructed from a constant string and a simple
    /// // parameter
    /// let basic_id = BenchmarkId::new("my_id", 5);
    ///
    /// // The function name can be a string
    /// let function_name = "test_string".to_string();
    /// let string_id = BenchmarkId::new(function_name, 12);
    ///
    /// // Benchmark IDs are passed to benchmark groups:
    /// let mut criterion = Criterion::default();
    /// let mut group = criterion.benchmark_group("My Group");
    /// // Generate a very large input
    /// let input : String = ::std::iter::repeat("X").take(1024 * 1024).collect();
    ///
    /// // Note that we don't have to use the input as the parameter in the ID
    /// group.bench_with_input(BenchmarkId::new("Test long string", "1MB X's"), &input, |b, i| {
    ///     b.iter(|| i.len())
    /// });
    /// ```
    pub fn new<S: Into<String>, P: ::std::fmt::Display>(
        function_name: S,
        parameter: P,
    ) -> BenchmarkId {
        BenchmarkId {
            function_name: Some(function_name.into()),
            parameter: Some(format!("{}", parameter)),
        }
    }

    /// Construct a new benchmark ID from just a parameter value. Use this when benchmarking a
    /// single function with a variety of different inputs.
    pub fn from_parameter<P: ::std::fmt::Display>(parameter: P) -> BenchmarkId {
        BenchmarkId {
            function_name: None,
            parameter: Some(format!("{}", parameter)),
        }
    }

    pub(crate) fn no_function() -> BenchmarkId {
        BenchmarkId {
            function_name: None,
            parameter: None,
        }
    }

    pub(crate) fn no_function_with_input<P: ::std::fmt::Display>(parameter: P) -> BenchmarkId {
        BenchmarkId {
            function_name: None,
            parameter: Some(format!("{}", parameter)),
        }
    }
}

mod private {
    pub trait Sealed {}
    impl Sealed for super::BenchmarkId {}
    impl<S: Into<String>> Sealed for S {}
}

/// Sealed trait which allows users to automatically convert strings to benchmark IDs.
pub trait IntoBenchmarkId: private::Sealed {
    fn into_benchmark_id(self) -> BenchmarkId;
}
impl IntoBenchmarkId for BenchmarkId {
    fn into_benchmark_id(self) -> BenchmarkId {
        self
    }
}
impl<S: Into<String>> IntoBenchmarkId for S {
    fn into_benchmark_id(self) -> BenchmarkId {
        let function_name = self.into();
        assert!(
            !function_name.is_empty(),
            "Function name must not be empty."
        );

        BenchmarkId {
            function_name: Some(function_name),
            parameter: None,
        }
    }
}

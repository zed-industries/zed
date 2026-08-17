#[cfg(feature = "plotters")]
use criterion::SamplingMode;
use criterion::{
    criterion_group, criterion_main, profiler::Profiler, BatchSize, BenchmarkId, Criterion,
};
use serde_json::value::Value;
use std::cell::{Cell, RefCell};
use std::cmp::max;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::{Duration, SystemTime};
use tempfile::{tempdir, TempDir};
use walkdir::WalkDir;

/*
 * Please note that these tests are not complete examples of how to use
 * Criterion.rs. See the benches folder for actual examples.
 */
fn temp_dir() -> TempDir {
    tempdir().unwrap()
}

// Configure a Criterion struct to perform really fast benchmarks. This is not
// recommended for real benchmarking, only for testing.
fn short_benchmark(dir: &TempDir) -> Criterion {
    Criterion::default()
        .output_directory(dir.path())
        .warm_up_time(Duration::from_millis(250))
        .measurement_time(Duration::from_millis(500))
        .nresamples(2000)
}

#[derive(Clone)]
struct Counter {
    counter: Rc<RefCell<usize>>,
}
impl Counter {
    fn count(&self) {
        *(*self.counter).borrow_mut() += 1;
    }

    fn read(&self) -> usize {
        *(*self.counter).borrow()
    }
}
impl Default for Counter {
    fn default() -> Counter {
        Counter {
            counter: Rc::new(RefCell::new(0)),
        }
    }
}

fn verify_file(dir: &PathBuf, path: &str) -> PathBuf {
    let full_path = dir.join(path);
    assert!(
        full_path.is_file(),
        "File {:?} does not exist or is not a file",
        full_path
    );
    let metadata = full_path.metadata().unwrap();
    assert!(metadata.len() > 0);
    full_path
}

fn verify_json(dir: &PathBuf, path: &str) {
    let full_path = verify_file(dir, path);
    let f = File::open(full_path).unwrap();
    serde_json::from_reader::<File, Value>(f).unwrap();
}

#[cfg(feature = "html_reports")]
fn verify_svg(dir: &PathBuf, path: &str) {
    verify_file(dir, path);
}

#[cfg(feature = "html_reports")]
fn verify_html(dir: &PathBuf, path: &str) {
    verify_file(dir, path);
}

fn verify_stats(dir: &PathBuf, baseline: &str) {
    verify_json(dir, &format!("{}/estimates.json", baseline));
    verify_json(dir, &format!("{}/sample.json", baseline));
    verify_json(dir, &format!("{}/tukey.json", baseline));
    verify_json(dir, &format!("{}/benchmark.json", baseline));
    #[cfg(feature = "csv_output")]
    verify_file(&dir, &format!("{}/raw.csv", baseline));
}

fn verify_not_exists(dir: &PathBuf, path: &str) {
    assert!(!dir.join(path).exists());
}

fn latest_modified(dir: &PathBuf) -> SystemTime {
    let mut newest_update: Option<SystemTime> = None;
    for entry in WalkDir::new(dir) {
        let entry = entry.unwrap();
        let modified = entry.metadata().unwrap().modified().unwrap();
        newest_update = match newest_update {
            Some(latest) => Some(max(latest, modified)),
            None => Some(modified),
        };
    }

    newest_update.expect("failed to find a single time in directory")
}

#[test]
fn test_creates_directory() {
    let dir = temp_dir();
    short_benchmark(&dir).bench_function("test_creates_directory", |b| b.iter(|| 10));
    assert!(dir.path().join("test_creates_directory").is_dir());
}

#[test]
fn test_without_plots() {
    let dir = temp_dir();
    short_benchmark(&dir)
        .without_plots()
        .bench_function("test_without_plots", |b| b.iter(|| 10));

    for entry in WalkDir::new(dir.path().join("test_without_plots")) {
        let entry = entry.ok();
        let is_svg = entry
            .as_ref()
            .and_then(|entry| entry.path().extension())
            .and_then(|ext| ext.to_str())
            .map(|ext| ext == "svg")
            .unwrap_or(false);
        assert!(
            !is_svg,
            "Found SVG file ({:?}) in output directory with plots disabled",
            entry.unwrap().file_name()
        );
    }
}

#[test]
fn test_save_baseline() {
    let dir = temp_dir();
    println!("tmp directory is {:?}", dir.path());
    short_benchmark(&dir)
        .save_baseline("some-baseline".to_owned())
        .bench_function("test_save_baseline", |b| b.iter(|| 10));

    let dir = dir.path().join("test_save_baseline");
    verify_stats(&dir, "some-baseline");

    verify_not_exists(&dir, "base");
}

#[test]
fn test_retain_baseline() {
    // Initial benchmark to populate
    let dir = temp_dir();
    short_benchmark(&dir)
        .save_baseline("some-baseline".to_owned())
        .bench_function("test_retain_baseline", |b| b.iter(|| 10));

    let pre_modified = latest_modified(&dir.path().join("test_retain_baseline/some-baseline"));

    short_benchmark(&dir)
        .retain_baseline("some-baseline".to_owned(), true)
        .bench_function("test_retain_baseline", |b| b.iter(|| 10));

    let post_modified = latest_modified(&dir.path().join("test_retain_baseline/some-baseline"));

    assert_eq!(pre_modified, post_modified, "baseline modified by retain");
}

#[test]
#[should_panic(expected = "Baseline 'some-baseline' must exist before comparison is allowed")]
fn test_compare_baseline_strict_panics_when_missing_baseline() {
    let dir = temp_dir();
    short_benchmark(&dir)
        .retain_baseline("some-baseline".to_owned(), true)
        .bench_function("test_compare_baseline", |b| b.iter(|| 10));
}

#[test]
fn test_compare_baseline_lenient_when_missing_baseline() {
    let dir = temp_dir();
    short_benchmark(&dir)
        .retain_baseline("some-baseline".to_owned(), false)
        .bench_function("test_compare_baseline", |b| b.iter(|| 10));
}

#[test]
fn test_sample_size() {
    let dir = temp_dir();
    let counter = Counter::default();

    let clone = counter.clone();
    short_benchmark(&dir)
        .sample_size(50)
        .bench_function("test_sample_size", move |b| {
            clone.count();
            b.iter(|| 10)
        });

    // This function will be called more than sample_size times because of the
    // warmup.
    assert!(counter.read() > 50);
}

#[test]
fn test_warmup_time() {
    let dir = temp_dir();
    let counter1 = Counter::default();

    let clone = counter1.clone();
    short_benchmark(&dir)
        .warm_up_time(Duration::from_millis(100))
        .bench_function("test_warmup_time_1", move |b| {
            clone.count();
            b.iter(|| 10)
        });

    let counter2 = Counter::default();
    let clone = counter2.clone();
    short_benchmark(&dir)
        .warm_up_time(Duration::from_millis(2000))
        .bench_function("test_warmup_time_2", move |b| {
            clone.count();
            b.iter(|| 10)
        });

    assert!(counter1.read() < counter2.read());
}

#[test]
fn test_measurement_time() {
    let dir = temp_dir();
    let counter1 = Counter::default();

    let clone = counter1.clone();
    short_benchmark(&dir)
        .measurement_time(Duration::from_millis(100))
        .bench_function("test_meas_time_1", move |b| b.iter(|| clone.count()));

    let counter2 = Counter::default();
    let clone = counter2.clone();
    short_benchmark(&dir)
        .measurement_time(Duration::from_millis(2000))
        .bench_function("test_meas_time_2", move |b| b.iter(|| clone.count()));

    assert!(counter1.read() < counter2.read());
}

#[test]
fn test_bench_function() {
    let dir = temp_dir();
    short_benchmark(&dir).bench_function("test_bench_function", move |b| b.iter(|| 10));
}

#[test]
fn test_filtering() {
    let dir = temp_dir();
    let counter = Counter::default();
    let clone = counter.clone();

    short_benchmark(&dir)
        .with_filter("Foo")
        .bench_function("test_filtering", move |b| b.iter(|| clone.count()));

    assert_eq!(counter.read(), 0);
    assert!(!dir.path().join("test_filtering").is_dir());
}

#[test]
fn test_timing_loops() {
    let dir = temp_dir();
    let mut c = short_benchmark(&dir);
    let mut group = c.benchmark_group("test_timing_loops");
    group.bench_function("iter_with_setup", |b| {
        b.iter_with_setup(|| vec![10], |v| v[0])
    });
    group.bench_function("iter_with_large_setup", |b| {
        b.iter_batched(|| vec![10], |v| v[0], BatchSize::NumBatches(1))
    });
    group.bench_function("iter_with_large_drop", |b| {
        b.iter_with_large_drop(|| vec![10; 100])
    });
    group.bench_function("iter_batched_small", |b| {
        b.iter_batched(|| vec![10], |v| v[0], BatchSize::SmallInput)
    });
    group.bench_function("iter_batched_large", |b| {
        b.iter_batched(|| vec![10], |v| v[0], BatchSize::LargeInput)
    });
    group.bench_function("iter_batched_per_iteration", |b| {
        b.iter_batched(|| vec![10], |v| v[0], BatchSize::PerIteration)
    });
    group.bench_function("iter_batched_one_batch", |b| {
        b.iter_batched(|| vec![10], |v| v[0], BatchSize::NumBatches(1))
    });
    group.bench_function("iter_batched_10_iterations", |b| {
        b.iter_batched(|| vec![10], |v| v[0], BatchSize::NumIterations(10))
    });
    group.bench_function("iter_batched_ref_small", |b| {
        b.iter_batched_ref(|| vec![10], |v| v[0], BatchSize::SmallInput)
    });
    group.bench_function("iter_batched_ref_large", |b| {
        b.iter_batched_ref(|| vec![10], |v| v[0], BatchSize::LargeInput)
    });
    group.bench_function("iter_batched_ref_per_iteration", |b| {
        b.iter_batched_ref(|| vec![10], |v| v[0], BatchSize::PerIteration)
    });
    group.bench_function("iter_batched_ref_one_batch", |b| {
        b.iter_batched_ref(|| vec![10], |v| v[0], BatchSize::NumBatches(1))
    });
    group.bench_function("iter_batched_ref_10_iterations", |b| {
        b.iter_batched_ref(|| vec![10], |v| v[0], BatchSize::NumIterations(10))
    });
}

// Verify that all expected output files are present
#[cfg(feature = "plotters")]
#[test]
fn test_output_files() {
    let tempdir = temp_dir();
    // Run benchmarks twice to produce comparisons
    for _ in 0..2 {
        let mut c = short_benchmark(&tempdir);
        let mut group = c.benchmark_group("test_output");
        group.sampling_mode(SamplingMode::Linear);
        group.bench_function("output_1", |b| b.iter(|| 10));
        group.bench_function("output_2", |b| b.iter(|| 20));
        group.bench_function("output_\\/*\"?", |b| b.iter(|| 30));
    }

    // For each benchmark, assert that the expected files are present.
    for x in 0..3 {
        let dir = if x == 2 {
            // Check that certain special characters are replaced with underscores
            tempdir.path().join("test_output/output______")
        } else {
            tempdir.path().join(format!("test_output/output_{}", x + 1))
        };

        verify_stats(&dir, "new");
        verify_stats(&dir, "base");
        verify_json(&dir, "change/estimates.json");

        #[cfg(feature = "html_reports")]
        {
            verify_svg(&dir, "report/MAD.svg");
            verify_svg(&dir, "report/mean.svg");
            verify_svg(&dir, "report/median.svg");
            verify_svg(&dir, "report/pdf.svg");
            verify_svg(&dir, "report/regression.svg");
            verify_svg(&dir, "report/SD.svg");
            verify_svg(&dir, "report/slope.svg");
            verify_svg(&dir, "report/typical.svg");
            verify_svg(&dir, "report/both/pdf.svg");
            verify_svg(&dir, "report/both/regression.svg");
            verify_svg(&dir, "report/change/mean.svg");
            verify_svg(&dir, "report/change/median.svg");
            verify_svg(&dir, "report/change/t-test.svg");

            verify_svg(&dir, "report/pdf_small.svg");
            verify_svg(&dir, "report/regression_small.svg");
            verify_svg(&dir, "report/relative_pdf_small.svg");
            verify_svg(&dir, "report/relative_regression_small.svg");
            verify_html(&dir, "report/index.html");
        }
    }

    #[cfg(feature = "html_reports")]
    {
        // Check for overall report files
        let dir = tempdir.path().join("test_output");

        verify_svg(&dir, "report/violin.svg");
        verify_html(&dir, "report/index.html");
    }

    // Run the final summary process and check for the report that produces
    short_benchmark(&tempdir).final_summary();

    #[cfg(feature = "html_reports")]
    {
        let dir = tempdir.path().to_owned();
        verify_html(&dir, "report/index.html");
    }
}

#[cfg(feature = "plotters")]
#[test]
fn test_output_files_flat_sampling() {
    let tempdir = temp_dir();
    // Run benchmark twice to produce comparisons
    for _ in 0..2 {
        let mut c = short_benchmark(&tempdir);
        let mut group = c.benchmark_group("test_output");
        group.sampling_mode(SamplingMode::Flat);
        group.bench_function("output_flat", |b| b.iter(|| 10));
    }

    let dir = tempdir.path().join("test_output/output_flat");

    verify_stats(&dir, "new");
    verify_stats(&dir, "base");
    verify_json(&dir, "change/estimates.json");

    #[cfg(feature = "html_reports")]
    {
        verify_svg(&dir, "report/MAD.svg");
        verify_svg(&dir, "report/mean.svg");
        verify_svg(&dir, "report/median.svg");
        verify_svg(&dir, "report/pdf.svg");
        verify_svg(&dir, "report/iteration_times.svg");
        verify_svg(&dir, "report/SD.svg");
        verify_svg(&dir, "report/typical.svg");
        verify_svg(&dir, "report/both/pdf.svg");
        verify_svg(&dir, "report/both/iteration_times.svg");
        verify_svg(&dir, "report/change/mean.svg");
        verify_svg(&dir, "report/change/median.svg");
        verify_svg(&dir, "report/change/t-test.svg");

        verify_svg(&dir, "report/pdf_small.svg");
        verify_svg(&dir, "report/iteration_times_small.svg");
        verify_svg(&dir, "report/relative_pdf_small.svg");
        verify_svg(&dir, "report/relative_iteration_times_small.svg");
        verify_html(&dir, "report/index.html");
    }
}

#[test]
#[should_panic(expected = "Benchmark function must call Bencher::iter or related method.")]
fn test_bench_with_no_iteration_panics() {
    let dir = temp_dir();
    short_benchmark(&dir).bench_function("no_iter", |_b| {});
}

#[test]
fn test_benchmark_group_with_input() {
    let dir = temp_dir();
    let mut c = short_benchmark(&dir);
    let mut group = c.benchmark_group("Test Group");
    for x in 0..2 {
        group.bench_with_input(BenchmarkId::new("Test 1", x), &x, |b, i| b.iter(|| i));
        group.bench_with_input(BenchmarkId::new("Test 2", x), &x, |b, i| b.iter(|| i));
    }
    group.finish();
}

#[test]
fn test_benchmark_group_without_input() {
    let dir = temp_dir();
    let mut c = short_benchmark(&dir);
    let mut group = c.benchmark_group("Test Group 2");
    group.bench_function("Test 1", |b| b.iter(|| 30));
    group.bench_function("Test 2", |b| b.iter(|| 20));
    group.finish();
}

#[test]
fn test_criterion_doesnt_panic_if_measured_time_is_zero() {
    let dir = temp_dir();
    let mut c = short_benchmark(&dir);
    c.bench_function("zero_time", |bencher| {
        bencher.iter_custom(|_iters| Duration::new(0, 0))
    });
}

mod macros {
    use super::{criterion_group, criterion_main, Criterion};

    #[test]
    #[should_panic(expected = "group executed")]
    fn criterion_main() {
        fn group() {}
        fn group2() {
            panic!("group executed");
        }

        criterion_main!(group, group2);

        main();
    }

    #[test]
    fn criterion_main_trailing_comma() {
        // make this a compile-only check
        // as the second logger initialization causes panic
        #[allow(dead_code)]
        fn group() {}
        #[allow(dead_code)]
        fn group2() {}

        criterion_main!(group, group2,);

        // silence dead_code warning
        if false {
            main()
        }
    }

    #[test]
    #[should_panic(expected = "group executed")]
    fn criterion_group() {
        fn group(_crit: &mut Criterion) {}
        fn group2(_crit: &mut Criterion) {
            panic!("group executed");
        }

        criterion_group!(test_group, group, group2);

        test_group();
    }

    #[test]
    #[should_panic(expected = "group executed")]
    fn criterion_group_trailing_comma() {
        fn group(_crit: &mut Criterion) {}
        fn group2(_crit: &mut Criterion) {
            panic!("group executed");
        }

        criterion_group!(test_group, group, group2,);

        test_group();
    }
}

struct TestProfiler {
    started: Rc<Cell<u32>>,
    stopped: Rc<Cell<u32>>,
}
impl Profiler for TestProfiler {
    fn start_profiling(&mut self, benchmark_id: &str, _benchmark_path: &Path) {
        assert!(benchmark_id.contains("profile_test"));
        self.started.set(self.started.get() + 1);
    }
    fn stop_profiling(&mut self, benchmark_id: &str, _benchmark_path: &Path) {
        assert!(benchmark_id.contains("profile_test"));
        self.stopped.set(self.stopped.get() + 1);
    }
}

// Verify that profilers are started and stopped as expected
#[test]
fn test_profiler_called() {
    let started = Rc::new(Cell::new(0u32));
    let stopped = Rc::new(Cell::new(0u32));
    let profiler = TestProfiler {
        started: started.clone(),
        stopped: stopped.clone(),
    };
    let dir = temp_dir();
    let mut criterion = short_benchmark(&dir)
        .with_profiler(profiler)
        .profile_time(Some(Duration::from_secs(1)));
    criterion.bench_function("profile_test", |b| b.iter(|| 10));
    assert_eq!(1, started.get());
    assert_eq!(1, stopped.get());
}

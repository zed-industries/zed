use crate::benchmark::BenchmarkConfig;
use crate::connection::OutgoingMessage;
use crate::measurement::Measurement;
use crate::report::{BenchmarkId, Report, ReportContext};
use crate::{black_box, ActualSamplingMode, Bencher, Criterion};
use std::marker::PhantomData;
use std::time::Duration;

/// PRIVATE
pub(crate) trait Routine<M: Measurement, T: ?Sized> {
    /// PRIVATE
    fn bench(&mut self, m: &M, iters: &[u64], parameter: &T) -> Vec<f64>;
    /// PRIVATE
    fn warm_up(&mut self, m: &M, how_long: Duration, parameter: &T) -> (u64, u64);

    /// PRIVATE
    fn test(&mut self, m: &M, parameter: &T) {
        self.bench(m, &[1u64], parameter);
    }

    /// Iterates the benchmarked function for a fixed length of time, but takes no measurements.
    /// This keeps the overall benchmark suite runtime constant-ish even when running under a
    /// profiler with an unknown amount of overhead. Since no measurements are taken, it also
    /// reduces the amount of time the execution spends in Criterion.rs code, which should help
    /// show the performance of the benchmarked code more clearly as well.
    fn profile(
        &mut self,
        measurement: &M,
        id: &BenchmarkId,
        criterion: &Criterion<M>,
        report_context: &ReportContext,
        time: Duration,
        parameter: &T,
    ) {
        criterion
            .report
            .profile(id, report_context, time.as_nanos() as f64);

        let mut profile_path = report_context.output_directory.clone();
        if (*crate::CARGO_CRITERION_CONNECTION).is_some() {
            // If connected to cargo-criterion, generate a cargo-criterion-style path.
            // This is kind of a hack.
            profile_path.push("profile");
            profile_path.push(id.as_directory_name());
        } else {
            profile_path.push(id.as_directory_name());
            profile_path.push("profile");
        }
        criterion
            .profiler
            .borrow_mut()
            .start_profiling(id.id(), &profile_path);

        let time = time.as_nanos() as u64;

        // TODO: Some profilers will show the two batches of iterations as
        // being different code-paths even though they aren't really.

        // Get the warmup time for one second
        let (wu_elapsed, wu_iters) = self.warm_up(measurement, Duration::from_secs(1), parameter);
        if wu_elapsed < time {
            // Initial guess for the mean execution time
            let met = wu_elapsed as f64 / wu_iters as f64;

            // Guess how many iterations will be required for the remaining time
            let remaining = (time - wu_elapsed) as f64;

            let iters = remaining / met;
            let iters = iters as u64;

            self.bench(measurement, &[iters], parameter);
        }

        criterion
            .profiler
            .borrow_mut()
            .stop_profiling(id.id(), &profile_path);

        criterion.report.terminated(id, report_context);
    }

    fn sample(
        &mut self,
        measurement: &M,
        id: &BenchmarkId,
        config: &BenchmarkConfig,
        criterion: &Criterion<M>,
        report_context: &ReportContext,
        parameter: &T,
    ) -> (ActualSamplingMode, Box<[f64]>, Box<[f64]>) {
        if config.quick_mode {
            let minimum_bench_duration = Duration::from_millis(100);
            let maximum_bench_duration = config.measurement_time; // default: 5 seconds
            let target_rel_stdev = config.significance_level; // default: 5%, 0.05

            use std::time::Instant;
            let time_start = Instant::now();

            let sq = |val| val * val;
            let mut n = 1;
            let mut t_prev = *self.bench(measurement, &[n], parameter).first().unwrap();

            // Early exit for extremely long running benchmarks:
            if time_start.elapsed() > maximum_bench_duration {
                let iters = vec![n as f64, n as f64].into_boxed_slice();
                // prevent gnuplot bug when all values are equal
                let elapsed = vec![t_prev, t_prev + 0.000001].into_boxed_slice();
                return (ActualSamplingMode::Flat, iters, elapsed);
            }

            // Main data collection loop.
            loop {
                let t_now = *self
                    .bench(measurement, &[n * 2], parameter)
                    .first()
                    .unwrap();
                let t = (t_prev + 2. * t_now) / 5.;
                let stdev = (sq(t_prev - t) + sq(t_now - 2. * t)).sqrt();
                // println!("Sample: {} {:.2}", n, stdev / t);
                let elapsed = time_start.elapsed();
                if (stdev < target_rel_stdev * t && elapsed > minimum_bench_duration)
                    || elapsed > maximum_bench_duration
                {
                    let iters = vec![n as f64, (n * 2) as f64].into_boxed_slice();
                    let elapsed = vec![t_prev, t_now].into_boxed_slice();
                    return (ActualSamplingMode::Linear, iters, elapsed);
                }
                n *= 2;
                t_prev = t_now;
            }
        }
        let wu = config.warm_up_time;
        let m_ns = config.measurement_time.as_nanos();

        criterion
            .report
            .warmup(id, report_context, wu.as_nanos() as f64);

        if let Some(conn) = &criterion.connection {
            conn.send(&OutgoingMessage::Warmup {
                id: id.into(),
                nanos: wu.as_nanos() as f64,
            })
            .unwrap();
        }

        let (wu_elapsed, wu_iters) = self.warm_up(measurement, wu, parameter);
        if crate::debug_enabled() {
            println!(
                "\nCompleted {} iterations in {} nanoseconds, estimated execution time is {} ns",
                wu_iters,
                wu_elapsed,
                wu_elapsed as f64 / wu_iters as f64
            );
        }

        // Initial guess for the mean execution time
        let met = wu_elapsed as f64 / wu_iters as f64;

        let n = config.sample_size as u64;

        let actual_sampling_mode = config
            .sampling_mode
            .choose_sampling_mode(met, n, m_ns as f64);

        let m_iters = actual_sampling_mode.iteration_counts(met, n, &config.measurement_time);

        let expected_ns = m_iters
            .iter()
            .copied()
            .map(|count| count as f64 * met)
            .sum();

        // Use saturating_add to handle overflow.
        let mut total_iters = 0u64;
        for count in m_iters.iter().copied() {
            total_iters = total_iters.saturating_add(count);
        }

        criterion
            .report
            .measurement_start(id, report_context, n, expected_ns, total_iters);

        if let Some(conn) = &criterion.connection {
            conn.send(&OutgoingMessage::MeasurementStart {
                id: id.into(),
                sample_count: n,
                estimate_ns: expected_ns,
                iter_count: total_iters,
            })
            .unwrap();
        }

        let m_elapsed = self.bench(measurement, &m_iters, parameter);

        let m_iters_f: Vec<f64> = m_iters.iter().map(|&x| x as f64).collect();

        (
            actual_sampling_mode,
            m_iters_f.into_boxed_slice(),
            m_elapsed.into_boxed_slice(),
        )
    }
}

pub struct Function<M: Measurement, F, T>
where
    F: FnMut(&mut Bencher<'_, M>, &T),
    T: ?Sized,
{
    f: F,
    // TODO: Is there some way to remove these?
    _phantom: PhantomData<T>,
    _phamtom2: PhantomData<M>,
}
impl<M: Measurement, F, T> Function<M, F, T>
where
    F: FnMut(&mut Bencher<'_, M>, &T),
    T: ?Sized,
{
    pub fn new(f: F) -> Function<M, F, T> {
        Function {
            f,
            _phantom: PhantomData,
            _phamtom2: PhantomData,
        }
    }
}

impl<M: Measurement, F, T> Routine<M, T> for Function<M, F, T>
where
    F: FnMut(&mut Bencher<'_, M>, &T),
    T: ?Sized,
{
    fn bench(&mut self, m: &M, iters: &[u64], parameter: &T) -> Vec<f64> {
        let f = &mut self.f;

        let mut b = Bencher {
            iterated: false,
            iters: 0,
            value: m.zero(),
            measurement: m,
            elapsed_time: Duration::from_millis(0),
        };

        iters
            .iter()
            .map(|iters| {
                b.iters = *iters;
                (*f)(&mut b, black_box(parameter));
                b.assert_iterated();
                m.to_f64(&b.value)
            })
            .collect()
    }

    fn warm_up(&mut self, m: &M, how_long: Duration, parameter: &T) -> (u64, u64) {
        let f = &mut self.f;
        let mut b = Bencher {
            iterated: false,
            iters: 1,
            value: m.zero(),
            measurement: m,
            elapsed_time: Duration::from_millis(0),
        };

        let mut total_iters = 0;
        let mut elapsed_time = Duration::from_millis(0);
        loop {
            (*f)(&mut b, black_box(parameter));

            b.assert_iterated();

            total_iters += b.iters;
            elapsed_time += b.elapsed_time;
            if elapsed_time > how_long {
                return (elapsed_time.as_nanos() as u64, total_iters);
            }

            b.iters = b.iters.wrapping_mul(2);
        }
    }
}

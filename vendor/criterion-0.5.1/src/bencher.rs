use std::iter::IntoIterator;
use std::time::Duration;
use std::time::Instant;

use crate::black_box;
use crate::measurement::{Measurement, WallTime};
use crate::BatchSize;

#[cfg(feature = "async")]
use std::future::Future;

#[cfg(feature = "async")]
use crate::async_executor::AsyncExecutor;

// ================================== MAINTENANCE NOTE =============================================
// Any changes made to either Bencher or AsyncBencher will have to be replicated to the other!
// ================================== MAINTENANCE NOTE =============================================

/// Timer struct used to iterate a benchmarked function and measure the runtime.
///
/// This struct provides different timing loops as methods. Each timing loop provides a different
/// way to time a routine and each has advantages and disadvantages.
///
/// * If you want to do the iteration and measurement yourself (eg. passing the iteration count
///   to a separate process), use `iter_custom`.
/// * If your routine requires no per-iteration setup and returns a value with an expensive `drop`
///   method, use `iter_with_large_drop`.
/// * If your routine requires some per-iteration setup that shouldn't be timed, use `iter_batched`
///   or `iter_batched_ref`. See [`BatchSize`](enum.BatchSize.html) for a discussion of batch sizes.
///   If the setup value implements `Drop` and you don't want to include the `drop` time in the
///   measurement, use `iter_batched_ref`, otherwise use `iter_batched`. These methods are also
///   suitable for benchmarking routines which return a value with an expensive `drop` method,
///   but are more complex than `iter_with_large_drop`.
/// * Otherwise, use `iter`.
pub struct Bencher<'a, M: Measurement = WallTime> {
    pub(crate) iterated: bool,         // Have we iterated this benchmark?
    pub(crate) iters: u64,             // Number of times to iterate this benchmark
    pub(crate) value: M::Value,        // The measured value
    pub(crate) measurement: &'a M,     // Reference to the measurement object
    pub(crate) elapsed_time: Duration, // How much time did it take to perform the iteration? Used for the warmup period.
}
impl<'a, M: Measurement> Bencher<'a, M> {
    /// Times a `routine` by executing it many times and timing the total elapsed time.
    ///
    /// Prefer this timing loop when `routine` returns a value that doesn't have a destructor.
    ///
    /// # Timing model
    ///
    /// Note that the `Bencher` also times the time required to destroy the output of `routine()`.
    /// Therefore prefer this timing loop when the runtime of `mem::drop(O)` is negligible compared
    /// to the runtime of the `routine`.
    ///
    /// ```text
    /// elapsed = Instant::now + iters * (routine + mem::drop(O) + Range::next)
    /// ```
    ///
    /// # Example
    ///
    /// ```rust
    /// #[macro_use] extern crate criterion;
    ///
    /// use criterion::*;
    ///
    /// // The function to benchmark
    /// fn foo() {
    ///     // ...
    /// }
    ///
    /// fn bench(c: &mut Criterion) {
    ///     c.bench_function("iter", move |b| {
    ///         b.iter(|| foo())
    ///     });
    /// }
    ///
    /// criterion_group!(benches, bench);
    /// criterion_main!(benches);
    /// ```
    ///
    #[inline(never)]
    pub fn iter<O, R>(&mut self, mut routine: R)
    where
        R: FnMut() -> O,
    {
        self.iterated = true;
        let time_start = Instant::now();
        let start = self.measurement.start();
        for _ in 0..self.iters {
            black_box(routine());
        }
        self.value = self.measurement.end(start);
        self.elapsed_time = time_start.elapsed();
    }

    /// Times a `routine` by executing it many times and relying on `routine` to measure its own execution time.
    ///
    /// Prefer this timing loop in cases where `routine` has to do its own measurements to
    /// get accurate timing information (for example in multi-threaded scenarios where you spawn
    /// and coordinate with multiple threads).
    ///
    /// # Timing model
    /// Custom, the timing model is whatever is returned as the Duration from `routine`.
    ///
    /// # Example
    /// ```rust
    /// #[macro_use] extern crate criterion;
    /// use criterion::*;
    /// use criterion::black_box;
    /// use std::time::Instant;
    ///
    /// fn foo() {
    ///     // ...
    /// }
    ///
    /// fn bench(c: &mut Criterion) {
    ///     c.bench_function("iter", move |b| {
    ///         b.iter_custom(|iters| {
    ///             let start = Instant::now();
    ///             for _i in 0..iters {
    ///                 black_box(foo());
    ///             }
    ///             start.elapsed()
    ///         })
    ///     });
    /// }
    ///
    /// criterion_group!(benches, bench);
    /// criterion_main!(benches);
    /// ```
    ///
    #[inline(never)]
    pub fn iter_custom<R>(&mut self, mut routine: R)
    where
        R: FnMut(u64) -> M::Value,
    {
        self.iterated = true;
        let time_start = Instant::now();
        self.value = routine(self.iters);
        self.elapsed_time = time_start.elapsed();
    }

    #[doc(hidden)]
    pub fn iter_with_setup<I, O, S, R>(&mut self, setup: S, routine: R)
    where
        S: FnMut() -> I,
        R: FnMut(I) -> O,
    {
        self.iter_batched(setup, routine, BatchSize::PerIteration);
    }

    /// Times a `routine` by collecting its output on each iteration. This avoids timing the
    /// destructor of the value returned by `routine`.
    ///
    /// WARNING: This requires `O(iters * mem::size_of::<O>())` of memory, and `iters` is not under the
    /// control of the caller. If this causes out-of-memory errors, use `iter_batched` instead.
    ///
    /// # Timing model
    ///
    /// ``` text
    /// elapsed = Instant::now + iters * (routine) + Iterator::collect::<Vec<_>>
    /// ```
    ///
    /// # Example
    ///
    /// ```rust
    /// #[macro_use] extern crate criterion;
    ///
    /// use criterion::*;
    ///
    /// fn create_vector() -> Vec<u64> {
    ///     # vec![]
    ///     // ...
    /// }
    ///
    /// fn bench(c: &mut Criterion) {
    ///     c.bench_function("with_drop", move |b| {
    ///         // This will avoid timing the Vec::drop.
    ///         b.iter_with_large_drop(|| create_vector())
    ///     });
    /// }
    ///
    /// criterion_group!(benches, bench);
    /// criterion_main!(benches);
    /// ```
    ///
    pub fn iter_with_large_drop<O, R>(&mut self, mut routine: R)
    where
        R: FnMut() -> O,
    {
        self.iter_batched(|| (), |_| routine(), BatchSize::SmallInput);
    }

    /// Times a `routine` that requires some input by generating a batch of input, then timing the
    /// iteration of the benchmark over the input. See [`BatchSize`](enum.BatchSize.html) for
    /// details on choosing the batch size. Use this when the routine must consume its input.
    ///
    /// For example, use this loop to benchmark sorting algorithms, because they require unsorted
    /// data on each iteration.
    ///
    /// # Timing model
    ///
    /// ```text
    /// elapsed = (Instant::now * num_batches) + (iters * (routine + O::drop)) + Vec::extend
    /// ```
    ///
    /// # Example
    ///
    /// ```rust
    /// #[macro_use] extern crate criterion;
    ///
    /// use criterion::*;
    ///
    /// fn create_scrambled_data() -> Vec<u64> {
    ///     # vec![]
    ///     // ...
    /// }
    ///
    /// // The sorting algorithm to test
    /// fn sort(data: &mut [u64]) {
    ///     // ...
    /// }
    ///
    /// fn bench(c: &mut Criterion) {
    ///     let data = create_scrambled_data();
    ///
    ///     c.bench_function("with_setup", move |b| {
    ///         // This will avoid timing the to_vec call.
    ///         b.iter_batched(|| data.clone(), |mut data| sort(&mut data), BatchSize::SmallInput)
    ///     });
    /// }
    ///
    /// criterion_group!(benches, bench);
    /// criterion_main!(benches);
    /// ```
    ///
    #[inline(never)]
    pub fn iter_batched<I, O, S, R>(&mut self, mut setup: S, mut routine: R, size: BatchSize)
    where
        S: FnMut() -> I,
        R: FnMut(I) -> O,
    {
        self.iterated = true;
        let batch_size = size.iters_per_batch(self.iters);
        assert!(batch_size != 0, "Batch size must not be zero.");
        let time_start = Instant::now();
        self.value = self.measurement.zero();

        if batch_size == 1 {
            for _ in 0..self.iters {
                let input = black_box(setup());

                let start = self.measurement.start();
                let output = routine(input);
                let end = self.measurement.end(start);
                self.value = self.measurement.add(&self.value, &end);

                drop(black_box(output));
            }
        } else {
            let mut iteration_counter = 0;

            while iteration_counter < self.iters {
                let batch_size = ::std::cmp::min(batch_size, self.iters - iteration_counter);

                let inputs = black_box((0..batch_size).map(|_| setup()).collect::<Vec<_>>());
                let mut outputs = Vec::with_capacity(batch_size as usize);

                let start = self.measurement.start();
                outputs.extend(inputs.into_iter().map(&mut routine));
                let end = self.measurement.end(start);
                self.value = self.measurement.add(&self.value, &end);

                black_box(outputs);

                iteration_counter += batch_size;
            }
        }

        self.elapsed_time = time_start.elapsed();
    }

    /// Times a `routine` that requires some input by generating a batch of input, then timing the
    /// iteration of the benchmark over the input. See [`BatchSize`](enum.BatchSize.html) for
    /// details on choosing the batch size. Use this when the routine should accept the input by
    /// mutable reference.
    ///
    /// For example, use this loop to benchmark sorting algorithms, because they require unsorted
    /// data on each iteration.
    ///
    /// # Timing model
    ///
    /// ```text
    /// elapsed = (Instant::now * num_batches) + (iters * routine) + Vec::extend
    /// ```
    ///
    /// # Example
    ///
    /// ```rust
    /// #[macro_use] extern crate criterion;
    ///
    /// use criterion::*;
    ///
    /// fn create_scrambled_data() -> Vec<u64> {
    ///     # vec![]
    ///     // ...
    /// }
    ///
    /// // The sorting algorithm to test
    /// fn sort(data: &mut [u64]) {
    ///     // ...
    /// }
    ///
    /// fn bench(c: &mut Criterion) {
    ///     let data = create_scrambled_data();
    ///
    ///     c.bench_function("with_setup", move |b| {
    ///         // This will avoid timing the to_vec call.
    ///         b.iter_batched(|| data.clone(), |mut data| sort(&mut data), BatchSize::SmallInput)
    ///     });
    /// }
    ///
    /// criterion_group!(benches, bench);
    /// criterion_main!(benches);
    /// ```
    ///
    #[inline(never)]
    pub fn iter_batched_ref<I, O, S, R>(&mut self, mut setup: S, mut routine: R, size: BatchSize)
    where
        S: FnMut() -> I,
        R: FnMut(&mut I) -> O,
    {
        self.iterated = true;
        let batch_size = size.iters_per_batch(self.iters);
        assert!(batch_size != 0, "Batch size must not be zero.");
        let time_start = Instant::now();
        self.value = self.measurement.zero();

        if batch_size == 1 {
            for _ in 0..self.iters {
                let mut input = black_box(setup());

                let start = self.measurement.start();
                let output = routine(&mut input);
                let end = self.measurement.end(start);
                self.value = self.measurement.add(&self.value, &end);

                drop(black_box(output));
                drop(black_box(input));
            }
        } else {
            let mut iteration_counter = 0;

            while iteration_counter < self.iters {
                let batch_size = ::std::cmp::min(batch_size, self.iters - iteration_counter);

                let mut inputs = black_box((0..batch_size).map(|_| setup()).collect::<Vec<_>>());
                let mut outputs = Vec::with_capacity(batch_size as usize);

                let start = self.measurement.start();
                outputs.extend(inputs.iter_mut().map(&mut routine));
                let end = self.measurement.end(start);
                self.value = self.measurement.add(&self.value, &end);

                black_box(outputs);

                iteration_counter += batch_size;
            }
        }
        self.elapsed_time = time_start.elapsed();
    }

    // Benchmarks must actually call one of the iter methods. This causes benchmarks to fail loudly
    // if they don't.
    pub(crate) fn assert_iterated(&mut self) {
        assert!(
            self.iterated,
            "Benchmark function must call Bencher::iter or related method."
        );
        self.iterated = false;
    }

    /// Convert this bencher into an AsyncBencher, which enables async/await support.
    #[cfg(feature = "async")]
    pub fn to_async<'b, A: AsyncExecutor>(&'b mut self, runner: A) -> AsyncBencher<'a, 'b, A, M> {
        AsyncBencher { b: self, runner }
    }
}

/// Async/await variant of the Bencher struct.
#[cfg(feature = "async")]
pub struct AsyncBencher<'a, 'b, A: AsyncExecutor, M: Measurement = WallTime> {
    b: &'b mut Bencher<'a, M>,
    runner: A,
}
#[cfg(feature = "async")]
impl<'a, 'b, A: AsyncExecutor, M: Measurement> AsyncBencher<'a, 'b, A, M> {
    /// Times a `routine` by executing it many times and timing the total elapsed time.
    ///
    /// Prefer this timing loop when `routine` returns a value that doesn't have a destructor.
    ///
    /// # Timing model
    ///
    /// Note that the `AsyncBencher` also times the time required to destroy the output of `routine()`.
    /// Therefore prefer this timing loop when the runtime of `mem::drop(O)` is negligible compared
    /// to the runtime of the `routine`.
    ///
    /// ```text
    /// elapsed = Instant::now + iters * (routine + mem::drop(O) + Range::next)
    /// ```
    ///
    /// # Example
    ///
    /// ```rust
    /// #[macro_use] extern crate criterion;
    ///
    /// use criterion::*;
    /// use criterion::async_executor::FuturesExecutor;
    ///
    /// // The function to benchmark
    /// async fn foo() {
    ///     // ...
    /// }
    ///
    /// fn bench(c: &mut Criterion) {
    ///     c.bench_function("iter", move |b| {
    ///         b.to_async(FuturesExecutor).iter(|| async { foo().await } )
    ///     });
    /// }
    ///
    /// criterion_group!(benches, bench);
    /// criterion_main!(benches);
    /// ```
    ///
    #[inline(never)]
    pub fn iter<O, R, F>(&mut self, mut routine: R)
    where
        R: FnMut() -> F,
        F: Future<Output = O>,
    {
        let AsyncBencher { b, runner } = self;
        runner.block_on(async {
            b.iterated = true;
            let time_start = Instant::now();
            let start = b.measurement.start();
            for _ in 0..b.iters {
                black_box(routine().await);
            }
            b.value = b.measurement.end(start);
            b.elapsed_time = time_start.elapsed();
        });
    }

    /// Times a `routine` by executing it many times and relying on `routine` to measure its own execution time.
    ///
    /// Prefer this timing loop in cases where `routine` has to do its own measurements to
    /// get accurate timing information (for example in multi-threaded scenarios where you spawn
    /// and coordinate with multiple threads).
    ///
    /// # Timing model
    /// Custom, the timing model is whatever is returned as the Duration from `routine`.
    ///
    /// # Example
    /// ```rust
    /// #[macro_use] extern crate criterion;
    /// use criterion::*;
    /// use criterion::black_box;
    /// use criterion::async_executor::FuturesExecutor;
    /// use std::time::Instant;
    ///
    /// async fn foo() {
    ///     // ...
    /// }
    ///
    /// fn bench(c: &mut Criterion) {
    ///     c.bench_function("iter", move |b| {
    ///         b.to_async(FuturesExecutor).iter_custom(|iters| {
    ///             async move {
    ///                 let start = Instant::now();
    ///                 for _i in 0..iters {
    ///                     black_box(foo().await);
    ///                 }
    ///                 start.elapsed()
    ///             }
    ///         })
    ///     });
    /// }
    ///
    /// criterion_group!(benches, bench);
    /// criterion_main!(benches);
    /// ```
    ///
    #[inline(never)]
    pub fn iter_custom<R, F>(&mut self, mut routine: R)
    where
        R: FnMut(u64) -> F,
        F: Future<Output = M::Value>,
    {
        let AsyncBencher { b, runner } = self;
        runner.block_on(async {
            b.iterated = true;
            let time_start = Instant::now();
            b.value = routine(b.iters).await;
            b.elapsed_time = time_start.elapsed();
        })
    }

    #[doc(hidden)]
    pub fn iter_with_setup<I, O, S, R, F>(&mut self, setup: S, routine: R)
    where
        S: FnMut() -> I,
        R: FnMut(I) -> F,
        F: Future<Output = O>,
    {
        self.iter_batched(setup, routine, BatchSize::PerIteration);
    }

    /// Times a `routine` by collecting its output on each iteration. This avoids timing the
    /// destructor of the value returned by `routine`.
    ///
    /// WARNING: This requires `O(iters * mem::size_of::<O>())` of memory, and `iters` is not under the
    /// control of the caller. If this causes out-of-memory errors, use `iter_batched` instead.
    ///
    /// # Timing model
    ///
    /// ``` text
    /// elapsed = Instant::now + iters * (routine) + Iterator::collect::<Vec<_>>
    /// ```
    ///
    /// # Example
    ///
    /// ```rust
    /// #[macro_use] extern crate criterion;
    ///
    /// use criterion::*;
    /// use criterion::async_executor::FuturesExecutor;
    ///
    /// async fn create_vector() -> Vec<u64> {
    ///     # vec![]
    ///     // ...
    /// }
    ///
    /// fn bench(c: &mut Criterion) {
    ///     c.bench_function("with_drop", move |b| {
    ///         // This will avoid timing the Vec::drop.
    ///         b.to_async(FuturesExecutor).iter_with_large_drop(|| async { create_vector().await })
    ///     });
    /// }
    ///
    /// criterion_group!(benches, bench);
    /// criterion_main!(benches);
    /// ```
    ///
    pub fn iter_with_large_drop<O, R, F>(&mut self, mut routine: R)
    where
        R: FnMut() -> F,
        F: Future<Output = O>,
    {
        self.iter_batched(|| (), |_| routine(), BatchSize::SmallInput);
    }

    #[doc(hidden)]
    pub fn iter_with_large_setup<I, O, S, R, F>(&mut self, setup: S, routine: R)
    where
        S: FnMut() -> I,
        R: FnMut(I) -> F,
        F: Future<Output = O>,
    {
        self.iter_batched(setup, routine, BatchSize::NumBatches(1));
    }

    /// Times a `routine` that requires some input by generating a batch of input, then timing the
    /// iteration of the benchmark over the input. See [`BatchSize`](enum.BatchSize.html) for
    /// details on choosing the batch size. Use this when the routine must consume its input.
    ///
    /// For example, use this loop to benchmark sorting algorithms, because they require unsorted
    /// data on each iteration.
    ///
    /// # Timing model
    ///
    /// ```text
    /// elapsed = (Instant::now * num_batches) + (iters * (routine + O::drop)) + Vec::extend
    /// ```
    ///
    /// # Example
    ///
    /// ```rust
    /// #[macro_use] extern crate criterion;
    ///
    /// use criterion::*;
    /// use criterion::async_executor::FuturesExecutor;
    ///
    /// fn create_scrambled_data() -> Vec<u64> {
    ///     # vec![]
    ///     // ...
    /// }
    ///
    /// // The sorting algorithm to test
    /// async fn sort(data: &mut [u64]) {
    ///     // ...
    /// }
    ///
    /// fn bench(c: &mut Criterion) {
    ///     let data = create_scrambled_data();
    ///
    ///     c.bench_function("with_setup", move |b| {
    ///         // This will avoid timing the to_vec call.
    ///         b.iter_batched(|| data.clone(), |mut data| async move { sort(&mut data).await }, BatchSize::SmallInput)
    ///     });
    /// }
    ///
    /// criterion_group!(benches, bench);
    /// criterion_main!(benches);
    /// ```
    ///
    #[inline(never)]
    pub fn iter_batched<I, O, S, R, F>(&mut self, mut setup: S, mut routine: R, size: BatchSize)
    where
        S: FnMut() -> I,
        R: FnMut(I) -> F,
        F: Future<Output = O>,
    {
        let AsyncBencher { b, runner } = self;
        runner.block_on(async {
            b.iterated = true;
            let batch_size = size.iters_per_batch(b.iters);
            assert!(batch_size != 0, "Batch size must not be zero.");
            let time_start = Instant::now();
            b.value = b.measurement.zero();

            if batch_size == 1 {
                for _ in 0..b.iters {
                    let input = black_box(setup());

                    let start = b.measurement.start();
                    let output = routine(input).await;
                    let end = b.measurement.end(start);
                    b.value = b.measurement.add(&b.value, &end);

                    drop(black_box(output));
                }
            } else {
                let mut iteration_counter = 0;

                while iteration_counter < b.iters {
                    let batch_size = ::std::cmp::min(batch_size, b.iters - iteration_counter);

                    let inputs = black_box((0..batch_size).map(|_| setup()).collect::<Vec<_>>());
                    let mut outputs = Vec::with_capacity(batch_size as usize);

                    let start = b.measurement.start();
                    // Can't use .extend here like the sync version does
                    for input in inputs {
                        outputs.push(routine(input).await);
                    }
                    let end = b.measurement.end(start);
                    b.value = b.measurement.add(&b.value, &end);

                    black_box(outputs);

                    iteration_counter += batch_size;
                }
            }

            b.elapsed_time = time_start.elapsed();
        })
    }

    /// Times a `routine` that requires some input by generating a batch of input, then timing the
    /// iteration of the benchmark over the input. See [`BatchSize`](enum.BatchSize.html) for
    /// details on choosing the batch size. Use this when the routine should accept the input by
    /// mutable reference.
    ///
    /// For example, use this loop to benchmark sorting algorithms, because they require unsorted
    /// data on each iteration.
    ///
    /// # Timing model
    ///
    /// ```text
    /// elapsed = (Instant::now * num_batches) + (iters * routine) + Vec::extend
    /// ```
    ///
    /// # Example
    ///
    /// ```rust
    /// #[macro_use] extern crate criterion;
    ///
    /// use criterion::*;
    /// use criterion::async_executor::FuturesExecutor;
    ///
    /// fn create_scrambled_data() -> Vec<u64> {
    ///     # vec![]
    ///     // ...
    /// }
    ///
    /// // The sorting algorithm to test
    /// async fn sort(data: &mut [u64]) {
    ///     // ...
    /// }
    ///
    /// fn bench(c: &mut Criterion) {
    ///     let data = create_scrambled_data();
    ///
    ///     c.bench_function("with_setup", move |b| {
    ///         // This will avoid timing the to_vec call.
    ///         b.iter_batched(|| data.clone(), |mut data| async move { sort(&mut data).await }, BatchSize::SmallInput)
    ///     });
    /// }
    ///
    /// criterion_group!(benches, bench);
    /// criterion_main!(benches);
    /// ```
    ///
    #[inline(never)]
    pub fn iter_batched_ref<I, O, S, R, F>(&mut self, mut setup: S, mut routine: R, size: BatchSize)
    where
        S: FnMut() -> I,
        R: FnMut(&mut I) -> F,
        F: Future<Output = O>,
    {
        let AsyncBencher { b, runner } = self;
        runner.block_on(async {
            b.iterated = true;
            let batch_size = size.iters_per_batch(b.iters);
            assert!(batch_size != 0, "Batch size must not be zero.");
            let time_start = Instant::now();
            b.value = b.measurement.zero();

            if batch_size == 1 {
                for _ in 0..b.iters {
                    let mut input = black_box(setup());

                    let start = b.measurement.start();
                    let output = routine(&mut input).await;
                    let end = b.measurement.end(start);
                    b.value = b.measurement.add(&b.value, &end);

                    drop(black_box(output));
                    drop(black_box(input));
                }
            } else {
                let mut iteration_counter = 0;

                while iteration_counter < b.iters {
                    let batch_size = ::std::cmp::min(batch_size, b.iters - iteration_counter);

                    let inputs = black_box((0..batch_size).map(|_| setup()).collect::<Vec<_>>());
                    let mut outputs = Vec::with_capacity(batch_size as usize);

                    let start = b.measurement.start();
                    // Can't use .extend here like the sync version does
                    for mut input in inputs {
                        outputs.push(routine(&mut input).await);
                    }
                    let end = b.measurement.end(start);
                    b.value = b.measurement.add(&b.value, &end);

                    black_box(outputs);

                    iteration_counter += batch_size;
                }
            }
            b.elapsed_time = time_start.elapsed();
        });
    }
}

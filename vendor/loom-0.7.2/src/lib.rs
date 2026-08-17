#![deny(missing_debug_implementations, missing_docs, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Loom is a tool for testing concurrent programs.
//!
//! At a high level, it runs tests many times, permuting the possible concurrent executions of each
//! test according to what constitutes valid executions under the [C11 memory model][spec]. It then
//! uses state reduction techniques to avoid combinatorial explosion of the number of possible
//! executions.
//!
//! # Background
//!
//! Testing concurrent programs is challenging; concurrent strands of execution can interleave in
//! all sorts of ways, and each such interleaving might expose a concurrency bug in the program.
//! Some bugs may be so rare that they only occur under a very small set of possible executions,
//! and may not surface even if you run the code millions or billions of times.
//!
//! Loom provides a way to deterministically explore the various possible execution permutations
//! without relying on random executions. This allows you to write tests that verify that your
//! concurrent code is correct under _all_ executions, not just "most of the time".
//!
//! Consider a simple example:
//!
//! ```no_run
//! use std::sync::Arc;
//! use std::sync::atomic::AtomicUsize;
//! use std::sync::atomic::Ordering::SeqCst;
//! use std::thread;
//!
//! # /*
//! #[test]
//! # */
//! fn test_concurrent_logic() {
//!     let v1 = Arc::new(AtomicUsize::new(0));
//!     let v2 = v1.clone();
//!
//!     thread::spawn(move || {
//!         v1.store(1, SeqCst);
//!     });
//!
//!     assert_eq!(0, v2.load(SeqCst));
//! }
//! ```
//!
//! This program is incorrect: the main thread might yield between spawning the thread that stores
//! to `v1` and loading from `v2`, during which time the spawned thread may get to run and store 1
//! into `v1`. **Most** of the time, the main thread will get to the assertion before the spawned
//! thread executes, so the assertion will succeed. But, every once in a while, the spawned thread
//! manages to run just in time and the assertion will fail! This is obviously a contrived example,
//! but in practice many concurrent programs exhibit similar behavior -- they operate correctly
//! under most executions, but _some_ executions end up producing buggy behavior.
//!
//! Historically, the strategy for testing concurrent code has been to run tests in loops and hope
//! that an execution fails. Or to place the testing host under load while running the test suite
//! in an attempt to produce less frequently exercised executions. However, this kind of testing is
//! not reliable, and, in the event an iteration should fail, debugging the cause is exceedingly
//! difficult.
//!
//! The problem is compounded when other memory orderings than `SeqCst` are considered, where bugs
//! may only occur on hardware with particular memory characteristics, and thus **no** amount of
//! iteration will demonstrate the bug on different hardware!
//!
//! # Solution
//!
//! Loom fixes the problem by simulating the operating system's scheduler and Rust's memory model
//! such that all possible valid behaviors are explored and tested. To see how this works out in
//! practice, the above example can be rewritten to use loom's concurrency types as:
//!
//! ```no_run
//! use loom::sync::atomic::AtomicUsize;
//! use loom::thread;
//!
//! use std::sync::Arc;
//! use std::sync::atomic::Ordering::SeqCst;
//!
//! # /*
//! #[test]
//! # */
//! fn test_concurrent_logic() {
//!     loom::model(|| {
//!         let v1 = Arc::new(AtomicUsize::new(0));
//!         let v2 = v1.clone();
//!
//!         thread::spawn(move || {
//!             v1.store(1, SeqCst);
//!         });
//!
//!         assert_eq!(0, v2.load(SeqCst));
//!     });
//! }
//! ```
//!
//! Loom will run the closure provided to `loom::model` many times over, and each time a different
//! thread scheduling will be used. That is, one execution will have the spawned thread run after
//! the load from `v2`, and another will have the spawned thread run before the store to `v2`.
//! Thus, the test is guaranteed to fail.
//!
//! # Writing tests
//!
//! Test cases using loom must be fully deterministic. All sources of non-determism must be via loom
//! types so that loom can expose different possible values on each execution of the test closure.
//! Other sources of non-determinism like random number generation or system calls cannot be
//! modeled directly by loom, and must be mocked to be testable by loom.
//!
//! To model synchronization non-determinism, tests must use the loom synchronization types, such
//! as [`Atomic*`](sync::atomic), [`Mutex`](sync::Mutex), [`RwLock`](sync::RwLock),
//! [`Condvar`](sync::Condvar), as well as other concurrency primitives like [`thread::spawn`],
//! [`UnsafeCell`](cell::UnsafeCell), and [`lazy_static!`]. However, when **not** running loom
//! tests, the `std` should be used, since the loom runtime won't be active. This means that
//! library code will need to use conditional compilation to decide which types to use.
//!
//! It is recommended to use a `loom` cfg flag to signal using the loom types. You can do this by
//! passing `RUSTFLAGS="--cfg loom"` as part of the command when you want to run the loom tests.
//! Then modify your `Cargo.toml` to include loom like this:
//!
//! ```toml
//! [target.'cfg(loom)'.dependencies]
//! loom = "0.7"
//! ```
//!
//! One common strategy to use the right types with and without loom is to create a module in your
//! crate named `sync` or any other name of your choosing. In this module, list out the types that
//! need to be toggled between loom and `std`:
//!
//! ```
//! #[cfg(loom)]
//! pub(crate) use loom::sync::atomic::AtomicUsize;
//!
//! #[cfg(not(loom))]
//! pub(crate) use std::sync::atomic::AtomicUsize;
//! ```
//!
//! Then, elsewhere in the library:
//!
//! ```ignore
//! use crate::sync::AtomicUsize;
//! ```
//!
//! ## Handling Loom API differences.
//!
//! Most of loom's type are drop-in replacements for their counterpart in `std`, but sometimes
//! there are minor API differences that you must work around. If your library must use Loom APIs
//! that differ from `std` types, then the library will be required to implement those APIs for
//! `std`. For example, for `UnsafeCell`, in the library's source, add the following:
//!
//! ```
//! #![cfg(not(loom))]
//!
//! #[derive(Debug)]
//! pub(crate) struct UnsafeCell<T>(std::cell::UnsafeCell<T>);
//!
//! impl<T> UnsafeCell<T> {
//!     pub(crate) fn new(data: T) -> UnsafeCell<T> {
//!         UnsafeCell(std::cell::UnsafeCell::new(data))
//!     }
//!
//!     pub(crate) fn with<R>(&self, f: impl FnOnce(*const T) -> R) -> R {
//!         f(self.0.get())
//!     }
//!
//!     pub(crate) fn with_mut<R>(&self, f: impl FnOnce(*mut T) -> R) -> R {
//!         f(self.0.get())
//!     }
//! }
//! ```
//!
//! ## Yielding
//!
//! Some concurrent algorithms assume a fair scheduler. For example, a spin lock assumes that, at
//! some point, another thread will make enough progress for the lock to become available. This
//! presents a challenge for loom as its scheduler is, by design, not fair. It is specifically
//! trying to emulate every _possible_ execution, which may mean that another thread does not get
//! to run for a very long time (see also [Spinlocks Considered Harmful]). In such cases, loops
//! must include calls to [`loom::thread::yield_now`](thread::yield_now). This tells loom that
//! another thread needs to be scheduled in order for the current one to make progress.
//!
//! # Running Loom Tests
//!
//! Loom tests must be run separately, with `RUSTFLAGS="--cfg loom"` specified (assuming you went
//! with the `cfg` approach suggested above). For example, if the library includes a test file:
//! `tests/loom_my_struct.rs` that includes tests with [`loom::model`](mod@model), then run the
//! following command:
//!
//! ```console
//! RUSTFLAGS="--cfg loom" cargo test --test loom_my_struct --release
//! ```
//!
//! Note that you will generally want to run loom tests with `--release` since loom must execute
//! each test closure a large number of times, at which point the speed win from optimized code
//! makes a big difference.
//!
//! # Debugging Loom Failures
//!
//! Loom's deterministic execution allows the specific chain of events leading to a test failure
//! can be isolated for debugging. When a loom test fails, the first step is to isolate the exact
//! execution path that resulted in the failure. To do this, Loom is able to output the execution
//! path to a file. Two environment variables are useful for this process:
//!
//! - `LOOM_CHECKPOINT_FILE`
//! - `LOOM_CHECKPOINT_INTERVAL`
//!
//! The first specifies the file to write to and read from. The second specifies how often to write
//! to the file. If the execution fails on the 10,000,000th permutation, it is faster to write to a
//! file every 10,000 iterations instead of every single one.
//!
//! To isolate the exact failing path, first run the following command to generate the checkpoint
//! for the failing scenario:
//!
//! ```console
//! LOOM_CHECKPOINT_FILE=my_test.json [other env vars] \
//!     cargo test --test loom_my_struct --release [failing test]
//! ```
//!
//! Then this to check that the next permutation indeed triggers the fault:
//!
//! ```console
//! LOOM_CHECKPOINT_INTERVAL=1 LOOM_CHECKPOINT_FILE=my_test.json [other env vars] \
//!     cargo test --test loom_my_struct --release [failing test]
//! ```
//!
//! The test should fail on the first permutation, effectively isolating the failure
//! scenario.
//!
//! The next step is to enable additional log output for just the failing permutation. Again, there
//! are some environment variables for this:
//!
//! - `LOOM_LOG`
//! - `LOOM_LOCATION`
//!
//! The first environment variable, `LOOM_LOG`, outputs a marker on every thread switch. This helps
//! with tracing the exact steps in a threaded environment that results in the test failure.
//!
//! The second, `LOOM_LOCATION`, enables location tracking. This includes additional information in
//! panic messages that helps identify which specific field resulted in the error.
//!
//! Put together, the command becomes (yes, we know this is not great... but it works):
//!
//! ```console
//! LOOM_LOG=trace \
//!     LOOM_LOCATION=1 \
//!     LOOM_CHECKPOINT_INTERVAL=1 \
//!     LOOM_CHECKPOINT_FILE=my_test.json \
//!     RUSTFLAGS="--cfg loom" \
//!     [other env vars] \
//!     cargo test --test loom_my_struct --release [failing test]
//! ```
//!
//! This should provide you with a trace of all the concurrency events leading up to the failure,
//! which should allow you to identify how the bug is triggered.
//!
//! # Limitations and Caveats
//!
//! ## Intrusive Implementation
//!
//! Loom works by intercepting all loads, stores, and other concurrency-sensitive operations (like
//! spawning threads) that may trigger concurrency bugs in an applications. But this interception
//! is not automatic -- it requires that the code being tested specifically uses the loom
//! replacement types. Any code that does not use loom's replacement types is invisible to loom,
//! and thus won't be subject to the loom model's permutation.
//!
//! While it is relatively simple to utilize loom's types in a single crate through the root-level
//! `#[cfg(loom)] mod sync` approach suggested earlier, more complex use-cases may require the use
//! of a library that itself uses concurrent constructs like locks and channels. In such cases,
//! that library must _also_ be augmented to support loom to achieve complete execution coverage.
//!
//! Note that loom still works if some concurrent operations are hidden from it (for example, if
//! you use `std::sync::Arc` instead of `loom::sync::Arc`). It just means that loom won't be able
//! to reason about the interaction between those operations and the other concurrent operations in
//! your program, and thus certain executions that are possible in the real world won't be modeled.
//!
//! ## Large Models
//!
//! By default, loom runs an **exhaustive** check of your program's possible concurrent executions
//! where **all** possible interleavings are checked. Loom's state reduction algorithms (see
//! "Implementation" below) significantly reduce the state space that must be explored, but complex
//! models can still take **significant** time to complete.
//!
//! To handle such large models in a more reasonable amount of time, you may need to **not** run
//! an exhaustive check, and instead tell loom to prune out interleavings that are unlikely to
//! reveal additional bugs. You do this by providing loom with a _thread pre-emption bound_. If you
//! set such a bound, loom will check all possible executions that include **at most** `n` thread
//! pre-emptions (where one thread is forcibly stopped and another one runs in its place. **In
//! practice, setting the thread pre-emption bound to 2 or 3 is enough to catch most bugs** while
//! significantly reducing the number of possible executions.
//!
//! To set the thread pre-emption bound, set the `LOOM_MAX_PREEMPTIONS` environment
//! variable when running tests (or set
//! [`Builder::preemption_bound`](model::Builder::preemption_bound)). For example:
//!
//! ```console
//! LOOM_MAX_PREEMPTIONS=3 RUSTFLAGS="--cfg loom" cargo test --test loom_my_struct --release
//! ```
//!
//! ## Relaxed Memory Ordering
//!
//! The [`Relaxed` memory ordering](std::sync::atomic::Ordering::Relaxed) allows particularly
//! strange executions. For example, in the following code snippet, it is [completely
//! legal][spec-relaxed] for `r1 == r2 == 42`!
//!
//! ```rust,no_run
//! # use std::sync::atomic::{AtomicUsize, Ordering};
//! # use std::thread;
//! # let x: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
//! # let y: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
//! thread::spawn(move || {
//!   let r1 = y.load(Ordering::Relaxed); // A
//!   x.store(r1, Ordering::Relaxed);     // B
//! });
//! thread::spawn(move || {
//!   let r2 = x.load(Ordering::Relaxed); // C
//!   y.store(42, Ordering::Relaxed);     // D
//! });
//! ```
//!
//! Unfortunately, it is not possible for loom to completely model all the interleavings that
//! relaxed memory ordering allows. This is because the relaxed memory ordering allows memory
//! operations to be re-ordered within a single thread -- B can run *before* A -- which loom cannot
//! emulate. The same restriction applies to certain reorderings that are possible across different
//! atomic variables with other memory orderings, and means that there are certain concurrency bugs
//! that loom cannot catch.
//!
//! ## Combinatorial Explosion with Many Threads
//!
//! The number of possible execution interleavings grows exponentially with the number of threads,
//! as each possible execution of each additional thread must be taken into account for each
//! possible execution of the current threads. Loom mitigates this to an extent by reducing the
//! state space (see "Implementation" below) through _equivalent execution elimination_. For
//! example, if two threads **read** from the same atomic variable, loom does not attempt another
//! execution given that the order in which two threads read from the same atomic cannot impact the
//! execution.
//!
//! However, even with equivalent execution elimination, the number of possible executions grows
//! significantly with each new thread, to the point where checking becomes infeasible. Loom
//! therefore specifically limits the number of threads it will model (see [`MAX_THREADS`]), and
//! tailors its implementation to that limit.
//!
//! # Implementation
//!
//! Loom is an implementation of techniques described in [CDSChecker: Checking Concurrent Data
//! Structures Written with C/C++ Atomics][cdschecker]. Please see the paper for much more detail
//! on equivalent execution elimination and the other techniques loom uses to accurately model the
//! [C11 memory model][spec].
//!
//! [spec]: https://en.cppreference.com/w/cpp/atomic/memory_order
//! [spec-relaxed]: https://en.cppreference.com/w/cpp/atomic/memory_order#Relaxed_ordering
//! [Spinlocks Considered Harmful]: https://matklad.github.io/2020/01/02/spinlocks-considered-harmful.html
//! [cdschecker]: http://demsky.eecs.uci.edu/publications/c11modelcheck.pdf

macro_rules! if_futures {
    ($($t:tt)*) => {
        cfg_if::cfg_if! {
            if #[cfg(feature = "futures")] {
                #[cfg_attr(docsrs, doc(cfg(feature = "futures")))]
                $($t)*
            }
        }
    }
}

macro_rules! dbg {
    ($($t:tt)*) => {
        $($t)*
    };
}

#[macro_use]
mod rt;

pub use rt::{explore, skip_branch, stop_exploring};
// Expose for documentation purposes.
pub use rt::MAX_THREADS;

pub mod alloc;
pub mod cell;
pub mod hint;
pub mod lazy_static;
pub mod model;
pub mod sync;
pub mod thread;

#[doc(inline)]
pub use crate::model::model;

if_futures! {
    pub mod future;
}

/// Mock version of `std::thread_local!`.
// This is defined *after* all other code in `loom`, since we use
// `scoped_thread_local!` internally, which uses the `std::thread_local!` macro
// without namespacing it. Defining this after all other `loom` modules
// prevents internal code from accidentally using the mock thread local instead
// of the real one.
#[macro_export]
macro_rules! thread_local {
    // empty (base case for the recursion)
    () => {};

    // process multiple declarations
    ($(#[$attr:meta])* $vis:vis static $name:ident: $t:ty = $init:expr; $($rest:tt)*) => (
        $crate::__thread_local_inner!($(#[$attr])* $vis $name, $t, $init);
        $crate::thread_local!($($rest)*);
    );

    // handle a single declaration
    ($(#[$attr:meta])* $vis:vis static $name:ident: $t:ty = $init:expr) => (
        $crate::__thread_local_inner!($(#[$attr])* $vis $name, $t, $init);
    );
}

/// Mock version of `lazy_static::lazy_static!`.
#[macro_export]
macro_rules! lazy_static {
    ($(#[$attr:meta])* static ref $N:ident : $T:ty = $e:expr; $($t:tt)*) => {
        // use `()` to explicitly forward the information about private items
        $crate::__lazy_static_internal!($(#[$attr])* () static ref $N : $T = $e; $($t)*);
    };
    ($(#[$attr:meta])* pub static ref $N:ident : $T:ty = $e:expr; $($t:tt)*) => {
        $crate::__lazy_static_internal!($(#[$attr])* (pub) static ref $N : $T = $e; $($t)*);
    };
    ($(#[$attr:meta])* pub ($($vis:tt)+) static ref $N:ident : $T:ty = $e:expr; $($t:tt)*) => {
        $crate::__lazy_static_internal!($(#[$attr])* (pub ($($vis)+)) static ref $N : $T = $e; $($t)*);
    };
    () => ()
}

#[macro_export]
#[doc(hidden)]
macro_rules! __thread_local_inner {
    ($(#[$attr:meta])* $vis:vis $name:ident, $t:ty, $init:expr) => {
        $(#[$attr])* $vis static $name: $crate::thread::LocalKey<$t> =
            $crate::thread::LocalKey {
                init: (|| { $init }) as fn() -> $t,
                _p: std::marker::PhantomData,
            };
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __lazy_static_internal {
    // optional visibility restrictions are wrapped in `()` to allow for
    // explicitly passing otherwise implicit information about private items
    ($(#[$attr:meta])* ($($vis:tt)*) static ref $N:ident : $T:ty = $init:expr; $($t:tt)*) => {
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        $(#[$attr])*
        $($vis)* struct $N {__private_field: ()}
        #[doc(hidden)]
        $($vis)* static $N: $N = $N {__private_field: ()};
        impl ::core::ops::Deref for $N {
            type Target = $T;
            // this and the two __ functions below should really also be #[track_caller]
            fn deref(&self) -> &$T {
                #[inline(always)]
                fn __static_ref_initialize() -> $T { $init }

                #[inline(always)]
                fn __stability() -> &'static $T {
                    static LAZY: $crate::lazy_static::Lazy<$T> =
                        $crate::lazy_static::Lazy {
                            init: __static_ref_initialize,
                            _p: core::marker::PhantomData,
                        };
                    LAZY.get()
                }
                __stability()
            }
        }
        $crate::lazy_static!($($t)*);
    };
    () => ()
}

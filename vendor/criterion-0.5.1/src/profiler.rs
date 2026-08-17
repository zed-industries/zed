//! This module provides an extension trait which allows in-process profilers
//! to be hooked into the `--profile-time` argument at compile-time. Users of
//! out-of-process profilers such as perf don't need to do anything special.

use std::path::Path;

/// Extension trait for external crates to implement which provides start/stop
/// hooks when profiling (but not when benchmarking) functions.
pub trait Profiler {
    /// This function is called when Criterion.rs starts profiling a particular
    /// benchmark. It provides the stringified benchmark ID and
    /// a path to a directory where the profiler can store its data.
    fn start_profiling(&mut self, benchmark_id: &str, benchmark_dir: &Path);

    /// This function is called after Criterion.rs stops profiling a particular
    /// benchmark. The benchmark ID and directory are the same as in the call
    /// to `start`, provided for convenience.
    fn stop_profiling(&mut self, benchmark_id: &str, benchmark_dir: &Path);
}

/// Dummy profiler implementation, representing cases where the profiler is
/// an external process (eg. perftools, etc.) which do not require start/stop
/// hooks. This implementation does nothing and is used as the default.
pub struct ExternalProfiler;
impl Profiler for ExternalProfiler {
    fn start_profiling(&mut self, _benchmark_id: &str, _benchmark_dir: &Path) {}
    fn stop_profiling(&mut self, _benchmark_id: &str, _benchmark_dir: &Path) {}
}

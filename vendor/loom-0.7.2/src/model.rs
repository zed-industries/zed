//! Model concurrent programs.

use crate::rt::{self, Execution, Scheduler};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

use tracing::{info, subscriber};
use tracing_subscriber::{fmt, EnvFilter};

const DEFAULT_MAX_THREADS: usize = 5;
const DEFAULT_MAX_BRANCHES: usize = 1_000;

/// Configure a model
#[derive(Debug)]
#[non_exhaustive] // Support adding more fields in the future
pub struct Builder {
    /// Max number of threads to check as part of the execution.
    ///
    /// This should be set as low as possible and must be less than
    /// [`MAX_THREADS`](crate::MAX_THREADS).
    pub max_threads: usize,

    /// Maximum number of thread switches per permutation.
    ///
    /// Defaults to `LOOM_MAX_BRANCHES` environment variable.
    pub max_branches: usize,

    /// Maximum number of permutations to explore.
    ///
    /// Defaults to `LOOM_MAX_PERMUTATIONS` environment variable.
    pub max_permutations: Option<usize>,

    /// Maximum amount of time to spend on checking
    ///
    /// Defaults to `LOOM_MAX_DURATION` environment variable.
    pub max_duration: Option<Duration>,

    /// Maximum number of thread preemptions to explore
    ///
    /// Defaults to `LOOM_MAX_PREEMPTIONS` environment variable.
    pub preemption_bound: Option<usize>,

    /// When doing an exhaustive check, uses the file to store and load the
    /// check progress
    ///
    /// Defaults to `LOOM_CHECKPOINT_FILE` environment variable.
    pub checkpoint_file: Option<PathBuf>,

    /// How often to write the checkpoint file
    ///
    /// Defaults to `LOOM_CHECKPOINT_INTERVAL` environment variable.
    pub checkpoint_interval: usize,

    /// When `true` loom won't start state exploration until `explore_state` is
    /// called.
    pub expect_explicit_explore: bool,

    /// When `true`, locations are captured on each loom operation.
    ///
    /// Note that is is **very** expensive. It is recommended to first isolate a
    /// failing iteration using `LOOM_CHECKPOINT_FILE`, then enable location
    /// tracking.
    ///
    /// Defaults to `LOOM_LOCATION` environment variable.
    pub location: bool,

    /// Log execution output to stdout.
    ///
    /// Defaults to existence of `LOOM_LOG` environment variable.
    pub log: bool,
}

impl Builder {
    /// Create a new `Builder` instance with default values.
    pub fn new() -> Builder {
        use std::env;

        let checkpoint_interval = env::var("LOOM_CHECKPOINT_INTERVAL")
            .map(|v| {
                v.parse()
                    .expect("invalid value for `LOOM_CHECKPOINT_INTERVAL`")
            })
            .unwrap_or(20_000);

        let max_branches = env::var("LOOM_MAX_BRANCHES")
            .map(|v| v.parse().expect("invalid value for `LOOM_MAX_BRANCHES`"))
            .unwrap_or(DEFAULT_MAX_BRANCHES);

        let location = env::var("LOOM_LOCATION").is_ok();

        let log = env::var("LOOM_LOG").is_ok();

        let max_duration = env::var("LOOM_MAX_DURATION")
            .map(|v| {
                let secs = v.parse().expect("invalid value for `LOOM_MAX_DURATION`");
                Duration::from_secs(secs)
            })
            .ok();

        let max_permutations = env::var("LOOM_MAX_PERMUTATIONS")
            .map(|v| {
                v.parse()
                    .expect("invalid value for `LOOM_MAX_PERMUTATIONS`")
            })
            .ok();

        let preemption_bound = env::var("LOOM_MAX_PREEMPTIONS")
            .map(|v| v.parse().expect("invalid value for `LOOM_MAX_PREEMPTIONS`"))
            .ok();

        let checkpoint_file = env::var("LOOM_CHECKPOINT_FILE")
            .map(|v| v.parse().expect("invalid value for `LOOM_CHECKPOINT_FILE`"))
            .ok();

        Builder {
            max_threads: DEFAULT_MAX_THREADS,
            max_branches,
            max_duration,
            max_permutations,
            preemption_bound,
            checkpoint_file,
            checkpoint_interval,
            expect_explicit_explore: false,
            location,
            log,
        }
    }

    /// Set the checkpoint file.
    pub fn checkpoint_file(&mut self, file: &str) -> &mut Self {
        self.checkpoint_file = Some(file.into());
        self
    }

    /// Check the provided model.
    pub fn check<F>(&self, f: F)
    where
        F: Fn() + Sync + Send + 'static,
    {
        let mut i = 1;
        let mut _span = tracing::info_span!("iter", message = i).entered();

        let mut execution = Execution::new(
            self.max_threads,
            self.max_branches,
            self.preemption_bound,
            !self.expect_explicit_explore,
        );
        let mut scheduler = Scheduler::new(self.max_threads);

        if let Some(ref path) = self.checkpoint_file {
            if path.exists() {
                execution.path = checkpoint::load_execution_path(path);
                execution.path.set_max_branches(self.max_branches);
            }
        }

        execution.log = self.log;
        execution.location = self.location;

        let f = Arc::new(f);

        let start = Instant::now();
        loop {
            if i % self.checkpoint_interval == 0 {
                info!(parent: None, "");
                info!(
                    parent: None,
                    " ================== Iteration {} ==================", i
                );
                info!(parent: None, "");

                if let Some(ref path) = self.checkpoint_file {
                    checkpoint::store_execution_path(&execution.path, path);
                }

                if let Some(max_permutations) = self.max_permutations {
                    if i >= max_permutations {
                        return;
                    }
                }

                if let Some(max_duration) = self.max_duration {
                    if start.elapsed() >= max_duration {
                        return;
                    }
                }
            }

            let f = f.clone();

            scheduler.run(&mut execution, move || {
                f();

                let lazy_statics = rt::execution(|execution| execution.lazy_statics.drop());

                // drop outside of execution
                drop(lazy_statics);

                rt::thread_done();
            });

            execution.check_for_leaks();

            i += 1;

            // Create the next iteration's `tracing` span before trying to step to the next
            // execution, as the `Execution` will capture the current span when
            // it's reset.
            _span = tracing::info_span!(parent: None, "iter", message = i).entered();
            if let Some(next) = execution.step() {
                execution = next;
            } else {
                info!(parent: None, "Completed in {} iterations", i - 1);
                return;
            }
        }
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

/// Run all concurrent permutations of the provided closure.
///
/// Uses a default [`Builder`] which can be affected by environment variables.
pub fn model<F>(f: F)
where
    F: Fn() + Sync + Send + 'static,
{
    let subscriber = fmt::Subscriber::builder()
        .with_env_filter(EnvFilter::from_env("LOOM_LOG"))
        .with_test_writer()
        .without_time()
        .finish();

    subscriber::with_default(subscriber, || {
        Builder::new().check(f);
    });
}

#[cfg(feature = "checkpoint")]
mod checkpoint {
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    pub(crate) fn load_execution_path(fs_path: &Path) -> crate::rt::Path {
        let mut file = File::open(fs_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        serde_json::from_str(&contents).unwrap()
    }

    pub(crate) fn store_execution_path(path: &crate::rt::Path, fs_path: &Path) {
        let serialized = serde_json::to_string(path).unwrap();

        let mut file = File::create(fs_path).unwrap();
        file.write_all(serialized.as_bytes()).unwrap();
    }
}

#[cfg(not(feature = "checkpoint"))]
mod checkpoint {
    use std::path::Path;

    pub(crate) fn load_execution_path(_fs_path: &Path) -> crate::rt::Path {
        panic!("not compiled with `checkpoint` feature")
    }

    pub(crate) fn store_execution_path(_path: &crate::rt::Path, _fs_path: &Path) {
        panic!("not compiled with `checkpoint` feature")
    }
}

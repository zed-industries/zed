use once_cell::sync::OnceCell;
use std::{
    fmt,
    sync::atomic::{AtomicUsize, Ordering},
};

pub(crate) static GLOBAL_EXECUTOR_CONFIG: OnceCell<Config> = OnceCell::new();

/// Configuration to init the thread pool for the multi-threaded global executor.
#[derive(Default)]
pub struct GlobalExecutorConfig {
    /// The environment variable from which we'll try to parse the number of threads to spawn.
    env_var: Option<&'static str>,
    /// The minimum number of threads to spawn.
    min_threads: Option<usize>,
    /// The maximum number of threads to spawn.
    max_threads: Option<usize>,
    /// The closure function used to get the name of the thread. The name can be used for identification in panic messages.
    thread_name_fn: Option<Box<dyn Fn() -> String + Send + Sync>>,
}

impl fmt::Debug for GlobalExecutorConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GlobalExecutorConfig")
            .field("env_var", &self.env_var)
            .field("min_threads", &self.min_threads)
            .field("max_threads", &self.max_threads)
            .finish()
    }
}

impl GlobalExecutorConfig {
    /// Use the specified environment variable to find the number of threads to spawn.
    pub fn with_env_var(mut self, env_var: &'static str) -> Self {
        self.env_var = Some(env_var);
        self
    }

    /// Use the specified value as the minimum number of threads.
    pub fn with_min_threads(mut self, min_threads: usize) -> Self {
        self.min_threads = Some(min_threads);
        self
    }

    /// Use the specified value as the maximum number of threads for async tasks.
    /// To limit the maximum number of threads for blocking tasks, please use the
    /// `BLOCKING_MAX_THREADS` environment variable.
    pub fn with_max_threads(mut self, max_threads: usize) -> Self {
        self.max_threads = Some(max_threads);
        self
    }

    /// Use the specified prefix to name the threads.
    pub fn with_thread_name_fn(
        mut self,
        thread_name_fn: impl Fn() -> String + Send + Sync + 'static,
    ) -> Self {
        self.thread_name_fn = Some(Box::new(thread_name_fn));
        self
    }

    pub(crate) fn seal(self) -> Config {
        let min_threads = std::env::var(self.env_var.unwrap_or("ASYNC_GLOBAL_EXECUTOR_THREADS"))
            .ok()
            .and_then(|threads| threads.parse().ok())
            .or(self.min_threads)
            .unwrap_or_else(|| std::thread::available_parallelism().map_or(1, usize::from))
            .max(1);
        let max_threads = self.max_threads.unwrap_or(min_threads * 4).max(min_threads);
        Config {
            min_threads,
            max_threads,
            thread_name_fn: self.thread_name_fn.unwrap_or_else(|| {
                Box::new(|| {
                    static GLOBAL_EXECUTOR_NEXT_THREAD: AtomicUsize = AtomicUsize::new(1);
                    format!(
                        "async-global-executor-{}",
                        GLOBAL_EXECUTOR_NEXT_THREAD.fetch_add(1, Ordering::SeqCst)
                    )
                })
            }),
        }
    }
}

// The actual configuration, computed from the given GlobalExecutorConfig
pub(crate) struct Config {
    pub(crate) min_threads: usize,
    pub(crate) max_threads: usize,
    pub(crate) thread_name_fn: Box<dyn Fn() -> String + Send + Sync>,
}

impl Default for Config {
    fn default() -> Self {
        GlobalExecutorConfig::default().seal()
    }
}

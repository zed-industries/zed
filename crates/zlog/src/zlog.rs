//! # logger
pub use log as log_impl;

mod env_config;
pub mod scope_map;

pub const SCOPE_DEPTH_MAX: usize = 4;

pub fn init_from_env() {
    let Ok(env_config) = std::env::var("ZED_LOG").or_else(|_| std::env::var("RUST_LOG")) else {
        return;
    };
    match env_config::parse(&env_config) {
        Ok(filter) => {
            scope_map::init_env_filter(filter);
            scope_map::refresh();
            // TODO: set max level once removing `env_logger` and `simple_log` crates
        }
        Err(err) => {
            eprintln!("Failed to parse log filter: {}", err);
        }
    }
}

/// because we are currently just wrapping the `log` crate in `zlog`,
/// we need to work around the fact that the `log` crate only provides a
/// single global level filter. In order to have more precise control until
/// we no longer wrap `log`, we bump up the priority of log level so that it
/// will be logged, even if the actual level is lower
/// This is fine for now, as we use a `info` level filter by default in releases,
/// which hopefully won't result in confusion like `warn` or `error` levels might.
pub fn min_printed_log_level(level: log_impl::Level) -> log_impl::Level {
    // this logic is defined based on the logic used in the `log` crate,
    // which checks that a logs level is <= both of these values,
    // so we take the minimum of the two values to ensure that check passes
    let level_min_static = log_impl::STATIC_MAX_LEVEL;
    let level_min_dynamic = log_impl::max_level();
    if level <= level_min_static && level <= level_min_dynamic {
        return level;
    }
    return log_impl::LevelFilter::min(level_min_static, level_min_dynamic)
        .to_level()
        .unwrap_or(level);
}

#[macro_export]
macro_rules! log {
    ($logger:expr, $level:expr, $($arg:tt)+) => {
        let level = $level;
        let logger = $logger;
        let (enabled, level) = $crate::scope_map::is_scope_enabled(&logger.scope, level);
        if enabled {
            $crate::log_impl::log!(level, "[{}]: {}", &logger.fmt_scope(), format!($($arg)+));
        }
    }
}

#[macro_export]
macro_rules! trace {
    ($logger:expr => $($arg:tt)+) => {
        $crate::log!($logger, $crate::log_impl::Level::Trace, $($arg)+);
    };
    ($($arg:tt)+) => {
        $crate::log!($crate::default_logger!(), $crate::log_impl::Level::Trace, $($arg)+);
    };
}

#[macro_export]
macro_rules! debug {
    ($logger:expr => $($arg:tt)+) => {
        $crate::log!($logger, $crate::log_impl::Level::Debug, $($arg)+);
    };
    ($($arg:tt)+) => {
        $crate::log!($crate::default_logger!(), $crate::log_impl::Level::Debug, $($arg)+);
    };
}

#[macro_export]
macro_rules! info {
    ($logger:expr => $($arg:tt)+) => {
        $crate::log!($logger, $crate::log_impl::Level::Info, $($arg)+);
    };
    ($($arg:tt)+) => {
        $crate::log!($crate::default_logger!(), $crate::log_impl::Level::Info, $($arg)+);
    };
}

#[macro_export]
macro_rules! warn {
    ($logger:expr => $($arg:tt)+) => {
        $crate::log!($logger, $crate::log_impl::Level::Warn, $($arg)+);
    };
    ($($arg:tt)+) => {
        $crate::log!($crate::default_logger!(), $crate::log_impl::Level::Warn, $($arg)+);
    };
}

#[macro_export]
macro_rules! error {
    ($logger:expr => $($arg:tt)+) => {
        $crate::log!($logger, $crate::log_impl::Level::Error, $($arg)+);
    };
    ($($arg:tt)+) => {
        $crate::log!($crate::default_logger!(), $crate::log_impl::Level::Error, $($arg)+);
    };
}

/// Creates a timer that logs the duration it was active for either when
/// it is dropped, or when explicitly stopped using the `end` method.
/// Logs at the `trace` level.
/// Note that it will include time spent across await points
/// (i.e. should not be used to measure the performance of async code)
/// However, this is a feature not a bug, as it allows for a more accurate
/// understanding of how long the action actually took to complete, including
/// interruptions, which can help explain why something may have timed out,
/// why it took longer to complete than it would had the await points resolved
/// immediately, etc.
#[macro_export]
macro_rules! time {
    ($logger:expr => $name:expr) => {
        $crate::Timer::new($logger, $name)
    };
    ($name:expr) => {
        time!($crate::default_logger!() => $name)
    };
}

#[macro_export]
macro_rules! scoped {
    ($parent:expr => $name:expr) => {{
        let parent = $parent;
        let name = $name;
        let mut scope = parent.scope;
        let mut index = 1; // always have crate/module name
        while index < scope.len() && !scope[index].is_empty() {
            index += 1;
        }
        if index >= scope.len() {
            #[cfg(debug_assertions)]
            {
                panic!("Scope overflow trying to add scope {}", name);
            }
            #[cfg(not(debug_assertions))]
            {
                $crate::warn!(
                    parent =>
                    "Scope overflow trying to add scope {}... ignoring scope",
                    name
                );
            }
        }
        scope[index] = name;
        $crate::Logger { scope }
    }};
    ($name:expr) => {
        $crate::scoped!($crate::default_logger!() => $name)
    };
}

#[macro_export]
macro_rules! default_logger {
    () => {
        $crate::Logger {
            scope: $crate::private::scope_new(&[$crate::crate_name!()]),
        }
    };
}

#[macro_export]
macro_rules! crate_name {
    () => {
        $crate::private::extract_crate_name_from_module_path(module_path!())
    };
}

/// functions that are used in macros, and therefore must be public,
/// but should not be used directly
pub mod private {
    use super::*;

    pub fn extract_crate_name_from_module_path(module_path: &'static str) -> &'static str {
        return module_path
            .split_once("::")
            .map(|(crate_name, _)| crate_name)
            .unwrap_or(module_path);
    }

    pub fn scope_new(scopes: &[&'static str]) -> Scope {
        assert!(scopes.len() <= SCOPE_DEPTH_MAX);
        let mut scope = [""; SCOPE_DEPTH_MAX];
        scope[0..scopes.len()].copy_from_slice(scopes);
        scope
    }

    pub fn scope_alloc_new(scopes: &[&str]) -> ScopeAlloc {
        assert!(scopes.len() <= SCOPE_DEPTH_MAX);
        let mut scope = [""; SCOPE_DEPTH_MAX];
        scope[0..scopes.len()].copy_from_slice(scopes);
        scope.map(|s| s.to_string())
    }

    pub fn scope_to_alloc(scope: &Scope) -> ScopeAlloc {
        return scope.map(|s| s.to_string());
    }
}

pub type Scope = [&'static str; SCOPE_DEPTH_MAX];
pub type ScopeAlloc = [String; SCOPE_DEPTH_MAX];
const SCOPE_STRING_SEP: &'static str = ".";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Logger {
    pub scope: Scope,
}

impl Logger {
    pub fn fmt_scope(&self) -> String {
        let mut last = 0;
        for s in self.scope {
            if s.is_empty() {
                break;
            }
            last += 1;
        }

        return self.scope[0..last].join(SCOPE_STRING_SEP);
    }
}

pub struct Timer {
    pub logger: Logger,
    pub start_time: std::time::Instant,
    pub name: &'static str,
    pub warn_if_longer_than: Option<std::time::Duration>,
    pub done: bool,
}

impl Drop for Timer {
    fn drop(&mut self) {
        self.finish();
    }
}

impl Timer {
    #[must_use = "Timer will stop when dropped, the result of this function should be saved in a variable prefixed with `_` if it should stop when dropped"]
    pub fn new(logger: Logger, name: &'static str) -> Self {
        return Self {
            logger,
            name,
            start_time: std::time::Instant::now(),
            warn_if_longer_than: None,
            done: false,
        };
    }
    pub fn warn_if_gt(mut self, warn_limit: std::time::Duration) -> Self {
        self.warn_if_longer_than = Some(warn_limit);
        return self;
    }

    pub fn end(mut self) {
        self.finish();
    }

    fn finish(&mut self) {
        if self.done {
            return;
        }
        let elapsed = self.start_time.elapsed();
        if let Some(warn_limit) = self.warn_if_longer_than {
            if elapsed > warn_limit {
                crate::warn!(
                    self.logger =>
                    "Timer '{}' took {:?}. Which was longer than the expected limit of {:?}",
                    self.name,
                    elapsed,
                    warn_limit
                );
                self.done = true;
                return;
            }
        }
        crate::trace!(
            self.logger =>
            "Timer '{}' finished in {:?}",
            self.name,
            elapsed
        );
        self.done = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crate_name() {
        assert_eq!(crate_name!(), "zlog");
    }
}

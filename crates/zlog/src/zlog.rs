//! # logger
pub use log as log_impl;

mod env_config;
pub mod filter;
pub mod sink;

pub use sink::{flush, init_output_file, init_output_stdout};

pub const SCOPE_DEPTH_MAX: usize = 4;

pub fn init() {
    process_env();
    log::set_logger(&ZLOG).expect("Logger should not be initialized twice");
    log::set_max_level(log::LevelFilter::max());
}

pub fn process_env() {
    let Ok(env_config) = std::env::var("ZED_LOG").or_else(|_| std::env::var("RUST_LOG")) else {
        return;
    };
    match env_config::parse(&env_config) {
        Ok(filter) => {
            filter::init_env_filter(filter);
            filter::refresh();
        }
        Err(err) => {
            eprintln!("Failed to parse log filter: {}", err);
        }
    }
}

static ZLOG: Zlog = Zlog {};

pub struct Zlog {}

impl log::Log for Zlog {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        filter::is_possibly_enabled_level(metadata.level())
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let scope = match record.module_path_static() {
            Some(module_path) => {
                // TODO: better module name -> scope translation
                let crate_name = private::extract_crate_name_from_module_path(module_path);
                private::scope_new(&[crate_name])
            }
            // TODO: when do we hit this
            None => private::scope_new(&["*unknown*"]),
        };
        let level = record.metadata().level();
        if !filter::is_scope_enabled(&scope, level) {
            return;
        }
        sink::submit(sink::Record {
            scope,
            level,
            message: record.args(),
        });
    }

    fn flush(&self) {
        // todo: necessary?
        sink::flush();
    }
}

#[macro_export]
macro_rules! log {
    ($logger:expr, $level:expr, $($arg:tt)+) => {
        let level = $level;
        let logger = $logger;
        let enabled = $crate::filter::is_scope_enabled(&logger.scope, level);
        if enabled {
            $crate::sink::submit($crate::sink::Record {
                scope: logger.scope,
                level,
                message: &format_args!($($arg)+),
            });
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
/// why it took longer to complete than it would have had the await points resolved
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
const SCOPE_STRING_SEP_STR: &'static str = ".";
const SCOPE_STRING_SEP_CHAR: char = '.';

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Logger {
    pub scope: Scope,
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        filter::is_possibly_enabled_level(metadata.level())
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let level = record.metadata().level();
        if !filter::is_scope_enabled(&self.scope, level) {
            return;
        }
        sink::submit(sink::Record {
            scope: self.scope,
            level,
            message: record.args(),
        });
    }
    fn flush(&self) {
        sink::flush();
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

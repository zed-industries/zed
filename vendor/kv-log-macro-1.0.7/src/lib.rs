//! Log macro for log's kv-unstable backend.
//!
//! ## Example
//!
//! ```rust
//! use kv_log_macro::info;
//!
//! femme::start(log::LevelFilter::Info).unwrap();
//!
//! info!("hello");
//! info!("hello",);
//! info!("hello {}", "cats");
//! info!("hello {}", "cats",);
//! info!("hello {}", "cats", {
//!     cat_1: "chashu",
//!     cat_2: "nori",
//! });
//! ```

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples, unreachable_pub)]
// #![cfg_attr(test, deny(warnings))]

use log::{logger, LevelFilter, Record};

use std::fmt;

// publicly exporting so $crate::Level works.
pub use log::Level;

/// The statically resolved maximum log level.
pub const STATIC_MAX_LEVEL: LevelFilter = log::STATIC_MAX_LEVEL;

/// Returns the current maximum log level.
#[inline]
pub fn max_level() -> LevelFilter {
    log::max_level()
}

/// The standard logging macro.
///
/// ```
/// use kv_log_macro::info;
///
/// info!("hello");
/// info!("hello",);
/// info!("hello {}", "cats");
/// info!("hello {}", "cats",);
/// info!("hello {}", "cats", {
///     cat_1: "chashu",
///     cat_2: "nori",
/// });
/// ```
#[macro_export(local_inner_macros)]
macro_rules! log {
    // log!(target: "...", "...")
    (target: $target:expr, $lvl:expr, $e:expr) => {
        $crate::log_impl!(target: $target, $lvl, ($e));
    };

    // log!(target: "...", "...", args...)
    (target: $target:expr, $lvl:expr, $e:expr, $($rest:tt)*) => {
        $crate::log_impl!(target: $target, $lvl, ($e) $($rest)*);
    };

    // log!("...", args...)
    ($lvl:expr, $($arg:tt)+) => ($crate::log!(target: __log_module_path!(), $lvl, $($arg)+))
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! log_impl {
    // End of macro input
    (target: $target:expr, $lvl:expr, ($($arg:expr),*)) => {{
        let lvl = $lvl;
        if lvl <= $crate::STATIC_MAX_LEVEL && lvl <= $crate::max_level() {
            $crate::__private_api_log(
                __log_format_args!($($arg),*),
                lvl,
                &($target, __log_module_path!(), __log_file!(), __log_line!()),
                None,
            );
        }
    }};

    // // Trailing k-v pairs containing no trailing comma
    (target: $target:expr, $lvl:expr, ($($arg:expr),*) { $($key:ident : $value:expr),* }) => {{
        if $lvl <= $crate::STATIC_MAX_LEVEL && $lvl <= $crate::max_level() {
            $crate::__private_api_log(
                __log_format_args!($($arg),*),
                $lvl,
                &(__log_module_path!(), __log_module_path!(), __log_file!(), __log_line!()),
                Some(&[$((__log_stringify!($key), &$value)),*])
            );
        }
    }};

    // Trailing k-v pairs with trailing comma
    (target: $target:expr, $lvl:expr, ($($e:expr),*) { $($key:ident : $value:expr,)* }) => {
        $crate::log_impl!(target: $target, $lvl, ($($e),*) { $($key : $value),* });
    };

    // Last expression arg with no trailing comma
    (target: $target:expr, $lvl:expr, ($($e:expr),*) $arg:expr) => {
        $crate::log_impl!(target: $target, $lvl, ($($e,)* $arg));
    };

    // Expression arg
    (target: $target:expr, $lvl:expr, ($($e:expr),*) $arg:expr, $($rest:tt)*) => {
        $crate::log_impl!(target: $target, $lvl, ($($e,)* $arg) $($rest)*);
    };
}

/// Logs a message at the trace level.
#[macro_export(local_inner_macros)]
macro_rules! trace {
    (target: $target:expr, $($arg:tt)+) => (
        log!(target: $target, $crate::Level::Trace, $($arg)+);
    );
    ($($arg:tt)+) => (
        log!($crate::Level::Trace, $($arg)+);
    )
}

/// Logs a message at the debug level.
#[macro_export(local_inner_macros)]
macro_rules! debug {
    (target: $target:expr, $($arg:tt)+) => (
        log!(target: $target, $crate::Level::Debug, $($arg)+);
    );
    ($($arg:tt)+) => (
        log!($crate::Level::Debug, $($arg)+);
    )
}

/// Logs a message at the info level.
#[macro_export(local_inner_macros)]
macro_rules! info {
    (target: $target:expr, $($arg:tt)+) => (
        log!(target: $target, $crate::Level::Info, $($arg)+);
    );
    ($($arg:tt)+) => (
        log!($crate::Level::Info, $($arg)+);
    )
}

/// Logs a message at the warn level.
#[macro_export(local_inner_macros)]
macro_rules! warn {
    (target: $target:expr, $($arg:tt)+) => (
        log!(target: $target, $crate::Level::Warn, $($arg)+);
    );
    ($($arg:tt)+) => (
        log!($crate::Level::Warn, $($arg)+);
    )
}

/// Logs a message at the error level.
#[macro_export(local_inner_macros)]
macro_rules! error {
    (target: $target:expr, $($arg:tt)+) => (
        log!(target: $target, $crate::Level::Error, $($arg)+);
    );
    ($($arg:tt)+) => (
        log!($crate::Level::Error, $($arg)+);
    )
}

/// Determines if a message logged at the specified level in that module will
/// be logged.
#[macro_export(local_inner_macros)]
macro_rules! log_enabled {
    (target: $target:expr, $lvl:expr) => {{
        let lvl = $lvl;
        lvl <= $crate::STATIC_MAX_LEVEL
            && lvl <= $crate::max_level()
            && $crate::__private_api_enabled(lvl, $target)
    }};
    ($lvl:expr) => {
        log_enabled!(target: __log_module_path!(), $lvl)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_format_args {
    ($($args:tt)*) => {
        format_args!($($args)*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_module_path {
    () => {
        module_path!()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_file {
    () => {
        file!()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_line {
    () => {
        line!()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_stringify {
    ($($args:tt)*) => {
        stringify!($($args)*)
    };
}

// WARNING: this is not part of the crate's public API and is subject to change at any time
#[doc(hidden)]
pub fn __private_api_log(
    args: fmt::Arguments<'_>,
    level: Level,
    &(target, module_path, file, line): &(&str, &'static str, &'static str, u32),
    kvs: Option<&[(&str, &dyn log::kv::ToValue)]>,
) {
    logger().log(
        &Record::builder()
            .args(args)
            .level(level)
            .target(target)
            .module_path_static(Some(module_path))
            .file_static(Some(file))
            .line(Some(line))
            .key_values(&kvs)
            .build(),
    );
}

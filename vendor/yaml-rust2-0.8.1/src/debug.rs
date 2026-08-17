//! Debugging helpers.
//!
//! Debugging is governed by two conditions:
//!   1. The build mode. Debugging code is not emitted in release builds and thus not available.
//!   2. The `YAMLALL_DEBUG` environment variable. If built in debug mode, the program must be fed
//!      the `YAMLALL_DEBUG` variable in its environment. While debugging code is present in debug
//!      build, debug helpers will only trigger if that variable is set when running the program.

// If a debug build, use stuff in the debug submodule.
#[cfg(feature = "debug_prints")]
pub use debug::enabled;

// Otherwise, just export dummies for publicly visible functions.
/// Evaluates to nothing.
#[cfg(not(feature = "debug_prints"))]
macro_rules! debug_print {
    ($($arg:tt)*) => {{}};
}

#[cfg(feature = "debug_prints")]
#[macro_use]
#[allow(clippy::module_inception)]
mod debug {
    use std::sync::OnceLock;

    /// If debugging is [`enabled`], print the format string on the error output.
    macro_rules! debug_print {
    ($($arg:tt)*) => {{
        if $crate::debug::enabled() {
            eprintln!($($arg)*)
        }
    }};
    }

    /// Return whether debugging features are enabled in this execution.
    #[cfg(debug_assertions)]
    pub fn enabled() -> bool {
        static ENABLED: OnceLock<bool> = OnceLock::new();
        *ENABLED.get_or_init(|| std::env::var("YAMLRUST2_DEBUG").is_ok())
    }
}

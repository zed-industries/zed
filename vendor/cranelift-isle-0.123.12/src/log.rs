//! Debug logging from the ISLE compiler itself.

/// Log a compiler-internal message for debugging purposes.
#[cfg(feature = "logging")]
#[macro_export]
macro_rules! log {
    ($($msg:tt)*) => {
        ::log::trace!($($msg)*)
    };
}

/// Log a compiler-internal message for debugging purposes.
#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! log {
    ($($msg:tt)*) => {};
}

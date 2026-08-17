//! Macros useful when writing procedural syntax extensions.
//!
//! The macros themselves are ordinary `macro_rules!` macros.

/// Call `span_err` on an `ExtCtxt` and return `DummyResult::any`.
#[macro_export]
macro_rules! ext_bail {
    ($cx:expr, $sp:expr, $msg:expr) => {{
        $cx.span_err($sp, $msg);
        return ::syntax::ext::base::DummyResult::any($sp);
    }}
}

/// `ext_bail!` if the condition `$e` is true.
#[macro_export]
macro_rules! ext_bail_if {
    ($e:expr, $cx:expr, $sp:expr, $msg:expr) => {{
        if $e { ext_bail!($cx, $sp, $msg) }
    }}
}

/// Unwrap the `Option` `$e`, or `ext_bail!`.
#[macro_export]
macro_rules! ext_expect {
    ($cx:expr, $sp:expr, $e:expr, $msg:expr) => {{
        match $e {
            Some(x) => x,
            None => ext_bail!($cx, $sp, $msg),
        }
    }}
}

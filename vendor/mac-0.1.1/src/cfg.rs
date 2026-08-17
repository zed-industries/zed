//! Macros for conditional compilation.

/// Compile-time matching on config variables.
///
/// Only the branch relevant on your machine will be type-checked!
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate mac;
/// # fn main() {
/// let mascot = match_cfg! {
///     (target_os = "linux") => "penguin",
///     (target_os = "openbsd") => "blowfish",
///     _ => "unknown",
/// };
/// println!("{}", mascot);
/// # }
/// ```
///
#[macro_export]
macro_rules! match_cfg {
    ( $( ($cfg:meta) => $e:expr, )* _ => $last:expr, ) => {
        match () {
            $(
                #[cfg($cfg)]
                () => $e,
            )*

            #[cfg(all( $( not($cfg) ),* ))]
            () => $last,
        }
    };

    ( $( ($cfg:meta) => $e:expr, )* ) => {
        match_cfg! {
            $(
                ($cfg) => $e,
            )*

            _ => {
                #[allow(dead_code)]
                #[static_assert]
                static MATCH_CFG_FALLBACK_UNREACHABLE: bool = false;
            }
        }
    };
}

/// Compile-time conditional expression.
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate mac;
/// # fn main() {
/// if_cfg!(test {
///     println!("Crate built as a test suite");
/// })
/// # }
/// ```
///
/// Unlike `if cfg!(...)`, this will not even compile the unused branch.
///
/// ```
/// # #[macro_use] extern crate mac;
/// # fn main() {
/// let x = if_cfg!(any(bleh, blah="bluh") {
///     some_undefined_function_name();
///     2 + "doesn't even typecheck"
/// } else {
///     3
/// });
///
/// assert_eq!(x, 3);
/// # }
/// ```
#[macro_export]
macro_rules! if_cfg {
    ($cfg:meta $t:block else $f:block) => {
        match_cfg! {
            ($cfg) => $t,
            _ => $f,
        }
    };

    ($cfg:meta $t:block) => {
        if_cfg!($cfg $t else { })
    };
}

/// Helper function that appends extra documentation to [`crate::Builder`] methods that support regular
/// expressions in their input.
macro_rules! regex_option {
    ($(#[$attrs:meta])* pub fn $($tokens:tt)*) => {
        $(#[$attrs])*
        ///
        /// Regular expressions are supported. Check the [regular expression
        /// arguments](./struct.Builder.html#regular-expression-arguments) section and the
        /// [regex](https://docs.rs/regex) crate documentation for further information.
        pub fn $($tokens)*
    };
}

/// Helper macro to set the default value of each option.
///
/// This macro is an internal implementation detail of the `options` macro and should not be used
/// directly.
macro_rules! default {
    () => {
        Default::default()
    };
    ($expr:expr) => {
        $expr
    };
}

/// Helper macro to set the conversion to CLI arguments for each option.
///
/// This macro is an internal implementation detail of the `options` macro and should not be used
/// directly.
macro_rules! as_args {
    ($flag:literal) => {
        |field, args| AsArgs::as_args(field, args, $flag)
    };
    ($expr:expr) => {
        $expr
    };
}

/// Helper function to ignore an option when converting it into CLI arguments.
///
/// This function is only used inside `options` and should not be used in other contexts.
pub(super) fn ignore<T>(_: &T, _: &mut Vec<String>) {}

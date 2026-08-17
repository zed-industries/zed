//! Macros for defining extra assertions that should only be checked in testing
//! and/or CI when the `__testing_only_extra_assertions` feature is enabled.

/// Simple macro that forwards to assert! when using
/// `__testing_only_extra_assertions`.
macro_rules! extra_assert {
    ( $cond:expr ) => {
        if cfg!(feature = "__testing_only_extra_assertions") {
            assert!($cond);
        }
    };
    ( $cond:expr , $( $arg:tt )+ ) => {
        if cfg!(feature = "__testing_only_extra_assertions") {
            assert!($cond, $( $arg )* )
        }
    };
}

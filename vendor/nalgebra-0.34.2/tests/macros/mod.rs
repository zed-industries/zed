mod matrix;
mod stack;

/// Wrapper for `assert_eq` that also asserts that the types are the same
// For some reason, rustfmt totally messes up the formatting of this macro.
// For now we skip, but once https://github.com/rust-lang/rustfmt/issues/6131
// is fixed, we can perhaps remove the skip attribute
#[rustfmt::skip]
macro_rules! assert_eq_and_type {
    ($left:expr, $right:expr $(,)?) => {
        {
            fn check_statically_same_type<T>(_: &T, _: &T) {}
            check_statically_same_type(&$left, &$right);
        }
        assert_eq!($left, $right);
    };
}

pub(crate) use assert_eq_and_type;

//! Macros for writing test suites.

/// Generate a test function `$name` which asserts that `$left` and `$right`
/// are equal.
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate mac;
/// mod test {
/// #   // doesn't actually run the test :/
///     test_eq!(two_and_two_is_four, 2 + 2, 4);
/// }
/// # fn main() { }
/// ```
#[macro_export]
macro_rules! test_eq {
    ($name:ident, $left:expr, $right:expr) => {
        #[test]
        fn $name() {
            assert_eq!($left, $right);
        }
    }
}

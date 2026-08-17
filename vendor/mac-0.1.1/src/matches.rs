//! Pattern Matching macros.

/// Returns true if an expression matches a pattern.
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate mac;
///
/// # fn main() {
/// assert!(matches!(2, 1 | 2 | 3));
/// assert!(matches!('x', 'a' ... 'z'));
/// assert!(!matches!(Some(1), None));
/// assert!(matches!(Some(42), Some(n) if n == 42));
/// # }
/// ```
#[macro_export]
macro_rules! matches {
    ($expr:expr, $($pat:tt)+) => {
        _tt_as_expr_hack! {
            match $expr {
                $($pat)+ => true,
                _        => false
            }
        }
    }
}

/// Work around "error: unexpected token: `an interpolated tt`", whatever that
/// means. (Probably rust-lang/rust#22819.)
#[doc(hidden)]
#[macro_export]
macro_rules! _tt_as_expr_hack {
    ($value:expr) => ($value)
}

#[test]
fn test_matches() {
    let foo = Some("-12");
    assert!(matches!(foo, Some(bar) if
        matches!(bar.as_bytes()[0], b'+' | b'-') &&
        matches!(bar.as_bytes()[1], b'0'... b'9')
    ));
}

#[macro_export]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr, $precision:expr) => {{
        let (a, b) = (&$a, &$b);
        let eps = Decimal::new(1, $precision);
        let abs = (*a - *b).abs();
        assert!(
            abs <= eps,
            "assertion failed: `(left ~= right)` \n   \
             left: `{:?}`,\n  \
             right: `{:?}`,\n \
             expect: `{:?}`,\n   \
             real: `{:?}`",
            *a,
            *b,
            eps,
            abs,
        );
    }};
    ($a:expr, $b:expr, $precision:expr, $($arg:tt)+) => {{
        let (a, b) = (&$a, &$b);
        let eps = Decimal::new(1, $precision);
        let abs = (*a - *b).abs();
        assert!(
            abs <= eps,
            "assertion failed: `(left ~= right)` \n   \
             left: `{:?}`,\n  \
             right: `{:?}`,\n \
             expect: `{:?}`,\n   \
             real: `{:?}`: {}",
            *a,
            *b,
            eps,
            abs,
            format_args!($($arg)+),
        );
    }};
}

#[macro_export]
macro_rules! either {
    ($result:expr, $legacy_result:expr) => {
        if cfg!(feature = "legacy-ops") {
            $legacy_result
        } else {
            $result
        }
    };
}
